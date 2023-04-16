mod subcmd;

use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::string::String;
use std::fs;
use clap::{arg, ArgAction, ArgGroup, Command, value_parser};

#[derive(Clone, Copy)]
enum CheatVersion {
  Japanese,
  English,
  German,
  Spanish,
  French,
  Italian,
  None,
}

fn main() {
  let matches = Command::new("Golden Sun Password Exporter")
    .version("0.2.0")
    .author("Hambaka")
    .about("A simple tool for a GBA game called Golden Sun\nYou can use this tool to export Golden Sun password to a text file/memory dump binary file/cheat file")
    .allow_negative_numbers(true)
    .propagate_version(true)
    .subcommand_required(true)
    .arg_required_else_help(true)
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
            -a --all "Export all existing save data in save file"
          )
            .action(ArgAction::SetTrue)
            .required(false)
        )
        .arg(
          arg!(
            -g --grade <VALUE> "Password grade"
          )
            .required(false)
            .default_value("g")
        )
        .arg(
          arg!(
            -t --text <VALUE> "Password version"
          )
            .required(false)
            .default_value("e")
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
            -o --output <OUTPUT_DIR> "Output directory"
          )
            .value_parser(value_parser!(PathBuf))
            .required(false)
        ),
    )
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
        .group(
          ArgGroup::new("txt_args")
            .required(true)
            .args(&["text", "memory", "cheat"])
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
        .group(
          ArgGroup::new("dmp_args")
            .required(true)
            .args(&["text", "cheat"])
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

      /* Default value is "false", only export password from clear save data.
         Set it to "true" will export password from all existing save data in save file,
         even it is not a clear data. */
      let to_export_all_data = sub_matches.get_flag("all");

      /* Get save data from save file with slot number.
         Also check if the save data is clear data. */
      let mut raw_save_file = Vec::new();
      input_file.read_to_end(&mut raw_save_file).unwrap();
      let save_data_map = subcmd::sav::get_raw_save_data(to_export_all_data, &raw_save_file);

      if save_data_map.is_empty() {
        if !to_export_all_data {
          eprintln!("There is no clear data in save file!");
        } else {
          eprintln!("There is no save data in save file!");
        }
        return;
      }

      let grade = sub_matches.get_one::<String>("grade").unwrap();
      let password_grade = match grade.as_str() {
        "g" => subcmd::sav::PasswordGrade::Gold,
        "s" => subcmd::sav::PasswordGrade::Silver,
        "b" => subcmd::sav::PasswordGrade::Bronze,
        _ => subcmd::sav::PasswordGrade::Gold,
      };

      let text = sub_matches.get_one::<String>("text").unwrap();
      let password_version = match text.as_str() {
        "j" => subcmd::txt::PasswordVersion::Hiragana,
        "e" => subcmd::txt::PasswordVersion::LetterNumberSymbol,
        _ => subcmd::txt::PasswordVersion::LetterNumberSymbol,
      };

      let to_export_memory_dump = sub_matches.get_flag("memory");

      let cheat_version;
      if let Some(cheat) = sub_matches.get_one::<String>("cheat") {
        cheat_version = match cheat.as_str() {
          "j" => CheatVersion::Japanese,
          "u" => CheatVersion::English,
          "g" => CheatVersion::German,
          "s" => CheatVersion::Spanish,
          "f" => CheatVersion::French,
          "i" => CheatVersion::Italian,
          _ => CheatVersion::English,
        };
      } else {
        cheat_version = CheatVersion::None;
      }

      let output_dir_str;
      if let Some(output_path_buf) = sub_matches.get_one::<PathBuf>("output") {
        output_dir_str = String::from(output_path_buf.to_str().unwrap());
      } else {
        output_dir_str = String::from(sav_input_path.parent().unwrap().join(format!("{}_output", sav_input_path.file_stem().unwrap().to_str().unwrap())).to_str().unwrap());
      }
      let output_dir_path = Path::new(output_dir_str.as_str());
      fs::create_dir_all(output_dir_path).expect("Failed to create output directory!");

      for (key, val) in save_data_map.iter() {
        let raw_data = subcmd::sav::get_save_data(val.get_data());
        let password_bytes = subcmd::sav::gen_password_bytes(password_grade, raw_data.0, raw_data.1, raw_data.2, raw_data.3, raw_data.4, raw_data.5);
        let sub_dir_str = subcmd::sav::create_sub_dir(key.clone(), val.get_is_clear(), output_dir_str.as_str());
        write_password_text_file(&password_bytes, password_version, sub_dir_str.as_str());
        if to_export_memory_dump {
          write_memory_dump_file(&password_bytes, sub_dir_str.as_str());
        }
        write_cheat_file(&password_bytes, cheat_version, sub_dir_str.as_str());
      }
    }
    Some(("txt", sub_matches)) => {
      //Read password text file
      let txt_input_path = sub_matches.get_one::<PathBuf>("INPUT_FILE").unwrap();
      let mut input_file = File::open(txt_input_path).expect("An error occurred while opening save file!");
      let mut password = String::new();
      input_file.read_to_string(&mut password).unwrap();
      // If the file is empty, exit.
      if password.is_empty() {
        println!("The text file is empty!");
        return;
      }
      let mut password_version = subcmd::txt::get_save_type(password.as_ref());
      let password_bytes = subcmd::txt::txt_to_dmp(password, password_version);
      if password_bytes.len() != 16 && password_bytes.len() != 61 && password_bytes.len() != 260 {
        println!("Password's length is not valid!");
        return;
      }

      password_version = match password_version {
        subcmd::txt::PasswordVersion::Hiragana => subcmd::txt::PasswordVersion::LetterNumberSymbol,
        subcmd::txt::PasswordVersion::LetterNumberSymbol => subcmd::txt::PasswordVersion::Hiragana,
      };
      let to_convert_password = sub_matches.get_flag("text");
      let to_export_memory_dump = sub_matches.get_flag("memory");
      let cheat_version;
      if let Some(cheat) = sub_matches.get_one::<String>("cheat") {
        cheat_version = match cheat.as_str() {
          "j" => CheatVersion::Japanese,
          "u" => CheatVersion::English,
          "g" => CheatVersion::German,
          "s" => CheatVersion::Spanish,
          "f" => CheatVersion::French,
          "i" => CheatVersion::Italian,
          _ => CheatVersion::English,
        };
      } else {
        cheat_version = CheatVersion::None;
      }

      let output_dir_str;
      if let Some(output_path_buf) = sub_matches.get_one::<PathBuf>("output") {
        output_dir_str = String::from(output_path_buf.to_str().unwrap());
      } else {
        output_dir_str = String::from(txt_input_path.parent().unwrap().join(format!("{}_output", txt_input_path.file_stem().unwrap().to_str().unwrap())).to_str().unwrap());
      }
      let output_dir_path = Path::new(output_dir_str.as_str());
      fs::create_dir_all(output_dir_path).expect("Failed to create output directory!");


      if to_convert_password {
        write_password_text_file(&password_bytes, password_version, output_dir_str.as_str());
      }
      if to_export_memory_dump {
        write_memory_dump_file(&password_bytes, output_dir_str.as_str());
      }
      write_cheat_file(&password_bytes, cheat_version, output_dir_str.as_str());
    }
    Some(("dmp", sub_matches)) => {
      // Read password memory dump file.
      let dmp_input_path = sub_matches.get_one::<PathBuf>("INPUT_FILE").unwrap();
      let mut input_file = File::open(dmp_input_path).expect("An error occurred while opening save file!");

      let file_size = input_file.metadata().unwrap().len();
      if file_size != 16 && file_size != 61 && file_size != 260 {
        eprintln!("The size of password memory dump file is not valid!");
        return;
      }
      let mut password_bytes = Vec::new();
      input_file.read_to_end(&mut password_bytes).unwrap();

      let text = sub_matches.get_one::<String>("text").unwrap();
      let password_version = match text.as_str() {
        "j" => subcmd::txt::PasswordVersion::Hiragana,
        "e" => subcmd::txt::PasswordVersion::LetterNumberSymbol,
        _ => subcmd::txt::PasswordVersion::LetterNumberSymbol,
      };

      let cheat_version;
      if let Some(cheat) = sub_matches.get_one::<String>("cheat") {
        cheat_version = match cheat.as_str() {
          "j" => CheatVersion::Japanese,
          "u" => CheatVersion::English,
          "g" => CheatVersion::German,
          "s" => CheatVersion::Spanish,
          "f" => CheatVersion::French,
          "i" => CheatVersion::Italian,
          _ => CheatVersion::English,
        };
      } else {
        cheat_version = CheatVersion::None;
      }

      let output_dir_str;
      if let Some(output_path_buf) = sub_matches.get_one::<PathBuf>("output") {
        output_dir_str = String::from(output_path_buf.to_str().unwrap());
      } else {
        output_dir_str = String::from(dmp_input_path.parent().unwrap().join(format!("{}_output", dmp_input_path.file_stem().unwrap().to_str().unwrap())).to_str().unwrap());
      }
      let output_dir_path = Path::new(output_dir_str.as_str());
      fs::create_dir_all(output_dir_path).expect("Failed to create output directory!");

      write_password_text_file(&password_bytes, password_version, output_dir_str.as_str());
      write_cheat_file(&password_bytes, cheat_version, output_dir_str.as_str());
    }
    _ => unreachable!(),
  }
}

