use clap::builder::PossibleValue;
use clap::ValueEnum;

#[derive(Clone, Copy)]
pub enum PasswordGrade {
  Gold,
  Silver,
  Bronze,
}

impl PasswordGrade {
  pub fn is_gold(&self) -> bool {
    matches!(*self, PasswordGrade::Gold)
  }

  pub fn is_silver(&self) -> bool {
    matches!(*self, PasswordGrade::Silver)
  }

  pub fn is_bronze(&self) -> bool {
    matches!(*self, PasswordGrade::Bronze)
  }
}

impl ValueEnum for PasswordGrade {
  fn value_variants<'a>() -> &'a [Self] {
    &[Self::Gold, Self::Silver, Self::Bronze]
  }

  fn to_possible_value(&self) -> Option<PossibleValue> {
    Some(match self {
      Self::Gold => PossibleValue::new("g").help("Gold grade password"),
      Self::Silver => PossibleValue::new("s").help("Silver grade password"),
      Self::Bronze => PossibleValue::new("b").help("Bronze grade password"),
    })
  }
}

#[derive(Clone, Copy)]
pub enum PasswordVersion {
  //Hiragana. Japanese version only
  Japanese,
  //Letter, number, and symbol.
  English,
}

impl PasswordVersion {
  pub fn rev(&self) -> PasswordVersion {
    match *self {
      PasswordVersion::Japanese => PasswordVersion::English,
      PasswordVersion::English => PasswordVersion::Japanese,
    }
  }
}

impl ValueEnum for PasswordVersion {
  fn value_variants<'a>() -> &'a [Self] {
    &[Self::Japanese, Self::English]
  }

  fn to_possible_value(&self) -> Option<PossibleValue> {
    Some(match self {
      Self::Japanese => PossibleValue::new("j").help("Japanese version password"),
      Self::English => PossibleValue::new("e").help("English version password"),
    })
  }
}

#[derive(Clone, Copy)]
pub enum CheatVersion {
  Japan,
  USA,
  Europe,
  Germany,
  Spain,
  France,
  Italy,
}

// For Golden Sun: Lost Age
impl CheatVersion {
  pub fn get_address(&self) -> u32 {
    match *self {
      // 0x0200A78A
      CheatVersion::Japan => 0x0200_A78A,
      // 0x0200A74A
      CheatVersion::USA | CheatVersion::Europe => 0x0200_A74A,
      // 0x0200A742
      CheatVersion::Germany | CheatVersion::France | CheatVersion::Italy => 0x0200_A742,
      // 0x0200A73E
      CheatVersion::Spain => 0x0200_A73E,
    }
  }
}

impl ValueEnum for CheatVersion {
  fn value_variants<'a>() -> &'a [Self] {
    &[Self::Japan, Self::USA, Self::Europe, Self::Germany, Self::Spain, Self::France, Self::Italy]
  }

  fn to_possible_value(&self) -> Option<PossibleValue> {
    Some(match self {
      Self::Japan => PossibleValue::new("j").help("Ougon no Taiyou - Ushinawareshi Toki (Japan)"),
      Self::USA => PossibleValue::new("u").help("Golden Sun - The Lost Age (USA, Europe)"),
      Self::Europe => PossibleValue::new("e").help("Golden Sun - The Lost Age (USA, Europe)"),
      Self::Germany => PossibleValue::new("g").help("Golden Sun - Die Vergessene Epoche (Germany)"),
      Self::Spain => PossibleValue::new("s").help("Golden Sun - La Edad Perdida (Spain)"),
      Self::France => PossibleValue::new("f").help("Golden Sun - L'Age Perdu (France)"),
      Self::Italy => PossibleValue::new("i").help("Golden Sun - L'Era Perduta (Italy)"),
    })
  }
}
