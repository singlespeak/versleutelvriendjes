use std::collections::BTreeMap;

use std::time::Instant;
static ALPHABET: [char;26] = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
static ETAOIN: &'static str = "ETAOINSHRDLCUMWFGYPBVKJXQZ";

pub fn score(input: &str) -> Option<u32> {

    match get_char_freq(input) {
        Ok(s)   => {
            let now = Instant::now();
            let mut score: u32 = 0;
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
            let duration = now.elapsed();
            //println!("score: {}.{}", duration.as_secs(), duration.subsec_nanos());
            Some(score)
        },
        Err(_)  => {return Option::None}
    }
}

fn get_char_freq(v: &str) -> Result<String, String> {
    let mut map = init_char_freq_map();
    //let input = v.clone();
    for ch in v.chars() {
        if ch.is_control() && ch != '\n' && ch != '\r' && ch != '\t' {
            return Result::Err(String::from("Control character found in decrypted text."));
        }
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
//    match String::from_utf8(input) {
//        Ok(s) => {
//
//            for ch in s.chars() {
//                if ch.is_control() && ch != '\n' && ch != '\r' && ch != '\t' {
//                    return Result::Err(String::from("Control character found in decrypted text."));
//                }
//                let upper = ch.to_uppercase().next().unwrap();
//                if let Some(x) = map.get_mut(&upper) {
//                    *x += 1;
//                }
//            }
//
//            let mut map_flip = init_count_map(&map);
//
//            for ochar in ETAOIN.chars().rev() {
//                let ee = map.get(&ochar).unwrap();
//                if let Some(x) = map_flip.get_mut(ee) {
//                    x.push(ochar);
//                }
//            }
//
//            let mut res = String::new();
//
//            let ctr = map_flip.len();
//            for i in (0..ctr).rev() {
//                res.push_str(map_flip.values().nth(i).unwrap());
//            }
//            Ok(res)
//        },
//        Err(e)    => Err(e.to_string())
//    }
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