fn byte_to_jp(input: u8) -> char {
  match input {
    0x00 => 'あ',
    0x01 => 'い',
    0x02 => 'う',
    0x03 => 'え',
    0x04 => 'お',

    0x05 => 'か',
    0x06 => 'き',
    0x07 => 'く',
    0x08 => 'け',
    0x09 => 'こ',

    0x0A => 'さ',
    0x0B => 'し',
    0x0C => 'す',
    0x0D => 'せ',
    0x0E => 'そ',

    0x0F => 'た',
    0x10 => 'ち',
    0x11 => 'つ',
    0x12 => 'て',
    0x13 => 'と',

    0x14 => 'な',
    0x15 => 'に',
    0x16 => 'ぬ',
    0x17 => 'ね',
    0x18 => 'の',

    0x19 => 'は',
    0x1A => 'ひ',
    0x1B => 'ふ',
    0x1C => 'へ',
    0x1D => 'ほ',

    0x1E => 'ま',
    0x1F => 'み',
    0x20 => 'む',
    0x21 => 'め',
    0x22 => 'も',

    0x23 => 'や',
    0x24 => 'ゆ',
    0x25 => 'よ',

    0x26 => 'ら',
    0x27 => 'り',
    0x28 => 'る',
    0x29 => 'れ',
    0x2A => 'ろ',

    0x2B => 'わ',
    0x2D => 'を',
    0x2C => 'ん',

    0x2E => 'が',
    0x2F => 'ぎ',
    0x30 => 'ぐ',
    0x31 => 'げ',
    0x32 => 'ご',

    0x33 => 'ざ',
    0x34 => 'じ',
    0x35 => 'ず',
    0x36 => 'ぜ',
    0x37 => 'ぞ',

    0x38 => 'だ',
    0x39 => 'で',
    0x3A => 'ど',

    0x3B => 'ば',
    0x3C => 'び',
    0x3D => 'ぶ',
    0x3E => 'べ',
    0x3F => 'ぼ',
    _ => '？',
  }
}

