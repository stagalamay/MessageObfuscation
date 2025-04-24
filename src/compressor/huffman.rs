use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum HNodeType {
    Parent,
    Leaf,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct HNode {
    node_val: u8,
    node_type: HNodeType,
    freq: i32,
    l_child: Option<Box<HNode>>,
    r_child: Option<Box<HNode>>,
}

impl Ord for HNode {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.freq.cmp(&other.freq) {
            Ordering::Equal => {
                match (self.node_type, other.node_type) {
                    (HNodeType::Parent, _) => Ordering::Greater,
                    (_, HNodeType::Parent) => Ordering::Less,
                    _ => Ordering::Equal,
                }
            }
            ordering => ordering.reverse(),
        }
    }
}

impl PartialOrd for HNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

fn build_huffman_tree(text: &[u8]) -> HNode {
    let mut char_map = vec![0; 256];

    for &c in text {
        char_map[c as usize] += 1;
    }

    let mut frequency_arr: Vec<HNode> = char_map
        .iter()
        .enumerate()
        .filter(|(_, &val)| val > 0)
        .map(|(i, &val)| HNode {
            node_val: i as u8,
            node_type: HNodeType::Leaf,
            freq: val,
            l_child: None,
            r_child: None,
        })
        .collect();

    frequency_arr.sort_by(|a, b| a.freq.cmp(&b.freq).then(a.node_val.cmp(&b.node_val)));

    let mut pq: BinaryHeap<HNode> = frequency_arr.into();

    while pq.len() > 1 {
        let u1 = pq.pop().unwrap();
        let u2 = pq.pop().unwrap();
        let new_freq = u1.freq + u2.freq;

        let (left_child, right_child) = if u1 < u2 {
            (u1, u2)
        } else {
            (u2, u1)
        };

        let new_node = HNode {
            node_val: 0_u8,
            node_type: HNodeType::Parent,
            freq: new_freq,
            l_child: Some(Box::new(left_child)),
            r_child: Some(Box::new(right_child)),
        };

        pq.push(new_node);
    }

    pq.pop().unwrap()
}

fn build_codebook(htree_node: &HNode, codeword_stack: &mut String, codeword_map: &mut HashMap<u8, String>) {
    if htree_node.node_type == HNodeType::Leaf {
        codeword_map.insert(htree_node.node_val, codeword_stack.clone());
    } else {
        if let Some(ref left_child) = htree_node.l_child {
            codeword_stack.push('1');
            build_codebook(&*left_child, codeword_stack, codeword_map);
            codeword_stack.pop();
        }
        if let Some(ref right_child) = htree_node.r_child {
            codeword_stack.push('0');
            build_codebook(&*right_child, codeword_stack, codeword_map);
            codeword_stack.pop();
        }
    }
}

pub fn encode_huffman(numbers: &Vec<u8>, codeword_map: &HashMap<u8, String>) -> Vec<u8> {
    let mut encoded_bits: Vec<u8> = Vec::new();
    for &num in numbers {
        let codeword = codeword_map.get(&num).unwrap();
        for c in codeword.chars() {
            let bit = match c {
                '0' => 0,
                '1' => 1,
                _ => panic!("Invalid codeword"),
            };
            encoded_bits.push(bit);
        }
    }
    encoded_bits
}

pub fn decode_huffman(encoded_bits: &[u8], codeword_map: &HashMap<u8, String>) -> Vec<u8> {
    let mut decoded_message = Vec::new();
    let mut current_codeword = String::new();

    for &bit in encoded_bits {
        current_codeword.push_str(match bit {
            1 => "1",
            _ => "0",
        });

        for (&num, codeword) in codeword_map.iter() {
            if current_codeword == *codeword {
                decoded_message.push(num);
                current_codeword.clear();
                break;
            }
        }
    }
    decoded_message
}

