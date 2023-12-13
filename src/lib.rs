extern "C" {
    pub fn wasm_input(is_public: u32) -> u64;
    pub fn require(cond: bool);
    pub fn poseidon_new();
    pub fn keccak_new(v: u64);
    pub fn wasm_dbg_char(v: u64);
    pub fn keccak_push(v: u64);
    pub fn keccak_finalize() -> u64;
}

fn read_public_input() -> u64 {
    unsafe { wasm_input(1) }
}

fn read_private_input() -> u64 {
    unsafe { wasm_input(0) }
}

extern crate byteorder;

fn u64_vec_to_u8_vec(input: Vec<u64>) -> Vec<u8> {
    let mut output = Vec::new();
    for num in input {
        let bytes = num.to_le_bytes();
        output.extend(bytes);
    }
    output
}

fn u8_vec_to_u64_vec(input: Vec<u8>) -> Vec<u64> {
    let mut output = input
        .chunks_exact(8)
        .map(|chunk| u64::from_le_bytes(chunk.try_into().unwrap()))
        .collect::<Vec<u64>>();
    let remainder = input.len() % 8;
    if remainder != 0 {
        let mut buffer = [0u8; 8];
        buffer[..remainder].copy_from_slice(&input[input.len() - remainder..]);
        let value = u64::from_le_bytes(buffer);
        output.push(value);
    }
    output
}

pub fn keccak256(input: &Vec<u8>) -> Vec<u8> {
    unsafe {
        keccak_new(1);
    }
    let input_u64 = u8_vec_to_u64_vec(input.clone());
    for i in 0..input_u64.len() {
        unsafe {
            keccak_push(input_u64[i]);
        }
    }
    let mut output_u64 = vec![0; 4];
    unsafe {
        for i in 0..4 {
            output_u64[i] = keccak_finalize();
        }
    }
    let output_u8: Vec<u8> = output_u64
        .iter()
        .flat_map(|&value| value.to_le_bytes().to_vec())
        .collect();
    output_u8
}

fn keccak256check(input: &Vec<u8>, output: &Vec<u8>) {
    let result = keccak256(&input);
    for i in 0..output.len() {
        unsafe { require(result[i] == output[i]) };
    }
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn zkmain() -> i64 {
    let input = [0u64; 17];
    let output: [u64;4] = [17376452488221285863, 9571781953733019530, 15391093639620504046, 13624874521033984333];
    let input_u8 = u64_vec_to_u8_vec(input.to_vec());
    let output_u8 = u64_vec_to_u8_vec(output.to_vec());

    keccak256check(&input_u8, &output_u8);

   0 
}
