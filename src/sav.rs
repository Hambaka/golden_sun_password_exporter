use std::collections::HashMap;
use crate::enums::PasswordGrade;

/* Links to other Golden Sun reference guide (save editing):
   https://gamefaqs.gamespot.com/gba/468548-golden-sun/faqs/43776
   https://gamefaqs.gamespot.com/gba/561356-golden-sun-the-lost-age/faqs/30811
   ----------------------------------------------------------------------------------
   More reference info/comment about GBA Golden Sun series save file from Dyrati (in "Obababot")

   https://github.com/Dyrati/obababot/blob/main/obababot/gsfuncs.py
   At line 579, the "get_save_data" function takes raw binary .sav data and returns individual save slots with all of the info from each valid save.
   The function checks the file at 0x1000 byte intervals.

   The first 16 bytes of each interval (the header) are organized as follows:
   - 7 bytes for the ASCII string "CAMELOT"
   - 1 byte for the slot number
   - 2 bytes for a checksum
   - 2 bytes for a priority number
   - 4 bytes of garbage data

   A header is valid if the first 7 bytes spell "CAMELOT", and the slot number is less than 16.
   In the case where multiple headers have the same slot number, use the header with the highest priority number.
   That should leave you with up to 3 valid headers.
   The next 0x2FF0 bytes after the header constitute the save data for that file. (Note: GS2 only)
   ----------------------------------------------------------------------------------
   Additional reference info/comment about the first Golden Sun save file from Dyrati

   For GS1, each save splits into two parts.
   In the .sav file, each section is 0x1000 bytes long.
   However two separate sections are joined together to create one save file.
   Some sections have slot numbers of 3, 4, or 5,
   those sections are the second half of slots 0, 1, and 2 respectively. */

/// 7 bytes for the ASCII string "CAMELOT" in each save's header.
const HEADER_CAMELOT_ASCII_STRING: &str = "CAMELOT";

/// Golden Sun build date
/// Source: Golden Sun Hacking Community Discord Server
/// GS1 (J) = 0x159C
/// GS1 (U) = 0x1652
/// GS1 (G) = 0x1849
/// GS1 (S) = 0x1885
/// GS1 (F) = 0x1713
/// GS1 (I) = 0x1886
const GS_BUILD_DATE: [[u8; 2]; 6] = [[0x9C, 0x15], [0x52, 0x16], [0x49, 0x18], [0x85, 0x18], [0x13, 0x17], [0x86, 0x18]];

/// The size of each save slot is 4KB.
const SAVE_SLOT_SIZE: usize = 0x1000;

/// 64KB / 4KB = 16
const MAX_LOOP_COUNT: usize = 16;

/// All build date locations: 0x36 to 0x37, 0x250 to 0x251, 0x508 to 0x509
/// All stored values are same.
const FIRST_BUILD_DATE_LOCATION_INDEX: [usize; 2] = [0x36, 0x37];

/// All save map(location) locations  :0x410, 0x418, 0x490
/// All stored values are same.
const FIRST_SAVE_MAP_LOCATION_INDEX: usize = 0x410;

/// Clear data's save location value is 1.
const CLEAR_DATA_SAVE_MAP_VALUE: u8 = 0x01;

/// The 8th byte is the slot number, it only show 3 active save data in game.
const HEADER_SAVE_SLOT_NUMBER_LOCATION_INDEX: usize = 0x07;
/// The values for 3 'active' save data are: 0x00, 0x01 and 0x02.
/// And seems 0x10 and other values bigger than 0x02 are for backup save data.
/// But seems 0x10 is not a valid slot number?
const MAX_VALID_SLOT_NUMBER: u8 = 0x02;
const HEADER_PRIORITY_LOCATION_INDEX: [usize; 2] = [0x0A, 0x0B];

struct HeaderInfo {
  // Range: 0 <= x < 16
  index: usize,
  priority: u16,
  // Start from 0
  slot_number: u8,
  // Please note, the save location is not in save header, this is only for checking valid header/save
  is_clear: bool,
}

pub struct RawSaveData {
  // Please note, the priority is in save header, this is only for checking valid header/save
  priority: u16,
  is_clear: bool,
  data: Vec<u8>,
}

impl RawSaveData {
  pub fn get_priority(&self) -> &u16 {
    &self.priority
  }

  pub fn get_data(&self) -> &[u8] {
    &self.data
  }

