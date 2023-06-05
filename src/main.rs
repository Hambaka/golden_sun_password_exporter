mod enums;
mod sav;
mod convert;
mod output;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::string::String;
use clap::{arg, ArgAction, ArgGroup, Command, value_parser};

fn main() {
  // "long_about" looks weird?
  let mut about_string = String::new();
  about_string.push_str("A simple tool for a GBA game called Golden Sun.\n\n");
  about_string.push_str("You can use this tool to export password data to the following types of files:\n");
  about_string.push_str("1. Password text file. (Japanese, English)\n");
  about_string.push_str("2. Password memory dump binary file.\n");
  about_string.push_str("3. Password cheat codes text file for Golden Sun: The Lost Age. (Japan, USA/Europe, Germany, Spain, France, Italy)\n");
  about_string.push_str("4. Save data text file, which can be used in Dyrati's \"Golden Sun Password Generator\" spreadsheet.");

  let matches = Command::new("Golden Sun Password Exporter")
    .version("v0.4.4")
    .author("Hambaka")
    .about(about_string)
    .allow_negative_numbers(true)
    .propagate_version(true)
    .subcommand_required(true)
    .arg_required_else_help(true)
    .subcommand(
      Command::new("sav")
        .about("Export password data by reading a Golden Sun save file")
        .args(&[
          arg!(<INPUT_FILE> "Golden Sun save file").value_parser(value_parser!(PathBuf)).required(true),
          arg!(-g --grade <VALUE> "Target password grade").default_value("g"),
          arg!(-a --all "Export all existing valid save data in the save file").action(ArgAction::SetTrue),
          arg!(-t --text <VALUE> "Generate the specified version password text file"),
          arg!(-m --memory "Generate password memory dump binary file").action(ArgAction::SetTrue),
          arg!(-c --cheat <VALUE> "Generate the specified version password cheat codes text file"),
          arg!(-e --export "Export save data to a text file for Dyrati's \"Golden Sun Password Generator\"").action(ArgAction::SetTrue),
          arg!(-o --output <OUTPUT_DIR> "Output directory").value_parser(value_parser!(PathBuf))
        ])
        .group(ArgGroup::new("sav_args")
          .args(["text", "memory", "cheat", "export"])
          .required(true)
          .multiple(true)
        )
    )
    .subcommand(
      Command::new("txt")
        .about("Export password data by reading a Golden Sun password text file")
        .args(&[
          arg!(<INPUT_FILE> "Golden Sun password text file").value_parser(value_parser!(PathBuf)).required(true),
          arg!(-g --grade <VALUE> "Target password grade (for downgrade only)"),
          arg!(-t --text "Convert password to another version and generate the converted file").action(ArgAction::SetTrue),
          arg!(-m --memory "Generate password memory dump binary file").action(ArgAction::SetTrue),
          arg!(-c --cheat <VALUE> "Generate the specified version password cheat codes text file"),
          arg!(-e --export "Generate and export save data to a text file for Dyrati's \"Golden Sun Password Generator\"").action(ArgAction::SetTrue),
          arg!(-o --output <OUTPUT_DIR> "Output directory").value_parser(value_parser!(PathBuf))
        ])
        .group(ArgGroup::new("txt_args")
          .args(["grade", "text", "memory", "cheat", "export"])
          .required(true)
          .multiple(true)
        )
    )
    .subcommand(
      Command::new("dmp")
        .about("Export password data by reading a Golden Sun password memory dump binary file")
        .args(&[
          arg!(<INPUT_FILE> "Golden Sun password memory dump binary file").value_parser(value_parser!(PathBuf)).required(true),
          arg!(-g --grade <VALUE> "Target password grade (for downgrade only)"),
          arg!(-t --text <VALUE> "Generate the specified version password text file"),
          arg!(-c --cheat <VALUE> "Generate the specified version password cheat codes text file"),
          arg!(-e --export "Generate and export save data to a text file for Dyrati's \"Golden Sun Password Generator\"").action(ArgAction::SetTrue),
          arg!(-o --output <OUTPUT_DIR> "Output directory").value_parser(value_parser!(PathBuf))
        ])
        .group(ArgGroup::new("dmp_args")
          .args(["grade", "text", "cheat", "export"])
          .required(true)
          .multiple(true)
        )
    )
    .get_matches();

  match matches.subcommand() {
    // "sav" subcommand.
    Some(("sav", sub_matches)) => {
      // We should check the validity of the values of "grade", "text" and "cheat" arguments first.
      // "grade" argument.
      let grade = sub_matches.get_one::<String>("grade").unwrap();
      let target_password_grade;
      if let Some(password_grade) = enums::get_password_grade_by_arg(grade.as_str()) {
        target_password_grade = password_grade;
      } else {
        print_grade_arg_message();
        return;
      };

      // "text" argument.
      let mut password_version_option: Option<enums::PasswordVersion> = None;
      if let Some(text) = sub_matches.get_one::<String>("text") {
        if let Some(password_version) = enums::get_password_version(text.as_str()) {
          password_version_option = Some(password_version);
        } else {
          print_text_arg_message();
          return;
        };
      }

      // "cheat" argument.
      let mut cheat_version_option: Option<enums::CheatVersion> = None;
      if let Some(cheat) = sub_matches.get_one::<String>("cheat") {
        if let Some(cheat_version) = enums::get_cheat_version(cheat.as_str()) {
          cheat_version_option = Some(cheat_version);
        } else {
          print_cheat_arg_message();
          return;
        };
      }

      // Read save file.
      let sav_input_path = sub_matches.get_one::<PathBuf>("INPUT_FILE").unwrap();
      let mut input_file = File::open(sav_input_path).expect("An error occurred while opening save file!");
      /* Check the size of save file.
         The size of save file should be 64KB,
         though the .SaveRAM file created by Bizhawk is 128KB.
         Even its size is 128KB, seems it only use first 64KB space to store save data. */
      let file_size = input_file.metadata().unwrap().len();
      if file_size != 0x10000 && file_size != 0x20000 {
        eprintln!("The size of save file is not valid!");
        return;
      }
      let mut raw_save_file = Vec::new();
      input_file.read_to_end(&mut raw_save_file).unwrap();

      // Check if it is GS1(TBS) save file, also get loop start index.
      let loop_start_index;
      if let Some(index) = sav::get_loop_start_index_option(&raw_save_file) {
        loop_start_index = index;
      } else {
        eprintln!("It's not a valid Golden Sun 1 save file! Or there is no save data in save file!");
        return;
      };

      /* "all" flag.
         Default value is "false", only export password from clear save data.
         Set it to "true" will export password from all existing save data in save file,
         even it is not a clear data. */
      let to_export_all_data = sub_matches.get_flag("all");
      /* Get save data from save file with slot number.
         Also check if the save data is clear data. */
      let save_data_map = sav::get_raw_save_data_map(to_export_all_data, &raw_save_file, loop_start_index);
      if save_data_map.is_empty() {
        if to_export_all_data {
          eprintln!("There is no save data in save file!");
        } else {
          eprintln!("There is no clear data in save file!");
        }
        return;
      }

      // Create main output directory.
      let output_dir_str;
      if let Some(output_path_buf) = sub_matches.get_one::<PathBuf>("output") {
        output_dir_str = output::create_output_dir(output_path_buf, true);
      } else {
        output_dir_str = output::create_output_dir(sav_input_path, false);
      }

      // Write files.
      for (key, val) in &save_data_map {
        let password_bytes = sav::get_password_bytes_with_raw_save_bytes(val.get_data(), target_password_grade);
        // Key is save slot number: 0, 1, 2 -> 1, 2, 3
        let sub_dir_str = output::create_sav_sub_dir(*key + 1, val.get_is_clear(), output_dir_str.as_str());

        if let Some(password_version) = password_version_option {
          output::write_password_text_file_with_bytes(&password_bytes, password_version, sub_dir_str.as_str());
        }

        if sub_matches.get_flag("memory") {
          output::write_memory_dump_file(&password_bytes, sub_dir_str.as_str());
        }

        if let Some(cheat_version) = cheat_version_option {
          output::write_cheat_file(&password_bytes, cheat_version, sub_dir_str.as_str());
        }

        if sub_matches.get_flag("export") {
          if matches!(target_password_grade, enums::PasswordGrade::Gold) {
            output::write_game_data_text_file(sav::get_exported_data_for_dyrati_sheet_with_raw_save_bytes(val.get_data()).as_str(), sub_dir_str.as_str());
          } else {
            output::write_game_data_text_file(sav::get_exported_data_for_dyrati_sheet_by_bytes(password_bytes.as_slice(), target_password_grade).as_str(), sub_dir_str.as_str());
          }
        }
      }
    }
    // "txt" subcommand.
    Some(("txt", sub_matches)) => {
      // We should check the validity of "grade" and "cheat" arguments first.
      // "grade" argument.
      let mut target_password_grade_option: Option<enums::PasswordGrade> = None;
      if let Some(grade) = sub_matches.get_one::<String>("grade") {
        let target_password_grade;
        if let Some(password_grade) = enums::get_password_grade_by_arg(grade.as_str()) {
          target_password_grade = password_grade;
        } else {
          print_grade_arg_message();
          return;
        };
        target_password_grade_option = Some(target_password_grade);
      }

      // "cheat" argument.
      let mut cheat_version_option: Option<enums::CheatVersion> = None;
      if let Some(cheat) = sub_matches.get_one::<String>("cheat") {
        if let Some(cheat_version) = enums::get_cheat_version(cheat.as_str()) {
          cheat_version_option = Some(cheat_version);
        } else {
          print_cheat_arg_message();
          return;
        };
      }

      // Read password text file.
      let txt_input_path = sub_matches.get_one::<PathBuf>("INPUT_FILE").unwrap();
      let mut input_file = File::open(txt_input_path).expect("An error occurred while opening password text file!");
      let mut password = String::new();
      input_file.read_to_string(&mut password).unwrap();
      // If the text file is empty, exit.
      if password.is_empty() {
        eprintln!("The text file is empty!");
        return;
      }
      // Remove all line break and whitespace.
      let password_without_whitespace = convert::remove_whitespace(password.as_ref());
      // Check password's length, and get its grade.
      let chars_count = password_without_whitespace.chars().count();
      let source_password_grade = match chars_count {
        260 => enums::PasswordGrade::Gold,
        61 => enums::PasswordGrade::Silver,
        16 => enums::PasswordGrade::Bronze,
        _ => {
          eprintln!("Password's length is not valid!");
          return;
        }
      };

      // Check if it is possible to downgrade password
      let mut is_no_need_to_downgrade = true;
      if let Some(target_password_grade) = target_password_grade_option {
        if sav::can_downgrade(source_password_grade, target_password_grade) {
          is_no_need_to_downgrade = sav::no_need_to_downgrade(source_password_grade, target_password_grade);
        } else {
          eprintln!("It is not possible to downgrade your password to target password grade!");
          return;
        }
      }

      // Check its password version.
      let password_version = convert::get_password_version(password_without_whitespace.as_ref());
      // Check if it contains invalid char
      let has_invalid_char = match password_version {
        enums::PasswordVersion::Japanese => convert::contains_invalid_char_jp(password_without_whitespace.as_str()),
        enums::PasswordVersion::English => convert::contains_invalid_char_en(password_without_whitespace.as_str()),
      };
      if has_invalid_char {
        eprintln!("Password contains invalid character!");
        return;
      }

      // Check if the password is valid.
      let password_bytes = convert::txt_to_dmp(password_without_whitespace.as_str(), password_version);
      let mut password_bits;
      if let Some(bits) = sav::get_valid_password_bits_option(password_bytes.as_slice(), true) {
        password_bits = bits;
      } else {
        eprintln!("Password is invalid!");
        return;
      }

      // Get flags
      let to_convert_password = sub_matches.get_flag("text");
      let to_export_memory_dump = sub_matches.get_flag("memory");
      let to_export_data_text = sub_matches.get_flag("export");

      // Generate save data, and set target password bytes
      let save_data = sav::get_save_data_with_password_bits(&mut password_bits, source_password_grade);
      let target_password_bytes;
      if let Some(target_password_grade) = target_password_grade_option {
        if is_no_need_to_downgrade {
          if !to_convert_password && !to_export_memory_dump && cheat_version_option.is_none() && !to_export_data_text {
            eprintln!("There is no need to downgrade input file, and seems we don't need to generate and export any file...");
            return;
          } else {
            target_password_bytes = password_bytes;
          }
        } else {
          target_password_bytes = sav::get_password_bytes_with_save_data(&save_data, target_password_grade);
        }
      } else {
        target_password_bytes = password_bytes;
      }

      // Create output directory.
      let output_dir_str;
      if let Some(output_path_buf) = sub_matches.get_one::<PathBuf>("output") {
        output_dir_str = output::create_output_dir(output_path_buf, true);
      } else {
        output_dir_str = output::create_output_dir(txt_input_path, false);
      }

      // Write files.
      if to_convert_password {
        if target_password_grade_option.is_none() {
          output::write_converted_password_text_file(convert::txt_to_another_version(&password_without_whitespace, password_version).as_str(), enums::rev_password_version(password_version), output_dir_str.as_str());
        } else {
          output::write_password_text_file_with_bytes(&target_password_bytes, enums::rev_password_version(password_version), output_dir_str.as_str());
        }
      } else if target_password_grade_option.is_some() && !is_no_need_to_downgrade {
        output::write_password_text_file_with_bytes(&target_password_bytes, password_version, output_dir_str.as_str());
      }

      if to_export_memory_dump {
        output::write_memory_dump_file(&target_password_bytes, output_dir_str.as_str());
      }

      if let Some(cheat_version) = cheat_version_option {
        output::write_cheat_file(&target_password_bytes, cheat_version, output_dir_str.as_str());
      }

      if to_export_data_text {
        if let Some(target_password_grade) = target_password_grade_option {
          if is_no_need_to_downgrade {
            output::write_game_data_text_file(sav::get_exported_data_for_dyrati_sheet_by_save_data(&save_data).as_str(), output_dir_str.as_str());
          } else {
            output::write_game_data_text_file(sav::get_exported_data_for_dyrati_sheet_by_bytes(target_password_bytes.as_slice(), target_password_grade).as_str(), output_dir_str.as_str());
          }
        } else {
          output::write_game_data_text_file(sav::get_exported_data_for_dyrati_sheet_by_save_data(&save_data).as_str(), output_dir_str.as_str());
        }
      }
    }
    // "dmp" subcommand
    Some(("dmp", sub_matches)) => {
      // We should check the validity of "grade", "text" and "cheat" arguments first.
      // "grade" argument.
      let mut target_password_grade_option: Option<enums::PasswordGrade> = None;
      if let Some(grade) = sub_matches.get_one::<String>("grade") {
        let target_password_grade;
        if let Some(password_grade) = enums::get_password_grade_by_arg(grade.as_str()) {
          target_password_grade = password_grade;
        } else {
          print_grade_arg_message();
          return;
        };
        target_password_grade_option = Some(target_password_grade);
      }

      // "text" argument.
      let mut password_version_option: Option<enums::PasswordVersion> = None;
      if let Some(text) = sub_matches.get_one::<String>("text") {
        if let Some(password_version) = enums::get_password_version(text.as_str()) {
          password_version_option = Some(password_version);
        } else {
          print_text_arg_message();
          return;
        };
      }

      // "cheat" argument.
      let mut cheat_version_option: Option<enums::CheatVersion> = None;
      if let Some(cheat) = sub_matches.get_one::<String>("cheat") {
        if let Some(cheat_version) = enums::get_cheat_version(cheat.as_str()) {
          cheat_version_option = Some(cheat_version);
        } else {
          print_cheat_arg_message();
          return;
        };
      }

      // Read password memory dump file.
      let dmp_input_path = sub_matches.get_one::<PathBuf>("INPUT_FILE").unwrap();
      let mut input_file = File::open(dmp_input_path).expect("An error occurred while opening memory dump file!");
      // Check its file size, and get its grade.
      let file_size = input_file.metadata().unwrap().len();
      let source_password_grade = match file_size {
        260 => enums::PasswordGrade::Gold,
        61 => enums::PasswordGrade::Silver,
        16 => enums::PasswordGrade::Bronze,
        _ => {
          eprintln!("The size of password memory dump file is not valid!");
          return;
        }
      };

      // Check if it is possible to downgrade password
      let mut is_no_need_to_downgrade = true;
      if let Some(target_password_grade) = target_password_grade_option {
        if sav::can_downgrade(source_password_grade, target_password_grade) {
          is_no_need_to_downgrade = sav::no_need_to_downgrade(source_password_grade, target_password_grade);
        } else {
          eprintln!("It is not possible to downgrade your password to target password grade!");
          return;
        }
      }

      let mut password_bytes = Vec::new();
      input_file.read_to_end(&mut password_bytes).unwrap();
      // Check if it has contains invalid byte
      let has_invalid_byte = convert::contains_invalid_byte(password_bytes.as_slice());
      if has_invalid_byte {
        eprintln!("Password contains invalid byte!");
        return;
      }

      // Check if the password is valid;
      let mut password_bits;
      if let Some(bits) = sav::get_valid_password_bits_option(password_bytes.as_slice(), true) {
        password_bits = bits;
      } else {
        eprintln!("Password is invalid!");
        return;
      }

      // Get flag
      let to_export_data_text = sub_matches.get_flag("export");

      // Generate save data, and set target password bytes
      let save_data = sav::get_save_data_with_password_bits(&mut password_bits, source_password_grade);
      let target_password_bytes;
      if let Some(target_password_grade) = target_password_grade_option {
        if is_no_need_to_downgrade {
          if password_version_option.is_none() && cheat_version_option.is_none() && !to_export_data_text {
            eprintln!("There is no need to downgrade input file, and seems we don't need to generate and export any file...");
            return;
          } else {
            target_password_bytes = password_bytes;
          }
        } else {
          target_password_bytes = sav::get_password_bytes_with_save_data(&save_data, target_password_grade);
        }
      } else {
        target_password_bytes = password_bytes;
      }

      // Create output directory.
      let output_dir_str;
      if let Some(output_path_buf) = sub_matches.get_one::<PathBuf>("output") {
        output_dir_str = output::create_output_dir(output_path_buf, true);
      } else {
        output_dir_str = output::create_output_dir(dmp_input_path, false);
      }

      // Write files.
      if target_password_grade_option.is_some() && !is_no_need_to_downgrade {
        output::write_memory_dump_file(&target_password_bytes, output_dir_str.as_str());
      }

      if let Some(password_version) = password_version_option {
        output::write_password_text_file_with_bytes(&target_password_bytes, password_version, output_dir_str.as_str());
      }

      if let Some(cheat_version) = cheat_version_option {
        output::write_cheat_file(&target_password_bytes, cheat_version, output_dir_str.as_str());
      }

      if to_export_data_text {
        if let Some(target_password_grade) = target_password_grade_option {
          if is_no_need_to_downgrade {
            output::write_game_data_text_file(sav::get_exported_data_for_dyrati_sheet_by_save_data(&save_data).as_str(), output_dir_str.as_str());
          } else {
            output::write_game_data_text_file(sav::get_exported_data_for_dyrati_sheet_by_bytes(target_password_bytes.as_slice(), target_password_grade).as_str(), output_dir_str.as_str());
          }
        } else {
          output::write_game_data_text_file(sav::get_exported_data_for_dyrati_sheet_by_save_data(&save_data).as_str(), output_dir_str.as_str());
        }
      }
    }
    _ => unreachable!(),
  }
}

fn print_grade_arg_message() {
  eprintln!("Please input a valid password grade!");
  eprintln!("Available values: g, s, b");
  eprintln!("g: Gold, s: Silver, b: Bronze");
  eprintln!("Example: -g g");
}

fn print_text_arg_message() {
  eprintln!("Please input a valid password version!");
  eprintln!("Available values: j, e");
  eprintln!("j: Japanese, e: English");
  eprintln!("Example: -t e");
}

fn print_cheat_arg_message() {
  eprintln!("Please input a valid cheat version!");
  eprintln!("Available values: j, u, e, g, s, f, i");
  eprintln!("j: Japan,   u: USA/Europe, e: USA/Europe");
  eprintln!("g: Germany, s: Spanish,    f: France,    i: Italy");
  eprintln!("Example: -c u");
}