fn byte_to_en(input: u8) -> char {
  match input {
    0x00 => 'A',
    0x01 => 'B',
    0x02 => 'C',
    0x03 => 'D',
    0x04 => 'E',

    0x05 => 'F',
    0x06 => 'G',
    0x07 => 'H',
    0x08 => 'J',
    0x09 => 'K',

    0x0A => 'L',
    0x0B => 'M',
    0x0C => 'N',
    0x0D => 'P',
    0x0E => 'Q',

    0x0F => 'R',

    0x10 => 'S',
    0x11 => 'T',
    0x12 => 'U',
    0x13 => 'V',
    0x14 => 'W',

    0x15 => 'X',
    0x16 => 'Y',
    0x17 => 'Z',
    0x18 => '2',
    0x19 => '3',

    0x1A => '4',
    0x1B => '5',
    0x1C => '6',
    0x1D => '7',
    0x1E => '8',

    0x1F => '9',

    0x20 => 'a',
    0x21 => 'b',
    0x22 => 'c',
    0x23 => 'd',
    0x24 => 'e',

    0x25 => 'f',
    0x26 => 'g',
    0x27 => 'h',
    0x28 => 'i',
    0x29 => 'j',

    0x2A => 'k',
    0x2B => 'm',
    0x2C => 'n',
    0x2D => 'p',
    0x2E => 'q',

    0x2F => 'r',

    0x30 => 's',
    0x31 => 't',
    0x32 => 'u',
    0x33 => 'v',
    0x34 => 'w',

    0x35 => 'x',
    0x36 => 'y',
    0x37 => 'z',
    0x38 => '!',
    0x39 => '?',

    0x3A => '#',
    0x3B => '&',
    0x3C => '$',
    0x3D => '%',
    0x3E => '+',

    0x3F => '=',
    _ => '?',
  }
}

