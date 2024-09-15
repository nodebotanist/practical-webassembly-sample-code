#!/bin/bash

OPT=$1
PASSES=$2
OUTPUT_PREFIX="optimized/"

printf "Practical WebAssembly Optimizer Script\n"

# chapter 3

printf "Chapter 3 - lib/stats_calc.wasm\n"

CH03_PRE="lib/stats_calc"

printf "wasm-opt -O2 (release build): ${OUTPUT_PREFIX}stats_calc_release.wasm \n"
wasm-opt -o ${OUTPUT_PREFIX}stats_calc_release.wasm -O2 ${CH03_PRE}.wasm
printf "wasm-opt -Oz (min build): ${OUTPUT_PREFIX}stats_calc_min_Oz.wasm \n"
wasm-opt -o ${OUTPUT_PREFIX}stats_calc_min_Oz.wasm -Oz ${CH03_PRE}.wasm
printf "wasm-opt -O${PASSES} (custom build): ${OUTPUT_PREFIX}stats_calc_min_O${PASSES}.wasm\n"
wasm-opt -o ${OUTPUT_PREFIX}stats_calc_min_O${PASSES}.wasm -O${PASSES} ${CH03_PRE}.wasm

ORIG_SIZE=`stat -c "%s" lib/stats_calc.wasm`
RELEASE_SIZE=`stat -c "%s" ${OUTPUT_PREFIX}stats_calc_release.wasm`
MIN_SIZE=`stat -c "%s" ${OUTPUT_PREFIX}stats_calc_min_Oz.wasm`
CUSTOM_SIZE=`stat -c "%s" ${OUTPUT_PREFIX}stats_calc_min_O${PASSES}.wasm`

printf "Original -O0 size: $ORIG_SIZE bytes\n"
printf "Release -O2 size: $RELEASE_SIZE bytes\n"
printf "Min -Oz size: $MIN_SIZE bytes\n"
printf "Custom -O${PASSES} size: $CUSTOM_SIZE bytes\n\n"

# chapter 4

printf "Chapter 4 - lib/ch04/debug.wasm\n"

wasm-opt -o lib/ch04/release.wasm -all -Oz lib/ch04/debug.wasm
wasm-opt -o lib/ch04/optimized_O${PASSES}.wasm -all -O${PASSES} lib/ch04/debug.wasm

ORIG_SIZE=`stat -c "%s" lib/ch04/debug.wasm`
RELEASE_SIZE=`stat -c "%s" lib/ch04/release.wasm`
CUSTOM_SIZE=`stat -c "%s" lib/ch04/optimized_O${PASSES}.wasm`

printf "Original size: $ORIG_SIZE bytes\n"
printf "Release (-Oz) size: $RELEASE_SIZE bytes\n"
printf "Custom (-O${PASSES}) size: $CUSTOM_SIZE bytes\n\n"

# chapter 5

printf "Chapter 5- lib/ch05/roll_checker_bg.wasm:\n"

CH05_PRE="lib/ch05/roll_checker_bg"
CH05_OPT="optimized/ch05/roll_checker_bg"

wasm-opt -o ${CH05_OPT}_release.wasm -all -O2 ${CH05_PRE}.wasm
wasm-opt -o ${CH05_OPT}_min.wasm -all -Oz ${CH05_PRE}.wasm
wasm-opt -o ${CH05_OPT}_optimized_O${PASSES}.wasm -all -O${PASSES} ${CH05_PRE}.wasm

ORIG_SIZE=`stat -c "%s" ${CH05_PRE}.wasm`
RELEASE_SIZE=`stat -c "%s" ${CH05_OPT}_release.wasm`
MIN_SIZE=`stat -c "%s" ${CH05_OPT}_min.wasm`
CUSTOM_SIZE=`stat -c "%s" ${CH05_OPT}_optimized_O${PASSES}.wasm`

printf "Original size: $ORIG_SIZE bytes\n"
printf "Release (-O2) size: $RELEASE_SIZE bytes\n"
printf "Min (-Oz) size: $MIN_SIZE bytes\n"
printf "Custom (-O${PASSES}) size: $CUSTOM_SIZE bytes\n\n"
# wasm-opt -o lib/ch05/roll_checker_bg_optimized_small.wasm -all -Oz lib/ch05/roll_checker_bg.wasm
# wasm-opt -o lib/ch05/roll_checker_bg_O${PASSES}.wasm -all -O${PASSES} lib/ch05/roll_checker_bg.wasm

# chapter 6 (web)

# chapter 6 (wasmtime)

# chapter 7
