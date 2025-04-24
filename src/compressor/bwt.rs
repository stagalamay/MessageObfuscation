pub fn bwt_encode(text: &str) -> (String, usize) {
    let message = format!("{}\0", text);

    let rotations: Vec<String> = (0..message.len())
        .map(|i| format!("{}{}", &message[i..], &message[..i]))
        .collect();

    let mut sorted_rotations = rotations.clone();
    sorted_rotations.sort();

    let transformed_text: String = sorted_rotations
    .iter()
    .map(|rotation| rotation.chars().last().unwrap())
    .collect();
    let index = sorted_rotations
    .iter().position(|rotation| *rotation == message)
    .unwrap();

    (transformed_text, index)
}

pub fn bwt_decode(text: &String, index: usize) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut t = Vec::with_capacity(chars.len());

    for (i, &c) in chars.iter().enumerate() {
        t.push((c, i));
    }

    t.sort();

    let _l: Vec<usize> = t.iter().map(|&(_, idx)| idx).collect();

    let mut l_idx = index;
    let mut m = String::new();

    for _ in chars.iter().take(chars.len()) {
        let (cur_char, cur_idx) = t[l_idx];
        m.push(cur_char);
        l_idx = cur_idx;
    }
    m
}

fn main() {
    let message = "LET HIM COOK";
    let (transformed_text, index) = bwt_encode(&message.to_string());
    let i: usize = index as usize;
    let decoded = bwt_decode(&transformed_text, i);
    
    println!("Message: {}", message);
    println!("Transformed Text: {}", transformed_text);
    println!("Index: {}", index);
    println!("Decoded Text: {}", decoded);
}
