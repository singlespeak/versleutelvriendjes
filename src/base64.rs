pub mod convertor {
    const B64_CHARS: [u8; 64] = [
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

    const EQUALS: u8 = '=' as u8;

    pub fn to_base64(data: &[u8]) -> String {
        let byte_count = data.len();
        let char_count = (byte_count + 2) / 3 * 4;

        let mut byte_index = 0;
        let mut buffer: u8 = 0;
        let mut chars = Vec::<u8>::with_capacity(char_count);

        for element in data {
            match byte_index {
                0 => {
                    chars.push(B64_CHARS[(element >> 2) as usize]);
                    buffer = (element & 0x3) << 4;
                    byte_index = 1;
                }
                1 => {
                    chars.push(B64_CHARS[(buffer | (element >> 4)) as usize]);
                    buffer = (element & 0xf) << 2;
                    byte_index = 2;
                }
                2 => {
                    chars.push(B64_CHARS[(buffer | (element >> 6)) as usize]);
                    chars.push(B64_CHARS[(element & 0x3f) as usize]);
                    buffer = 0;
                    byte_index = 0;
                }
                _ => {
                    panic!("Internal error");
                }
            }
        }

        match byte_index {
            0 => {}
            1 => {
                chars.push(B64_CHARS[buffer as usize]);
                chars.push(EQUALS);
                chars.push(EQUALS);
            }
            2 => {
                chars.push(B64_CHARS[buffer as usize]);
                chars.push(EQUALS);
            }
            _ => {
                panic!("Internal error");
            }
        }

        unsafe {
            String::from_utf8_unchecked(chars)
        }
    }
}