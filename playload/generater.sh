#!/bin/bash

if [[ -d "playload" ]]; then
  BASE_DIR="$PWD/playload"
elif [[ "$(basename "$PWD")" == "playload" ]]; then
  BASE_DIR="$PWD"
else
  echo "Unable to determine the location of 'playload' folder."
  exit 1
fi

cd "$BASE_DIR/hello_app"
cargo build --target riscv64gc-unknown-none-elf --release
rust-objcopy --binary-architecture=riscv64 --strip-all -O binary ../../target/riscv64gc-unknown-none-elf/release/hello_app ./hello_app.bin
dd if=/dev/zero of=./apps.bin bs=1M count=32
dd if=./hello_app.bin of=./apps.bin conv=notrunc
mv $BASE_DIR/hello_app/apps.bin "$BASE_DIR/apps.bin"
