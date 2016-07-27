extern crate versleutelvriendjes;

fn main() {


    // encrypt:
    //let input = String::from("een twee drie hoedje van papier");
    //let encrypted = versleutelvriendjes::exc2::byte_pad(input,'X');

    // decrypt:
    let encrypted = String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let (decrypted,xor,score) = versleutelvriendjes::exc2::decrypt(encrypted,false);
    println!("{} - {} - {}",decrypted,xor,score);
}