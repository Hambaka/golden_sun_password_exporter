#[derive(Clone, Copy)]
pub enum PasswordVersion {
  Hiragana,
  LetterNumberSymbol,
}

pub fn get_save_type(password: &str) -> PasswordVersion {
  let first_char = password.chars().nth(0).unwrap();
  if first_char <= 'z' {
    PasswordVersion::LetterNumberSymbol
  } else {
    PasswordVersion::Hiragana
  }
}

/*
// (Unused) Convert English version password (letters, numbers, symbols) to Japanese version password (hiragana).
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
*/

/*
// (Unused) Convert Japanese version password (hiragana) to English version password (letters, numbers, symbols).
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
*/

/*
// (Unused) Source: https://stackoverflow.com/questions/57063777/remove-all-whitespace-from-a-string
fn remove_whitespace(s: &str) -> String {
  s.chars().filter(|c| !c.is_whitespace()).collect()
}
*/

// Source: https://stackoverflow.com/questions/57063777/remove-all-whitespace-from-a-string
fn remove_whitespace_mut(s: &mut String) {
  s.retain(|c| !c.is_whitespace());
}

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
    _ => 0xFF,
  }
}

pub fn txt_to_dmp(mut input: String, password_version: PasswordVersion) -> Vec<u8> {
  remove_whitespace_mut(&mut input);
  let dmp_bytes = match password_version {
    PasswordVersion::Hiragana => input.chars().map(jp_to_byte).collect(),
    PasswordVersion::LetterNumberSymbol => input.chars().map(en_to_byte).collect(),
  };
  return dmp_bytes;
}
