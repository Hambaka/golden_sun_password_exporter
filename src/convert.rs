use crate::enums::PasswordVersion;

/// Japanese Hiragana characters in Japanese version password.
/// Please note that 'を' and 'ん' are arranged in reverse order, because 'を' is 0x2D and 'ん' is 0x2C.
const VALID_PASSWORD_CHARS_JP: [char; 64] = [
  'あ', 'い', 'う', 'え', 'お',
  'か', 'き', 'く', 'け', 'こ',
  'さ', 'し', 'す', 'せ', 'そ',
  'た', 'ち', 'つ', 'て', 'と',
  'な', 'に', 'ぬ', 'ね', 'の',
  'は', 'ひ', 'ふ', 'へ', 'ほ',
  'ま', 'み', 'む', 'め', 'も',
  'や', 'ゆ', 'よ',
  'ら', 'り', 'る', 'れ', 'ろ',
  'わ', 'ん', 'を',
  'が', 'ぎ', 'ぐ', 'げ', 'ご',
  'ざ', 'じ', 'ず', 'ぜ', 'ぞ',
  'だ', 'で', 'ど',
  'ば', 'び', 'ぶ', 'べ', 'ぼ'
];

/// English letters, numbers, signs in English version password.
const VALID_PASSWORD_CHARS_EN: [char; 64] = [
  'A', 'B', 'C', 'D', 'E',
  'F', 'G', 'H', 'J', 'K',
  'L', 'M', 'N', 'P', 'Q',
  'R',
  'S', 'T', 'U', 'V', 'W',
  'X', 'Y', 'Z', '2', '3',
  '4', '5', '6', '7', '8',
  '9',
  'a', 'b', 'c', 'd', 'e',
  'f', 'g', 'h', 'i', 'j',
  'k', 'm', 'n', 'p', 'q',
  'r',
  's', 't', 'u', 'v', 'w',
  'x', 'y', 'z', '!', '?',
  '#', '&', '$', '%', '+',
  '='
];

/// All valid values for password bytes.
const VALID_PASSWORD_BYTES: [u8; 64] = [
  0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
  0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
  0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F,
  0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F
];

/*
// (Unused) Source: https://stackoverflow.com/questions/57063777/remove-all-whitespace-from-a-string
fn remove_whitespace_mut(s: &mut String) {
  s.retain(|c| !c.is_whitespace());
}
*/

// Source: https://stackoverflow.com/questions/57063777/remove-all-whitespace-from-a-string
pub fn remove_whitespace(s: &str) -> String {
  s.chars().filter(|c| !c.is_whitespace()).collect()
}

pub fn get_password_version(password: &str) -> PasswordVersion {
  let first_char = password.chars().next().unwrap();
  if first_char <= 'z' {
    PasswordVersion::English
  } else {
    PasswordVersion::Japanese
  }
}

pub fn contains_invalid_char_en(password: &str) -> bool {
  let mut result = false;
  for char in password.chars() {
    if !VALID_PASSWORD_CHARS_EN.contains(&char) {
      result = true;
      break;
    }
  }
  result
}

pub fn contains_invalid_char_jp(password: &str) -> bool {
  let mut result = false;
  for char in password.chars() {
    if !VALID_PASSWORD_CHARS_JP.contains(&char) {
      result = true;
      break;
    }
  }
  result
}

pub fn contains_invalid_byte(password_bytes: &[u8]) -> bool {
  let mut result = false;
  for byte in password_bytes {
    if !VALID_PASSWORD_BYTES.contains(byte) {
      result = true;
      break;
    }
  }
  result
}

/// Convert English version password (letters, numbers, signs) to Japanese version password (hiragana).
fn convert_en_to_jp(input: char) -> char {
  let index = VALID_PASSWORD_CHARS_EN.iter().position(|&x| x == input).unwrap();
  VALID_PASSWORD_CHARS_JP[index]
}

/// Convert Japanese version password (hiragana) to English version password (letters, numbers, signs).
fn convert_jp_to_en(input: char) -> char {
  let index = VALID_PASSWORD_CHARS_JP.iter().position(|&x| x == input).unwrap();
  VALID_PASSWORD_CHARS_EN[index]
}

fn convert_jp_to_byte(input: char) -> u8 {
  let index = VALID_PASSWORD_CHARS_JP.iter().position(|&x| x == input).unwrap();
  VALID_PASSWORD_BYTES[index]
}

fn convert_en_to_byte(input: char) -> u8 {
  let index = VALID_PASSWORD_CHARS_EN.iter().position(|&x| x == input).unwrap();
  VALID_PASSWORD_BYTES[index]
}

pub fn convert_byte_to_jp(input: u8) -> char {
  let index = VALID_PASSWORD_BYTES.iter().position(|&x| x == input).unwrap();
  VALID_PASSWORD_CHARS_JP[index]
}

pub fn convert_byte_to_en(input: u8) -> char {
  let index = VALID_PASSWORD_BYTES.iter().position(|&x| x == input).unwrap();
  VALID_PASSWORD_CHARS_EN[index]
}

pub fn convert_txt_to_dmp(password: &str, password_version: PasswordVersion) -> Vec<u8> {
  let dmp_bytes = match password_version {
    PasswordVersion::Japanese => password.chars().map(convert_jp_to_byte).collect(),
    PasswordVersion::English => password.chars().map(convert_en_to_byte).collect(),
  };
  dmp_bytes
}

pub fn convert_txt(password: &str, password_version: PasswordVersion) -> String {
  let converted_password = match password_version {
    PasswordVersion::English => password.chars().map(convert_en_to_jp).collect(),
    PasswordVersion::Japanese => password.chars().map(convert_jp_to_en).collect(),
  };

  converted_password
}
