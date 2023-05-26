use std::collections::HashMap;
use crate::enums::{get_password_grade_by_len, PasswordGrade};

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
   those sections are the second half of slots 0, 1, and 2 respectively.
   But seems the second half of the save doesn't store the data for generating password.
   ----------------------------------------------------------------------------------
   Golden Sun password generating explanation document by Dyrati
   Source: https://www.reddit.com/r/GoldenSun/comments/jon3h7/golden_sun_password_tools/
           https://docs.google.com/spreadsheets/d/1jQ2Zj2F57Fb4hs0pDYLCaZL-qry7gcVxNbhpq0BJDUs/

   Step 1: Convert relevant data into fixed point binary

   Levels: 7 bits for Isaac, Garet, Ivan, Mia
   Djinn: 7 bits for Venus, Mercury, Mars, Jupiter
   Events: 1 bit for Hammet, Colosso, Hsu, Deadbeard, Vale, Vault
   Psy Items: 1 bit for each item in the "PsyItems" list (1 if in party inventory, 0 if not)
   Coins: 24 bits
   Stats: 60 bits per character, arranged as follows:
     11 bits HP, 11 bits PP, 10 bits ATK, 10 bits DEF, 10 bits AGI, 8 bits LCK
   Item IDs: 9 bits per item, 15 items per character
   Item Amounts: 5 bits for every item in the "Stackables" list (whether or not item is in inventory), for each character. Subtract 1 from in-game amount

   Step 2: Arrange bits

   Syntax
     name = value (assign value to name)
     & (concatenate)
     name[a:b] (bits of name, from left to right, inclusive, starts at 1)
     rep(name, x) (name repeated x times)

   Arrangement
     levelbits = mia_level & ivan_level & garet_level & isaac_level
     levelbits = levelbits[21:28] & levelbits[13:20] & levelbits[5:12] & levelbits[1:4]
     djinnbits = jupiter_djinn & mars_djinn & mercury_djinn & venus_djinn
     djinnbits = djinnbits[25:28] & djinnbits[17:24] & djinnbits[9:16] & djinnbits[1:8]
     eventbits = 0 & 0 & vault_bit & vale_bit & deadbeard_bit & hsu_bit & colosso_bit & hammet_bit
     [character]_stats = hp & pp & atk & def & agi & lck
     stats = isaac_stats & garet_stats & ivan_stats & mia_stats
     [character]_item_ids = [character]_item1_id & [character]_item2_id & … & [character]_item15_id
     item_ids = isaac_item_ids & garet_item_ids & ivan_item_ids & mia_item_ids
     item_ids = item_ids[1:63] & 0
              & item_ids[64:126] & 0
              & item_ids[127:189] & 0
              & item_ids[190:252] & 0
              & item_ids[253:315] & 0
              & item_ids[316:378] & 0
              & item_ids[379:441] & 0
              & item_ids[442:504] & 0
              & item_ids[505:540]
     [character]_item_amounts = [character]_herb_count & [character]_nut_count & [character]_vial_count & … & [character]_crystalpowder_count
     item_amounts = isaac_item_amounts & garet_item_amounts & ivan_item_amounts & mia_item_amounts

     gold_bits = levelbits & djinnbits & eventbits & stats & rep(0,8) & item_ids & item_amounts & coins & rep(0,40)
     silver_bits = levelbits & djinnbits & eventbits & psyitems & stats
     bronze_bits = levelbits & djinnbits & eventbits & psyitems

   Step 3: Generate key from bits

     key = 0xFFFF (16 bit register, meaning that bits exceeding bit 15 are dropped)
     for byte in bits:
       key = byte*256 xor key
       for bit in key:
           key = key << 1 (bitshift left)
           if previous operation carried out a 1: key = key + 0xEFDF

     key = not(key)

   Step 4: Encrypt bits with key
     byte1 = key[1:8]
     byte2 = key[9:16]
     append byte1 to bits
     xor every byte in bits with byte2
     append byte2 to bits

   Step 5: Divide bits into 6 bit groups.  After every 9 groups, insert a group which is their sum modulo 2^6

   Step 6: Add each group's position to itself, and take result modulo 2^6
     group_0 = (group_0 + 0) mod 2^6
     group_1 = (group_1 + 1) mod 2^6
     …
     group_n = (group_n + n) mod 2^6

   Step 7: Substitute groups for characters in the "Chars" list

   See the code at https://github.com/Dyrati/Golden-Sun-Password-Transfer/blob/main/password.lua */

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

