//! CISC 260 - HW One - Part Two
//! Hexadecimal to Integer

use std::collections::HashMap;
use std::io;

fn main() {
    let input = get_input();
    let pad_input = pad_hex(&input);
    let int = to_int(&pad_input);

    println!("Integer, Calculated:\t{int}");
    println!("Integer, Actual:\t{:?}", u32::from_str_radix(input.as_str(), 16).unwrap());
}

fn get_input() -> String {
    let mut input = String::new();

    println!("Enter your 32-bit hexadecimal: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    input.trim().to_ascii_uppercase()
}

fn pad_hex(raw_hex: &str) -> String {
    format!("{raw_hex:0>8}")
}

fn to_int(hex: &str) -> u32 {
    let hex_map: HashMap<char, u32> = HashMap::from([
        ('0', 0),
        ('1', 1),
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('A', 10),
        ('B', 11),
        ('C', 12),
        ('D', 13),
        ('E', 14),
        ('F', 15),
    ]);

    // Iterate and sum the values via an accumulator, `acc`
    hex.char_indices().fold(0, |acc, (i, c)| {
        acc + u32::pow(16, 7 - i as u32) * hex_map.get(&c).unwrap()
    })
}

#[test]
fn test_to_int() {
    for val in ["0", "1", "A", "F0", "10000000", "0FDD5FDD", "FFFFFFFF"] {
        let t_val = pad_hex(val);
        let t_exp = u32::from_str_radix(val, 16).unwrap();
        assert_eq!(to_int(&t_val), t_exp);
    }
}