  pub fn get_is_clear(&self) -> bool {
    self.is_clear
  }

  fn set_priority(&mut self, val: u16) {
    self.priority = val;
  }

  fn set_data(&mut self, val: &[u8]) {
    self.data.clone_from_slice(val);
  }

  fn set_is_clear(&mut self, val: bool) {
    self.is_clear = val;
  }
}

struct SaveData {
  levels: [u8; 4],
  djinn: [u32; 4],
  events: [u8; 6],
  stats: [[u16; 6]; 4],
  items: [[u16; 15]; 4],
  coins: u32,
}

// Port from Dyrati's "Golden-Sun-Password-Transfer" lua script for "vba-rr" and "Bizhawk" emulators.
struct BitArray {
  bits: Vec<u8>,
}

// Port from Dyrati's "Golden-Sun-Password-Transfer" lua script for "vba-rr" and "Bizhawk" emulators.
impl BitArray {
  // Link: https://github.com/Dyrati/Golden-Sun-Password-Transfer/blob/5ec2d52553ec8f4e0fe77854bc2b31956ac09a11/password.lua#L16
  fn push_bit(&mut self, value: u32) {
    self.bits.push((value & 1) as u8);
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
        acc *= 2;
      }
    }
    acc
  }

  fn get_len(&self) -> usize {
    self.bits.len()
  }
}

pub fn check_save_type_with_loop_start_index(raw_save_file: &[u8]) -> (bool, usize) {
  let mut is_tbs_save = false;
  let mut loop_start_index = MAX_LOOP_COUNT;
  for i in 0..MAX_LOOP_COUNT {
    let Ok(header_string) = std::str::from_utf8(&raw_save_file[(i * SAVE_SLOT_SIZE)..(i * SAVE_SLOT_SIZE + HEADER_SAVE_SLOT_NUMBER_LOCATION_INDEX)]) else { continue; };
    if !header_string.eq(HEADER_CAMELOT_ASCII_STRING) {
      continue;
    }

    for gs_build_date in GS_BUILD_DATE {
      if u16::from_le_bytes(gs_build_date) == u16::from_le_bytes([raw_save_file[i * SAVE_SLOT_SIZE + FIRST_BUILD_DATE_LOCATION_INDEX[0]], raw_save_file[i * SAVE_SLOT_SIZE + FIRST_BUILD_DATE_LOCATION_INDEX[1]]]) {
        is_tbs_save = true;
        loop_start_index = i;
        break;
      }
    }

    if is_tbs_save {
      break;
    }
  }

  (is_tbs_save, loop_start_index)
}

pub fn get_raw_save_data(to_export_all_data: bool, raw_save_file: &[u8], loop_start_index: usize) -> HashMap<u8, RawSaveData> {
  let mut all_possible_headers = Vec::new();
  // "u8" is slot number. (Start from 0)
  let mut save_data_map: HashMap<u8, RawSaveData> = HashMap::new();
  for i in loop_start_index..MAX_LOOP_COUNT {
    if raw_save_file[i * SAVE_SLOT_SIZE + HEADER_SAVE_SLOT_NUMBER_LOCATION_INDEX] > MAX_VALID_SLOT_NUMBER {
      continue;
    }
    let is_clear_save = raw_save_file[i * SAVE_SLOT_SIZE + FIRST_SAVE_MAP_LOCATION_INDEX] == CLEAR_DATA_SAVE_MAP_VALUE;
    if !is_clear_save && !to_export_all_data {
      continue;
    }
    // Get all possible valid save headers first.
    all_possible_headers.push(HeaderInfo { index: i, priority: u16::from_le_bytes([raw_save_file[i * SAVE_SLOT_SIZE + HEADER_PRIORITY_LOCATION_INDEX[0]], raw_save_file[i * SAVE_SLOT_SIZE + HEADER_PRIORITY_LOCATION_INDEX[1]]]), slot_number: raw_save_file[i * SAVE_SLOT_SIZE + HEADER_SAVE_SLOT_NUMBER_LOCATION_INDEX], is_clear: is_clear_save });
  }

  // Get all valid save data.
  for (j, header) in all_possible_headers.iter().enumerate() {
    if j > 0 && save_data_map.contains_key(&header.slot_number) {
      if let Some(existed_save_data) = save_data_map.get(&header.slot_number) {
        if header.priority > *existed_save_data.get_priority() {
          save_data_map.remove(&header.slot_number);
        } else {
          continue;
        }
      }
    }
    /* Does not include first 16 bytes header.
       The data from 0xA40 (Felix, Jenna, Sheba, PC07) is useless for password generating.
       0xA40 - 0x10 = 0xA30 */
    let mut save_data = RawSaveData { priority: 0, is_clear: false, data: vec![0; 0xA30] };
    save_data.set_data(raw_save_file.get(header.index * SAVE_SLOT_SIZE + 0x10..header.index * SAVE_SLOT_SIZE + 0xA40).unwrap());
    save_data.set_is_clear(header.is_clear);
    save_data.set_priority(header.priority);
    // Key is save slot number: 0, 1, 2
    save_data_map.insert(header.slot_number, save_data);
  }

  save_data_map
}

