extern crate versleutelvriendjes;

use std::time::Instant;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::str;

fn main() {
//    let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
//    let xord = versleutelvriendjes::xor::xor_multibyte(input,"ICE");
//    let xord_v = versleutelvriendjes::hex::to_hex_string(&xord);
//    print!("{}",xord_v);

    let f = File::open("/home/vincent/dev/rust/projects/versleutelvriendjes/data/4.txt").unwrap();
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();

    let mut best_score = 0;
    let mut best_fit: String = String::new();
    let mut x: char = '0';
    for line in reader.lines() {
        let hex = line.unwrap();
        // work with buffer

        let encrypted = versleutelvriendjes::hex::str_to_hex_vec(&hex);



        if let Ok((res,ch,tscore)) = versleutelvriendjes::xor::decrypt_xor_single_byte(&encrypted,false,versleutelvriendjes::freq_analysis::score) {
            if tscore > best_score {
                best_fit = res;
                best_score = tscore;
                x = ch;
            }
            buffer.clear();
        }
    }

    println!("fit: {} - score: {} - char: {:?}", best_fit, best_score,x);
}