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

## README  

[![zh-Hans](https://img.shields.io/badge/-%E7%AE%80%E4%BD%93%E4%B8%AD%E6%96%87-black.svg?style=for-the-badge&logo=googletranslate&logoColor=gold)](https://github.com/Hambaka/golden_sun_password_exporter/blob/main/README.md)
[![en-US](https://img.shields.io/badge/-English%20(TODO)-black.svg?style=for-the-badge&logo=googletranslate&logoColor=gold)](https://github.com/Hambaka/golden_sun_password_exporter/blob/main/README.en-US.md)

## 使用方法

### 命令一览

```text
使用方法：golden_sun_password_exporter <命令>

命令：
  sav   通过读取《黄金太阳 开启的封印》的存档文件来导出密码数据
  txt   通过读取《黄金太阳 开启的封印》的密码文本文件来导出密码数据
  dmp   通过读取《黄金太阳 开启的封印》密码的内存转储二进制文件来导出密码数据
```

---

### 子命令：`sav`

通过读取《黄金太阳 开启的封印》的存档文件来导出密码数据。  

```text
使用方法：golden_sun_password_exporter sav [选项] <--text <VALUE>|--memory|--cheat <VALUE>|--export> <INPUT_FILE>

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

#### `sav`的参数和选项说明  

- 各参数和选项的输入位置随意，没有先后顺序的限制。
- `<INPUT_FILE>` 为《黄金太阳 开启的封印》的存档文件，任意版本的存档文件皆可，**必要参数**。
- `grade` 为**可选选项**，不使用时会自动指定默认值`g`，若使用则需要手动指定，有效的值为：
  - `g, s, b`
  - `g：金级别密码，s：银级别密码，b：铜级别密码`
- `all` 为**可选选项**，若不使用只会导出通关存档的数据，若使用则会导出全部存档的数据。
- `text` 为**可选选项**，若使用则需要手动指定值，有效的值为：
  - `j, e`
  - `j：日文版密码，e：英文版密码`
- `memory` 为**可选选项**。
- `cheat` 为**可选选项**，若使用则需要手动指定值，有效的值为：
  - `j, u, e, g, s, f, i`
  - `j：日版，u：欧/美版，e：欧/美版，g：德版，s：西班牙版，f：法版，i：意大利版，`
- `export` 为**可选选项**。
- `text`，`memory`，`cheat` 和 `export` 虽皆为可选选项，但是**必须要有其中一个**。
- `output` 是**可选选项**，若不使用会默认在输入文件的同目录下创建文件夹保存生成的文件。

#### `sav`的示例  

完整命令：

```bash
golden_sun_password_exporter sav 存档.sav --grade g --all --text e --memory --cheat e --export --output 输出文件夹
```

完整命令简易版：

```bash
golden_sun_password_exporter sav 存档.sav -g g -t e -c e -ame -o 输出文件夹 
```

其余示例（可自行调整）：

```bash
# 仅导出通关存档的数据；密码级别为默认的金级别；仅导出日文版的密码文本；
# 在输入文件的同目录下创建文件夹保存生成的文件
golden_sun_password_exporter sav 存档.sav -t j
# 不管是不是通关存档，数据全部导出；密码级别为银级别；
# 导出英文版的《黄金太阳 失落的时代》密码金手指；导出密码的内存转储二进制文件；
# 在输入文件的同目录下创建文件夹保存生成的文件
golden_sun_password_exporter sav 存档.sav -g s -c e -am
```

---

### 子命令：`txt`

通过读取《黄金太阳 开启的封印》的密码文本文件来导出密码数据。  

```text
使用方法：golden_sun_password_exporter txt [选项] <--grade <VALUE>|--text|--memory|--cheat <VALUE>|--export> <INPUT_FILE>

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

#### `txt`的参数和选项说明

- 各参数和选项的输入位置随意，没有先后顺序的限制。
- `<INPUT_FILE>` 为《黄金太阳 开启的封印》的密码文本文件，日文版和英文版密码皆可，**必要参数**。
- `grade` 为**可选选项**，若指定的级别高于输入文件的级别，则程序退出，若级别相同则忽略，若可降级则生成降级后的密码文本文件。若没有使用 `text` 选项，输出的密码版本同输入文件，有效的值为：
  - `g, s, b`
  - `g：金级别密码，s：银级别密码，b：铜级别密码`
- `text` 为**可选选项**，若使用则自动检测输入的密码文本的版本，并转换输出另外一个版本的密码文本文件，即日转英，英转日。
- `memory` 为**可选选项**。
- `cheat` 为**可选选项**，若使用则需要手动指定值，有效的值为：
  - `j, u, e, g, s, f, i`
  - `j：日版，u：欧/美版，e：欧/美版，g：德版，s：西班牙版，f：法版，i：意大利版，`
- `export` 为**可选选项**。
- `grade`, `text`，`memory`，`cheat` 和 `export` 虽皆为可选选项，但是**必须要有其中一个**。
- `output` 是**可选选项**，若不使用会默认在输入文件的同目录下创建文件夹保存生成的文件。

#### `txt`的示例

完整命令：

```bash
golden_sun_password_exporter txt 密码.txt --grade g --text --memory --cheat e --export --output 输出文件夹
```

完整命令简易版：

```bash
golden_sun_password_exporter txt 密码.txt -g g -c e -tme -o 输出文件夹
```

其余示例（可自行调整）：

```bash
# 仅转换密码版本；在输入文件的同目录下创建文件夹保存生成的转换后的密码文本文件
golden_sun_password_exporter txt 密码.txt -t
# 将密码降级为铜级别（前提是可降级），如果可降级则同时输出源文件语言版本的密码文本文件；
# 导出英文版的《黄金太阳 失落的时代》密码金手指；导出密码的内存转储二进制文件；
# 在输入文件的同目录下创建文件夹保存生成的文件
golden_sun_password_exporter txt 密码.txt -g b -c e -m
```

---

### 子命令：`dmp`

通过读取《黄金太阳 开启的封印》密码的内存转储二进制文件来导出密码数据。  

```text
使用方法：golden_sun_password_exporter dmp [选项] <--grade <VALUE>|--text <VALUE>|--cheat <VALUE>|--export> <INPUT_FILE>

参数：
  <INPUT_FILE>  《黄金太阳 开启的封印》密码的内存转储二进制文件

选项：
  -g, --grade <VALUE>        目标密码级别（仅用于降级）
  -t, --text <VALUE>         生成指定版本的密码文本文件
  -c, --cheat <VALUE>        生成指定版本的密码金手指文本文件
  -e, --export               生成存档数据并将其导出为文本文件，可搭配 Dyrati 的“黄金太阳密码生成器”使用
  -o, --output <OUTPUT_DIR>  输出文件夹
```

#### `dmp`的参数和选项说明

- 各参数和选项的输入位置随意，没有先后顺序的限制。
- `<INPUT_FILE>` 为《黄金太阳 开启的封印》密码的内存转储二进制文件，**必要参数**。
- `grade` 为**可选选项**，若指定的级别高于输入文件的级别，则程序退出，若级别相同则忽略，若可降级则生成降级后的内存转储二进制文件，有效的值为：
  - `g, s, b`
  - `g：金级别密码，s：银级别密码，b：铜级别密码`
- `text` 为**可选选项**，若使用则需要手动指定值，有效的值为：
  - `j, e`
  - `j：日文版密码，e：英文版密码`
- `cheat` 为**可选选项**，若使用则需要手动指定值，有效的值为：
  - `j, u, e, g, s, f, i`
  - `j：日版，u：欧/美版，e：欧/美版，g：德版，s：西班牙版，f：法版，i：意大利版，`
- `export` 为**可选选项**。
- `grade`，`memory`，`cheat` 和 `export` 虽皆为可选选项，但是**必须要有其中一个**。
- `output` 是**可选选项**，若不使用会默认在输入文件的同目录下创建文件夹保存生成的文件。

#### `dmp`的示例  

完整命令：

```bash
golden_sun_password_exporter dmp 密码.dmp --grade g --text e --cheat e --export --output 输出文件夹
```

完整命令简易版：

```bash
golden_sun_password_exporter dmp 密码.dmp -g g -t e -c e -e -o 输出文件夹
```

其余示例（可自行调整）：

```bash
# 在输入文件的同目录下创建文件夹，保存生成的日文版密码文本文件
golden_sun_password_exporter dmp 密码.dmp -t j
# 将密码降级为铜级别（前提是可降级），如果可降级则同时输出降级后的密码内存转储二进制文件；
# 导出存档数据为文本，可以在 Dyrati 的“黄金太阳密码生成器”电子表格中使用；
# 在输入文件的同目录下创建文件夹保存生成的文件
golden_sun_password_exporter dmp 密码.dmp -g b -e
```
