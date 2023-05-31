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
  let matches = Command::new("Golden Sun Password Exporter")
    .version("v0.4.1")
    .author("Hambaka")
    .about("A simple tool for a GBA game called Golden Sun\nYou can use this tool to export Golden Sun password to a text file/memory dump binary file/cheat file")
    .allow_negative_numbers(true)
    .propagate_version(true)
    .subcommand_required(true)
    .arg_required_else_help(true)
    // "sav" subcommand.
    .subcommand(
      Command::new("sav")
        .about("Read a save file to generate password text/memory dump binary/cheat files")
        .arg(
          arg!(
            <INPUT_FILE> "Golden Sun save file"
          )
            .value_parser(value_parser!(PathBuf))
            .required(true)
        )
        .arg(
          arg!(
            -g --grade <VALUE> "Target password grade"
          )
            .required(false)
            .default_value("g")
        )
        .arg(
          arg!(
            -a --all "Export all existing save data in the save file"
          )
            .action(ArgAction::SetTrue)
            .required(false)
        )
        .arg(
          arg!(
            -t --text <VALUE> "Generate password text file"
          )
            .required(false)
        )
        .arg(
          arg!(
            -m --memory "Generate password memory dump binary file"
          )
            .action(ArgAction::SetTrue)
            .required(false)
        )
        .arg(
          arg!(
            -c --cheat <VALUE> "Generate password cheat codes for Golden Sun: The Lost Age"
          )
            .required(false)
        )
        .arg(
          arg!(
            -e --export "Export data to a text file, which can be used in Dyrati's \"Golden Sun Password Generator\" spreadsheet"
          )
            .action(ArgAction::SetTrue)
            .required(false)
        )
        .group(
          ArgGroup::new("sav_args")
            .required(true)
            .args(["grade", "text", "memory", "cheat", "export"])
            .multiple(true)
        )
        .arg(
          arg!(
            -o --output <OUTPUT_DIR> "Output directory"
          )
            .value_parser(value_parser!(PathBuf))
            .required(false)
        ),
    )
    // "txt" subcommand.
    .subcommand(
      Command::new("txt")
        .about("Read a password text file to generate an another version password text/memory dump binary/cheat file")
        .arg(
          arg!(
            <INPUT_FILE> "Golden Sun password text file"
          )
            .value_parser(value_parser!(PathBuf))
            .required(true)
        )
        .arg(
          arg!(
            -g --grade <VALUE> "Target password grade (only for downgrade)"
          )
            .required(false)
        )
        .arg(
          arg!(
            -t --text "Generate another version password text file"
          )
            .action(ArgAction::SetTrue)
            .required(false)
        )
        .arg(
          arg!(
            -m --memory "Generate password memory dump binary file"
          )
            .action(ArgAction::SetTrue)
            .required(false)
        )
        .arg(
          arg!(
            -c --cheat <VALUE> "Generate password cheat codes for Golden Sun: The Lost Age"
          )
            .required(false)
        )
        .arg(
          arg!(
            -e --export "Export data to a text file, which can be used in Dyrati's \"Golden Sun Password Generator\" spreadsheet"
          )
            .action(ArgAction::SetTrue)
            .required(false)
        )
        .group(
          ArgGroup::new("txt_args")
            .required(true)
            .args(["grade", "text", "memory", "cheat", "export"])
            .multiple(true)
        )
        .arg(
          arg!(
            -o --output <OUTPUT_DIR> "Output directory"
          )
            .value_parser(value_parser!(PathBuf))
            .required(false)
        ),
    )
    // "dmp" subcommand.
    .subcommand(
      Command::new("dmp")
        .about("Read a password memory dump binary file to generate a password text/cheat file")
        .arg(
          arg!(
            <INPUT_FILE> "Golden Sun password memory dump binary file"
          )
            .value_parser(value_parser!(PathBuf))
            .required(true)
        )
        .arg(
          arg!(
            -g --grade <VALUE> "Target password grade (only for downgrade)"
          )
            .required(false)
        )
        .arg(
          arg!(
            -t --text <VALUE> "Generate password text file"
          )
            .required(false)
        )
        .arg(
          arg!(
            -c --cheat <VALUE> "Generate password cheat codes for Golden Sun: The Lost Age"
          )
            .required(false)
        )
        .arg(
          arg!(
            -e --export "Export data to a text file, which can be used in Dyrati's \"Golden Sun Password Generator\" spreadsheet"
          )
            .action(ArgAction::SetTrue)
            .required(false)
        )
        .group(
          ArgGroup::new("dmp_args")
            .required(true)
            .args(["grade", "text", "cheat", "export"])
            .multiple(true)
        )
        .arg(
          arg!(
            -o --output <OUTPUT_DIR> "Output directory"
          )
            .value_parser(value_parser!(PathBuf))
            .required(false)
        ),
    )
    .get_matches();

  match matches.subcommand() {
    // "sav" subcommand.
    Some(("sav", sub_matches)) => {
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

      /* "all" flag.
         Default value is "false", only export password from clear save data.
         Set it to "true" will export password from all existing save data in save file,
         even it is not a clear data. */
      let to_export_all_data = sub_matches.get_flag("all");

      let mut raw_save_file = Vec::new();
      input_file.read_to_end(&mut raw_save_file).unwrap();

      // Check if it is GS1(TBS) save file, also get loop start index.
      let (is_tbs_save, loop_start_index) = sav::check_is_tbs_sav_and_get_loop_start_index(&raw_save_file);
      if !is_tbs_save {
        eprintln!("It's not a valid Golden Sun 1 save file! Or there is no save data in save file!");
        return;
      }

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

      // "grade" argument.
      let grade = sub_matches.get_one::<String>("grade").unwrap();
      let target_password_grade = enums::get_password_grade_by_arg(grade.as_str());

      // "text" argument.
      let mut password_version_option: Option<enums::PasswordVersion> = None;
      if let Some(text) = sub_matches.get_one::<String>("text") {
        password_version_option = Some(enums::get_password_version(text.as_str()));
      }

      // "memory" flag.
      let to_export_memory_dump = sub_matches.get_flag("memory");

      // "cheat" argument.
      let mut cheat_version_option: Option<enums::CheatVersion> = None;
      if let Some(cheat) = sub_matches.get_one::<String>("cheat") {
        cheat_version_option = Some(enums::get_cheat_version(cheat.as_str()));
      }

      // "export" flag.
      let to_export_data_text = sub_matches.get_flag("export");

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
        if to_export_memory_dump {
          output::write_memory_dump_file(&password_bytes, sub_dir_str.as_str());
        }
        if let Some(cheat_version) = cheat_version_option {
          output::write_cheat_file(&password_bytes, cheat_version, sub_dir_str.as_str());
        }
        if to_export_data_text {
          if matches!(target_password_grade, enums::PasswordGrade::Gold) {
            output::write_text_data_file(sav::get_exported_data_for_dyrati_sheet_with_raw_save_bytes(val.get_data()).as_str(), sub_dir_str.as_str());
          } else {
            output::write_text_data_file(sav::get_exported_data_for_dyrati_sheet_by_bytes(password_bytes.as_slice(), target_password_grade).as_str(), sub_dir_str.as_str());
          }
        }
      }
    }
    // "txt" subcommand.
    Some(("txt", sub_matches)) => {
      // Read password text file.
      let txt_input_path = sub_matches.get_one::<PathBuf>("INPUT_FILE").unwrap();
      let mut input_file = File::open(txt_input_path).expect("An error occurred while opening save file!");
      let mut password = String::new();
      input_file.read_to_string(&mut password).unwrap();

      // If the text file is empty, exit.
      if password.is_empty() {
        eprintln!("The text file is empty!");
        return;
      }

      // Check its password version and its character count.
      let password_version = convert::get_password_version(password.as_ref());
      // Remove all line break and whitespace.
      let password_without_whitespace = convert::remove_whitespace(password.as_ref());
      // Check password's length.
      if password_without_whitespace.chars().count() != 16 && password_without_whitespace.chars().count() != 61 && password_without_whitespace.chars().count() != 260 {
        eprintln!("Password's length is not valid!");
        return;
      }
      // Check if it contains invalid char
      let has_invalid_char = match password_version {
        enums::PasswordVersion::Japanese => convert::contains_invalid_char_jp(password_without_whitespace.as_str()),
        enums::PasswordVersion::English => convert::contains_invalid_char_en(password_without_whitespace.as_str()),
      };
      if has_invalid_char {
        eprintln!("Password contains invalid character!");
        return;
      }

      // Check if the password is valid;
      let password_bytes = convert::convert_txt_to_dmp(password_without_whitespace.as_str(), password_version);
      let mut password_bits;
      if let Some(bits) = sav::get_valid_password_bits_option(password_bytes.as_slice(), true) {
        password_bits = bits;
      } else {
        eprintln!("Password is invalid!");
        return;
      }

      // "grade" argument.
      let source_password_grade = enums::get_password_grade_by_bytes_len(password_bytes.len());
      let mut target_password_grade_option: Option<enums::PasswordGrade> = None;
      let mut is_no_need_to_downgrade = false;
      if let Some(grade) = sub_matches.get_one::<String>("grade") {
        let target_password_grade = enums::get_password_grade_by_arg(grade.as_str());
        target_password_grade_option = Some(target_password_grade);
        if sav::can_downgrade(source_password_grade, target_password_grade) {
          is_no_need_to_downgrade = sav::no_need_to_downgrade(source_password_grade, target_password_grade);
        } else {
          eprintln!("It is not possible to downgrade your password to target password grade!");
          return;
        }
      }

      // "text" flag.
      let to_convert_password = sub_matches.get_flag("text");

      // "memory" flag.
      let to_export_memory_dump = sub_matches.get_flag("memory");

      // "cheat" argument.
      let mut cheat_version_option: Option<enums::CheatVersion> = None;
      if let Some(cheat) = sub_matches.get_one::<String>("cheat") {
        cheat_version_option = Some(enums::get_cheat_version(cheat.as_str()));
      }

      // "export" flag.
      let to_export_data_text = sub_matches.get_flag("export");

      // Create output directory.
      let output_dir_str;
      if let Some(output_path_buf) = sub_matches.get_one::<PathBuf>("output") {
        output_dir_str = output::create_output_dir(output_path_buf, true);
      } else {
        output_dir_str = output::create_output_dir(txt_input_path, false);
      }

      let save_data = sav::get_save_data_with_password_bits(&mut password_bits, source_password_grade);
      let target_password_bytes;
      if let Some(target_password_grade) = target_password_grade_option {
        if is_no_need_to_downgrade {
          target_password_bytes = password_bytes;
        } else {
          target_password_bytes = sav::get_password_bytes_with_save_data(&save_data, target_password_grade);
        }
      } else {
        target_password_bytes = password_bytes;
      }

      // Write files.
      if to_convert_password {
        if target_password_grade_option.is_none() {
          output::write_converted_password_text_file(convert::convert_txt(&password_without_whitespace, password_version).as_str(), output_dir_str.as_str());
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
            output::write_text_data_file(sav::get_exported_data_for_dyrati_sheet_by_save_data(&save_data).as_str(), output_dir_str.as_str());
          } else {
            output::write_text_data_file(sav::get_exported_data_for_dyrati_sheet_by_bytes(target_password_bytes.as_slice(), target_password_grade).as_str(), output_dir_str.as_str());
          }
        } else {
          output::write_text_data_file(sav::get_exported_data_for_dyrati_sheet_by_save_data(&save_data).as_str(), output_dir_str.as_str());
        }
      }
    }
    // "dmp" subcommand
    Some(("dmp", sub_matches)) => {
      // Read password memory dump file.
      let dmp_input_path = sub_matches.get_one::<PathBuf>("INPUT_FILE").unwrap();
      let mut input_file = File::open(dmp_input_path).expect("An error occurred while opening save file!");

      // Check its file size.
      let file_size = input_file.metadata().unwrap().len();
      if file_size != 16 && file_size != 61 && file_size != 260 {
        eprintln!("The size of password memory dump file is not valid!");
        return;
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

      // "grade" argument.
      let source_password_grade = enums::get_password_grade_by_bytes_len(password_bytes.len());
      let mut target_password_grade_option: Option<enums::PasswordGrade> = None;
      let mut is_no_need_to_downgrade = false;
      if let Some(grade) = sub_matches.get_one::<String>("grade") {
        let target_password_grade = enums::get_password_grade_by_arg(grade.as_str());
        target_password_grade_option = Some(target_password_grade);
        if sav::can_downgrade(source_password_grade, target_password_grade) {
          is_no_need_to_downgrade = sav::no_need_to_downgrade(source_password_grade, target_password_grade);
        } else {
          eprintln!("It is not possible to downgrade your password to target password grade!");
          return;
        }
      }

      // "text" argument.
      let mut password_version_option: Option<enums::PasswordVersion> = None;
      if let Some(text) = sub_matches.get_one::<String>("text") {
        password_version_option = Some(enums::get_password_version(text.as_str()));
      }

      // "cheat" argument.
      let mut cheat_version_option: Option<enums::CheatVersion> = None;
      if let Some(cheat) = sub_matches.get_one::<String>("cheat") {
        cheat_version_option = Some(enums::get_cheat_version(cheat.as_str()));
      }

      // "export" flag.
      let to_export_data_text = sub_matches.get_flag("export");

      // Create output directory.
      let output_dir_str;
      if let Some(output_path_buf) = sub_matches.get_one::<PathBuf>("output") {
        output_dir_str = output::create_output_dir(output_path_buf, true);
      } else {
        output_dir_str = output::create_output_dir(dmp_input_path, false);
      }

      let save_data = sav::get_save_data_with_password_bits(&mut password_bits, source_password_grade);
      let target_password_bytes;
      if let Some(target_password_grade) = target_password_grade_option {
        if is_no_need_to_downgrade {
          target_password_bytes = password_bytes;
        } else {
          target_password_bytes = sav::get_password_bytes_with_save_data(&save_data, target_password_grade);
        }
      } else {
        target_password_bytes = password_bytes;
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
            output::write_text_data_file(sav::get_exported_data_for_dyrati_sheet_by_save_data(&save_data).as_str(), output_dir_str.as_str());
          } else {
            output::write_text_data_file(sav::get_exported_data_for_dyrati_sheet_by_bytes(target_password_bytes.as_slice(), target_password_grade).as_str(), output_dir_str.as_str());
          }
        } else {
          output::write_text_data_file(sav::get_exported_data_for_dyrati_sheet_by_save_data(&save_data).as_str(), output_dir_str.as_str());
        }
      }
    }
    _ => unreachable!(),
  }
}
