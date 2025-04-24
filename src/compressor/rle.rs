use std::io;

pub fn rle_encode(text: &Vec<u8>) -> Vec<u8> {
    let mut n_zero: u8 = 0;
    let mut l: Vec<u8> = Vec::new();
    let s = text;
    
    for i in 0..s.len() + 1 {
        let current = s.get(i).cloned();            // Get the current element from the input vector
        if i < s.len() && current == Some(0) {
            n_zero += 1;                            // Count consecutive zeros
        } else {
            let n_z = n_zero + 1;
            let binary = format!("{:b}", n_z);      // Convert the count to binary representation
            let binary_without_msb = binary.chars().skip(1).collect::<String>();                // Remove the most significant bit
            l.extend(binary_without_msb.chars().rev().filter_map(|num| num.to_digit(2).map(|digit| digit as u8)));  // Add each binary digit to the output vector
            n_zero = 0;
            l.extend(current.into_iter().map(|current_value| current_value.wrapping_add(2)));   // Add the current value plus 2 to the output vector
        }
    }
    l
}

pub fn rle_decode(data: &Vec<u8>) -> Vec<u8> {
    let mut n_zero: Vec<u8> = Vec::new();
    let mut s: Vec<u8> = Vec::new();
    let l = data;

    for i in 0..l.len() + 1 {
        let current = l.get(i).cloned();               // Get the current element from the encoded vector
        if i <= l.len() - 1 && (current.map_or(false, |value| value == 0 || value == 1)) {
            n_zero.push(current.unwrap());
        } else {
            let n_z = n_zero.iter().rev().fold(1, |acc, &bit| (acc << 1) | bit);    // Convert the zero values to a count
            s.extend(std::iter::repeat(0).take((n_z - 1) as usize));                // Add the corresponding number of zeros to the output vector
            n_zero.clear();
            if let Some(current_value) = current {
                s.push(current_value.wrapping_sub(2));  // Add the current value minus 2 to the output vector
            }
        }
    }
    s
}

fn main() {
    let mut input = String::new();
    println!("Enter a vector (e.g., [3, 3, 4, 1, 1, 1, 0, 0, 4, 4]): ");
    io::stdin().read_line(&mut input).expect("Failed to read input.");

    let sequence: Vec<u8> = input
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|s| s.trim().parse().expect("Invalid number"))
        .collect();

    let encoded_sequence = rle_encode(&sequence.iter().map(|&x| x as u8).collect());
    println!("Encoded sequence: {:?}", encoded_sequence);

    let decoded_sequence = rle_decode(&encoded_sequence);
    println!("Decoded sequence: {:?}", decoded_sequence);
}
