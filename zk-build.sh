#!/bin/bash

set -e
set -x

#rm -rf output
#rm -rf param
#rm -rf *.json
#mkdir output

#wasm image
FILE=$1
default=/home/frank/Projects/zkWasm-rust/pkg/output.wasm
default=~/Projects/rust-host-keccak256/pkg/zkwasm_host_keccak256_bg.wasm
FILE=${default}

#k
SPACE=${2:-22}

# Have `cuda` build
ZKWASM=~/Projects/zkWasm/target/release/delphinus-cli

export RUST_LOG=info
export RUST_BACKTRACE=1

${ZKWASM} --function zkmain -k ${SPACE} --host host --param ./param --output ./output --wasm ${FILE} setup

${ZKWASM} --function zkmain -k ${SPACE} --host host --param ./param --output ./output --wasm ${FILE} dry-run

${ZKWASM} --function zkmain -k ${SPACE} --host host --param ./param --output ./output --wasm ${FILE} single-prove

${ZKWASM} --function zkmain -k ${SPACE} --host host --param ./param --output ./output --wasm ${FILE} single-verify
