fn main() {
    let mystr = hex_str_to_b64_str(String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"));
    println!("{}",mystr);
}

fn hex_str_to_b64_str(challenge: String) -> String {
    let challenge_hex: Vec<u8> = str_to_hex_vec(challenge);
    hex_vec_to_b64_str(challenge_hex)
}

fn str_to_hex_vec(tstr: String) -> Vec<u8> {
    let mut buffer: u8 = 0;
    let mut sentinel = false;
    let mut result:Vec<u8> = Vec::new();
    for ch in tstr.chars() {
        let xx: u8 = map_char_hex(ch);
        if xx == 0xFF {
            break;
        }
        if sentinel {
            buffer = buffer << 4;
            buffer = buffer | xx;
            result.push(buffer);
            sentinel = false;
            buffer = 0;
            continue;
        }
        buffer = xx;
        sentinel = true;
    }
    result
}

fn hex_vec_to_b64_str(v: Vec<u8>) -> String {
    let mut buffer_size = 0;
    let mut acc: u8;
    let mut buffer: u8 = 0;
    let mut hex_vec:Vec<u8> = Vec::new();
    for el in v {
        match buffer_size {
            0 => {
                let mask: u8 = 3;
                acc = el >> 2;
                hex_vec.push(acc);
                buffer = el & mask;
                buffer_size = 2;
            }
            2 => {
                let mask: u8 = 15;
                acc = el >> 4;
                buffer = buffer << 4;
                acc = buffer | acc;
                hex_vec.push(acc);
                buffer = el & mask;
                buffer_size = 4;
            }
            4 => {
                acc = el >> 6;
                buffer = buffer << 2;
                acc = buffer | acc;
                hex_vec.push(acc);
                let mut newel = el << 2;
                newel = newel >> 2;
                hex_vec.push(newel);
                buffer = 0;
                buffer_size = 0;
            }
            _ => {}
        }
    }
    if buffer_size != 0 {
        buffer = buffer << (8 - buffer_size);
        hex_vec.push(buffer);
    }

    let base_64_array = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z','a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','0','1','2','3','4','5','6','7','8','9','+','/','='];
    let mut result = String::new();
    for el in hex_vec {
        result.push(base_64_array[el as usize]);
    }
    result
}

fn map_char_hex(tchar: char) -> u8 {
    match tchar {
        '0' => 0x00,
        '1' => 0x01,
        '2' => 0x02,
        '3' => 0x03,
        '4' => 0x04,
        '5' => 0x05,
        '6' => 0x06,
        '7' => 0x07,
        '8' => 0x08,
        '9' => 0x09,
        'a' => 0x0a,
        'b' => 0x0b,
        'c' => 0x0c,
        'd' => 0x0d,
        'e' => 0x0e,
        'f' => 0x0f,
        _   => 0xff
    }
}