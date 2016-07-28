use std::time::Instant;

pub fn print_byte_vec(v: &Vec<u8>) {
    let vv = v.clone();
    match String::from_utf8(vv) {
        Ok(s) => println!("{}", s),
        Err(e)    => println!("Not UTF-8: {:?}",e)
    }
}


pub fn xor(input: &[u8], tpad: &u8) -> String {
    let mut result: Vec<u8> = Vec::with_capacity(input.len());
    for el in input {
        result.push(el ^ tpad);
    }
    let res = String::from_utf8(result);
    match res {
        Ok(x) => x,
        Err(x) => String::from("")
    }
}

pub fn xor_multibyte(input: &str, xor: &str) -> Vec<u8> {
    let mut idx: usize = 0;
    let mut res: Vec<u8> = Vec::with_capacity(input.len());
    for byte in input.as_bytes() {
        let x = xor.as_bytes()[idx];
        res.push(byte ^ x);
        idx += 1;
        if idx == xor.len() {
            idx = 0;
        }
    }
    res
    //String::from_utf8(res).unwrap()
}

pub fn decrypt_xor_single_byte<F>(input: &[u8], printall: bool, score: F) -> Result<(String,char,u32), String> where F: Fn(&str) -> Option<u32> {
    let mut current_score = 0;
    let mut xor_char:char = '0';
    //let mut best_fit: Vec<u8> = Vec::with_capacity(input.len());
    let mut best_fit = String::with_capacity(input.len());

    for b in 0..255 {
        let now = Instant::now();
        let ch = b as u8;
        let padded = xor(input,&ch);
        if let Some(sc) = score(&padded) {
            if printall {
                print!("{} - ",ch as char);
                //print_byte_vec(&padded);
            }
            if sc > current_score {
                best_fit = padded;
                current_score = sc;
                xor_char = ch as char;
            }

        }
        let duration = now.elapsed();
        //println!("{}.{:09}", duration.as_secs(), duration.subsec_nanos());
    }
    Ok((best_fit,xor_char,current_score))
//    let tres = String::from_utf8(best_fit);
//    match tres {
//        Ok(x)   => {Ok((x,xor_char,current_score))},
//        Err(err)    => {Err(err.to_string())}
//    }

}