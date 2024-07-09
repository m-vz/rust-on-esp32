# Rust on ESP32

This repository contains the final code for the Rust on ESP32 workshop and the [presentation](presentation).

## Installing

0. Make sure you have [`rustup`](https://www.rust-lang.org/learn/get-started) installed on your system.
1. Make sure you have the latest Rust version installed with `rustup update`. If you don't have any toolchain installed, install `stable`.
2. Install the dependencies used for the workshop: `cargo install cargo-generate espup espflash`
3. Install the Xtensa Rust toolchain with `espup install -f export-esp.sh`. (The `-f export-esp.sh` is optional; the file will be saved in `$HOME` if it isn't provided.)
4. Make sure the toolchain was installed correctly: `ls -a ~/.rustup/toolchains`
5. Source the generated export file with `source export-esp.sh`. This needs to be done in each new shell session (or the exports can be added to the shell profile).

The rest of this process will be done together, so wait here unless you want to wait later.

6. Generate a new project with `cargo generate esp-rs/esp-template` choosing
    - `rust-on-esp32` or a more creative project name
    - `esp32`
    - `true`
    - `false`
    - `true`
    - `false`
    - `false`
    - `false`
    - `yes`
7. Enter the project directory: `cd rust-on-esp32`
8. Build the project with `cargo build --release` or upload it with `cargo run --release`.

## Troubleshooting

If `espflash` is missing permission to access device, add the user to the `dialout` group with `sudo usermod -a -G dialout $USER`.
This requires logging out and in again.

If `cargo build` fails with the error
```sh
error: linker `xtensa-esp32-elf-gcc` not found
  |
  = note: No such file or directory (os error 2)
```
make sure you have sourced `export-esp.sh`.
