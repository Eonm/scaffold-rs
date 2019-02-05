# scaffold-rs

![GitHub language count](https://img.shields.io/badge/language-rust-blue.svg) ![GitHub](https://img.shields.io/github/license/mashape/apistatus.svg) [![Build Status](https://travis-ci.org/Eonm/scaffold-rs.svg?branch=master)](https://travis-ci.org/Eonm/scaffold-rs)


Create files and directories recursively

## Usage

__Display help:__

```sh
  scaffold-rs -h
```

__Basic usage :__

```sh
  scaffold-rs scaffold -t template_file.json
```

```sh
  scaffold-rs scaffold -i "input_string"
```

__Dry run :__

Only display paths. Generated files and directories won't be created.

```sh
  scaffold-rs scaffold -t template_file.json --dry-run
```

__Add more verbosity :__

```sh
  scaffold-rs scaffold -t template_file.json -v
```

## Templates

Saffold-rs uses json templates to create files and directories.

```json
{
	"name": "fake model",
	"paths": [
		"./path_[1-20]/file_[2-10].txt",
		"file_[2-10].txt"
	],
	"licence": "MIT",
	"author": "eonm",
	"email": "",
	"notes": ""
}
```

A typical template contains the following elements:
* a name
* a list of paths
* a licence
* an author
* an email
* some notes

__Only the name and the paths are compulsory fields__

### Path syntax

__Directories :__

Unlike files, directories paths always end with "/".

__Range :__

Ranges are written as follow : __[startingNumber-endingNumber]__

```sh
./dir/files_[1-6].txt
# will produce :

## dir are created first
# ./dir/

## files are created next
# ./dir/files_1.txt
# ./dir/files_2.txt
# ./dir/files_3.txt
# ./dir/files_4.txt
# ./dir/files_5.txt
# ./dir/files_6.txt
```
Multiple ranges can be defined for one path

```sh
./dir_[1-2]/files_[1-3].txt
# will produce :

## dir are created first
#  ./dir_1
#  ./dir_2

## files are created next
# ./dir_1/files_1.txt
# ./dir_1/files_2.txt
# ./dir_1/files_3.txt
# ./dir_2/files_1.txt
# ./dir_2/files_2.txt
# ./dir_2/files_3.txt
```

__Parent Directory :__

References to a parent directory are written as follow : __[*]__

```sh
./chapter_1/[*]_section_[1-6].txt
# will produce :

## dir are created first
# ./chapter_1

## files are created next
# ./chapter_1/chapter_1_section_1.txt
# ./chapter_1/chapter_1_section_2.txt
# ./chapter_1/chapter_1_section_3.txt
# ./chapter_1/chapter_1_section_4.txt
# ./chapter_1/chapter_1_section_5.txt
# ./chapter_1/chapter_1_section_6.txt
```

Multiple references to a parent directory can be defined

```sh
./chapter_[1-2]/[*]_sections/[*]_[1-3].txt
# will produce :

## dir are created first
# ./chapter_1/chapter_1_sections/
# ./chapter_2/chapter_2_sections/

## files are created next
# ./chapter_1/chapter_1_sections/chapter_1_sections_1.txt
# ./chapter_1/chapter_1_sections/chapter_1_sections_2.txt
# ./chapter_1/chapter_1_sections/chapter_1_sections_3.txt
# ./chapter_2/chapter_2_sections/chapter_2_sections_1.txt
# ./chapter_2/chapter_2_sections/chapter_2_sections_2.txt
# ./chapter_2/chapter_2_sections/chapter_2_sections_3.txt
```

## Build

On linux :

```sh
  make release
```

On windows :

```cmd
  cargo build --release
```

## Install

On linux :

```sh
  make release
  sudo make install
```

## Uninstall

On linux :

```sh
  sudo make uninstall
```

## Test

On linux :

```sh
  make test
```

On windows :

```cmd
  cargo test
```

## Handle errors

By default all errors and info are logged in a file called `scaffold_log.txt`

Most of the time, errors are due to invalid permissions.
