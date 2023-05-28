#[derive(Clone, Copy)]
pub enum PasswordGrade {
  Gold,
  Silver,
  Bronze,
}

#[derive(Clone, Copy)]
pub enum PasswordVersion {
  //Hiragana. Japanese version only
  Japanese,
  //Letter, number, and symbol.
  English,
}

#[derive(Clone, Copy)]
pub enum CheatVersion {
  Japanese,
  English,
  German,
  Spanish,
  French,
  Italian,
}

pub fn get_password_grade_by_arg(grade_arg_str: &str) -> PasswordGrade {
  match grade_arg_str {
    "s" => PasswordGrade::Silver,
    "b" => PasswordGrade::Bronze,
    // Include "g"
    _ => PasswordGrade::Gold,
  }
}

pub fn get_password_grade_by_len(password_bytes_len: usize) -> PasswordGrade {
  match password_bytes_len {
    260 => PasswordGrade::Gold,
    61 => PasswordGrade::Silver,
    16 => PasswordGrade::Bronze,
    _ => unreachable!(),
  }
}

pub fn get_password_version(text_arg_str: &str) -> PasswordVersion {
  match text_arg_str {
    "j" => PasswordVersion::Japanese,
    // Include "e"
    _ => PasswordVersion::English,
  }
}

pub fn rev_password_version(password_version: PasswordVersion) -> PasswordVersion {
  match password_version {
    PasswordVersion::Japanese => PasswordVersion::English,
    PasswordVersion::English => PasswordVersion::Japanese,
  }
}

pub fn get_cheat_version(cheat_version: &str) -> CheatVersion {
  match cheat_version {
    "j" => CheatVersion::Japanese,
    "g" => CheatVersion::German,
    "s" => CheatVersion::Spanish,
    "f" => CheatVersion::French,
    "i" => CheatVersion::Italian,
    // Include "e"
    _ => CheatVersion::English,
  }
}

// For Golden Sun: Lost Age
pub fn get_cheat_address(cheat_version: CheatVersion) -> i32 {
  match cheat_version {
    // 0x0200A78A
    CheatVersion::Japanese => 0x0200_A78A,
    // 0x0200A74A
    CheatVersion::English => 0x0200_A74A,
    // 0x0200A742
    CheatVersion::German | CheatVersion::French | CheatVersion::Italian => 0x0200_A742,
    // 0x0200A73E
    CheatVersion::Spanish => 0x0200_A73E,
  }
}