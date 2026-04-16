use crate::types::{Bytes, Hex};

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
