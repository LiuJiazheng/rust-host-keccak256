mod keccak256;

use crate::keccak256::Keccak;
pub use crate::keccak256::KECCAK_HASHER;

struct Generator {
    pub cursor: usize,
    pub values: Vec<u64>,
}

impl Generator {
    fn gen(&mut self) -> u64 {
        let r = self.values[self.cursor];
        self.cursor += 1;
        r
    }
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

fn u64_vec_to_u8_vec(input: Vec<u64>) -> Vec<u8> {
    let mut output = Vec::new();
    for num in input {
        let bytes = num.to_le_bytes();
        output.extend(bytes);
    }
    output
}

fn helper(input: &[u64;17]) -> (Vec<u8>, Vec<u8>) {
    use sha3::{Digest, Sha3_256};
    let input_u8 = u64_vec_to_u8_vec(input.to_vec());
    // create a SHA3-256 object
    let mut hasher = Sha3_256::new();
    // write input message
    hasher.update(&input_u8);
    let result = hasher.finalize();
    let output_u8 = result.to_vec();
    (input_u8, output_u8)
}

struct Keccak256Context {
    pub hasher: Option<Keccak>,
    pub generator: Generator,
    pub buf: Vec<u64>,
}

impl Keccak256Context {
    fn default() -> Self {
        Keccak256Context {
            hasher: None,
            generator: Generator {
                cursor: 0,
                values: vec![],
            },
            buf: vec![],
        }
    }

    pub fn keccak_new(&mut self, new: usize) {
        self.buf = vec![];
        if new != 0 {
            self.hasher = Some(KECCAK_HASHER.clone());
        }
    }

    pub fn keccak_push(&mut self, v: u64) {
        self.buf.push(v);
    }

    pub fn keccak_finalize(&mut self) -> u64 {
        println!("###buf len is {}", self.buf.len());
        assert!(self.buf.len() == 17);
        if self.generator.cursor == 0 {
            println!("###perform hash my hash is {:?}", self.hasher);
            self.hasher.as_mut().map(|s| {
                let r = s.update_exact(&self.buf.clone().try_into().unwrap());
                self.generator.values = r.to_vec();
            });
        }
        self.generator.gen()
    }
}

fn zkwasm_keccak(input: &[u64;17]) -> Vec<u64> {
    let mut context = Keccak256Context::default();
    context.keccak_new(1);
    for i in 0..input.len() {
        context.keccak_push(input[i]);
    }
    let mut output_u64 = vec![0; 4];
    for i in 0..4 {
        output_u64[i] = context.keccak_finalize();
    }
    output_u64
}

fn main() {
    // empty input
    let emtpy_standard_input = [0u64; 17];
    let output_u64 = zkwasm_keccak(&emtpy_standard_input);
    let (_, output_u8) = helper(&emtpy_standard_input);
    println!("output_u64: {:?}", output_u64);
    println!("output_u64.len(): {:?}", output_u64.len());
    println!("alternative output_u64: {:?}", u8_vec_to_u64_vec(output_u8));
}
