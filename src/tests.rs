use std::{any::Any, str::FromStr};

use crate::crypt::detect_keysize;
use crate::types::{Base64, Bytes, DecodeError, Hex};

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
    let b64_str =
        Base64("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string());
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

#[test]
fn test_b64_invalid_char() {
    let str = "I'm killing your brain like a poisonous mushroom";

    assert_eq!(
        DecodeError("".to_string()).type_id(),
        Base64::from_str(str).unwrap_err().type_id()
    )
}

#[test]
fn test_hex_invalid_char() {
    let str = "02b4f3dj91256a";

    assert_eq!(
        DecodeError("".to_string()).type_id(),
        Hex::from_str(str).unwrap_err().type_id()
    )
}

#[test]
fn test_b64_invalid_len() {
    let str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb";

    assert_eq!(
        DecodeError("".to_string()).type_id(),
        Base64::from_str(str).unwrap_err().type_id()
    )
}

#[test]
fn test_hex_invalid_len() {
    let str = "02b4f3d9125";

    assert_eq!(
        DecodeError("".to_string()).type_id(),
        Base64::from_str(str).unwrap_err().type_id()
    )
}

#[test]
fn test_hamming_distance() {
    let b1 = Bytes(b"this is a test".to_vec());
    let b2 = Bytes(b"wokka wokka!!!".to_vec());

    let result = b1.hamming_distance(&b2);
    assert_eq!(result, 37u32)
}

#[test]
fn test_detect_keysize() {
    let cypher_text = "1b090f16491b445f550b0a3d41110c061914071d131168285a0e061851530c1d1042230f16491a5c120155166804140d1c095c531a14453c091311491d441a0713094232150
f0c4e5b15550b0a3d135a061b0b551e06520d29171f0c4e1a14101a1f00681508170c646316191e4968091f100c4947531d1d1268161f450502141107170423410e0a0c4e57
0a161e00423815171b4e5c121b1649680c0342010f5a1759521c271408420b01500a55130b2c41170b070b3e271d171c68021b0c49195d071d0111290f1e421d065153131
7210f1d4205075a167f552629140907490f5a0a1a1c006816120d49085d1d110145240e0c0749075a533d170924411903074e501c55130b3115120b07091853171307316b
0d1c4e551d1152082d411e0d074940531b17002c41140d49015a165517093b045a16064e59121e174521155a16011c5b06121a453c091f420a1c55090c782b2741180e061
4101a1f003b41190e061d5153011d452e000e03054e401c55060d21125a121b075a10100116680014064908551f19170b680014050c023e2410520424131f030d1714151a
0b2c41160d1f0b141a1b522d2d0d164e491a5c165500003b155a0b1a4e5112060b4968031b0010647d531402153a04190b081a5153141e09681512031d4e1c0410550924411
d071d4e401b071d102f095a0b1d473e310006453f045a0c0c0b50531a07176807080b0c00500055060a6802150f0c4e561216194560161f420a0f5a53111d45211553683001
415407174525185a1200025812075e4525185a0a0c0f46075201452e14160400025816075e452a140e421d065101105516680f15421e0f4d53011a0c3b41160b07075a14520
1453b0816140c1c3e36031717310e14074e1d14141a1c0064410d074e1c5153131306210f1d42084e43120778322d41190307494053121711680814421d0141101d52122115
124208004d1c1b1745290f030f061c515f55130b2c4c7031011b40530002496818151749055a1c02521120000e42204e581c031745310e0f682b1b40530c1d106f131f420f0
14614100611210f5d421e065b5302174529131f682000141255020c3c410e0a081a14171a171626460e42050b405306070b68151210061b531b7f210a2504120d1e4e431655
1f0426001d070d4e401c5501002d410916081c47792c1d103a411207081c405f551f1c68091f031b1a1853021a0026413342080314041c060d68181517633a431c55010a3d0
d094e49015a1655150a290d56421e0b131f19520b2d171f1049025b0010524d3f09150344015c5a7f552629140907490f5a0a1a1c006816120d49085d1d110145240e0c0749
075a533d170924411903074e501c55130b3115120b07091853171307316b230d1c4e551d1152082d411e0d074940531b17002c41140d49015a165517093b045a16064e59121
e174521155a16011c5b06121a453c091f420a1c55090c782b2741180e061914101a1f003b41190e061d5153011d452e000e03054e521c07521120080942191c5d1d1617163b
411b0c0d4e5212191e0026411b0c0e0b5879221745290d0807080a4d53131d1026055a0e061851531c1c450004160e454e401b1052172d120e42001d141614011c644118030
b173e321b0b0a26045a15010114151c1c013b41160d1f0b141a1b522d2d0d16420a0f5a53111d45290f031601075a1459520729030342410b55000c5b6f110e0f4208005053
1817452c0e14451d4e5a16101645260e5a0d070b141619010068151542040f5f16551b1168151210061b531b55060d2d41191008144d793b1d452a0d1515490d5b1e1001452
b0d15110c4e401c5514043c0016420f014653011a0c3b410a1000005716060145290f1e420f0f581f101c45290f1d07056463165513093a041b06104e521c001c01680d1514
0c4e5d1d553a00240d56421d0651530717163c411311490b55000c5e452a00181b";

    let bytes = Bytes::try_from(Hex::from_str(cypher_text).unwrap()).unwrap();
    let keysize = detect_keysize(&bytes);
    assert_eq!(keysize, "Hazbin4Sure".len())
}

#[test]
fn test_crack_repeating_from_file() {
    let output = std::process::Command::new("cargo")
        .args([
            "run",
            "--",
            "xor",
            "crack-repeating",
            "--file",
            "/home/ggbits/Documents/crypto_1_6.txt",
        ])
        .output()
        .expect("failed to run");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));
}
