#!/bin/bash

# "Global" vars, if you will 
PASSES=$1 # grabs the second arg (first is filename)

# Input/output file prefixes and postfixes
INPUT_PREFIX="lib/"
OUTPUT_PREFIX="optimized/"
RELEASE_POSTFIX="_release.wasm"
MIN_POSTFIX="_min_Oz.wasm"
CUSTOM_POSTFIX="_min_O${PASSES}.wasm"

# Create src/output path array
declare -A IO_PATHS
IO_PATHS["CH03_IN"]="${INPUT_PREFIX}stats_calc.wasm"
IO_PATHS["CH03_OUT"]="${OUTPUT_PREFIX}ch03/stats_calc"
IO_PATHS["CH04_IN"]="${INPUT_PREFIX}ch04/debug.wasm"
IO_PATHS["CH04_OUT"]="${OUTPUT_PREFIX}ch04/"
IO_PATHS["CH05_IN"]="${INPUT_PREFIX}ch05/roll_checker_bg.wasm"
IO_PATHS["CH05_OUT"]="${OUTPUT_PREFIX}ch05/roll_checker_bg"
IO_PATHS["CH06.1_IN"]="${INPUT_PREFIX}ch06/web/roll_checker_bg.wasm"
IO_PATHS["CH06.1_OUT"]="${OUTPUT_PREFIX}ch06/web/roll_checker_bg"
IO_PATHS["CH06.2_IN"]="${INPUT_PREFIX}ch06/wasmtime/wasmtime_runner.wasm"
IO_PATHS["CH06.2_OUT"]="${OUTPUT_PREFIX}ch06/wasmtime/wasmtime_runner"
IO_PATHS["CH07_IN"]="${INPUT_PREFIX}ch07/roll_checker_bg.wasm"
IO_PATHS["CH07_OUT"]="${OUTPUT_PREFIX}ch07/roll_checker_bg"

# Get started
printf "Practical WebAssembly Optimizer Script\n\n"

for chapter in 3 4 5 6.1 6.2 7
do
    CH_STR="CH0${chapter}"
    CH_IN="${CH_STR}_IN"
    CH_OUT="${CH_STR}_OUT"
    CH_IN_PATH=${IO_PATHS[$CH_IN]}
    CH_OUT_PATH=${IO_PATHS[$CH_OUT]}

    printf "====================\n\n$CH_STR: "
    printf ${CH_IN_PATH} 
    printf "\n\n"

    # Actually do the optimizations
    wasm-opt -o ${CH_OUT_PATH}${RELEASE_POSTFIX} -all -O2 ${CH_IN_PATH}
    wasm-opt -o ${CH_OUT_PATH}${MIN_POSTFIX} -all -Oz ${CH_IN_PATH}
    wasm-opt -o ${CH_OUT_PATH}${CUSTOM_POSTFIX} -all -O${PASSES} ${CH_IN_PATH}

    # Print the intended results of each run
    printf "wasm-opt -O2 (release build): ${CH_OUT_PATH}${RELEASE_POSTFIX} \n"
    printf "wasm-opt -Oz (min build): ${CH_OUT_PATH}${MIN_POSTFIX} \n"
    printf "wasm-opt -O${PASSES} (custom build): ${CH_OUT_PATH}${CUSTOM_POSTFIX}\n"

    # Get file sizes
    ORIG_SIZE=`stat -c "%s" ${CH_IN_PATH}`
    RELEASE_SIZE=`stat -c "%s" ${CH_OUT_PATH}${RELEASE_POSTFIX}`
    MIN_SIZE=`stat -c "%s" ${CH_OUT_PATH}${MIN_POSTFIX}`
    CUSTOM_SIZE=`stat -c "%s" ${CH_OUT_PATH}${CUSTOM_POSTFIX}`
    
    # Report file sizes
    printf "Original -O0 size: $ORIG_SIZE bytes\n"
    printf "Release -O2 size: $RELEASE_SIZE bytes\n"
    printf "Min -Oz size: $MIN_SIZE bytes\n"
    printf "Custom -O${PASSES} size: $CUSTOM_SIZE bytes\n\n====================\n\n"
done









# # chapter 4



# printf "Chapter 4 - ${CH04_SRC}\n"

# wasm-opt -o ${CH04_OPT}release.wasm -all -O2 ${CH04_SRC}
# wasm-opt -o ${CH04_OPT}min_Oz.wasm -all -Oz ${CH04_SRC}
# wasm-opt -o ${CH04_OPT}min_O${PASSES}.wasm -all -O${PASSES} ${CH04_SRC}

# printf "wasm-opt -O2 (release build): ${CH03_OPT}stats_calc${RELEASE_POSTFIX} \n"
# printf "wasm-opt -Oz (min build): ${CH03_OPT}stats_calc${MIN_POSTFIX} \n"
# printf "wasm-opt -O${PASSES} (custom build): ${CH03_OPT}stats_calc${CUSTOM_POSTFIX}\n"

# ORIG_SIZE=`stat -c "%s" ${CH04_SRC}`
# RELEASE_SIZE=`stat -c "%s" ${CH04_OPT}release.wasm`
# MIN_SIZE=`stat -c "%s" ${CH04_OPT}min_Oz.wasm`
# CUSTOM_SIZE=`stat -c "%s" ${CH04_OPT}min_O${PASSES}.wasm`

# printf "Original size: $ORIG_SIZE bytes\n"
# printf "Release (-O2) size: $RELEASE_SIZE bytes\n"
# printf "Min (-Oz) size: $MIN_SIZE bytes\n"
# printf "Custom (-O${PASSES}) size: $CUSTOM_SIZE bytes\n\n"

# # chapter 5

# printf "Chapter 5- ${INPUT_PREFIX}ch05/"



# wasm-opt -o ${CH05_OPT}${RELEASE_POSTFIX} -all -O2 ${CH05_PRE}.wasm
# wasm-opt -o ${CH05_OPT}_min.wasm -all -Oz ${CH05_PRE}.wasm
# wasm-opt -o ${CH05_OPT}_optimized_O${PASSES}.wasm -all -O${PASSES} ${CH05_PRE}.wasm

# ORIG_SIZE=`stat -c "%s" ${CH05_PRE}.wasm`
# RELEASE_SIZE=`stat -c "%s" ${CH05_OPT}${RELEASE_POSTFIX}`
# MIN_SIZE=`stat -c "%s" ${CH05_OPT}_min.wasm`
# CUSTOM_SIZE=`stat -c "%s" ${CH05_OPT}_optimized_O${PASSES}.wasm`

# printf "Original size: $ORIG_SIZE bytes\n"
# printf "Release (-O2) size: $RELEASE_SIZE bytes\n"
# printf "Min (-Oz) size: $MIN_SIZE bytes\n"
# printf "Custom (-O${PASSES}) size: $CUSTOM_SIZE bytes\n\n"
# wasm-opt -o ${INPUT_PREFIX}ch05/roll_checker_bg_optimized_small.wasm -all -Oz ${INPUT_PREFIX}ch05/roll_checker_bg.wasm
# wasm-opt -o ${INPUT_PREFIX}ch05/roll_checker_bg_O${PASSES}.wasm -all -O${PASSES} ${INPUT_PREFIX}ch05/roll_checker_bg.wasm

# chapter 6 (web)

# chapter 6 (wasmtime)

# chapter 7
