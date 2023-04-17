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

pub fn get_password_grade(grade_arg_str: &str) -> PasswordGrade {
  match grade_arg_str {
    "g" => PasswordGrade::Gold,
    "s" => PasswordGrade::Silver,
    "b" => PasswordGrade::Bronze,
    _ => PasswordGrade::Gold,
  }
}

pub fn get_password_version(text_arg_str: &str) -> PasswordVersion {
  match text_arg_str {
    "j" => PasswordVersion::Japanese,
    "e" => PasswordVersion::English,
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
    "e" => CheatVersion::English,
    "g" => CheatVersion::German,
    "s" => CheatVersion::Spanish,
    "f" => CheatVersion::French,
    "i" => CheatVersion::Italian,
    _ => CheatVersion::English,
  }
}

// For Golden Sun: Lost Age
pub fn get_cheat_address(cheat_version: CheatVersion) -> i32 {
  match cheat_version {
    CheatVersion::Japanese => 0x0200A78A,
    CheatVersion::English => 0x0200A74A,
    CheatVersion::German => 0x0200A742,
    CheatVersion::Spanish => 0x0200A73E,
    CheatVersion::French => 0x0200A742,
    CheatVersion::Italian => 0x0200A742,
  }
}