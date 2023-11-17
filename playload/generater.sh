#!/bin/bash

if [[ -d "playload" ]]; then
  BASE_DIR="$PWD/playload"
elif [[ "$(basename "$PWD")" == "playload" ]]; then
  BASE_DIR="$PWD"
else
  echo "Unable to determine the location of 'playload' folder."
  exit 1
fi

# Input: The Name of Directory
function generateBinrary() {
  # Create Binrary
  echo $1
  cd "$BASE_DIR/$1"
  # echo `pwd`
  cargo build --target riscv64gc-unknown-none-elf --release
  # remove symbol information
  rust-objcopy --binary-architecture=riscv64 --strip-all -O binary ../../target/riscv64gc-unknown-none-elf/release/$1 ./$1.bin

  # return size
  return $(stat -c%s "./$1.bin")
}


generateBinrary "hello_nop"
hello_nop_size=$?
echo "hello_nop_size: " $hello_nop_size

generateBinrary "hello_app"
hello_app_size=$?
echo "hello_app_size: " $hello_app_size

# ][  u16  ]  [  u16  ]
# ][        4B        ] [ package 1 ]
# ][        4B + size_of(package 1) ] [ NEXT ] 

cd $BASE_DIR
hex1=$(printf "%02x" $hello_nop_size)
hex2=$(printf "%02x" $hello_app_size)
hex="$hex1$hex2"
echo -n $hex | xxd -r -p > size.bin
echo "size.bin: " $(stat -c%s "./size.bin")

dd if=/dev/zero                  of=./apps.bin               bs=1M count=32
dd if=./size.bin                 of=./apps.bin conv=notrunc
dd if=./hello_nop/hello_nop.bin  of=./apps.bin conv=notrunc  bs=1B seek=2
dd if=./hello_app/hello_app.bin  of=./apps.bin conv=notrunc  bs=1B seek=$(($hello_nop_size + 2))
# seek=$(($hello_nop_size + 2))



# # Using Zero to fill the block to 32M(32Times, one times for 1M)
# dd if=/dev/zero of=./apps.bin bs=1M count=32
# # Add origin app into the end of the file (not cover)
# dd if=./size.bin of=./apps.bin conv=notrunc       bs=1B seek=2
# dd if=./hello_app.bin of=./apps.bin conv=notrunc  bs=1B seek=4
# mv $BASE_DIR/hello_app/apps.bin "$BASE_DIR/apps.bin"

