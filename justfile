VERSION := `toml get Cargo.toml package.version | jq -r`
TARGET_DIR := "target/release"
DEVICE := "/dev/cu.usbmodem14401"
export TAG:=`toml get Cargo.toml "package.version" | jq -r .`

# List available commands
_default:
  just --choose --chooser "fzf +s -x --tac --cycle"

# build
build:
    cargo build

# flash to arduino
flash: build
    #!/usr/bin/env bash
    FILE=target/avr-atmega328p/debug/rust-arduino-blink.elf
    avrdude -q -Cavrdude.conf -patmega328p -carduino -P{{DEVICE}} -D "-Uflash:w:$FILE:e"