/* Port from Dyrati's "Golden-Sun-Password-Transfer" lua script for "vba-rr" and "Bizhawk" emulators.
   Link: https://github.com/Dyrati/Golden-Sun-Password-Transfer/blob/5ec2d52553ec8f4e0fe77854bc2b31956ac09a11/password.lua#L8 */
fn get_event_flag(raw_save: &[u8], flag: u32) -> u8 {
  let byte_pos = (flag >> 3) as usize;
  let bit_pos = flag & 7;
  (raw_save[0x40 + byte_pos] >> bit_pos) & 1
}

/* Port from Dyrati's "Golden-Sun-Password-Transfer" lua script for "vba-rr" and "Bizhawk" emulators.
   Link: https://github.com/Dyrati/Golden-Sun-Password-Transfer/blob/5ec2d52553ec8f4e0fe77854bc2b31956ac09a11/password.lua#L35 */
fn get_save_data(raw_save: &[u8]) -> SaveData {
  // [u8; 4]
  let mut levels = [0; 4];
  // [u32; 4]
  // All: [0x7F, 0x7F, 0x7F, 0x7F]
  let mut djinn = [0; 4];
  // [u8; 6]
  // All: [1, 1, 0, 1, 1, 1]
  let mut events = [0; 6];
  // [[u16; 6]; 4]
  let mut stats = [[0; 6]; 4];
  // [[u16; 15]; 4]
  let mut items = [[0; 15]; 4];

  for i in 0..4 {
    let base = 0x500 + 0x14C * i;
    levels[i] = raw_save[base + 0xF];

    for j in 0..4 {
      djinn[j] |= u32::from_le_bytes([raw_save[base + 0xF8 + 4 * j], raw_save[base + 0xF9 + 4 * j], raw_save[base + 0xFA + 4 * j], raw_save[base + 0xFB + 4 * j]]);
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
    stats[i][5] = u16::from(raw_save[base + 0x1E]);

    for j in 0..15 {
      items[i][j] = u16::from_le_bytes([raw_save[base + 0xD8 + 2 * j], raw_save[base + 0xD9 + 2 * j]]);
    }
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

  // u32
  let coins = u32::from_le_bytes([raw_save[0x250], raw_save[0x251], raw_save[0x252], raw_save[0x253]]);
  SaveData { levels, djinn, events, stats, items, coins }
}

/* Port from Dyrati's "Golden-Sun-Password-Transfer" lua script for "vba-rr" and "Bizhawk" emulators.
   Link: https://github.com/Dyrati/Golden-Sun-Password-Transfer/blob/5ec2d52553ec8f4e0fe77854bc2b31956ac09a11/password.lua#L79 */
fn gen_password_bytes(grade: PasswordGrade, save_data: &SaveData) -> Vec<u8> {
  let mut bits = BitArray { bits: Vec::new() };

  // Insert 7 bits per level, 7 bits per jinn element
  let mut level_bits = BitArray { bits: Vec::new() };
  let mut djinn_bits = BitArray { bits: Vec::new() };
  for i in (0..=3).rev() {
    level_bits.push_bits(u32::from(save_data.levels[i]), 7);
  }

  for i in (0..=3).rev() {
    djinn_bits.push_bits(save_data.djinn[i], 7);
  }

  for i in (11..=27).rev().step_by(8) {
    bits.push_bits(u32::from(level_bits.sub_bits(i - 7, i)), 8);
  }

  bits.push_bits(u32::from(level_bits.sub_bits(0, 3)), 4);
  bits.push_bits(u32::from(djinn_bits.sub_bits(24, 27)), 4);

  for i in (7..=23).rev().step_by(8) {
    bits.push_bits(u32::from(djinn_bits.sub_bits(i - 7, i)), 8);
  }

  for i in (0..=7).rev() {
    if i < save_data.events.len() {
      bits.push_bit(u32::from(save_data.events[i]));
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
    let psynergy_items = [0xC8, 0xC9, 0xCA, 0xCB, 0xCC, 0xCD, 0xCE, 0xCF];
    let mut flags = 0;
    for items_per_person in &save_data.items {
      for item in items_per_person {
        let id = item & 0x1FF;
        for (j, psynergy_item) in psynergy_items.iter().enumerate() {
          if id == *psynergy_item {
            flags |= u32::pow(2, j as u32);
          }
        }
      }
    }
    bits.push_bits(flags, 8);
  }

  // If password grade is gold or silver, insert stats.
  if !matches!(grade, PasswordGrade::Bronze) {
    for stats_per_person in &save_data.stats {
      // HP
      bits.push_bits(u32::from(stats_per_person[0]), 11);
      // EP
      bits.push_bits(u32::from(stats_per_person[1]), 11);
      // Attack
      bits.push_bits(u32::from(stats_per_person[2]), 10);
      // Defense
      bits.push_bits(u32::from(stats_per_person[3]), 10);
      // Agility
      bits.push_bits(u32::from(stats_per_person[4]), 10);
      // Luck
      bits.push_bits(u32::from(stats_per_person[5]), 8);
    }
  }

  // If password grade is gold, insert items and coins.
  if matches!(grade, PasswordGrade::Gold) {
    bits.push_bits(0, 8);
    let mut counter = 0;
    for items_per_person in &save_data.items {
      for item in items_per_person {
        let id = item & 0x1FF;
        bits.push_bits(u32::from(id), 9);
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
    for items_per_person in &save_data.items {
      for stackable_item in &stackable_items {
        let mut quantity = 0;
        for item in items_per_person {
          let id = item & 0x1FF;
          if id == *stackable_item {
            quantity = item >> 11;
          }
        }
        bits.push_bits(u32::from(quantity), 5);
      }
    }
    bits.push_bits(save_data.coins, 24);
  }

  // Append 0 until reaching the correct password size.
  for _i in 0..8 * size - bits.get_len() {
    bits.push_bit(0);
  }

  // Encrypt with key 0x1021.
  let mut x_sum = 0xFFFF_u64;

  for i in 0..size {
    let byte = u64::from(bits.sub_bits(8 * i, 8 * i + 7));
    x_sum ^= byte << 8;
    for _j in 0..8 {
      if (x_sum & 0x8000) == 0 {
        x_sum <<= 1;
      } else {
        x_sum = (x_sum << 1) + 0xFFFF_EFDF;
      }
    }
  }

  x_sum = (!x_sum) & 0xFFFF;
  let xor_value = (x_sum & 0xFF) as u32;
  bits.push_bits((x_sum >> 8) as u32, 8);
  bits.push_bits(xor_value, 8);

  for i in 0..=size {
    let byte = u32::from(bits.sub_bits(8 * i, 8 * i + 7));
    bits.replace_bits(byte ^ xor_value, 8, 8 * i);
  }

  // Split into 6-bit entries
  let mut password_bytes = Vec::new();
  let mut acc = 0_u32;
  let max_size = (size + 2) * 8;

  for i in (0..max_size).step_by(6) {
    let mut max = i + 5;
    let entry;

    if max <= bits.get_len() {
      entry = bits.sub_bits(i, max);
    } else {
      let last_left_shift = max - max_size + 1;
      max = bits.get_len() - 1;
      entry = bits.sub_bits(i, max) << last_left_shift;
    }

    password_bytes.push(entry);
    acc += u32::from(entry);
    // Insert checksum for each line
    if password_bytes.len() % 10 == 9 {
      password_bytes.push((acc & 0x3F) as u8);
      acc = 0;
    }
  }

  for (i, password_byte) in password_bytes.iter_mut().enumerate() {
    let mut temp = u16::from(*password_byte);
    temp += i as u16;
    *password_byte = (temp & 0x3F) as u8;
  }

  password_bytes
}

pub fn get_password_bytes(raw_save: &[u8], grade: PasswordGrade) -> Vec<u8> {
  let save_data = get_save_data(raw_save);
  gen_password_bytes(grade, &save_data)
}
