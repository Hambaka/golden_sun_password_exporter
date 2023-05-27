mod enums;
mod sav;
mod text;
mod output;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::string::String;
use clap::{arg, ArgAction, ArgGroup, Command, value_parser};

fn main() {
  let matches = Command::new("Golden Sun Password Exporter")
    .version("v0.4.0")
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
            .required(true)
        )
        .arg(
          arg!(
            -a --all "Export all existing save data in save file"
          )
            .action(ArgAction::SetTrue)
            .required(false)
        )
        .arg(
          arg!(
            -t --text <VALUE> "Password version"
          )
            .required(false)
        )
        .arg(
          arg!(
            -m --memory "Generate memory dump file"
          )
            .action(ArgAction::SetTrue)
            .required(false)
        )
        .arg(
          arg!(
            -c --cheat <VALUE> "Generate cheats according to the language version"
          )
            .required(false)
        )
        .arg(
          arg!(
            -e --export "Export game data to a text file for Dyrati's \"Golden Sun Password Generator\" spreadsheet"
          )
            .action(ArgAction::SetTrue)
            .required(false)
        )
        .group(
          ArgGroup::new("sav_args")
            .required(true)
            .args(["text", "memory", "cheat", "export"])
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
            -g --grade <VALUE> "Target password grade"
          )
            .required(false)
        )
        .arg(
          arg!(
            -t --text "Convert password to another version"
          )
            .action(ArgAction::SetTrue)
            .required(false)
        )
        .arg(
          arg!(
            -m --memory "Generate memory dump file"
          )
            .action(ArgAction::SetTrue)
            .required(false)
        )
        .arg(
          arg!(
            -c --cheat <VALUE> "Generate cheats according to the language version"
          )
            .required(false)
        )
        .arg(
          arg!(
            -e --export "Export game data to a text file for Dyrati's \"Golden Sun Password Generator\" spreadsheet"
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
            -g --grade <VALUE> "Target password grade"
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
            -c --cheat <VALUE> "Generate cheats according to the language version"
          )
            .required(false)
        )
        .arg(
          arg!(
            -e --export "Export game data to a text file for Dyrati's \"Golden Sun Password Generator\" spreadsheet"
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
      let check_is_tbs_with_loop_start_index = sav::check_save_type_with_loop_start_index(&raw_save_file);
      let is_tbs_save = check_is_tbs_with_loop_start_index.0;
      if !is_tbs_save {
        eprintln!("It's not a valid Golden Sun 1 save file! Or there is no save data in save file!");
        return;
      }
      let loop_start_index = check_is_tbs_with_loop_start_index.1;

      /* Get save data from save file with slot number.
         Also check if the save data is clear data. */
      let save_data_map = sav::get_raw_save_data(to_export_all_data, &raw_save_file, loop_start_index);
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
        let password_bytes = sav::get_password_bytes_by_raw_save(val.get_data(), target_password_grade);
        // Key is save slot number: 0, 1, 2 -> 1, 2, 3
        let sub_dir_str = output::create_sav_sub_dir(*key + 1, val.get_is_clear(), output_dir_str.as_str());

        if let Some(password_version) = password_version_option {
          output::write_password_text_file(&password_bytes, password_version, sub_dir_str.as_str());
        }
        if to_export_memory_dump {
          output::write_memory_dump_file(&password_bytes, sub_dir_str.as_str());
        }
        if let Some(cheat_version) = cheat_version_option {
          output::write_cheat_file(&password_bytes, cheat_version, sub_dir_str.as_str());
        }
        if to_export_data_text {
          output::write_text_data_file(sav::get_exported_data_for_dyrati_sheet_by_raw_save(val.get_data()).as_str(), sub_dir_str.as_str());
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
        println!("The text file is empty!");
        return;
      }

      // Check its password version and file size.
      let password_version = text::check_password_version(password.as_ref());
      let password_bytes = text::txt_to_dmp(password, password_version);
      if password_bytes.len() != 16 && password_bytes.len() != 61 && password_bytes.len() != 260 {
        println!("Password's length is not valid!");
        return;
      }

      // "grade" argument.
      let mut target_password_grade_option: Option<enums::PasswordGrade> = None;
      let mut is_no_need_to_downgrade = false;
      if let Some(grade) = sub_matches.get_one::<String>("grade") {
        let target_password_grade = enums::get_password_grade_by_arg(grade.as_str());
        target_password_grade_option = Some(target_password_grade);
        let source_password_grade = enums::get_password_grade_by_len(password_bytes.len());

        if sav::get_is_able_to_downgrade(source_password_grade, target_password_grade) {
          is_no_need_to_downgrade = sav::get_is_no_need_to_downgrade(source_password_grade, target_password_grade);
        } else {
          println!("It is not possible to downgrade your password to target password grade!");
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

      let target_password_bytes;
      if let Some(target_password_grade) = target_password_grade_option {
        if is_no_need_to_downgrade {
          target_password_bytes = password_bytes;
        } else {
          target_password_bytes = sav::get_password_bytes_by_password_bytes(&password_bytes, target_password_grade);
        }
      } else {
        target_password_bytes = password_bytes;
      }

      // Write files.
      if to_convert_password {
        output::write_password_text_file(&target_password_bytes, enums::rev_password_version(password_version), output_dir_str.as_str());
      } else if let Some(..) = target_password_grade_option {
        if !is_no_need_to_downgrade {
          output::write_password_text_file(&target_password_bytes, password_version, output_dir_str.as_str());
        }
      }
      if to_export_memory_dump {
        output::write_memory_dump_file(&target_password_bytes, output_dir_str.as_str());
      }
      if let Some(cheat_version) = cheat_version_option {
        output::write_cheat_file(&target_password_bytes, cheat_version, output_dir_str.as_str());
      }
      if to_export_data_text {
        output::write_text_data_file(sav::get_exported_data_for_dyrati_sheet_by_bytes(&target_password_bytes).as_str(), output_dir_str.as_str());
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

      // "grade" argument.
      let mut target_password_grade_option: Option<enums::PasswordGrade> = None;
      let mut is_no_need_to_downgrade = false;
      if let Some(grade) = sub_matches.get_one::<String>("grade") {
        let target_password_grade = enums::get_password_grade_by_arg(grade.as_str());
        target_password_grade_option = Some(target_password_grade);
        let source_password_grade = enums::get_password_grade_by_len(password_bytes.len());

        if sav::get_is_able_to_downgrade(source_password_grade, target_password_grade) {
          is_no_need_to_downgrade = sav::get_is_no_need_to_downgrade(source_password_grade, target_password_grade);
        } else {
          println!("It is not possible to downgrade your password to target password grade!");
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

      let target_password_bytes;
      if let Some(target_password_grade) = target_password_grade_option {
        if is_no_need_to_downgrade {
          target_password_bytes = password_bytes;
        } else {
          target_password_bytes = sav::get_password_bytes_by_password_bytes(&password_bytes, target_password_grade);
        }
      } else {
        target_password_bytes = password_bytes;
      }

      // Write files.
      if let Some(..) = target_password_grade_option {
        if !is_no_need_to_downgrade {
          output::write_memory_dump_file(&target_password_bytes, output_dir_str.as_str());
        }
      }
      if let Some(password_version) = password_version_option {
        output::write_password_text_file(&target_password_bytes, password_version, output_dir_str.as_str());
      }
      if let Some(cheat_version) = cheat_version_option {
        output::write_cheat_file(&target_password_bytes, cheat_version, output_dir_str.as_str());
      }
      if to_export_data_text {
        output::write_text_data_file(sav::get_exported_data_for_dyrati_sheet_by_bytes(&target_password_bytes).as_str(), output_dir_str.as_str());
      }
    }
    _ => unreachable!(),
  }
}
