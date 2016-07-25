pub mod convertor {
    pub fn ascii_to_hex_str(tstr: String) -> String {
        let mut ret = String::new();
        for ch in tstr.chars() {
            let left: u8 = (ch as u8) >> 4;
            let right: u8 = (ch as u8) & 0x0F;
            ret.push(map_hex_char(left));
            ret.push(map_hex_char(right));
        }
        ret
    }

    pub fn hex_str_to_b64_str(challenge: String) -> String {
        let challenge_hex: Vec<u8> = str_to_hex_vec(challenge);
        hex_vec_to_b64_str(challenge_hex)
    }

    pub fn str_to_hex_vec(tstr: String) -> Vec<u8> {
        let mut buffer: u8 = 0;
        let mut sentinel = false;
        let mut result: Vec<u8> = Vec::new();
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
        let byte_count = v.len();
        let char_count = (byte_count + 2) / 3 * 4;

        let mut buffer_size = 0;
        let mut acc: u8;
        let mut buffer: u8 = 0;
        let mut hex_vec: Vec<u8> = Vec::new();
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
                    let newel = el & 0x3F;
                    hex_vec.push(newel);
                    buffer = 0;
                    buffer_size = 0;
                }
                _ => {}
            }
        }
        if buffer_size != 0 {
            buffer = buffer << (6 - buffer_size);
            hex_vec.push(buffer);
        }

        let base_64_array = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/', '='];
        let mut result = String::with_capacity(char_count);
        for el in hex_vec {
            result.push(base_64_array[el as usize]);
        }
        if buffer_size == 2 {
            result.push('=');
            result.push('=');
        }
            else if buffer_size == 4 {
            result.push('=');
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
            _ => 0xff
        }
    }


    pub fn to_hex(data: &[u8]) -> String {
        let mut chars = Vec::<u8>::with_capacity(data.len() * 2);

        for b in data {
            let val = *b as i32;
            let high = (val >> 4) & 0xF;
            let low = val & 0xF;
            chars.push(CHARS[high as usize]);
            chars.push(CHARS[low as usize]);
        }

        String::from_utf8(chars).unwrap()
    }

    fn map_hex_char(tchar: u8) -> char {
        match tchar {
            0x00 => '0',
            0x01 => '1',
            0x02 => '2',
            0x03 => '3',
            0x04 => '4',
            0x05 => '5',
            0x06 => '6',
            0x07 => '7',
            0x08 => '8',
            0x09 => '9',
            0x0A => 'a',
            0x0B => 'b',
            0x0C => 'c',
            0x0D => 'd',
            0x0E => 'e',
            0x0F => 'f',
            _ => 'X'
        }
    }

    const CHARS: [u8; 16] = [
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
}