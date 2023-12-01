#!/bin/bash

if [[ -d "playload" ]]; then
  BASE_DIR="$PWD/playload"
elif [[ "$(basename "$PWD")" == "playload" ]]; then
  BASE_DIR="$PWD"
else
  echo "Unable to determine the location of 'playload' folder."
  exit 1
fi

# # Input: The Name of Directory
# # Output: Hex Size of Binary 
# function generateBinary() {
#   # echo "====================" $1 "===================="
#   # Create Binrary
#   cd "$BASE_DIR/$1"
#   # echo `pwd`
#   cargo build --target riscv64gc-unknown-none-elf --release
#   # remove symbol information
#   rust-objcopy --binary-architecture=riscv64 --strip-all -O binary ../../target/riscv64gc-unknown-none-elf/release/$1 ./$1.bin
# 
#   # return size
#   # echo $(stat -c%s "./$1.bin") "0x$(stat -c%s "./$1.bin" | xargs printf "%02x")"
#   echo "$(stat -c%s "./$1.bin" | xargs printf "%04x")"
#   # return "$(stat -c%s "./$1.bin" | xargs printf "%02x")"
# }
# 
# app_names=("hello_app" "hello_d")
# declare -A app_sizes
# declare -a link
# NUM_OF_IMAGE=0
# 
# echo "==================== HEAD OF GEN ==================="
# 
# for name in "${app_names[@]}"; do
#   echo name: $name
#   app_size=$(generateBinary $name)
#   app_sizes["$name"]=$app_size
#   link+=${app_size}
#   NUM_OF_IMAGE=$(expr $NUM_OF_IMAGE + 1)
# done
# 
# echo "app_sizes: ${app_sizes[@]}"
# echo "NUM_OF_IMAGE": $NUM_OF_IMAGE
# 
# echo "==================== TAIL OF GEN ==================="
# 
# # PFLASH 32M ]
# # PFLASH 32M ] [ NUM_OF_IMAGE ]
# # PFLASH 32M ] [    u16:2B    ] [ BYTE_LIST:4B*NUM_OF_IMAGE ] 
# # PFLASH 32M ] [     2B + 4B * NUM_OF_IMAGE                 ] [  ] [  ] [  ] 
# 
# cd $BASE_DIR
# printf "%02x" $NUM_OF_IMAGE | xxd -r -p >num.bin # NOTE: not allow app > 255
# echo -n "${app_sizes[@]}" | xxd -r -p > size.bin
# echo "size.bin size: $(stat -c%s "./size.bin")"
# 
# dd if=/dev/zero   of=./apps.bin              bs=1M count=32
# dd if=./num.bin   of=./apps.bin conv=notrunc 
# dd if=./size.bin  of=./apps.bin conv=notrunc bs=1B seek=2
# 
# start_offset=$((2 + 4 * $NUM_OF_IMAGE)) # NUM_OF_IMAGE:2B + IMAGE_SIZE:4B * NUM_OF_IMAGE
# echo "start_offset" $start_offset
# for ((i=0; i<${#app_names[@]}; i++)); do
#   app_name=${app_names[i]}
#   app_size=${app_sizes[i]}
#   dd if="$BASE_DIR/$app_name/$app_name.bin" of=./apps.bin conv=notrunc bs=1B seek=$start_offset
#   start_offset=$((start_offset + app_size))
# done


##################
# ELF Operations #
##################

# Use Some Ways to gain ELF files
# TODO: use our musl
# TODO: modify musl cross toolchain
# $ riscv64-linux-musl-gcc -c hello.c
# $ riscv64-linux-musl-ld hello.o -L /usr/musl-riscv64 -l:libc.so -e 0000000000000000 --dynamic-linker /lib/ld-musl-riscv64.so.1
# FIXME: It seems dynamic linker version couldn't work in qemu-riscv64 with a error call `Invalid ELF image for this architecture`
# The ELF file will be located under /playload


function generateBinary() {
  echo "$(stat -c%s "$BASE_DIR/$1" | xargs printf "%08x")"
}

# BASE ON THE NEED OF RELOCATIONAL OBJECT FILE AND SHARED OBJECT AT THE SAME TIME
# FIXME: value too great for base case libc could only be place at tail
app_names=("hello.o" "libc.so")
declare -A app_sizes
NUM_OF_IMAGE=0

echo "==================== HEAD OF GEN ==================="

for name in "${app_names[@]}"; do
  echo name: $name
  app_size=$(generateBinary $name)
  app_sizes["$name"]=$app_size
  NUM_OF_IMAGE=$(expr $NUM_OF_IMAGE + 1)
done

echo "app_sizes: ${app_sizes[@]}"
echo "NUM_OF_IMAGE": $NUM_OF_IMAGE

echo "==================== TAIL OF GEN ==================="

# # PFLASH 32M ]
# # PFLASH 32M ] [ NUM_OF_IMAGE ]
# # PFLASH 32M ] [    u16:2B    ] [ BYTE_LIST:4B*NUM_OF_IMAGE ] 
# # PFLASH 32M ] [     2B + 4B * NUM_OF_IMAGE                 ] [  ] [  ] [  ] 
# 1B=u8, 2B=u16, 4B=u32, 8B=u64
cd $BASE_DIR
printf "%02x" $NUM_OF_IMAGE | xxd -r -p > num.bin # NOTE: not allow app > 255
echo -n "${app_sizes[@]}" | xxd -r -p > size.bin
echo "size.bin size: $(stat -c%s "./size.bin")"

dd if=/dev/zero   of=./apps.bin              bs=1M count=32
dd if=./num.bin   of=./apps.bin conv=notrunc 
dd if=./size.bin  of=./apps.bin conv=notrunc bs=1B seek=2

echo "=== LOADING ==="
start_offset=$((2 + 4 * $NUM_OF_IMAGE)) # NUM_OF_IMAGE:2B + IMAGE_SIZE:4B * NUM_OF_IMAGE
echo "start_offset:" $start_offset
for ((i=0; i<${#app_names[@]}; i++)); do
  echo "================================="
  app_name=${app_names[i]}
  app_size=${app_sizes[$app_name]}
  app_size_dec=$((16#${app_size}))
  dd if="$BASE_DIR/$app_name" of=./apps.bin conv=notrunc bs=1B seek=$start_offset
  echo "name" $app_name
  echo "offset:" $start_offset
  echo "len" ${#app_sizes[@]}
  echo "app_size:" $app_size
  echo "app_size_dec" $app_size_dec
  start_offset=$((start_offset + app_size_dec))
done

# Copy the library files generate by musl
cp ../../Project/musl-cross-make/build/local/riscv64-linux-musl/obj_musl/lib/ . -r
