# anicli-list
anicli-list is a CLI program made in rust, to list animes

## Features
    + Add animes
    + Remove animes
    + Add waifus UwU
    + Delete waifus UwU
    + Rank animes
    + List all animes
    + Show a specific list
    + Change the folder where the anime are stored
    + Change lists

## Install
Dependencies:
    + Rust
    + Cargo
## Script Install
Run the script installers/installer.sh

```bash
chmod +x ./installers/installer.sh
./installers/installer.sh
```

The anicli-list binary will be moved to /usr/bin

## Manual Install
Create the folder ~/.config/anicli-list and then place the path.conf file, inside this file put the folder where you want the lists to be stored, it is recommended for example: /home/user/Documents/animes/db/

```bash
mkdir ~/.config/anicli-list
echo "/home/user/Documents/animes/db/" > ~/.config/anicli-list/path.conf
```

Then compile the project with:

```bash
cargo build -r
```

Then move the newly created binary to /usr/bin

```bash
mv target/release/anicli-list /usr/bin
```

## How use
To use it, you always need some argument
```bash
anicli-list --help
```
