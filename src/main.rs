extern crate versleutelvriendjes;

fn main() {


//    encrypt:
    let ns = String::from("hoedje van papier");
    //let ns = String::from("'etaoin shrdlu' (eh-tay-oh-in shird-loo), was believed to be the twelve most common letters in the English language.");
    let encrypted = versleutelvriendjes::exc2::byte_pad(ns,'s');
    println!("{}",encrypted);
    // decrypt
    //let input_string = String::from("0412011d1a1d1449530512011a12111f1653131012032c101c061d0713531a00531200001a141d161753071c5f53110607531d1605160153060016175f5350280412011d5b061d060016172c0512011a12111f16005a2e531c1d53110a5317161512061f075d");
    let decrypted = versleutelvriendjes::exc2::decrypt(encrypted);
    println!("{}",decrypted);
}