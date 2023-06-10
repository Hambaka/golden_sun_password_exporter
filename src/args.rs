use std::path::PathBuf;
use clap::{arg, ArgAction, ArgGroup, Command, crate_version, value_parser};
use crate::util::enums::{CheatVersion, PasswordGrade, PasswordVersion};

pub fn build_cli() -> Command {
  Command::new("Golden Sun Password Exporter")
    .version(crate_version!())
    .author("Hambaka")
    .about(
      "A simple tool for a GBA game called Golden Sun.\n\n\
      You can use this tool to export password data to the following types of files:\n\
      1. Password text file. (Japanese, English)\n\
      2. Password memory dump binary file.\n\
      3. Password cheat codes text file for Golden Sun: The Lost Age. (Japan, USA/Europe, Germany, Spain, France, Italy)\n\
      4. Save data text file, which can be used in Dyrati's \"Golden Sun Password Generator\" spreadsheet.")
    .propagate_version(true)
    .subcommand_required(true)
    .arg_required_else_help(true)
    .subcommand(
      Command::new("sav")
        .about("Export password data by reading a Golden Sun save file")
        .args(&[
          arg!(<INPUT_FILE> "Golden Sun save file").value_parser(value_parser!(PathBuf)).required(true),
          arg!(-g --grade <VALUE> "Target password grade").value_parser(clap::builder::EnumValueParser::<PasswordGrade>::new()).default_value("g"),
          arg!(-a --all "Export all existing valid save data in the save file").action(ArgAction::SetTrue),
          arg!(-t --text <VALUE> "Generate the specified version password text file").value_parser(clap::builder::EnumValueParser::<PasswordVersion>::new()),
          arg!(-m --memory "Generate password memory dump binary file").action(ArgAction::SetTrue),
          arg!(-c --cheat <VALUE> "Generate the specified version password cheat codes text file").value_parser(clap::builder::EnumValueParser::<CheatVersion>::new()),
          arg!(-e --export "Export save data to a text file for Dyrati's \"Golden Sun Password Generator\"").action(ArgAction::SetTrue),
          arg!(-o --output <OUTPUT_DIR> "Output directory").value_parser(value_parser!(PathBuf))
        ])
        .group(ArgGroup::new("sav_args")
          .args(["text", "memory", "cheat", "export"])
          .required(true)
          .multiple(true)
        )
    )
    .subcommand(
      Command::new("txt")
        .about("Export password data by reading a Golden Sun password text file")
        .args(&[
          arg!(<INPUT_FILE> "Golden Sun password text file").value_parser(value_parser!(PathBuf)).required(true),
          arg!(-g --grade <VALUE> "Target password grade (for downgrade only)").value_parser(clap::builder::EnumValueParser::<PasswordGrade>::new()),
          arg!(-t --text "Convert password to another version and generate the converted file").action(ArgAction::SetTrue),
          arg!(-m --memory "Generate password memory dump binary file").action(ArgAction::SetTrue),
          arg!(-c --cheat <VALUE> "Generate the specified version password cheat codes text file").value_parser(clap::builder::EnumValueParser::<CheatVersion>::new()),
          arg!(-e --export "Generate and export save data to a text file for Dyrati's \"Golden Sun Password Generator\"").action(ArgAction::SetTrue),
          arg!(-o --output <OUTPUT_DIR> "Output directory").value_parser(value_parser!(PathBuf))
        ])
        .group(ArgGroup::new("txt_args")
          .args(["grade", "text", "memory", "cheat", "export"])
          .required(true)
          .multiple(true)
        )
    )
    .subcommand(
      Command::new("dmp")
        .about("Export password data by reading a Golden Sun password memory dump binary file")
        .args(&[
          arg!(<INPUT_FILE> "Golden Sun password memory dump binary file").value_parser(value_parser!(PathBuf)).required(true),
          arg!(-g --grade <VALUE> "Target password grade (for downgrade only)").value_parser(clap::builder::EnumValueParser::<PasswordGrade>::new()),
          arg!(-t --text <VALUE> "Generate the specified version password text file").value_parser(clap::builder::EnumValueParser::<PasswordVersion>::new()),
          arg!(-c --cheat <VALUE> "Generate the specified version password cheat codes text file").value_parser(clap::builder::EnumValueParser::<CheatVersion>::new()),
          arg!(-e --export "Generate and export save data to a text file for Dyrati's \"Golden Sun Password Generator\"").action(ArgAction::SetTrue),
          arg!(-o --output <OUTPUT_DIR> "Output directory").value_parser(value_parser!(PathBuf))
        ])
        .group(ArgGroup::new("dmp_args")
          .args(["grade", "text", "cheat", "export"])
          .required(true)
          .multiple(true)
        )
    )
}