fn write_password_text_file(password_bytes: &[u8], password_version: subcmd::txt::PasswordVersion, sub_dir_str: &str) {
  let mut text = String::new();

  match password_version {
    subcmd::txt::PasswordVersion::Hiragana => {
      for i in 0..password_bytes.len() {
        text.push(byte_to_jp(password_bytes[i]));
        if (i + 1) % 50 == 0 {
          text.push('\n');
          text.push('\n');
        } else if (i + 1) % 10 == 0 {
          text.push('\n');
        } else if (i + 1) % 5 == 0 {
          text.push('　');
        }
      }
    }
    subcmd::txt::PasswordVersion::LetterNumberSymbol => {
      for i in 0..password_bytes.len() {
        text.push(byte_to_en(password_bytes[i]));
        if (i + 1) % 50 == 0 {
          text.push('\n');
          text.push('\n');
        } else if (i + 1) % 10 == 0 {
          text.push('\n');
        } else if (i + 1) % 5 == 0 {
          text.push(' ');
        }
      }
    }
  }
  let output_path = Path::new(sub_dir_str).join("password.txt");
  let mut output_file = File::create(output_path).expect("Failed to create password text file!");
  output_file.write_all(text.as_bytes()).expect("Failed to write to password text file!");
}

/* Write password bytes to a binary file.
   After you go to password input screen in GS2, you can import it via emulator's memory viewer.
   Though you have to choose the correct address and import it, you can check the address below. */
fn write_memory_dump_file(password_bytes: &[u8], sub_dir_str: &str) {
  let output_path = Path::new(sub_dir_str).join("memory.dmp");
  let mut output_file = File::create(output_path).expect("Failed to create memory dump file!");
  output_file.write_all(password_bytes).expect("Failed to write to memory dump file!");
}

/* I'm not sure, maybe you can use this kind of raw cheat code on your phone?
   Then you don't have to input password manually. */
fn write_cheat_file(password_bytes: &[u8], cheat_version: CheatVersion, sub_dir_str: &str) {
  // The address for password input screen in Golden Sun: The Lost Age
  let mut address = match cheat_version {
    CheatVersion::Japanese => 0x0200A78A,
    CheatVersion::English => 0x0200A74A,
    CheatVersion::German => 0x0200A742,
    CheatVersion::Spanish => 0x0200A73E,
    CheatVersion::French => 0x0200A742,
    CheatVersion::Italian => 0x0200A742,
    CheatVersion::None => return,
  };
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
