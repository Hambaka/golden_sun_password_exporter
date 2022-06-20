use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::string::String;
use std::{fs, process};
use clap::{AppSettings, arg, ArgAction, Command};

#[derive(Clone, Copy)]
enum PasswordVersion {
  Hiragana,
  LetterNumberSymbol,
}

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

#[derive(Clone, Copy)]
enum PasswordGrade {
  Gold,
  Silver,
  Bronze,
}

// Port from Dyrati's "Golden-Sun-Password-Transfer" lua script for "vba-rr" and "Bizhawk" emulators.
struct BitArray {
  bits: Vec<u8>,
}

// Port from Dyrati's "Golden-Sun-Password-Transfer" lua script for "vba-rr" and "Bizhawk" emulators.
impl BitArray {
  // Link: https://github.com/Dyrati/Golden-Sun-Password-Transfer/blob/5ec2d52553ec8f4e0fe77854bc2b31956ac09a11/password.lua#L16
  fn push_bit(&mut self, value: u32) {
    self.bits.push(((value >> 0) & 1) as u8);
  }

  // Link: https://github.com/Dyrati/Golden-Sun-Password-Transfer/blob/5ec2d52553ec8f4e0fe77854bc2b31956ac09a11/password.lua#L16
  fn push_bits(&mut self, value: u32, size: usize) {
    for i in 0..size {
      self.bits.push(((value >> (size - i - 1)) & 1) as u8);
    }
  }

  // Link: https://github.com/Dyrati/Golden-Sun-Password-Transfer/blob/5ec2d52553ec8f4e0fe77854bc2b31956ac09a11/password.lua#L16
  fn replace_bits(&mut self, value: u32, size: usize, pos: usize) {
    for i in 0..size {
      self.bits[pos + i] = ((value >> (size - i - 1)) & 1) as u8;
    }
  }

  // Link: https://github.com/Dyrati/Golden-Sun-Password-Transfer/blob/5ec2d52553ec8f4e0fe77854bc2b31956ac09a11/password.lua#L23
  fn sub_bits(&mut self, min: usize, max: usize) -> u8 {
    let mut acc = 0;
    for i in 0..=(max - min) {
      if min < self.bits.len() {
        acc = 2 * acc + self.bits[min + i];
      } else {
        acc = 2 * acc + 0;
      }
    }
    return acc;
  }
}

