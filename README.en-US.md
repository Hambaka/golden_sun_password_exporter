# README (TODO)
[![zh-Hans](https://img.shields.io/badge/-%E7%AE%80%E4%BD%93%E4%B8%AD%E6%96%87-black.svg?style=for-the-badge&logo=googletranslate&logoColor=yellow)](https://github.com/Hambaka/golden_sun_password_exporter/blob/main/README.md)
[![en-US](https://img.shields.io/badge/-English-black.svg?style=for-the-badge&logo=googletranslate&logoColor=yellow)](https://github.com/Hambaka/golden_sun_password_exporter/blob/main/README.en-US.md)
---
# golden_sun_password_exporter

![Rust](https://img.shields.io/badge/language-Rust-DEA584.svg?style=flat-square&logo=rust)
[![GitHub license](https://img.shields.io/github/license/Hambaka/golden_sun_password_exporter?style=flat-square)](https://raw.githubusercontent.com/Hambaka/golden_sun_password_exporter/master/LICENSE)
![Platform](https://img.shields.io/badge/platform%20(x86--64)-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey?style=flat-square)
[![Version](https://img.shields.io/github/v/release/Hambaka/golden_sun_password_exporter?label=version&style=flat-square)](https://github.com/Hambaka/golden_sun_password_exporter/releases/latest)

Read a Golden Sun save file/password text file/password memory dump binary file to generate password text/memory dump binary/cheat files for Golden Sun: The Lost Age

## Usage (TODO)
### Commands summary
```
Usage: golden_sun_password_exporter <COMMAND>

Commands:
  sav   Read a save file to generate password text/memory dump binary/cheat files
  txt   Read a password text file to generate an another version password text/memory dump binary/cheat file
  dmp   Read a password memory dump binary file to generate a password text/cheat file
```
### Subcommand: sav (TODO)
```
Usage: golden_sun_password_exporter sav [OPTIONS] --grade <VALUE> <--text <VALUE>|--memory|--cheat <VALUE>|--export> <INPUT_FILE>

Arguments:
  <INPUT_FILE>  Golden Sun save file

Options:
  -g, --grade <VALUE>        Target password grade
  -a, --all                  Export all existing save data in save file
  -t, --text <VALUE>         Password version
  -m, --memory               Generate memory dump file
  -c, --cheat <VALUE>        Generate cheats according to the language version
  -e, --export               Export game data to a text file for Dyrati's "Golden Sun Password Generator" spreadsheet
  -o, --output <OUTPUT_DIR>  Output directory
```
### Subcommand: txt (TODO)
```
Usage: golden_sun_password_exporter txt [OPTIONS] <--grade <VALUE>|--text|--memory|--cheat <VALUE>|--export> <INPUT_FILE>

Arguments:
  <INPUT_FILE>  Golden Sun password text file

Options:
  -g, --grade <VALUE>        Target password grade
  -t, --text                 Convert password to another version
  -m, --memory               Generate memory dump file
  -c, --cheat <VALUE>        Generate cheats according to the language version
  -e, --export               Export game data to a text file for Dyrati's "Golden Sun Password Generator" spreadsheet
  -o, --output <OUTPUT_DIR>  Output directory
```
### Subcommand: dmp (TODO)
```
Usage: golden_sun_password_exporter dmp [OPTIONS] <--grade <VALUE>|--text <VALUE>|--cheat <VALUE>|--export> <INPUT_FILE>

Arguments:
  <INPUT_FILE>  Golden Sun password memory dump binary file

Options:
  -g, --grade <VALUE>        Target password grade
  -t, --text <VALUE>         Generate password text file
  -c, --cheat <VALUE>        Generate cheats according to the language version
  -e, --export               Export game data to a text file for Dyrati's "Golden Sun Password Generator" spreadsheet
  -o, --output <OUTPUT_DIR>  Output directory
```