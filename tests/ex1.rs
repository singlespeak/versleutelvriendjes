extern crate versleutelvriendjes;

#[test]
fn to_base64() {
    let hex = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let base64 = versleutelvriendjes::hex2b64::hex_str_to_b64_str(hex);

    let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    assert_eq!(expected, base64);
}