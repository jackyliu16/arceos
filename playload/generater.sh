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
  echo "$(stat -c%s "./$1.bin" | xargs printf "%04x")"
  # return "$(stat -c%s "./$1.bin" | xargs printf "%02x")"
}

app_names=("hello_nop" "hello_app")
declare -A app_sizes
declare -a link
app_num=0

echo "==================== HEAD OF GEN ==================="

for name in "${app_names[@]}"; do
  echo name: $name
  app_size=$(generateBinary $name)
  app_sizes["$name"]=$app_size
  link+=${app_size}
done

echo "app_sizes: ${app_sizes[@]}"
echo "link: ${link}"

echo "==================== TAIL OF GEN ==================="

# PFLASH 32M ]
# PFLASH 32M ] [ NUM_OF_IMAGE ]
# PFLASH 32M ] [    u16:2B    ] [ BYTE_LIST:2B*NUM_OF_IMAGE ] 
# PFLASH 32M ] [                                            ] [  ] [  ] [  ] 

cd $BASE_DIR
echo -n "${app_sizes[@]}" | xxd -r -p > size.bin
echo "size.bin size: $(stat -c%s "./size.bin")"

dd if=/dev/zero                  of=./apps.bin               bs=1M count=32
dd if=./size.bin                 of=./apps.bin conv=notrunc
# dd if=./hello_nop/hello_nop.bin  of=./apps.bin conv=notrunc  bs=1B seek=2
# dd if=./hello_app/hello_app.bin  of=./apps.bin conv=notrunc  bs=1B seek=$(($hello_nop_size + 2))
# seek=$(($hello_nop_size + 2))



# # Using Zero to fill the block to 32M(32Times, one times for 1M)
# dd if=/dev/zero of=./apps.bin bs=1M count=32
# # Add origin app into the end of the file (not cover)
# dd if=./size.bin of=./apps.bin conv=notrunc       bs=1B seek=2
# dd if=./hello_app.bin of=./apps.bin conv=notrunc  bs=1B seek=4
# mv $BASE_DIR/hello_app/apps.bin "$BASE_DIR/apps.bin"

