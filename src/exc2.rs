use hex2b64::convertor;
use std::collections::BTreeMap;
use std::str;
use std::string::FromUtf8Error;

static ALPHABET: [char;26] = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
static ETAOIN: &'static str = "ETAOINSHRDLCUMWFGYPBVKJXQZ";

pub fn score(input: &Vec<u8>) -> i32 {
    match get_char_freq(input) {
        Ok(s)   => {
            let mut score = 0;
            let good = &ETAOIN[0..6];
            let bad = &ETAOIN[(ETAOIN.len()-6)..];
            let mut chunk = &s[0..6];

            for ch in chunk.chars() {
                if good.contains(ch) {
                    score += 1;
                }
            }
            chunk = &s[(s.len()-6)..];

            for ch in chunk.chars() {
                if bad.contains(ch) {
                    score += 1;
                }
            }
            score
        },
        Err(_)  => 0
    }
}

fn get_char_freq(v: &Vec<u8>) -> Result<String, FromUtf8Error> {
    let mut map = init_char_freq_map();
    let input = v.clone();
    //print_char_vec(&input);
    match String::from_utf8(input) {
        Ok(s) => {
            for ch in s.chars() {
                let upper = ch.to_uppercase().next().unwrap();
                if let Some(x) = map.get_mut(&upper) {
                    *x += 1;
                }
            }

            let mut map_flip = init_count_map(&map);

            for ochar in ETAOIN.chars().rev() {
                let ee = map.get(&ochar).unwrap();
                if let Some(x) = map_flip.get_mut(ee) {
                    x.push(ochar);
                }
            }

            let mut res = String::new();

            let ctr = map_flip.len();
            for i in (0..ctr).rev() {
                res.push_str(map_flip.values().nth(i).unwrap());
            }
            Ok(res)
        },
        Err(e)    => Err(e)
    }
}

pub fn print_char_vec(v: &Vec<u8>) {
    let vv = v.clone();
    match String::from_utf8(vv) {
        Ok(s) => println!("{}", s),
        Err(e)    => println!("Not UTF-8: {:?}",e)
    }
}

fn init_char_freq_map() -> BTreeMap<char, isize> {
    let mut map = BTreeMap::new();
    for ch in ALPHABET.iter() {
        map.insert(*ch,0 as isize);
    }
    map
}
fn init_count_map(source: &BTreeMap<char, isize>) -> BTreeMap<isize, String> {
    let mut map = BTreeMap::new();
    for (_, count) in source {
        map.entry(*count).or_insert(String::new());
    }
    map
}

pub fn byte_pad(input: String, tpad: char) -> String {
    let ns_hex_str = convertor::ascii_to_hex_str(input);
    let ns_hex_vec = convertor::str_to_hex_vec(ns_hex_str);
    let ns_padded = pad(&ns_hex_vec,&(tpad as u8));
    convertor::to_hex(&ns_padded)
}

pub fn pad(input: &Vec<u8>, tpad: &u8) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::with_capacity(input.len());
    for el in input {
        result.push(el ^ tpad);
    }
    result
}

pub fn decrypt(input_string: String, printall: bool) -> (String,char,i32) {
    let hex_vec = convertor::str_to_hex_vec(input_string);
    let mut current_score = 0;
    let mut xor:char = '0';
    let mut best_fit: Vec<u8> = Vec::with_capacity(hex_vec.len());
    for b in 0..255 {
        let ch = b as u8;
        let padded = pad(&hex_vec,&ch);
        let sc = score(&padded);
        if printall {
            print!("{} - ",ch as char);
            print_char_vec(&padded);
        }
        if sc > current_score {
            best_fit = padded;
            current_score = sc;
            xor = ch as char;
        }
    }

    let tres = String::from_utf8(best_fit).unwrap();
    (tres,xor,current_score)
}