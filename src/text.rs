use crate::enums::PasswordVersion;

pub fn check_password_version(password: &str) -> PasswordVersion {
  let first_char = password.chars().next().unwrap();
  if first_char <= 'z' {
    PasswordVersion::English
  } else {
    PasswordVersion::Japanese
  }
}

/// Convert English version password (letters, numbers, symbols) to Japanese version password (hiragana).
fn convert_en_to_jp(input: char) -> char {
  match input {
    'A' => 'あ',
    'B' => 'い',
    'C' => 'う',
    'D' => 'え',
    'E' => 'お',

    'F' => 'か',
    'G' => 'き',
    'H' => 'く',
    'J' => 'け',
    'K' => 'こ',

    'L' => 'さ',
    'M' => 'し',
    'N' => 'す',
    'P' => 'せ',
    'Q' => 'そ',

    'R' => 'た',

    'S' => 'ち',
    'T' => 'つ',
    'U' => 'て',
    'V' => 'と',
    'W' => 'な',

    'X' => 'に',
    'Y' => 'ぬ',
    'Z' => 'ね',
    '2' => 'の',
    '3' => 'は',

    '4' => 'ひ',
    '5' => 'ふ',
    '6' => 'へ',
    '7' => 'ほ',
    '8' => 'ま',

    '9' => 'み',

    'a' => 'む',
    'b' => 'め',
    'c' => 'も',
    'd' => 'や',
    'e' => 'ゆ',

    'f' => 'よ',
    'g' => 'ら',
    'h' => 'り',
    'i' => 'る',
    'j' => 'れ',

    'k' => 'ろ',
    'm' => 'わ',
    'n' => 'ん',
    'p' => 'を',
    'q' => 'が',

    'r' => 'ぎ',

    's' => 'ぐ',
    't' => 'げ',
    'u' => 'ご',
    'v' => 'ざ',
    'w' => 'じ',

    'x' => 'ず',
    'y' => 'ぜ',
    'z' => 'ぞ',
    '!' => 'だ',
    '?' => 'で',

    '#' => 'ど',
    '&' => 'ば',
    '$' => 'び',
    '%' => 'ぶ',
    '+' => 'べ',

    '=' => 'ぼ',

    ' ' => '　',
    _ => input,
  }
}

/// Convert Japanese version password (hiragana) to English version password (letters, numbers, symbols).
fn convert_jp_to_en(input: char) -> char {
  match input {
    'あ' => 'A',
    'い' => 'B',
    'う' => 'C',
    'え' => 'D',
    'お' => 'E',

    'か' => 'F',
    'き' => 'G',
    'く' => 'H',
    'け' => 'J',
    'こ' => 'K',

    'さ' => 'L',
    'し' => 'M',
    'す' => 'N',
    'せ' => 'P',
    'そ' => 'Q',

    'た' => 'R',
    'ち' => 'S',
    'つ' => 'T',
    'て' => 'U',
    'と' => 'V',

    'な' => 'W',
    'に' => 'X',
    'ぬ' => 'Y',
    'ね' => 'Z',
    'の' => '2',

    'は' => '3',
    'ひ' => '4',
    'ふ' => '5',
    'へ' => '6',
    'ほ' => '7',

    'ま' => '8',
    'み' => '9',
    'む' => 'a',
    'め' => 'b',
    'も' => 'c',

    'や' => 'd',
    'ゆ' => 'e',
    'よ' => 'f',

    'ら' => 'g',
    'り' => 'h',
    'る' => 'i',
    'れ' => 'j',
    'ろ' => 'k',

    'わ' => 'm',
    'を' => 'p',
    'ん' => 'n',

    'が' => 'q',
    'ぎ' => 'r',
    'ぐ' => 's',
    'げ' => 't',
    'ご' => 'u',

    'ざ' => 'v',
    'じ' => 'w',
    'ず' => 'x',
    'ぜ' => 'y',
    'ぞ' => 'z',

    'だ' => '!',
    'で' => '?',
    'ど' => '#',

    'ば' => '&',
    'び' => '$',
    'ぶ' => '%',
    'べ' => '+',
    'ぼ' => '=',

    '　' => ' ',
    _ => input,
  }
}

// Source: https://stackoverflow.com/questions/57063777/remove-all-whitespace-from-a-string
fn remove_whitespace(s: &str) -> String {
  s.chars().filter(|c| !c.is_whitespace()).collect()
}

/*
// (Unused) Source: https://stackoverflow.com/questions/57063777/remove-all-whitespace-from-a-string
fn remove_whitespace_mut(s: &mut String) {
  s.retain(|c| !c.is_whitespace());
}
*/

