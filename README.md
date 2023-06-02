# README（待完善）
[![zh-Hans](https://img.shields.io/badge/-%E7%AE%80%E4%BD%93%E4%B8%AD%E6%96%87-black.svg?style=for-the-badge&logo=googletranslate&logoColor=yellow)](https://github.com/Hambaka/golden_sun_password_exporter/blob/main/README.md)
[![en-US](https://img.shields.io/badge/-English-black.svg?style=for-the-badge&logo=googletranslate&logoColor=yellow)](https://github.com/Hambaka/golden_sun_password_exporter/blob/main/README.en-US.md)
---
# golden_sun_password_exporter

![Rust](https://img.shields.io/badge/language-Rust-DEA584.svg?style=flat-square&logo=rust)
[![GitHub license](https://img.shields.io/github/license/Hambaka/golden_sun_password_exporter?style=flat-square)](https://raw.githubusercontent.com/Hambaka/golden_sun_password_exporter/master/LICENSE)
![Platform](https://img.shields.io/badge/platform%20(x86--64)-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey?style=flat-square)
[![Version](https://img.shields.io/github/v/release/Hambaka/golden_sun_password_exporter?label=version&style=flat-square)](https://github.com/Hambaka/golden_sun_password_exporter/releases/latest)

为 GBA 上的 **《黄金太阳 开启的封印》** 开发的一个很简单的小工具，你可以使用本工具将密码导出为以下的几种文件：  
- 密码文本文件（日文版，英文版）
- 密码的内存转储二进制文件
- 密码金手指文本文件，可在 **《黄金太阳 失落的时代》** 中使用（日，美，德，西，法，意版）
- 存档数据的文本文件，可以在 Dyrati 的 ["黄金太阳密码生成器"](https://www.reddit.com/r/GoldenSun/comments/jon3h7/golden_sun_password_tools/) 表格中使用

## 使用方法
### 命令一览
```
使用方法：golden_sun_password_exporter.exe <命令>

命令：
  sav   通过读取《黄金太阳 开启的封印》的存档文件来导出密码数据
  txt   通过读取《黄金太阳 开启的封印》的密码文本文件来导出密码数据
  dmp   通过读取《黄金太阳 开启的封印》密码的内存转储二进制文件来导出密码数据
```
### 子命令：`sav`
通过读取《黄金太阳 开启的封印》的存档文件来导出密码数据。  
```
使用方法：golden_sun_password_exporter.exe sav [选项] <--text <VALUE>|--memory|--cheat <VALUE>|--export> <INPUT_FILE>

参数：
  <INPUT_FILE>  《黄金太阳 开启的封印》的存档文件

选项：
  -g, --grade <VALUE>        目标密码级别 [默认值：g]
  -a, --all                  导出存档文件中所有有效的存档数据
  -t, --text <VALUE>         生成指定版本的密码文本文件
  -m, --memory               生成密码的内存转储二进制文件
  -c, --cheat <VALUE>        生成指定版本的密码金手指文本文件
  -e, --export               将存档数据导出为文本文件，可搭配 Dyrati 的“黄金太阳密码生成器”使用
  -o, --output <OUTPUT_DIR>  输出文件夹
```
### 子命令：`txt`
通过读取《黄金太阳 开启的封印》的密码文本文件来导出密码数据。  
```
使用方法：golden_sun_password_exporter.exe txt [选项] <--text|--memory|--cheat <VALUE>|--export> <INPUT_FILE>

参数：
  <INPUT_FILE>  《黄金太阳 开启的封印》的密码文本文件

选项：
  -g, --grade <VALUE>        目标密码级别（仅用于降级）
  -t, --text                 转换密码文本至另一个版本，并生成转换后的密码文本文件
  -m, --memory               生成密码的内存转储二进制文件
  -c, --cheat <VALUE>        生成指定版本的密码金手指文本文件
  -e, --export               生成存档数据并将其导出为文本文件，可搭配 Dyrati 的“黄金太阳密码生成器”使用
  -o, --output <OUTPUT_DIR>  输出文件夹
```
### 子命令：`dmp`
通过读取《黄金太阳 开启的封印》密码的内存转储二进制文件来导出密码数据。  
```
使用方法：golden_sun_password_exporter.exe dmp [选项] <--text <VALUE>|--cheat <VALUE>|--export> <INPUT_FILE>

参数：
  <INPUT_FILE>  《黄金太阳 开启的封印》密码的内存转储二进制文件

选项：
  -g, --grade <VALUE>        目标密码级别（仅用于降级）
  -t, --text <VALUE>         生成指定版本的密码文本文件
  -c, --cheat <VALUE>        生成指定版本的密码金手指文本文件
  -e, --export               生成存档数据并将其导出为文本文件，可搭配 Dyrati 的“黄金太阳密码生成器”使用
  -o, --output <OUTPUT_DIR>  输出文件夹
```