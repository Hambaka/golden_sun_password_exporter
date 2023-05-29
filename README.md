# README（待完成）
[![zh-Hans](https://img.shields.io/badge/-%E7%AE%80%E4%BD%93%E4%B8%AD%E6%96%87-black.svg?style=for-the-badge&logo=googletranslate&logoColor=yellow)](https://github.com/Hambaka/golden_sun_password_exporter/blob/main/README.md)
[![en-US](https://img.shields.io/badge/-English-black.svg?style=for-the-badge&logo=googletranslate&logoColor=yellow)](https://github.com/Hambaka/golden_sun_password_exporter/blob/main/README.en-US.md)
---
# golden_sun_password_exporter

![Rust](https://img.shields.io/badge/language-Rust-DEA584.svg?style=flat-square&logo=rust)
[![GitHub license](https://img.shields.io/github/license/Hambaka/golden_sun_password_exporter?style=flat-square)](https://raw.githubusercontent.com/Hambaka/golden_sun_password_exporter/master/LICENSE)
![Platform](https://img.shields.io/badge/platform%20(x86--64)-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey?style=flat-square)
[![Version](https://img.shields.io/github/v/release/Hambaka/golden_sun_password_exporter?label=version&style=flat-square)](https://github.com/Hambaka/golden_sun_password_exporter/releases/latest)

　　读取《黄金太阳 开启的封印》的存档文件/密码文本文件/密码内存转储二进制文件，以生成可用于《黄金太阳 失落的时代》的密码文本文件/密码内存转储二进制文件/金手指文件。  

## 使用方法（待完成）
### 命令总览
```
使用方法： golden_sun_password_exporter <命令>

命令：
  sav   读取存档文件以生成密码文本/内存转储/金手指
  txt   读取密码文本文件以生成另一个版本的密码文本文件/生成内存转储/金手指
  dmp   读取密码内存转储二进制文件来生成密码文本文件/金手指
```
### 子命令：sav（待完成）
```
使用方法： golden_sun_password_exporter sav [选项] --grade <密码级别> <--text <密码版本>|--memory|--cheat <金手指版本>|--export> <要读取的存档文件>

参数：
  <要读取的存档文件>  《黄金太阳 开启的封印》的存档文件

选项：
  -g, --grade <密码级别>              生成的密码级别
  -a, --all                           导出存档文件中的所有存档的数据
  -t, --text <密码版本>               密码版本
  -m, --memory                        生成密码的内存转储文件
  -c, --cheat <金手指版本>            生成指定语言版本的密码金手指
  -e, --export                        导出游戏数据到文本文件，可用搭配 Dyrati 的 "Golden Sun Password Generator" 密码生成器电子表格一起使用
  -o, --output <输出文件夹路径>       输出文件夹路径
```
### 子命令：txt（待完成）
```
使用方法： golden_sun_password_exporter txt [选项] <--grade <密码级别>|--text|--memory|--cheat <金手指版本>|--export> <要读取的密码文本文件>

参数：
  <要读取的密码文本文件>  黄金太阳的密码文本文件

Options:
  -g, --grade <密码级别>              生成的密码级别
  -t, --text                          转换文本密码至另一个版本
  -m, --memory                        生成密码的内存转储文件
  -c, --cheat <金手指版本>            生成指定语言版本的密码金手指
  -e, --export                        导出游戏数据到文本文件，可用搭配 Dyrati 的 "Golden Sun Password Generator" 密码生成器电子表格一起使用
  -o, --output <输出文件夹路径>       输出文件夹路径
```
### 子命令：dmp（待完成）
```
使用方法： golden_sun_password_exporter dmp [选项] <--grade <密码级别>|--text <密码版本>|--cheat <金手指版本>|--export> <要读取的密码内存转储文件>

参数：
  <要读取的密码内存转储文件>  黄金太阳的密码内存转储二进制文件

选项：
  -g, --grade <密码级别>              生成的密码级别
  -t, --text <密码版本>               生成密码文本文件
  -c, --cheat <金手指版本>            生成指定语言版本的密码金手指
  -e, --export                        导出游戏数据到文本文件，可用搭配 Dyrati 的 "Golden Sun Password Generator" 密码生成器电子表格一起使用
  -o, --output <输出文件夹路径>       输出文件夹路径
```