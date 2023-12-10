
extern crate byteorder;
use byteorder::{BigEndian, ByteOrder};


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

fn u64_vec_to_u8_vec(input: Vec<u64>) -> Vec<u8> {
    let mut output = Vec::new();
    for num in input {
        let bytes = num.to_le_bytes();
        output.extend(bytes);
    }
    output
}


fn main() {
    //let input:Vec<u8> = [102, 111, 111, 98, 97, 114, 0, 0, 122].to_vec();
    let input:Vec<u8> = [102, 111, 111, 98, 97, 114, 0, 0, 
    122, 0, 0, 0, 0, 0, 0, 0,
    121, 11].to_vec();
    //let input:Vec<u8> = [102, 111, 111].to_vec();
    //let input_2d = split_into_2d_array(input);
    //let output_u64 = u8_to_u64_2d(input_2d);
    println!("{:?}", input);
    let output_u64 = u8_vec_to_u64_vec(input);
    println!("{:?}", output_u64);
    let output_u8 = u64_vec_to_u8_vec(output_u64);
    println!("{:?}", output_u8);
    let mut output_u64 = [15334149082588785787, 16459938944588432275, 1975143395267388522, 11769824114587043277].to_vec();
    println!("{:?}", u64_vec_to_u8_vec(output_u64));
    output_u64 = [5428572333363565211, 690714843559919322, 3199586859266957939, 3354118150935030376].to_vec();
    println!("{:?}", u64_vec_to_u8_vec(output_u64));
}
