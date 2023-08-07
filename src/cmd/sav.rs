use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use clap::ArgMatches;
use crate::util::save;
use crate::util::output;
use crate::util::enums::{CheatVersion, PasswordVersion};

pub fn run(sub_matches: &ArgMatches) {
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
  if let Some(index) = save::get_loop_start_index_option(&raw_save_file) {
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
  let save_data_map = save::get_raw_save_data_map(to_export_all_data, &raw_save_file, loop_start_index);
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

  // "grade" argument.
  let target_password_grade = *sub_matches.get_one("grade").unwrap();
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

  // Write files.
  for (key, val) in &save_data_map {
    let password_bytes = save::get_password_bytes_with_raw_save_bytes(val.get_data(), target_password_grade);
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
      if target_password_grade.is_gold() {
        output::write_game_data_text_file(save::get_exported_data_for_dyrati_sheet_with_raw_save_bytes(val.get_data()).as_str(), sub_dir_str.as_str());
      } else {
        output::write_game_data_text_file(save::get_exported_data_for_dyrati_sheet_by_bytes(password_bytes.as_slice(), target_password_grade).as_str(), sub_dir_str.as_str());
      }
    }
  }
}
