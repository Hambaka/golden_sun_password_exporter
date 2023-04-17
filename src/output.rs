use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use crate::{enums, text};

pub fn create_sav_sub_dir(slot_num: u8, is_clear_data: bool, output_dir_str: &str) -> String {
  let output_path;
  if is_clear_data {
    output_path = Path::new(output_dir_str).join(format!("Save{:02}(Clear)", slot_num));
  } else {
    output_path = Path::new(output_dir_str).join(format!("Save{:02}", slot_num));
  }
  let sub_dir_str = String::from(output_path.to_str().unwrap());
  fs::create_dir_all(output_path).expect("Failed to create sub directory!");
  return sub_dir_str;
}

pub fn create_output_dir(output_path_buf: &PathBuf, has_output_arg: bool) -> String {
  let output_dir_str;
  if has_output_arg {
    output_dir_str = String::from(output_path_buf.to_str().unwrap());
  } else {
    output_dir_str = String::from(output_path_buf.parent().unwrap().join(format!("{}_output", output_path_buf.file_stem().unwrap().to_str().unwrap())).to_str().unwrap());
  }
  let output_dir_path = Path::new(output_dir_str.as_str());
  fs::create_dir_all(output_dir_path).expect("Failed to create output directory!");

  return output_dir_str;
}

pub fn write_password_text_file(password_bytes: &[u8], password_version: enums::PasswordVersion, output_dir_str: &str) {
  let mut password_text = String::new();

  match password_version {
    enums::PasswordVersion::Japanese => {
      for i in 0..password_bytes.len() {
        password_text.push(text::byte_to_jp(password_bytes[i]));
        if (i + 1) % 50 == 0 {
          password_text.push('\n');
          password_text.push('\n');
        } else if (i + 1) % 10 == 0 {
          password_text.push('\n');
        } else if (i + 1) % 5 == 0 {
          password_text.push('　');
        }
      }
    }
    enums::PasswordVersion::English => {
      for i in 0..password_bytes.len() {
        password_text.push(text::byte_to_en(password_bytes[i]));
        if (i + 1) % 50 == 0 {
          password_text.push('\n');
          password_text.push('\n');
        } else if (i + 1) % 10 == 0 {
          password_text.push('\n');
        } else if (i + 1) % 5 == 0 {
          password_text.push(' ');
        }
      }
    }
  }
  let output_path = Path::new(output_dir_str).join("password.txt");
  let mut output_file = File::create(output_path).expect("Failed to create password text file!");
  output_file.write_all(password_text.as_bytes()).expect("Failed to write to password text file!");
}

/* Write password bytes to a binary file.
   After you go to password input screen in GS2, you can import it via emulator's memory viewer.
   Though you have to choose the correct address and import it, you can check the address in "get_cheat_address" function of "enums.rs". */
pub fn write_memory_dump_file(password_bytes: &[u8], sub_dir_str: &str) {
  let output_path = Path::new(sub_dir_str).join("memory.dmp");
  let mut output_file = File::create(output_path).expect("Failed to create memory dump file!");
  output_file.write_all(password_bytes).expect("Failed to write to memory dump file!");
}

/* I'm not sure, maybe you can use this kind of raw cheat code on your phone?
   Then you don't have to input password manually. */
pub fn write_cheat_file(password_bytes: &[u8], cheat_version: enums::CheatVersion, sub_dir_str: &str) {
  // The address for password input screen in Golden Sun: The Lost Age
  let mut address = enums::get_cheat_address(cheat_version);
  let mut text = String::new();
  let size = password_bytes.len();
  let loop_num = (size - 2) / 4;
  let mut loop_count = 0;
  text.push_str(format!("{:08X}:{:04X}\n", address, u16::from_le_bytes([password_bytes[0], password_bytes[1]])).as_str());
  address += 2;
  for i in (2..size).step_by(4) {
    text.push_str(format!("{:08X}:{:08X}\n", address, u32::from_le_bytes([password_bytes[i], password_bytes[i + 1], password_bytes[i + 2], password_bytes[i + 3]])).as_str());
    address += 4;
    loop_count += 1;
    if loop_count == loop_num {
      if size % 2 == 0 {
        text.push_str(format!("{:08X}:{:04X}\n", address, u16::from_le_bytes([password_bytes[i + 4], password_bytes[i + 5]])).as_str());
      } else {
        text.push_str(format!("{:08X}:{:04X}\n", address, u16::from_le_bytes([password_bytes[i + 4], password_bytes[i + 5]])).as_str());
        text.push_str(format!("{:08X}:{:02X}\n", address + 2, password_bytes[i + 6]).as_str());
      }
      break;
    }
  }
  let output_path = Path::new(sub_dir_str).join("cheats.txt");
  let mut output_file = File::create(output_path).expect("Failed to create cheat file!");
  output_file.write_all(text.as_bytes()).expect("Failed to write to cheat file!");
}
