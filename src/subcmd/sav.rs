use std::collections::HashMap;
use std::path::Path;
use std::{fs, process};

#[derive(Clone, Copy)]
pub enum PasswordGrade {
  Gold,
  Silver,
  Bronze,
}

pub struct SaveData {
  is_clear: bool,
  data: Vec<u8>,
}

impl SaveData {
  pub fn get_data(&self) -> &[u8] {
    &self.data
  }

  pub fn get_is_clear(&self) -> bool {
    self.is_clear
  }
  /*
    fn set_is_clear(&mut self) ->&mut bool{
      &mut self.is_clear
    }
  */
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

pub fn get_raw_save_data(to_export_all_data: bool, raw_save_file: &Vec<u8>) -> HashMap<u8, SaveData> {
  let camelot_header = [0x43u8, 0x41u8, 0x4Du8, 0x45u8, 0x4Cu8, 0x4Fu8, 0x54u8];
  let mut save_data_map = HashMap::new();
  let mut blank_save_slot_count = 0;
  for i in 0..16 {
    /* Another lazy way to check if save slot has no save data.
       If the first byte is "FF", that means this slot does not contain any save data,
       then skip current iteration. */
    if raw_save_file[i * 0x1000] == 0xFF {
      blank_save_slot_count += 1;
      continue;
    }

    /* A lazy and inaccurate way to detect if save file is Golden Sun save file.
       In Golden Sun, each save data(slot) take 4KB (0x1000) space.
       The first 7 bytes of each slot containing save data are "CAMELOT". */
    for j in 0..7 {
      if raw_save_file[i * 0x1000 + j] != camelot_header[j] {
        eprintln!("The input save file is not Golden Sun save file!");
        process::exit(1);
      }
    }

    if raw_save_file[i * 0x1000] == 0x43 {
      /* The 8th byte is the slot number, it only show 3 active save data in game.
         So the values for those 3 active save data are: "0x00", "0x01" and "0x02".
         And seems "0x10" and other values bigger than "0x02" are for backup save data. */
      if raw_save_file[i * 0x1000 + 0x07] > 0x02 {
        continue;
      }

      /* Does not include first 16 bytes header.
         The data from 0xA40 (Felix, Jenna, Sheba, PC07) is useless for password generating.
         0xA40 - 0x10 = 0xA30 */
      let mut save_data = SaveData { is_clear: false, data: vec![0; 0xA30] };
      /* Seems there are three bytes stored save location: "0x410", "0x418" and "0x490".
         And the values are all the same.
         Clear data's save location value is 1. */
      if raw_save_file[i * 0x1000 + 0x410] != 0x01 {
        if !to_export_all_data {
          continue;
        }
      } else {
        save_data.is_clear = true;
      }
      save_data.data.clone_from_slice(raw_save_file.get(i * 0x1000 + 0x10..i * 0x1000 + 0xA40).unwrap());

      /* Key is save slot number: 0, 1, 2 -> 1, 2, 3
         Value is save data. */
      save_data_map.insert(raw_save_file.get(i * 0x1000 + 0x07).unwrap() + 1, save_data);
    }
  }
  if blank_save_slot_count == 16 {
    eprintln!("There is no data in save file!");
    process::exit(1);
  }

  return save_data_map;
}

/* Port from Dyrati's "Golden-Sun-Password-Transfer" lua script for "vba-rr" and "Bizhawk" emulators.
   Link: https://github.com/Dyrati/Golden-Sun-Password-Transfer/blob/5ec2d52553ec8f4e0fe77854bc2b31956ac09a11/password.lua#L8 */
fn get_event_flag(raw_save: &[u8], flag: i32) -> u8 {
  let byte_pos = (flag >> 3) as usize;
  let bit_pos = flag & 7;
  return (raw_save[(0x40 + byte_pos)] >> bit_pos) & 1;
}

/* Port from Dyrati's "Golden-Sun-Password-Transfer" lua script for "vba-rr" and "Bizhawk" emulators.
   Link: https://github.com/Dyrati/Golden-Sun-Password-Transfer/blob/5ec2d52553ec8f4e0fe77854bc2b31956ac09a11/password.lua#L35 */
pub fn get_save_data(raw_save: &[u8]) -> ([u8; 4], [u32; 4], [u8; 6], [[u16; 6]; 4], [[u16; 15]; 4], u32) {
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

    // Saved Hametto/Hammet
    events[0] = get_event_flag(raw_save, 0x941);
    // Beat Colosso (Won the final fight against Navampa)
    events[1] = get_event_flag(raw_save, 0x951);
    // Ulmuch/Hsu was rescued by Hamo/Hama and Kouran/Feizhi alone, and Robin/Isaac's party didn't lend a hand
    events[2] = get_event_flag(raw_save, 0x8B3);
    // Beat Talos/Deadbeard
    events[3] = get_event_flag(raw_save, 0x8D1);
    // Visited Haidia/Vale after entering Kalay (Return to Haidia/Vale and find out that Robin/Isaac's mother's sick.)
    events[4] = get_event_flag(raw_save, 0x81E);
    // Visited Coopup/Vault after entering Kalay (Return to Coopup/Vault and talk with the mayor about the thieves, and find out that thieves have fled Coopup/Vault.)
    events[5] = get_event_flag(raw_save, 0x868);
  }
  coins = u32::from_le_bytes([raw_save[0x250], raw_save[0x251], raw_save[0x252], raw_save[0x253]]);
  return (levels, jinn, events, stats, items, coins);
}

/* Port from Dyrati's "Golden-Sun-Password-Transfer" lua script for "vba-rr" and "Bizhawk" emulators.
   Link: https://github.com/Dyrati/Golden-Sun-Password-Transfer/blob/5ec2d52553ec8f4e0fe77854bc2b31956ac09a11/password.lua#L79 */
pub fn gen_password_bytes(grade: PasswordGrade, levels: [u8; 4], jinn: [u32; 4], events: [u8; 6], stats: [[u16; 6]; 4], items: [[u16; 15]; 4], coins: u32) -> Vec<u8> {
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

pub fn create_sub_dir(slot_num: u8, is_clear_data: bool, output_dir_str: &str) -> String {
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