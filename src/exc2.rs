use hex2b64::convertor;
use std::collections::HashMap;

static CHAR_ARRAY: [char;52] = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z','a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];

pub fn pad(input: &Vec<u8>, tpad: &u8) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::with_capacity(input.len());
    for el in input {
        result.push(el ^ tpad);
    }
    result
}

pub fn score(input: &Vec<u8>) -> i32 {
    //print_char_vec(input);
    let map = init_hashmap();
    let mut othermap = init_hashmap();
    let mut cap_count = 0;
    let total_count: i32 = input.len() as i32;
    let mut abnormal_count = 0;
    let mut number_count = 0;
    let vowels: Vec<char> = vec!['a','e','i','o','u','y'];
    let mut was_vowel = false;
    let mut was_consonant = false;
    let mut consecutive_vowels = 0;
    let mut consecutive_consonants = 0;
    let mut max_consecutive_vowels = 0;
    let mut max_consecutive_consonants = 0;
    for el in input.iter() {
        //let mut tel = *el as char;
        let lower: u8;
        let el_char = *el as char;
        if !el_char.is_alphanumeric() && !el_char.is_whitespace() {
            abnormal_count += 1;
        }
        if el_char.is_numeric() {
            number_count += 1;
        }
        if el_char.is_alphabetic() {
            if vowels.contains(&el_char) {
                if was_vowel {
                    consecutive_vowels += 1;
                }
                else {
                    was_vowel = true;
                    was_consonant = false;
                    if consecutive_consonants > max_consecutive_consonants {
                        max_consecutive_consonants = consecutive_consonants;
                    }
                    consecutive_consonants = 0;
                }
            }
            else {
                if was_consonant {
                    consecutive_consonants += 1;
                } else {
                    was_consonant = true;
                    was_vowel = false;
                    if consecutive_vowels > max_consecutive_vowels {
                        max_consecutive_vowels = consecutive_vowels;
                    }
                    consecutive_vowels = 0;
                }
            }
        }
        else {
            if consecutive_vowels > max_consecutive_vowels {
                max_consecutive_vowels = consecutive_vowels;
            }
            if consecutive_consonants > max_consecutive_consonants {
                max_consecutive_consonants = consecutive_consonants;
            }
            consecutive_consonants = 0;
            consecutive_vowels = 0;
            was_consonant = false;
            was_vowel = false;
        }
        if el_char.is_uppercase() {
            lower = *el + 0x20;
            cap_count += 1;
        }
        else {
            lower = *el;
        }
        let lower_char = lower as char;
        match map.get(&lower_char) {
            //Some(tt) => println!("{}: {}", *el as char, tt),
            Some(_) => *othermap.get_mut(&lower_char).unwrap() += 1,
            None => {}
        }

    }

    //println!("total: {} - abnormal: {} - number: {} - caps: {} - consvowels: {} - conscons: {}", total_count, abnormal_count,number_count,cap_count,max_consecutive_vowels,max_consecutive_consonants);
    //let mut count_vec: Vec<_> = othermap.iter().collect();
    //count_vec.sort_by(|a, b| b.1.cmp(a.1));

    total_count - abnormal_count - (max_consecutive_consonants + 1) - (max_consecutive_vowels + 1)
}

pub fn print_char_vec(v: &Vec<u8>) {
    for ch in v {
        print!("{}", *ch as char);
    }
    println!("");
}

fn init_hashmap() -> HashMap<char, isize> {
    let mut map = HashMap::new();
    for ch in CHAR_ARRAY.iter() {
        map.insert(*ch,0 as isize);
    }
    map
}

pub fn byte_pad(input: String, tpad: char) -> String {
    let ns_hex_str = convertor::ascii_to_hex_str(input);
    let ns_hex_vec = convertor::str_to_hex_vec(ns_hex_str);
    let ns_padded = pad(&ns_hex_vec,&(tpad as u8));
    convertor::to_hex(&ns_padded)
}

pub fn decrypt(input_string: String) -> String {
    let hex_vec = convertor::str_to_hex_vec(input_string);
    let mut current_score = 0;
    let mut best_fit: Vec<u8> = Vec::with_capacity(hex_vec.len());
    for ch in CHARS.iter() {
        let padded = pad(&hex_vec,ch);
        let sc = score(&padded);
        if sc > current_score {
            best_fit = padded;
            current_score = sc;
        }
    }
    let mut res: String = String::with_capacity(best_fit.len());
    for ch in best_fit {
        res.push(ch as char);
    }

    res
}



const CHARS: [u8; 64] = [
    'A' as u8,
    'B' as u8,
    'C' as u8,
    'D' as u8,
    'E' as u8,
    'F' as u8,
    'G' as u8,
    'H' as u8,
    'I' as u8,
    'J' as u8,
    'K' as u8,
    'L' as u8,
    'M' as u8,
    'N' as u8,
    'O' as u8,
    'P' as u8,
    'Q' as u8,
    'R' as u8,
    'S' as u8,
    'T' as u8,
    'U' as u8,
    'V' as u8,
    'W' as u8,
    'X' as u8,
    'Y' as u8,
    'Z' as u8,
    'a' as u8,
    'b' as u8,
    'c' as u8,
    'd' as u8,
    'e' as u8,
    'f' as u8,
    'g' as u8,
    'h' as u8,
    'i' as u8,
    'j' as u8,
    'k' as u8,
    'l' as u8,
    'm' as u8,
    'n' as u8,
    'o' as u8,
    'p' as u8,
    'q' as u8,
    'r' as u8,
    's' as u8,
    't' as u8,
    'u' as u8,
    'v' as u8,
    'w' as u8,
    'x' as u8,
    'y' as u8,
    'z' as u8,
    '0' as u8,
    '1' as u8,
    '2' as u8,
    '3' as u8,
    '4' as u8,
    '5' as u8,
    '6' as u8,
    '7' as u8,
    '8' as u8,
    '9' as u8,
    '+' as u8,
    '/' as u8,
];