fn jp_to_byte(input: char) -> u8 {
  match input {
    'あ' => 0x00,
    'い' => 0x01,
    'う' => 0x02,
    'え' => 0x03,
    'お' => 0x04,

    'か' => 0x05,
    'き' => 0x06,
    'く' => 0x07,
    'け' => 0x08,
    'こ' => 0x09,

    'さ' => 0x0A,
    'し' => 0x0B,
    'す' => 0x0C,
    'せ' => 0x0D,
    'そ' => 0x0E,

    'た' => 0x0F,
    'ち' => 0x10,
    'つ' => 0x11,
    'て' => 0x12,
    'と' => 0x13,

    'な' => 0x14,
    'に' => 0x15,
    'ぬ' => 0x16,
    'ね' => 0x17,
    'の' => 0x18,

    'は' => 0x19,
    'ひ' => 0x1A,
    'ふ' => 0x1B,
    'へ' => 0x1C,
    'ほ' => 0x1D,

    'ま' => 0x1E,
    'み' => 0x1F,
    'む' => 0x20,
    'め' => 0x21,
    'も' => 0x22,

    'や' => 0x23,
    'ゆ' => 0x24,
    'よ' => 0x25,

    'ら' => 0x26,
    'り' => 0x27,
    'る' => 0x28,
    'れ' => 0x29,
    'ろ' => 0x2A,

    'わ' => 0x2B,
    'を' => 0x2D,
    'ん' => 0x2C,

    'が' => 0x2E,
    'ぎ' => 0x2F,
    'ぐ' => 0x30,
    'げ' => 0x31,
    'ご' => 0x32,

    'ざ' => 0x33,
    'じ' => 0x34,
    'ず' => 0x35,
    'ぜ' => 0x36,
    'ぞ' => 0x37,

    'だ' => 0x38,
    'で' => 0x39,
    'ど' => 0x3A,

    'ば' => 0x3B,
    'び' => 0x3C,
    'ぶ' => 0x3D,
    'べ' => 0x3E,
    'ぼ' => 0x3F,
    // For invalid value.
    _ => 0xFF,
  }
}

fn en_to_byte(input: char) -> u8 {
  match input {
    'A' => 0x00,
    'B' => 0x01,
    'C' => 0x02,
    'D' => 0x03,
    'E' => 0x04,

    'F' => 0x05,
    'G' => 0x06,
    'H' => 0x07,
    'J' => 0x08,
    'K' => 0x09,

    'L' => 0x0A,
    'M' => 0x0B,
    'N' => 0x0C,
    'P' => 0x0D,
    'Q' => 0x0E,

    'R' => 0x0F,

    'S' => 0x10,
    'T' => 0x11,
    'U' => 0x12,
    'V' => 0x13,
    'W' => 0x14,

    'X' => 0x15,
    'Y' => 0x16,
    'Z' => 0x17,
    '2' => 0x18,
    '3' => 0x19,

    '4' => 0x1A,
    '5' => 0x1B,
    '6' => 0x1C,
    '7' => 0x1D,
    '8' => 0x1E,

    '9' => 0x1F,

    'a' => 0x20,
    'b' => 0x21,
    'c' => 0x22,
    'd' => 0x23,
    'e' => 0x24,

    'f' => 0x25,
    'g' => 0x26,
    'h' => 0x27,
    'i' => 0x28,
    'j' => 0x29,

    'k' => 0x2A,
    'm' => 0x2B,
    'n' => 0x2C,
    'p' => 0x2D,
    'q' => 0x2E,

    'r' => 0x2F,

    's' => 0x30,
    't' => 0x31,
    'u' => 0x32,
    'v' => 0x33,
    'w' => 0x34,

    'x' => 0x35,
    'y' => 0x36,
    'z' => 0x37,
    '!' => 0x38,
    '?' => 0x39,

    '#' => 0x3A,
    '&' => 0x3B,
    '$' => 0x3C,
    '%' => 0x3D,
    '+' => 0x3E,

    '=' => 0x3F,
    // For invalid value.
    _ => 0xFF,
  }
}

pub fn byte_to_jp(input: u8) -> char {
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
    // For invalid value.
    _ => '？',
  }
}

pub fn byte_to_en(input: u8) -> char {
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
    // For invalid value.
    _ => '※',
  }
}

pub fn txt_to_dmp(input: &str, password_version: PasswordVersion) -> Vec<u8> {
  let string = remove_whitespace(input);
  let dmp_bytes = match password_version {
    PasswordVersion::Japanese => string.chars().map(jp_to_byte).collect(),
    PasswordVersion::English => string.chars().map(en_to_byte).collect(),
  };
  dmp_bytes
}

pub fn convert_txt(password: String, password_version: PasswordVersion) -> String{
  let converted_password = match password_version {
    PasswordVersion::English => password.chars().map(convert_en_to_jp).collect(),
    PasswordVersion::Japanese => password.chars().map(convert_jp_to_en).collect(),
  };
  converted_password
}
