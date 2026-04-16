use crate::types::{Base64, Bytes, Hex};

#[test]
fn test_bin_to_hex() {
    let bin_str = Bytes(b"I'm killing your brain like a poisonous mushroom".to_vec());
    assert_eq!(
        Hex(
            "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string()
        ),
        Hex::from(bin_str)
    )
}

#[test]
fn test_hex_to_bin() {
    let hex_str = Hex(
        "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string()
    );
    let bin_answer = Bytes(b"I'm killing your brain like a poisonous mushroom".to_vec());

    let result = Bytes::try_from(hex_str);

    assert_eq!(bin_answer, result.unwrap())
}

#[test]
fn test_b64_to_bin() {
    let b64_str = Base64("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string());
    let bin_answer = Bytes(b"I'm killing your brain like a poisonous mushroom".to_vec());

    let result = Bytes::try_from(b64_str);

    assert_eq!(bin_answer, result.unwrap())
}

#[test]
fn test_bin_to_b64() {
    let bin_str = Bytes(b"I'm killing your brain like a poisonous mushroom".to_vec());
    assert_eq!(
        Base64("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string()),
        Base64::from(bin_str)
    )
}
