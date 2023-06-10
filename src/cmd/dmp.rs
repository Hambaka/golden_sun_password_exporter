use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use clap::ArgMatches;
use crate::util::save;
use crate::util::output;
use crate::util::convert;
use crate::util::enums::{CheatVersion, PasswordGrade, PasswordVersion};

pub fn run(sub_matches: &ArgMatches) {
  // Read password memory dump file.
  let dmp_input_path = sub_matches.get_one::<PathBuf>("INPUT_FILE").unwrap();
  let mut input_file = File::open(dmp_input_path).expect("An error occurred while opening memory dump file!");
  // Check its file size, and get its grade.
  let file_size = input_file.metadata().unwrap().len();
  let source_password_grade = match file_size {
    260 => PasswordGrade::Gold,
    61 => PasswordGrade::Silver,
    16 => PasswordGrade::Bronze,
    _ => {
      eprintln!("The size of password memory dump file is not valid!");
      return;
    }
  };

  // "grade" argument.
  let mut target_password_grade_option: Option<PasswordGrade> = None;
  if let Some(password_grade) = sub_matches.get_one("grade") {
    target_password_grade_option = Some(*password_grade);
  }
  // Check if it is possible to downgrade password
  let mut is_no_need_to_downgrade = true;
  if let Some(target_password_grade) = target_password_grade_option {
    if save::can_downgrade(source_password_grade, target_password_grade) {
      is_no_need_to_downgrade = save::no_need_to_downgrade(source_password_grade, target_password_grade);
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
  if let Some(bits) = save::get_valid_password_bits_option(password_bytes.as_slice(), true) {
    password_bits = bits;
  } else {
    eprintln!("Password is invalid!");
    return;
  }

  // Get flag
  let to_export_data_text = sub_matches.get_flag("export");
  // "text" argument.
  let mut password_version_option: Option<PasswordVersion> = None;
  if let Some(password_version) = sub_matches.get_one("text") {
    password_version_option = Some(*password_version);
  }
  // "cheat" argument.
  let mut cheat_version_option: Option<CheatVersion> = None;
  if let Some(cheat_version) = sub_matches.get_one("cheat") {
    cheat_version_option = Some(*cheat_version);
  }

  // Generate save data, and set target password bytes
  let save_data = save::get_save_data_with_password_bits(&mut password_bits, source_password_grade);
  let target_password_bytes;
  if let Some(target_password_grade) = target_password_grade_option {
    if is_no_need_to_downgrade {
      if password_version_option.is_none() && cheat_version_option.is_none() && !to_export_data_text {
        eprintln!("There is no need to downgrade input file, and seems we don't need to generate and export any file...");
        return;
      }
      target_password_bytes = password_bytes;
    } else {
      target_password_bytes = save::get_password_bytes_with_save_data(&save_data, target_password_grade);
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
        output::write_game_data_text_file(save::get_exported_data_for_dyrati_sheet_by_save_data(&save_data).as_str(), output_dir_str.as_str());
      } else {
        output::write_game_data_text_file(save::get_exported_data_for_dyrati_sheet_by_bytes(target_password_bytes.as_slice(), target_password_grade).as_str(), output_dir_str.as_str());
      }
    } else {
      output::write_game_data_text_file(save::get_exported_data_for_dyrati_sheet_by_save_data(&save_data).as_str(), output_dir_str.as_str());
    }
  }
}