pub fn build_canonical_codebook(codebook: &HashMap<u8, String>) -> Vec<(u8, String)> {
    let mut sorted_map: Vec<(u8, String)> = codebook.iter().map(|(&key, value)| (key, value.clone())).collect();
    sorted_map.sort_by(|(_, ref value1), (_, ref value2)| {
        let length_comparison = value1.len().cmp(&value2.len());
        if length_comparison != Ordering::Equal {
            return length_comparison;
        }
        value1.cmp(value2)
    });

    if let Some((_, value)) = sorted_map.first_mut() {
        let zero_padding = "0".repeat(value.len());
        *value = zero_padding;
    }

    let mut l_canon = match sorted_map.get(0) {
        Some((_, codeword)) => codeword.len(),
        None => 0,
    };
    let mut c_canon = 0;

    let mut canonical_codebook = Vec::new();
    canonical_codebook.push((sorted_map[0].0.into(), sorted_map[0].1.clone()));

    let mut iter = sorted_map.iter().skip(1);
    while let Some((val, codeword)) = iter.next() {
        c_canon += 1;
        let cur_len = codeword.len();

        if cur_len > l_canon {
            c_canon <<= cur_len - l_canon;
            l_canon = cur_len;
        } else {
            c_canon &= (1 << l_canon) - 1;
        }
        let new_codeword = format!("{:0width$b}", c_canon, width = cur_len);
        canonical_codebook.push((val.clone(), new_codeword));
    }

    canonical_codebook
}

pub fn canonical_encode_bits(message: &[u8], canonical_codebook: &[(u8, String)]) -> Vec<u8> {
    let mut encoded_bits = Vec::new();

    for &num in message {
        for (val, codeword) in canonical_codebook {
            if num == *val {
                for bit in codeword.chars() {
                    let bit_value = if bit == '0' { 0 } else { 1 };
                    encoded_bits.push(bit_value);
                }
                break;
            }
        }
    }

    encoded_bits
}

pub fn canon_length(canonical_codebook: &[(u8, String)]) -> Vec<usize> {
    let mut lengths = vec![0; 10];

    for (value, codeword) in canonical_codebook {
        lengths[*value as usize] = codeword.len();
    }

    lengths
}

pub fn canonical_decode_bits(data: &[u8], canonical_codebook: &[(u8, String)]) -> Vec<u8> {
    let mut decoded_text = Vec::new();
    let mut cur_bits = String::new();

    for &bit in data {
        cur_bits.push_str(&bit.to_string());
        for (value, codeword) in canonical_codebook {
            if codeword == &cur_bits {
                decoded_text.push(*value);
                cur_bits.clear();
                break;
            }
        }
    }
    decoded_text
}

fn main() {
    let message: Vec<u8> = vec![5, 5, 6, 3, 3, 3, 1, 6, 6];
    println!("Original Message: {:?}", message);

    let huffman_tree = build_huffman_tree(&message);

    let mut codeword_map: HashMap<u8, String> = HashMap::new();
    build_codebook(&huffman_tree, &mut String::new(), &mut codeword_map);
    //println!("{:?}", huffman_tree);
    println!("Original Codebook: {:?}", codeword_map);

    let canonical_codebook = build_canonical_codebook(&codeword_map);
    println!("Canonical Codebook: {:?}", canonical_codebook);

    let encoded_bits = encode_huffman(&message, &codeword_map);
    println!("Encoded Bits: {:?}", encoded_bits);

    let canonical_encoded_bits = canonical_encode_bits(&message, &canonical_codebook);
    println!("Canonical Encoded Bits: {:?}", canonical_encoded_bits);

    let canon_length = canon_length(&canonical_codebook);
    println!("Canon Length: {:?}", canon_length);

    let decoded_message = decode_huffman(&encoded_bits, &codeword_map);
    println!("Decoded Message (Huffman only): {:?}", decoded_message); 

    let c_decoded_message = canonical_decode_bits(&canonical_encoded_bits, &canonical_codebook);
    println!("Decoded Message (Canonical): {:?}", c_decoded_message);
}
