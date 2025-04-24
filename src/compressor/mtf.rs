use std::io;

pub fn mtf_encode(text: &String, alphabet: &str) -> Vec<u8> {
    let mut lst: Vec<char> = alphabet.chars().collect();    // Convert the alphabet string into a character vector
    let mut output_arr: Vec<u8> = Vec::new();               // Initialize an empty vector to store the encoded values

    for input_char in text.chars() {
        let curr_index = lst.iter().position(|&c| c == input_char).unwrap_or_default();    // Find the index of the input character in the alphabet
        let temp_char = lst[curr_index];                                        // Retrieve the character at the current index
        output_arr.push(curr_index as u8);                                      // Add the current index to the output array
        lst.remove(curr_index);                                                 // Remove the current character from the list
        lst.insert(0, temp_char);                                               // Insert the current character at the front of the list
    }
    output_arr
}

pub fn mtf_decode(data: &Vec<u8>, alphabet: &str) -> String {
    let mut lst: Vec<char> = alphabet.chars().collect();    // Convert the alphabet string into a character vector
    let mut output_text = String::new();                    // Initialize an empty string to store the decoded text

    for &index in data {
        let decoded_char = lst[index as usize];     // Retrieve the character at the given index from the list
        output_text.push(decoded_char);             // Append the decoded character to the output text
        lst.remove(index as usize);                 // Remove the decoded character from the list
        lst.insert(0, decoded_char);                // Insert the decoded character at the front of the list
    }
    output_text
}

fn main() {
    let mut input_text = String::new();
    let mut alphabet = String::new();

    println!("Enter the input text: ");
    io::stdin().read_line(&mut input_text).expect("Failed to read input.");
    let input_text = input_text.trim();

    println!("Enter the alphabet: ");
    io::stdin().read_line(&mut alphabet).expect("Failed to read input.");
    let alphabet = alphabet.trim();

    let encoded_text = mtf_encode(&input_text.to_string(), &alphabet.to_string());
    println!("S: {:?}", encoded_text);
    let decoded_text = mtf_decode(&encoded_text, &alphabet.to_string());
    println!("M: {}", decoded_text);
}
