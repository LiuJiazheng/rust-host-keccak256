#!/bin/bash

set -e
set -x

# rm -rf output
# rm -rf param
# rm -rf *.json
# mkdir output

#wasm image
FILE=$1
default=/home/frank/Projects/zkWasm-rust/pkg/output.wasm
default=~/Projects/rust-host-keccak256/pkg/zkwasm_host_keccak256_bg.wasm
FILE=${default}

#k
SPACE=${2:-22}

# Have `cuda` build
# Guest Circuit
ZKWASM=~/Projects/zkWasm/target/release/delphinus-cli

export RUST_LOG=info
export RUST_BACKTRACE=1

${ZKWASM} --function zkmain -k ${SPACE} --host host --param ./param --output ./output --wasm ${FILE} setup

${ZKWASM} --function zkmain -k ${SPACE} --host host --param ./param --output ./output --wasm ${FILE} dry-run

${ZKWASM} --function zkmain -k ${SPACE} --host host --param ./param --output ./output --wasm ${FILE} single-prove

${ZKWASM} --function zkmain -k ${SPACE} --host host --param ./param --output ./output --wasm ${FILE} single-verify

# Host Circuit
HOST=~/Projects/zkWasm-host-circuits/target/release/zkwasm-host-circuits-prover

${HOST} --opname keccakhash --param ./param --output ./output --input external_host_table.json

# Batcher
BATCHER=~/Projects/continuation-batcher/target/release/circuit-batcher
GUESTNAME=zkwasm
HOSTNAME=host.KECCAKHASH
OUTNAME=keccak_batch

rm -f sol/contracts/AggregatorVerifierStep*.sol
rm -f sol/contracts/AggregatorConfig.sol

${BATCHER} --param ./param --output ./output batch -k ${SPACE} --challenge sha --info output/${GUESTNAME}.loadinfo.json output/${HOSTNAME}.loadinfo.json --name ${OUTNAME} --commits batch/batch.json

${BATCHER} --param ./param --output ./output verify --challenge sha --info output/${OUTNAME}.loadinfo.json

${BATCHER} --param ./param --output ./output solidity -k ${SPACE} --info output/${OUTNAME}.loadinfo.json output/${GUESTNAME}.loadinfo.json output/${HOSTNAME}.loadinfo.json --commits batch/batch.json

