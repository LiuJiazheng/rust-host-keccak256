#!/bin/bash

set -e
set -x

PWD=${PWD}
#wasm image
FILE=$1
DEFAULT_WASM=${PWD}/pkg/zkwasm_host_keccak256_bg.wasm

#k
SPACE=${2:-22}

# Have `cuda` build
# Guest Circuit
ZKWASM_DIR=${PWD}/zkWasm
ZKWASM=${ZKWASM_DIR}/target/release/delphinus-cli
OUTPUT=${PWD}/output
PARAM=${PWD}/param

export RUST_LOG=info
export RUST_BACKTRACE=1


if [ ! -f $DEFAULT_WASM ]; then
    echo -e "\n===building zkwasm_host_keccak256_bg wasm"
    wasm-pack build --release
fi

if [ ! -f "$ZKWASM" ]; then
    echo -e "\n===building zkWasm cuda"
    if command -v "nvcc" >/dev/null 2>&1; then
        cd $ZKWASM_DIR
        cargo build --features cuda --release
    else
        echo "Cuda is not installed!"
        exit 1
    fi
fi

if [ ! -d "output" ];then
    mkdir output
    mkdir param
fi

echo -e "\n===guest circuit proving"
${ZKWASM} --function zkmain -k ${SPACE} --host host --param ${PARAM} --output ${OUTPUT} --wasm ${DEFAULT_WASM} setup

${ZKWASM} --function zkmain -k ${SPACE} --host host --param ${PARAM} --output ${OUTPUT} --wasm ${DEFAULT_WASM} dry-run

${ZKWASM} --function zkmain -k ${SPACE} --host host --param ${PARAM} --output ${OUTPUT} --wasm ${DEFAULT_WASM} single-prove

${ZKWASM} --function zkmain -k ${SPACE} --host host --param ${PARAM} --output ${OUTPUT} --wasm ${DEFAULT_WASM} single-verify

# Host Circuit
HOST_DIR=${PWD}/zkWasm-host-circuits
HOST=${HOST_DIR}/target/release/zkwasm-host-circuits-prover
if [ ! -f "$HOST" ];then
    echo -e "\n===building zkWasm-host-circuits cuda"
    if command -v "nvcc" >/dev/null 2>&1; then
        cd $HOST_DIR
        cargo build --features cuda --release
    else
        echo "Cuda is not installed!"
        exit 1
    fi
fi

echo -e "\n===keccak host circuit proving"
${HOST} --opname keccakhash --param ${PARAM} --output ${OUTPUT} --input external_host_table.json


# Batcher
BATCHER_DIR=${PWD}/continuation-batcher
BATCHER=${BATCHER_DIR}/target/release/circuit-batcher
GUESTNAME=zkwasm
HOSTNAME=host.KECCAKHASH
OUTNAME=keccak_batch

if [ ! -f "$BATCHER" ];then
    echo -e "\n===building continuation-batcher cuda"
    if command -v "nvcc" >/dev/null 2>&1; then
        cd $BATCHER_DIR
        cargo build --features cuda --release
    else
        echo "Cuda is not installed!"
        exit 1
    fi
fi

echo -e "\n===continuation-batcher proving"
cd $PWD
${BATCHER} --param ${PARAM} --output ${OUTPUT} batch -k ${SPACE} --challenge sha --info output/${GUESTNAME}.loadinfo.json output/${HOSTNAME}.loadinfo.json --name ${OUTNAME} --commits playground/batch/batch.json

${BATCHER} --param ${PARAM} --output ${OUTPUT} verify --challenge sha --info output/${OUTNAME}.loadinfo.json

cd playground
${BATCHER} --param ${PARAM} --output ${OUTPUT} solidity -k ${SPACE} --info ${OUTPUT}/${OUTNAME}.loadinfo.json ${OUTPUT}/${GUESTNAME}.loadinfo.json ${OUTPUT}/${HOSTNAME}.loadinfo.json --commits batch/batch.json

