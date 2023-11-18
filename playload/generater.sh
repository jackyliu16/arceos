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
# Output: Hex Size of Binary 
function generateBinary() {
  # echo "====================" $1 "===================="
  # Create Binrary
  cd "$BASE_DIR/$1"
  # echo `pwd`
  cargo build --target riscv64gc-unknown-none-elf --release
  # remove symbol information
  rust-objcopy --binary-architecture=riscv64 --strip-all -O binary ../../target/riscv64gc-unknown-none-elf/release/$1 ./$1.bin

  # return size
  # echo $(stat -c%s "./$1.bin") "0x$(stat -c%s "./$1.bin" | xargs printf "%02x")"
  echo "0x$(stat -c%s "./$1.bin" | xargs printf "%02x")"
  # return "$(stat -c%s "./$1.bin" | xargs printf "%02x")"
}


echo "==================== HEAD OF GEN ==================="

nop_size=$(generateBinary "hello_nop")
printf "hello_nop_size: 0x%x %d\n" $nop_size $nop_size 

app_size=$(generateBinary "hello_app")
printf "hello_app_size: 0x%x %d\n" $app_size $app_size 

echo "==================== TAIL OF GEN ==================="

# PFLASH 32M ]
# PFLASH 32M ] [ NUM_OF_IMAGE ]
# PFLASH 32M ] [    u16:2B    ] [ BYTE_LIST:2B*NUM_OF_IMAGE ] 
# PFLASH 32M ] [                                            ] [  ] [  ] [  ] 

cd $BASE_DIR
echo -n $nop_size$app_size | xxd -r -p > size.bin
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