fn main() {
  let matches = Command::new("Golden Sun Password Exporter")
    .version("Version: 0.1.0")
    .author("Author: Hambaka")
    .about("About: \nRead save data to generate password and save it as a text file, \nand optionally generate memory dump file and cheats")
    .args_override_self(true)
    .global_setting(AppSettings::DeriveDisplayOrder)
    .allow_negative_numbers(true)
    .arg(
      arg!(
        <INPUT_FILE> "Golden Sun save file"
        )
        .required(true)
        .allow_invalid_utf8(true),
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
        .required(false)
        .allow_invalid_utf8(true),
    )
    .get_matches();

  // Read save file.
  let raw_input_path = matches.value_of_os("INPUT_FILE").unwrap();
  let mut input_file = File::open(raw_input_path).expect("An error occurred while opening save file!");

  // Check the size of save file.
  // The size of save file should be 64KB,
  // though seems the .SaveRAM file created by Bizhawk is 128KB.
  // Even its size 128KB, seems it only use first 64KB space to store save data.
  if input_file.metadata().unwrap().len() != 0x10000 && input_file.metadata().unwrap().len() != 0x20000 {
    eprintln!("The size of save file is not valid!");
    return;
  }

  // Default value is "false", only export password from clear save data.
  // Set it to true will export password from all existing save data in save file,
  // even it is not a clear data.
  let export_all_data = *matches.get_one::<bool>("all").expect("Defaulted by clap");

  // Get save data from save file with slot number.
  // Also check if the save data is clear data.
  let mut raw_save_file = Vec::new();
  input_file.read_to_end(&mut raw_save_file).unwrap();
  let result_data = get_save_data(export_all_data, &raw_save_file);
  let slot_num = result_data.0;
  let is_clear_data = result_data.1;
  let save_data = result_data.2;

  if slot_num.len() == 0 {
    if !export_all_data {
      eprintln!("There is no clear data in save file!");
    } else {
      eprintln!("There is no save data in save file!");
    }
    return;
  }

  let grade = matches.value_of("grade").unwrap();
  let password_grade = match grade {
    "g" => PasswordGrade::Gold,
    "s" => PasswordGrade::Silver,
    "b" => PasswordGrade::Bronze,
    _ => PasswordGrade::Gold,
  };

  let text = matches.value_of("text").unwrap();
  let password_version = match text {
    "j" => PasswordVersion::Hiragana,
    "e" => PasswordVersion::LetterNumberSymbol,
    _ => PasswordVersion::LetterNumberSymbol,
  };

  let export_memory_dump = *matches.get_one::<bool>("memory").expect("Defaulted by clap");

  let cheat_version;
  if matches.is_present("cheat") {
    let cheat = matches.value_of("cheat").unwrap();
    cheat_version = match cheat {
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
  if matches.is_present("output") {
    output_dir_str = String::from(Path::new(matches.value_of_os("output").unwrap().to_str().unwrap()).to_str().unwrap());
  } else {
    let input_path = Path::new(raw_input_path);
    output_dir_str = String::from(Path::new(input_path.parent().unwrap()).join(format!("{}_output", input_path.file_stem().unwrap().to_str().unwrap())).to_str().unwrap());
  }
  let output_dir_path = Path::new(output_dir_str.as_str());
  fs::create_dir_all(output_dir_path).expect("Failed to create output directory!");


  for i in 0..slot_num.len() {
    let raw_data = get_raw_data(save_data[i]);
    let password_bytes = gen_password_bytes(password_grade, raw_data.0, raw_data.1, raw_data.2, raw_data.3, raw_data.4, raw_data.5);
    let sub_dir_str = create_sub_dir(slot_num[i], is_clear_data[i], output_dir_str.as_str());
    write_password_text_file(password_bytes.as_slice(), password_version, sub_dir_str.as_str());
    if export_memory_dump {
      write_memory_dump_file(password_bytes.as_slice(), sub_dir_str.as_str());
    }
    write_cheat_file(password_bytes.as_slice(), cheat_version, sub_dir_str.as_str());
  }
}

fn get_save_data(export_all_data: bool, raw_save_file: &Vec<u8>) -> (Vec<u8>, Vec<bool>, Vec<&[u8]>) {
  let camelot_header = &[0x43u8, 0x41u8, 0x4Du8, 0x45u8, 0x4Cu8, 0x4Fu8, 0x54u8];
  let mut slot_num = Vec::new();
  let mut is_clear_data = Vec::new();
  let mut clear_data = Vec::new();
  let mut blank_save_slot_count = 0;
  for i in 0..16 {
    // A lazy and inaccurate way to detect if save file is Golden Sun save file.
    // In Golden Sun, each save data(slot) take 4KB (0x1000) space.
    // The first 7 bytes of each slot containing save data are "CAMELOT".
    for j in 0..7 {
      if raw_save_file[i * 0x1000 + j] != camelot_header[j] {
        eprintln!("The input save file is not Golden Sun save file!");
        process::exit(1);
      }
    }

    // Another lazy way to check if save slot has no save data.
    // If the first byte is "FF", that means this slot does not contain any save data,
    // then skip current iteration.
    if raw_save_file[i * 0x1000] == 0xFF {
      blank_save_slot_count += 1;
      continue;
    }

    if raw_save_file[i * 0x1000] == 0x43 {
      // The 8th byte is the slot number, it only show 3 active save data in game.
      // So the values for those 3 active save data are: "00", "01" and "02".
      // And seems "10" is for backup save data.
      if raw_save_file[i * 0x1000 + 0x07] > 0x02 {
        continue;
      }

      // Seems there are three bytes stored save location: "0x410", "0x418" and "0x490".
      // And the values are all the same.
      // Clear data's save location value is 1.
      if raw_save_file[i * 0x1000 + 0x410] != 0x01 {
        if export_all_data {
          is_clear_data.push(false);
        } else {
          continue;
        }
      } else {
        is_clear_data.push(true);
      }
      // 0, 1, 2 -> 1, 2, 3
      slot_num.push(raw_save_file.get(i * 0x1000 + 0x07).unwrap() + 1);
      // Does not include 16 bytes header.
      // The data from 0xA40 (Felix, Jenna, Sheba, PC07) is useless for password generating.
      clear_data.push(raw_save_file.get(i * 0x1000 + 0x10..i * 0x1000 + 0xA40).unwrap());
    }
  }
  if blank_save_slot_count == 16 {
    eprintln!("There is no data in save file!");
    process::exit(1);
  }

  return (slot_num, is_clear_data, clear_data);
}

// Port from Dyrati's "Golden-Sun-Password-Transfer" lua script for "vba-rr" and "Bizhawk" emulators.
// Link: https://github.com/Dyrati/Golden-Sun-Password-Transfer/blob/5ec2d52553ec8f4e0fe77854bc2b31956ac09a11/password.lua#L8
fn get_event_flag(raw_save: &[u8], flag: i32) -> u8 {
  let byte_pos = (flag >> 3) as usize;
  let bit_pos = flag & 7;
  return (raw_save[(0x40 + byte_pos)] >> bit_pos) & 1;
}

// Port from Dyrati's "Golden-Sun-Password-Transfer" lua script for "vba-rr" and "Bizhawk" emulators.
// Link: https://github.com/Dyrati/Golden-Sun-Password-Transfer/blob/5ec2d52553ec8f4e0fe77854bc2b31956ac09a11/password.lua#L35
fn get_raw_data(raw_save: &[u8]) -> ([u8; 4], [u32; 4], [u8; 6], [[u16; 6]; 4], [[u16; 15]; 4], u32) {
  // [u8; 4]
  let mut levels = [0; 4];
  // [u32; 4]
  let mut jinn = [0; 4];
  // [u8; 6]
  let mut events = [0; 6];
  // [[u16; 6]; 4]
  let mut stats = [[0; 6]; 4];
  // [[u16; 15]; 4]
  let mut items = [[0; 15]; 4];
  // u32
  let coins;

  for i in 0..4 {
    let base = 0x500 + 0x14C * i;
    levels[i] = raw_save[base + 0xF];

    for j in 0..4 {
      jinn[j] = jinn[j] | u32::from_le_bytes([raw_save[base + 0xF8 + 4 * j], raw_save[base + 0xF9 + 4 * j], raw_save[base + 0xFA + 4 * j], raw_save[base + 0xFB + 4 * j]]);
    }

    // HP
    stats[i][0] = u16::from_le_bytes([raw_save[base + 0x10], raw_save[base + 0x11]]);
    // EP
    stats[i][1] = u16::from_le_bytes([raw_save[base + 0x12], raw_save[base + 0x13]]);
    // Attack
    stats[i][2] = u16::from_le_bytes([raw_save[base + 0x18], raw_save[base + 0x19]]);
    // Defense
    stats[i][3] = u16::from_le_bytes([raw_save[base + 0x1A], raw_save[base + 0x1B]]);
    // Agility
    stats[i][4] = u16::from_le_bytes([raw_save[base + 0x1C], raw_save[base + 0x1D]]);
    // Luck
    stats[i][5] = raw_save[base + 0x1E] as u16;

    for j in 0..15 {
      items[i][j] = u16::from_le_bytes([raw_save[base + 0xD8 + 2 * j], raw_save[base + 0xD9 + 2 * j]]);
    }

    // Save Hamett(Hammet)
    events[0] = get_event_flag(raw_save, 0x941);
    // Beat Colosso
    events[1] = get_event_flag(raw_save, 0x951);
    // Ulmuch(Hsu) Died
    events[2] = get_event_flag(raw_save, 0x8B3);
    // Beat Talos(Deadbeard)
    events[3] = get_event_flag(raw_save, 0x8D1);
    // Return to Haidia(Vale)
    events[4] = get_event_flag(raw_save, 0x81E);
    // Return to Coopup(Vault)
    events[5] = get_event_flag(raw_save, 0x868);
  }
  coins = u32::from_le_bytes([raw_save[0x250], raw_save[0x251], raw_save[0x252], raw_save[0x253]]);
  return (levels, jinn, events, stats, items, coins);
}

// Port from Dyrati's "Golden-Sun-Password-Transfer" lua script for "vba-rr" and "Bizhawk" emulators.
// Link: https://github.com/Dyrati/Golden-Sun-Password-Transfer/blob/5ec2d52553ec8f4e0fe77854bc2b31956ac09a11/password.lua#L79
fn gen_password_bytes(grade: PasswordGrade, levels: [u8; 4], jinn: [u32; 4], events: [u8; 6], stats: [[u16; 6]; 4], items: [[u16; 15]; 4], coins: u32) -> Vec<u8> {
  let mut bits = BitArray { bits: Vec::new() };

  // Insert 7 bits per level, 7 bits per jinn element
  let mut level_bits = BitArray { bits: Vec::new() };
  let mut jinn_bits = BitArray { bits: Vec::new() };
  for i in (0..=3).rev() {
    level_bits.push_bits(levels[i] as u32, 7);
  }

  for i in (0..=3).rev() {
    jinn_bits.push_bits(jinn[i] as u32, 7);
  }

  for i in (11..=27).rev().step_by(8) {
    bits.push_bits(level_bits.sub_bits(i - 7, i) as u32, 8);
  }

  bits.push_bits(level_bits.sub_bits(0, 3) as u32, 4);
  bits.push_bits(jinn_bits.sub_bits(24, 27) as u32, 4);

  for i in (7..=23).rev().step_by(8) {
    bits.push_bits(jinn_bits.sub_bits(i - 7, i) as u32, 8);
  }

  for i in (0..=7).rev() {
    if i < events.len() {
      bits.push_bit(events[i] as u32);
    } else {
      bits.push_bit(0);
    }
  }

  let size = match grade {
    PasswordGrade::Bronze => 9,
    PasswordGrade::Silver => 39,
    PasswordGrade::Gold => 173,
  };

  // If password grade is silver or bronze,
  // insert 8 bits representing which of these items your party has.
  if !matches!(grade, PasswordGrade::Gold) {
    let energy_items = [0xC8, 0xC9, 0xCA, 0xCB, 0xCC, 0xCD, 0xCE, 0xCF];
    let mut flags = 0;
    for i in 0..4 {
      for item in items[i] {
        let id = item & 0x1FF;
        for j in 0..8 {
          if id == energy_items[j] {
            flags = flags | i32::pow(2, j as u32);
          }
        }
      }
    }
    bits.push_bits(flags as u32, 8);
  }

  // If password grade is gold or silver, insert stats.
  if !matches!(grade, PasswordGrade::Bronze) {
    for i in 0..4 {
      // HP
      bits.push_bits(stats[i][0] as u32, 11);
      // EP
      bits.push_bits(stats[i][1] as u32, 11);
      // Attack
      bits.push_bits(stats[i][2] as u32, 10);
      // Defense
      bits.push_bits(stats[i][3] as u32, 10);
      // Agility
      bits.push_bits(stats[i][4] as u32, 10);
      // Luck
      bits.push_bits(stats[i][5] as u32, 8);
    }
  }

  // If password grade is gold, insert items and coins.
  if matches!(grade, PasswordGrade::Gold) {
    bits.push_bits(0, 8);
    let mut counter = 0;
    for i in 0..4 {
      for item in items[i] {
        let id = item & 0x1FF;
        bits.push_bits(id as u32, 9);
        counter += 1;
        // Append a 0 bit every 7 items.
        if counter == 7 {
          bits.push_bit(0);
          counter = 0;
        }
      }
    }
    // List of all stackable items in GS1.
    let stackable_items = [0xB4, 0xB5, 0xB6, 0xB7, 0xBA, 0xBB, 0xBC, 0xBD, 0xBF, 0xC0, 0xC1, 0xC2, 0xC3, 0xC4, 0xE2, 0xE3, 0xE4, 0xE5, 0xEC, 0xEE, 0xEF, 0xF0, 0xF1];

    // Insert quantities of stackable items for each energist(adept).
    for i in 0..4 {
      for j in 0..stackable_items.len() {
        let mut quantity = 0;
        for item in items[i] {
          let id = item & 0x1FF;
          if id == stackable_items[j] {
            quantity = item >> 11;
          }
        }
        bits.push_bits(quantity as u32, 5);
      }
    }
    bits.push_bits(coins, 24);
  }

  // Append 0 until reaching the correct password size.
  for _i in 0..8 * size - bits.bits.len() {
    bits.push_bit(0);
  }

  // Encrypt with key 0x1021.
  let mut x_sum = 0xFFFF as u64;

  for i in 0..size {
    let byte = bits.sub_bits(8 * i, 8 * i + 7) as u64;
    x_sum = x_sum ^ (byte << 8);
    for _j in 0..8 {
      if (x_sum & 0x8000) != 0 {
        x_sum = (x_sum << 1) + 0xFFFFEFDF;
      } else {
        x_sum = x_sum << 1;
      }
    }
  }

  x_sum = (!x_sum) & 0xFFFF;
  let xor_value = (x_sum & 0xFF) as u32;
  bits.push_bits((x_sum >> 8) as u32, 8);
  bits.push_bits(xor_value, 8);

  for i in 0..=size {
    let byte = bits.sub_bits(8 * i, 8 * i + 7) as u32;
    bits.replace_bits(byte ^ xor_value, 8, 8 * i);
  }

  // Split into 6-bit entries
  let mut password_bytes = Vec::new();
  let mut acc = 0 as u32;
  let max_size = (size + 2) * 8;

  for i in (0..max_size).step_by(6) {
    let mut max = i + 5;
    let entry;

    if max <= bits.bits.len() {
      entry = bits.sub_bits(i, max);
    } else {
      let last_left_shift = max - max_size + 1;
      max = bits.bits.len() - 1;
      entry = bits.sub_bits(i, max) << last_left_shift;
    }

    password_bytes.push(entry);
    acc += entry as u32;
    // Insert checksum for each line
    if password_bytes.len() % 10 == 9 {
      password_bytes.push((acc & 0x3F) as u8);
      acc = 0;
    }
  }

  for i in 0..password_bytes.len() {
    let mut temp = password_bytes[i] as u16;
    temp += i as u16;
    password_bytes[i] = (temp & 0x3F) as u8;
  }

  return password_bytes;
}

fn create_sub_dir(slot_num: u8, is_clear_data: bool, output_dir_str: &str) -> String {
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
    _ => ' ',
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
    _ => ' '
  }
}

fn write_password_text_file(password_bytes: &[u8], password_version: PasswordVersion, sub_dir_str: &str) {
  let mut text = String::new();

  match password_version {
    PasswordVersion::Hiragana => {
      for i in 0..password_bytes.len() {
        text.push(byte_to_jp(password_bytes[i]));
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
    PasswordVersion::LetterNumberSymbol => {
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

// Write password bytes to a binary file.
// After you go to password input screen in GS2, you can import it via emulator's memory viewer.
// Though you have to choose the correct address and import it, you can check the address below.
fn write_memory_dump_file(password_bytes: &[u8], sub_dir_str: &str) {
  let output_path = Path::new(sub_dir_str).join("memory.dmp");
  let mut output_file = File::create(output_path).expect("Failed to create memory dump file!");
  output_file.write_all(password_bytes).expect("Failed to write to memory dump file!");
}

// I'm not sure, maybe you can use this kind of raw cheat code on your phone?
// Then you don't have to input password manually.
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
