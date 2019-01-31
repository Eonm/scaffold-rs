# scaffold-rs

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
# ./files_1/files_1.txt
# ./files_1/files_2.txt
# ./files_1/files_3.txt
# ./files_2/files_1.txt
# ./files_2/files_2.txt
# ./files_2/files_3.txt
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
