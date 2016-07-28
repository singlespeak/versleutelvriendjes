pub fn str_to_hex_vec(tstr: &str) -> Vec<u8> {
    let mut buffer: u8 = 0;
    let mut sentinel = false;
    let mut result: Vec<u8> = Vec::new();
    for ch in tstr.bytes() {
        let xx: u8 = map_char_hex(ch as char);
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

pub fn to_hex_string(data: &[u8]) -> String {
    let mut chars = Vec::<u8>::with_capacity(data.len() * 2);

    for b in data {
        let val = *b as i32;
        let high = (val >> 4) & 0xF;
        let low = val & 0xF;
        chars.push(HEX_CHARS[high as usize]);
        chars.push(HEX_CHARS[low as usize]);
    }

    String::from_utf8(chars).unwrap()
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
        _ => 0xff
    }
}

const HEX_CHARS: [u8; 16] = [
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
    'a' as u8,
    'b' as u8,
    'c' as u8,
    'd' as u8,
    'e' as u8,
    'f' as u8
];