/* Unused, since now we have a much better way to check if the save data is clear data.

   All save map(location) locations  :0x410 to 0x413, 0x418 to 0x41B
   Save map data -> Main:Minor
   All stored values are same.
   const FIRST_SAVE_MAP_MAIN_LOCATION_INDEX: [usize; 2] = [0x410, 0x411];
   const FIRST_SAVE_MAP_MINOR_LOCATION_INDEX: [usize; 2] = [0x412, 0x413];  */

/* Unused, since now we have a much better way to check if the save data is clear data.

   Clear data's save location value is -> 1:2
   const CLEAR_DATA_SAVE_MAP_VALUE: [u16; 2] = [0x01, 0x02]; */

/// For clear data, its value is 1.
/// If the save data is not a clear data, its value is 0.
const CLEAR_DATA_IDENTIFIER_VALUE: u8 = 0x01;
const CLEAR_DATA_IDENTIFIER_INDEX: usize = 0x31;

/// The 8th byte is the slot number, it only show 3 active save data in game.
const HEADER_SAVE_SLOT_NUMBER_LOCATION_INDEX: usize = 0x07;
/// The values for 3 'active' save data are: 0x00, 0x01 and 0x02.
/// For more information, please see the comment at the beginning of the code.
const MAX_VALID_SLOT_NUMBER: u8 = 0x02;
const HEADER_PRIORITY_LOCATION_INDEX: [usize; 2] = [0x0A, 0x0B];
/// Valid psynergy items in TBS
/// C8: Orb of Force
/// C9: Douse Drop
/// CA: Frost Jewel
/// CB: Lifting Gem
/// CC: Halt Gem
/// CD: Cloak Ball
/// CE: Carry Stone
/// CF: Catch Beads
const PSYNERGY_ITEMS: [u16; 8] = [0xC8, 0xC9, 0xCA, 0xCB, 0xCC, 0xCD, 0xCE, 0xCF];
/// Valid stackable items in TBS
/// B4: Herb
/// B5: Nut
/// B6: Vial
/// B7: Potion
/// BA: Psy Crystal
/// BB: Antidote
/// BC: Elixir
/// BD: Water of Life
/// BF: Power Bread
/// C0: Cookie
/// C1: Apple
/// C2: Hard Nut
/// C3: Mint
/// C4: Lucky Pepper
/// E2: Smoke Bomb
/// E3: Sleep Bomb
/// E4: Game Ticket
/// E5: Lucky Medal
/// EC: Sacred Feather
/// EE: Oil Drop
/// EF: Weasel's Claw
/// F0: Bramble Seed
/// F1: Crystal Powder
const STACKABLE_ITEMS: [u16; 23] = [0xB4, 0xB5, 0xB6, 0xB7, 0xBA, 0xBB, 0xBC, 0xBD, 0xBF, 0xC0, 0xC1, 0xC2, 0xC3, 0xC4, 0xE2, 0xE3, 0xE4, 0xE5, 0xEC, 0xEE, 0xEF, 0xF0, 0xF1];
/// Bronze: 9
/// Silver: 9, 19, 29, 39, 49, 59
/// Gold  : 9, 19, 29, 39, 49, 59, 69, 79, 89, 99, 109, 119, 129, 139, 149, 159, 169, 179, 189, 199, 209, 219, 229, 239, 249, 259
///
/// The values we want :
/// 9 - 0, 19 - 1, 29 - 2, 39 - 3, 49 - 4,
///  59 - 5, 69 - 6, 79 - 7, 89 - 8, 99 - 9,
///  109 - 10, 119 - 11, 129 - 12, 139 - 13, 149 - 14,
///  159 - 15, 169 - 16, 179 - 17, 189 - 18, 199 - 19,
///  209 - 20, 219 - 21, 229 - 22, 239 - 23, 249 - 24,
///  259 - 25
const CHECKSUM_INDEX: [usize; 26] = [9, 18, 27, 36, 45, 54, 63, 72, 81, 90, 99, 108, 117, 126, 135, 144, 153, 162, 171, 180, 189, 198, 207, 216, 225, 234];

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
  djinns: [u32; 4],
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
  fn sub_bits_u8(&mut self, min: usize, max: usize) -> u8 {
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

  // Link: https://github.com/Dyrati/Golden-Sun-Password-Transfer/blob/5ec2d52553ec8f4e0fe77854bc2b31956ac09a11/password.lua#L23
  fn sub_bits_u16(&mut self, min: usize, max: usize) -> u16 {
    let mut acc = 0;
    for i in 0..=(max - min) {
      if min < self.bits.len() {
        acc = 2 * acc + u16::from(self.bits[min + i]);
      } else {
        acc *= 2;
      }
    }
    acc
  }

  // Link: https://github.com/Dyrati/Golden-Sun-Password-Transfer/blob/5ec2d52553ec8f4e0fe77854bc2b31956ac09a11/password.lua#L23
  fn sub_bits_u32(&mut self, min: usize, max: usize) -> u32 {
    let mut acc = 0;
    for i in 0..=(max - min) {
      if min < self.bits.len() {
        acc = 2 * acc + u32::from(self.bits[min + i]);
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
    let is_clear_save = raw_save_file[i * SAVE_SLOT_SIZE + CLEAR_DATA_IDENTIFIER_INDEX] == CLEAR_DATA_IDENTIFIER_VALUE;
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
fn gen_save_data_by_raw_save(raw_save: &[u8]) -> SaveData {
  // [u8; 4]
  let mut levels = [0; 4];
  // [u32; 4]
  // All: [0x7F, 0x7F, 0x7F, 0x7F]
  let mut djinns = [0; 4];
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
      djinns[j] |= u32::from_le_bytes([raw_save[base + 0xF8 + 4 * j], raw_save[base + 0xF9 + 4 * j], raw_save[base + 0xFA + 4 * j], raw_save[base + 0xFB + 4 * j]]);
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
  SaveData { levels, djinns, events, stats, items, coins }
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
    djinn_bits.push_bits(save_data.djinns[i], 7);
  }

  for i in (11..=27).rev().step_by(8) {
    bits.push_bits(u32::from(level_bits.sub_bits_u8(i - 7, i)), 8);
  }

  bits.push_bits(u32::from(level_bits.sub_bits_u8(0, 3)), 4);
  bits.push_bits(u32::from(djinn_bits.sub_bits_u8(24, 27)), 4);

  for i in (7..=23).rev().step_by(8) {
    bits.push_bits(u32::from(djinn_bits.sub_bits_u8(i - 7, i)), 8);
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
    let mut flags = 0;
    for items_per_person in &save_data.items {
      for item in items_per_person {
        let id = item & 0x1FF;
        for (j, psynergy_item) in PSYNERGY_ITEMS.iter().enumerate() {
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

    // Insert quantities of stackable items for each energist(adept).
    for items_per_person in &save_data.items {
      for stackable_item in &STACKABLE_ITEMS {
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
    let byte = u64::from(bits.sub_bits_u8(8 * i, 8 * i + 7));
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
    let byte = u32::from(bits.sub_bits_u8(8 * i, 8 * i + 7));
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
      entry = bits.sub_bits_u8(i, max);
    } else {
      let last_left_shift = max - max_size + 1;
      max = bits.get_len() - 1;
      entry = bits.sub_bits_u8(i, max) << last_left_shift;
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

pub fn get_password_bytes_by_raw_save(raw_save: &[u8], grade: PasswordGrade) -> Vec<u8> {
  let save_data = gen_save_data_by_raw_save(raw_save);
  gen_password_bytes(grade, &save_data)
}

fn gen_save_data_by_password_bytes(password_bytes: &[u8]) -> SaveData {
  /* TEMP Default Values
     Order: Isaac, Garet, Ivan, Mia */
  let mut levels: [u8; 4] = [28, 28, 28, 28];
  /* TEMP Default Values
     Venus:   0101101 (Order: Bane, Ground, Sap, Vine, Quartz, Granite, Flint)
     Mercury: 0001111 (Order: Dew, Tonic, Hail, Spritz, Mist, Sleet, Fizz)
     Mars:    1101110 (Order: Torch, Flash, Ember, Scorch, Corona, Fever, Forge)
     Jupiter: 1001111 (Order: Luff, Squall, Kite, Smog, Zephyr, Breeze, Gust) */
  let mut djinns: [u32; 4] = [45, 15, 110, 79];
  /* TEMP Default values
     Order is: Hammet, Colosso, Hsu, Deadbeard, Vale, Vault*/
  let mut events: [u8; 6] = [0, 0, 1, 0, 0, 0];
  /* TEMP Default values
     Isaac: HP, EP, Attack, Defense, Agility, Luck
     Garet: HP, EP, Attack, Defense, Agility, Luck
     Ivan: HP, EP, Attack, Defense, Agility, Luck
     Mia: HP, EP, Attack, Defense, Agility, Luck */
  let mut stats: [[u16; 6]; 4] = [
    [241, 101, 116, 53, 117, 3],
    [257, 97, 112, 53, 103, 2],
    [220, 117, 100, 51, 124, 4],
    [230, 115, 102, 49, 109, 5]
  ];
  /* TEMP Default Values
     Isaac: Great Sword, Steel Armor, Knight's Shield, Knight's Helm
     Garet: Great Axe, Steel Armor, Knight's Shield, Knight's Helm
     Ivan:  Master Rapier, Silver Vest, Silver Armlet, Platinum Circlet
     Mia:   War Mace, Silver Vest, Silver Armlet, Platinum Circlet
  */
  let mut items: [[u16; 15]; 4] = [
    [4, 80, 121, 150, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [33, 80, 121, 150, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [19, 94, 139, 169, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [46, 94, 139, 169, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
  ];
  // Coins, literally.
  let mut coins: u32 = 0;

  let password_grade = get_password_grade_by_len(password_bytes.len());
  let mut bytes_data = vec![0_u8; password_bytes.len()];
  /* Remove each byte's position data by minus its index.
     I'm not sure if it's the correct way to do that, but it works.
     Maybe better way for this should be like this:

     if i < 0x40 {
       vec_data[i] = ((u16::from(*data) + 0x40 - (i as u16) & 0x3F) as u8;
     } else if i < 0x80 {
       vec_data[i] = ((u16::from(*data) + 0x80 - (i as u16) & 0x3F) as u8;
     } else {
       vec_data[i] = ((u16::from(*data) + 0x100 - (i as u16) & 0x3F) as u8;
     }

     But "+ 0x100" works for all elements, so...

     Notes about Hex to Bin:
     0x40  ->    100 0000
     0x80  ->   1000 0000
     0x100 -> 1 0000 0000
     0x3F  ->     11 1111 */
  for (i, data) in password_bytes.iter().enumerate() {
    bytes_data[i] = ((u16::from(*data) + 0x100 - (i as u16)) & 0x3F) as u8;
  }

  /* Remove all checksum bytes.
     Those checksum bytes are useless for getting game data. */
  match password_grade {
    PasswordGrade::Gold => {
      // Or you can use: for i in 9..260.step_by(9)
      for i in CHECKSUM_INDEX {
        bytes_data.remove(i);
      }
    }
    PasswordGrade::Silver => {
      // Or you can use: for i in 9..61.step_by(9)
      for index in CHECKSUM_INDEX.iter().take(6) {
        bytes_data.remove(*index);
      }
    }
    PasswordGrade::Bronze => {
      bytes_data.remove(CHECKSUM_INDEX[0]);
    }
  }

  // Convert bytes to bit array
  let size = match password_grade {
    // Gold: 260 - 26 = 234 => 173
    PasswordGrade::Gold => 173,
    // Silver: 61 - 6 = 55 => 39
    PasswordGrade::Silver => 39,
    // Bronze: 16 - 1 = 15 => 9
    PasswordGrade::Bronze => 9,
  };
  let mut bits = BitArray { bits: Vec::new() };
  let max_size = (size + 2) * 8;

  for (j, i) in (0..max_size).step_by(6).enumerate() {
    let max = i + 5;
    let entry = bytes_data[j];

    if max <= max_size {
      bits.push_bits(u32::from(entry), 6);
    } else {
      let last_right_shift = max - max_size + 1;
      bits.push_bits(u32::from(entry >> last_right_shift), max_size - i);
    }
  }

  // Get key to decrypt all bits with it.
  let key_second_half = bits.sub_bits_u8(max_size - 8, max_size - 1);

  /* We only need the second half of the key to decrypt all bits.
   But if you really want to know the full key, then:

   let key_first_half_encrypted = bits.sub_bits(max_size-16, max_size-9);
   let key_first_half = key_first_half_encrypted ^ key_second_half;
   let key = u16::from_be_bytes([key_first_half,key_second_half]); */


  // Decrypt bits
  for i in 0..=size {
    let byte = u32::from(bits.sub_bits_u8(8 * i, 8 * i + 7));
    bits.replace_bits(byte ^ u32::from(key_second_half), 8, 8 * i);
  }

  /* Remove all useless bits.
     Gold grade:
     Step 1: Remove 16 bits of the key for encryption/decryption, and those 40 useless "blank" bits before the key bits.
     Step 2: Remove 8 useless bits in item bits, since the password already appended a 0 bit every 7 items. (each item take 9 bits, so that means, it append a 0 bit every 63 bits)
     Step 3: Remove 8 useless "blank" bits ("00000000") between stats bits and item bits.
     Step 4: Remove 2 useless "blank" bits ("00") before events' bits.

     Silver and bronze grades:
     Step 1: Remove 16 bits of the key for encryption/decryption.
     Step 2: Remove two useless bits "00" before events' bits. */
  match password_grade {
    PasswordGrade::Gold => {
      // 16 + 40
      for _i in 0..56 {
        bits.bits.remove(bits.get_len() - 1);
      }
      // 28 + 28 + 8 + 240 + 8
      for i in (1..=8).rev() {
        bits.bits.remove(312 + i * 64);
      }
      // 28 + 28 + 8 + 240
      for _i in 0..8 {
        bits.bits.remove(304);
      }
      // 28 + 28
      for _i in 0..2 {
        bits.bits.remove(56);
      }
    }
    PasswordGrade::Silver | PasswordGrade::Bronze => {
      for _i in 0..16 {
        bits.bits.remove(bits.get_len() - 1);
      }
      for _i in 0..2 {
        bits.bits.remove(56);
      }
    }
  }

  /* Get level bits, rearrange it to normal order.
     Each level value has 7 bits.
     Order is: Mia, Ivan, Garet, Isaac */
  let mut level_bits = BitArray { bits: Vec::new() };
  level_bits.push_bits(u32::from(bits.sub_bits_u8(24, 27)), 4);
  for i in (7..=23).rev().step_by(8) {
    level_bits.push_bits(u32::from(bits.sub_bits_u8(i - 7, i)), 8);
  }
  for (i, level) in levels.iter_mut().enumerate() {
    *level = level_bits.sub_bits_u8(21 - i * 7, 27 - i * 7);
  }

  /* Get djinn bits, rearrange it to normal order.
     Each element has 7 bits.
     Order is: Jupiter, Mars, Mercury, Venus */
  let mut djinn_bits = BitArray { bits: Vec::new() };
  for i in (39..=55).rev().step_by(8) {
    djinn_bits.push_bits(u32::from(bits.sub_bits_u8(i - 7, i)), 8);
  }
  djinn_bits.push_bits(u32::from(bits.sub_bits_u8(28, 31)), 4);
  for (i, djinn) in djinns.iter_mut().enumerate() {
    *djinn = u32::from(djinn_bits.sub_bits_u8(21 - i * 7, 27 - i * 7));
  }

  /* Get all event bits.
     Order is: Vault, Vale, Deadbeard, Hsu, Colosso, Hammet*/
  for (i, event) in events.iter_mut().enumerate() {
    *event = bits.bits[61 - i];
  }

  /* If password grade is silver or bronze, check 8 psynergy items.
     if it does has one, insert it back to Isaac's inventory */
  if !matches!(password_grade, PasswordGrade::Gold) {
    let mut isaac_inventory_start_index = 4;
    for (i, psynergy_item) in PSYNERGY_ITEMS.iter().enumerate() {
      if bits.bits[62 + i] == 1 {
        items[0][isaac_inventory_start_index] = *psynergy_item;
        isaac_inventory_start_index += 1;
      }
    }
  }

  // If password grade is gold or silver, get stats.
  if !matches!(password_grade, PasswordGrade::Bronze) {
    let start_index = if matches!(password_grade, PasswordGrade::Gold) {
      62
    } else {
      70
    };

    for (i, stats_per_person) in stats.iter_mut().enumerate() {
      stats_per_person[0] = bits.sub_bits_u16(start_index + i * 60, start_index + 10 + i * 60);
      stats_per_person[1] = bits.sub_bits_u16(start_index + 11 + i * 60, start_index + 21 + i * 60);
      stats_per_person[2] = bits.sub_bits_u16(start_index + 22 + i * 60, start_index + 31 + i * 60);
      stats_per_person[3] = bits.sub_bits_u16(start_index + 32 + i * 60, start_index + 41 + i * 60);
      stats_per_person[4] = bits.sub_bits_u16(start_index + 42 + i * 60, start_index + 51 + i * 60);
      stats_per_person[5] = bits.sub_bits_u16(start_index + 52 + i * 60, start_index + 59 + i * 60);
    }
  }

  if matches!(password_grade, PasswordGrade::Gold) {
    let stackable_item_index_map: HashMap<u16, usize> = HashMap::from([
      (0xB4, 0), (0xB5, 1), (0xB6, 2), (0xB7, 3), (0xBA, 4),
      (0xBB, 5), (0xBC, 6), (0xBD, 7), (0xBF, 8), (0xC0, 9),
      (0xC1, 10), (0xC2, 11), (0xC3, 12), (0xC4, 13), (0xE2, 14),
      (0xE3, 15), (0xE4, 16), (0xE5, 17), (0xEC, 18), (0xEE, 19),
      (0xEF, 20), (0xF0, 21), (0xF1, 22)
    ]);

    let item_id_start_index = 302;
    let stackable_item_quantity_start_index = 842;
    for (i, items_per_person) in items.iter_mut().enumerate() {
      for (j, item) in items_per_person.iter_mut().enumerate() {
        let item_id = bits.sub_bits_u16(item_id_start_index + i * 135 + j * 9, item_id_start_index + 8 + i * 135 + j * 9);
        if let Some(stackable_item_index) = stackable_item_index_map.get(&item_id) {
          let item_quantity = bits.sub_bits_u8(stackable_item_quantity_start_index + i * 115 + stackable_item_index * 5, stackable_item_quantity_start_index + 4 + i * 115 + stackable_item_index * 5);
          if item_quantity > 0 {
            let quantity_part = u16::from(item_quantity) << 11;
            *item = quantity_part + item_id;
          }
        } else {
          *item = item_id;
        }
      }
    }
    coins = bits.sub_bits_u32(bits.get_len() - 24, bits.get_len() - 1);
  }

  SaveData { levels, djinns, events, stats, items, coins }
}

pub fn get_is_able_to_downgrade(source_grade: PasswordGrade, target_grade: PasswordGrade) -> bool {
  match source_grade {
    PasswordGrade::Gold => {
      match target_grade {
        PasswordGrade::Gold | PasswordGrade::Silver | PasswordGrade::Bronze => true,
      }
    }
    PasswordGrade::Silver => {
      match target_grade {
        PasswordGrade::Gold => false,
        PasswordGrade::Silver | PasswordGrade::Bronze => true,
      }
    }
    PasswordGrade::Bronze => {
      match target_grade {
        PasswordGrade::Gold | PasswordGrade::Silver => false,
        PasswordGrade::Bronze => true,
      }
    }
  }
}

pub fn get_is_no_need_to_downgrade(source_grade: PasswordGrade, target_grade: PasswordGrade) -> bool {
  match source_grade {
    PasswordGrade::Gold => {
      match target_grade {
        PasswordGrade::Gold => true,
        PasswordGrade::Silver | PasswordGrade::Bronze => false,
      }
    }
    PasswordGrade::Silver => {
      match target_grade {
        PasswordGrade::Gold | PasswordGrade::Silver => true,
        PasswordGrade::Bronze => false,
      }
    }
    PasswordGrade::Bronze => {
      match target_grade {
        PasswordGrade::Gold | PasswordGrade::Silver | PasswordGrade::Bronze => true,
      }
    }
  }
}

pub fn get_password_bytes_by_password_bytes(password_bytes: &[u8], grade: PasswordGrade) -> Vec<u8> {
  let save_data = gen_save_data_by_password_bytes(password_bytes);
  gen_password_bytes(grade, &save_data)
}
