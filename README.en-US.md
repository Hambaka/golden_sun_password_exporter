# golden_sun_password_exporter

![Rust](https://img.shields.io/badge/language-Rust-DEA584.svg?style=flat-square&logo=rust)
[![GitHub license](https://img.shields.io/github/license/Hambaka/golden_sun_password_exporter?style=flat-square)](https://raw.githubusercontent.com/Hambaka/golden_sun_password_exporter/master/LICENSE)
![Platform](https://img.shields.io/badge/platform%20(x86--64)-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey?style=flat-square)
[![Version](https://img.shields.io/github/v/release/Hambaka/golden_sun_password_exporter?label=version&style=flat-square)](https://github.com/Hambaka/golden_sun_password_exporter/releases/latest)

A simple tool for a GBA game called **Golden Sun**. You can use this tool to export password data to the following types of files:  

- Password text file (Japanese, English)
- Password memory dump binary file
- Password cheat codes text file for **Golden Sun: The Lost Age** (Japan, USA/Europe, Germany, Spain, France, Italy)
- Save data text file, which can be used in Dyrati's ["Golden Sun Password Generator"](https://www.reddit.com/r/GoldenSun/comments/jon3h7/golden_sun_password_tools/) spreadsheet

## README  

[![zh-Hans](https://img.shields.io/badge/-%E7%AE%80%E4%BD%93%E4%B8%AD%E6%96%87-black.svg?style=for-the-badge&logo=googletranslate&logoColor=gold)](https://github.com/Hambaka/golden_sun_password_exporter/blob/main/README.md)
[![en-US](https://img.shields.io/badge/-English%20(TODO)-black.svg?style=for-the-badge&logo=googletranslate&logoColor=gold)](https://github.com/Hambaka/golden_sun_password_exporter/blob/main/README.en-US.md)


## Usage  

### Command List

```text
Usage: golden_sun_password_exporter <COMMAND>

Commands:
  sav   Export password data by reading a Golden Sun save file
  txt   Export password data by reading a Golden Sun password text file
  dmp   Export password data by reading a Golden Sun password memory dump binary file
```

### Subcommand: `sav`  

Export password data by reading a Golden Sun save file.  

```text
Usage: golden_sun_password_exporter sav [OPTIONS] <--text <VALUE>|--memory|--cheat <VALUE>|--export> <INPUT_FILE>

Arguments:
  <INPUT_FILE>
          Golden Sun save file

Options:
  -g, --grade <VALUE>
          Target password grade

          [default: g]

          Possible values:
          - g: Gold grade password
          - s: Silver grade password
          - b: Bronze grade password

  -a, --all
          Export all existing valid save data in the save file

  -t, --text <VALUE>
          Generate the specified version password text file

          Possible values:
          - j: Japanese version password
          - e: English version password

  -m, --memory
          Generate password memory dump binary file

  -c, --cheat <VALUE>
          Generate the specified version password cheat codes text file

          Possible values:
          - j: Ougon no Taiyou - Ushinawareshi Toki (Japan)
          - u: Golden Sun - The Lost Age (USA, Europe)
          - e: Golden Sun - The Lost Age (USA, Europe)
          - g: Golden Sun - Die Vergessene Epoche (Germany)
          - s: Golden Sun - La Edad Perdida (Spain)
          - f: Golden Sun - L'Age Perdu (France)
          - i: Golden Sun - L'Era Perduta (Italy)

  -e, --export
          Export save data to a text file for Dyrati's "Golden Sun Password Generator"

  -o, --output <OUTPUT_DIR>
          Output directory
```

### Subcommand: `txt`  

Export password data by reading a Golden Sun password text file.  

```text
Usage: golden_sun_password_exporter txt [OPTIONS] <--grade <VALUE>|--text|--memory|--cheat <VALUE>|--export> <INPUT_FILE>

Arguments:
  <INPUT_FILE>
          Golden Sun password text file

Options:
  -g, --grade <VALUE>
          Target password grade (for downgrade only)

          Possible values:
          - g: Gold grade password
          - s: Silver grade password
          - b: Bronze grade password

  -t, --text
          Convert password to another version and generate the converted file

  -m, --memory
          Generate password memory dump binary file

  -c, --cheat <VALUE>
          Generate the specified version password cheat codes text file

          Possible values:
          - j: Ougon no Taiyou - Ushinawareshi Toki (Japan)
          - u: Golden Sun - The Lost Age (USA, Europe)
          - e: Golden Sun - The Lost Age (USA, Europe)
          - g: Golden Sun - Die Vergessene Epoche (Germany)
          - s: Golden Sun - La Edad Perdida (Spain)
          - f: Golden Sun - L'Age Perdu (France)
          - i: Golden Sun - L'Era Perduta (Italy)

  -e, --export
          Generate and export save data to a text file for Dyrati's "Golden Sun Password Generator"

  -o, --output <OUTPUT_DIR>
          Output directory
```

### Subcommand: `dmp`  
Export password data by reading a Golden Sun password memory dump binary file.  

```text
Usage: golden_sun_password_exporter dmp [OPTIONS] <--grade <VALUE>|--text <VALUE>|--cheat <VALUE>|--export> <INPUT_FILE>

Arguments:
  <INPUT_FILE>
          Golden Sun password memory dump binary file

Options:
  -g, --grade <VALUE>
          Target password grade (for downgrade only)

          Possible values:
          - g: Gold grade password
          - s: Silver grade password
          - b: Bronze grade password

  -t, --text <VALUE>
          Generate the specified version password text file

          Possible values:
          - j: Japanese version password
          - e: English version password

  -c, --cheat <VALUE>
          Generate the specified version password cheat codes text file

          Possible values:
          - j: Ougon no Taiyou - Ushinawareshi Toki (Japan)
          - u: Golden Sun - The Lost Age (USA, Europe)
          - e: Golden Sun - The Lost Age (USA, Europe)
          - g: Golden Sun - Die Vergessene Epoche (Germany)
          - s: Golden Sun - La Edad Perdida (Spain)
          - f: Golden Sun - L'Age Perdu (France)
          - i: Golden Sun - L'Era Perduta (Italy)

  -e, --export
          Generate and export save data to a text file for Dyrati's "Golden Sun Password Generator"

  -o, --output <OUTPUT_DIR>
          Output directory
```
