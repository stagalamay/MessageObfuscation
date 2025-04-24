use crate::compressor::bwt::{bwt_encode, bwt_decode};
use crate::compressor::mtf::{mtf_encode, mtf_decode};
use crate::compressor::rle::{rle_encode, rle_decode};
//use crate::compressor::huffman::{huffman_encode, huffman_decode};
use base85::{encode, decode};

static ALPHABET: &str = "diropql";
const MEMORY_SIZE: usize = 10000;

fn compress(program: &String) -> (Vec<u8>, usize) {

    let (compressed_program, index) = bwt_encode(program);
    //println!("AFTER BWT ENCODING: {}", compressed_program);

    let compressed_program = mtf_encode(&program, ALPHABET);
    //println!("AFTER MTF ENCODING: {:?}", compressed_program);

    let compressed_program = rle_encode(&compressed_program);
    //println!("AFTER RLE ENCODING: {:?}", compressed_program);

    // let mut compressed_program = encode_huffman(rle_program);
    // println!("AFTER HUFFMAN ENCODING: {}", compressed_program);

    (compressed_program, index)
}

fn decompress(program: &Vec<u8>, meta: &DpqlzMeta, _alphabet: &str) -> String {
//fn decompress(program: &String, index: &u64) -> String {

    // let mut decoded_huff_program = decode_huffman(program, );

    let decoded_program = rle_decode(program);
    //println!("AFTER RLE DECODING: {:?}", decoded_program);

    let decoded_program = mtf_decode(&decoded_program, &ALPHABET);
    //println!("AFTER MTF DECODING: {}", decoded_program);

    let decoded_program_bwt = bwt_decode(&decoded_program, (meta.bwt_idx as usize));
    //println!("AFTER BWT DECODING: {}", decoded_program);

    decoded_program
}

// MODULE DIROPQRL

pub fn write_diropql(text: &str) -> String {
    
    let mut program = String::new();

    // Set memory pointer to the first cell
    program.push_str("r");

    for c in text.chars() {
        let ascii_val = c as u8;

        // Increment the current memory cell by the ASCII value
        program.push_str(&format!("{}", "i".repeat(ascii_val as usize)));

        // Output the value in the current memory cell
        program.push_str("o");

        // Set memory pointer to the next cell
        program.push_str(&format!("{}", "r"));
    }

    program

}

pub fn read_diropql(prog: &str) -> String {
    let mut memory = vec![0u8; MEMORY_SIZE];
    let mut mp = 0usize;
    let mut ip = 0usize;
    let mut oq = String::new();

    while ip < prog.len() {
        let command = prog.chars().nth(ip).unwrap();

        match command {
            'l' => mp = (mp + MEMORY_SIZE - 1) % MEMORY_SIZE,
            'r' => mp = (mp + 1) % MEMORY_SIZE,
            'i' => memory[mp] = memory[mp].wrapping_add(1),
            'd' => memory[mp] = memory[mp].wrapping_sub(1),
            'o' => oq.push(memory[mp] as char),
            'p' => {
                if memory[mp] == 0 {
                    let mut count = 1;
                    while count > 0 {
                        ip += 1;
                        if prog.chars().nth(ip).unwrap() == 'p' {
                            count += 1;
                        } else if prog.chars().nth(ip).unwrap() == 'q' {
                            count -= 1;
                        }
                    }
                }
            }
            'q' => {
                if memory[mp] != 0 {
                    let mut count = 1;
                    while count > 0 {
                        ip -= 1;
                        if prog.chars().nth(ip).unwrap() == 'q' {
                            count += 1;
                        } else if prog.chars().nth(ip).unwrap() == 'p' {
                            count -= 1;
                        }
                    }
                }
            }
            _ => {}
        }
        ip += 1;
    }
    oq
}

// MODULE ZIP

pub struct DpqlzMeta {
    pub mlen: u64,
    pub moffset: u8,
    pub bwt_idx: u64,
    //pub huffman_tree: Vec<u8>,
}

