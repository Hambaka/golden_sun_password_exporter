mod args;
mod cmd;
mod util;

fn main() {
  let matches = args::build_cli().get_matches();
  match matches.subcommand() {
    Some(("sav", sub_matches)) => cmd::sav::run(sub_matches),
    Some(("txt", sub_matches)) => cmd::txt::run(sub_matches),
    Some(("dmp", sub_matches)) => cmd::dmp::run(sub_matches),
    _ => unreachable!(),
  }
}