pub fn write_diropqlz(text: &String) -> String {

    let diropql_program = write_diropql(text);
    let (compressed_program, index) = compress(&diropql_program);
    let offset = compressed_program.len() % 8;

    //println!("MLEN BEFORE DECODING: {}", compressed_program.len());
    //println!("MOFFSET BEFORE DECODING: {}", offset);
    //println!("BWT INDEX BEFORE DECODING: {}", index);

    let meta = DpqlzMeta {
        mlen: compressed_program.len() as u64,
        moffset: offset as u8,
        bwt_idx: index as u64,
        // huffman_tree: vec![0; 16],
    };

    let diropqlz_program = write_meta(&meta, compressed_program);
    diropqlz_program
}

pub fn write_meta(meta: &DpqlzMeta, program: Vec<u8>) -> String {

    let mut output = String::new();

    // Convert meta into the diropqlz metadata layout
    let obfuscated_length = ((program.len() as f64 * 8.0).ceil() / 8.0) as u64;

    output.push_str(&obfuscated_length.to_string());
    output.push_str(&meta.moffset.to_string());
    output.push_str(&meta.bwt_idx.to_string());
    // output.push_str(&huffman_tree);

    let str_program = String::from_utf8(program).unwrap();
    output.push_str(&str_program);

    //println!("BEFORE BASE85 ENCODING: {}", output);

    let compressed_program = output.as_bytes();
    let mprime = encode(compressed_program);
    let diropqrlz_file = "DIROPQLZ".to_string() + &mprime;

    diropqrlz_file
}

pub fn read_diropqlz(program: &String) -> String {

    // Remove the magic string
    let program = program[8..].to_string();

    let (meta, diropqlz_program) = read_meta(&program);
    let decompressed_program = decompress(&diropqlz_program, &meta, &ALPHABET);
    //let decompressed_program = decompress(&diropqlz_program, &meta.bwt_idx);

    //println!("HUF BITLENS AFTER DECODING: {:?}", meta.huf_bitlens);

    let diropql_program = read_diropql(&decompressed_program);

    diropql_program
}

pub fn read_meta(program: &String) -> (DpqlzMeta, Vec<u8>) {

    let mprime = decode(program).unwrap();

    let compressed_program = std::str::from_utf8(&mprime).unwrap();
    let compressed_program_bytes = compressed_program.as_bytes();

    //println!("AFTER BASE85 DECODING: {}", compressed_program);

    // Extract the metadata values
    let length_bytes = &compressed_program_bytes[..3];
    let mlen = std::str::from_utf8(&length_bytes).unwrap();
    let mlen_u64: u64 = mlen.parse().unwrap();

    let ignore_bits_byte = compressed_program_bytes[3];
    let binding = [ignore_bits_byte];
    let moffset = std::str::from_utf8(&binding).unwrap();
    let moffset_u8: u8 = moffset.parse().unwrap();

    let index_bytes = &compressed_program_bytes[4..8];
    let bwt_idx = std::str::from_utf8(&index_bytes).unwrap();
    let bwt_idx_u64: u64 = bwt_idx.parse().unwrap();

    //let huff_bytes = &compressed_program_bytes[7..11];
    //let huf_bitlens = huff_bytes.to_vec();

    // Create a DpqlzMeta struct with the extracted metadata
    let meta = DpqlzMeta {
        mlen: mlen_u64,
        moffset: moffset_u8,
        bwt_idx: bwt_idx_u64,
        //huf_bitlens,
    };

    //println!("MLEN AFTER DECODING: {}", meta.mlen);
    //println!("MOFFSET AFTER DECODING: {}", meta.moffset);
    //println!("BWT INDEX AFTER DECODING: {}", meta.bwt_idx);

    let obfuscated_bytes = &compressed_program_bytes[8..];

    (meta, obfuscated_bytes.to_vec())
}
