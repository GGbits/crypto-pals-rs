use std::collections::HashMap;

use crate::Bytes;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ScoredCandidate {
    pub(crate) key: u8,
    pub(crate) plaintext: Bytes,
    pub(crate) score: f64,
}

fn english_frequencies() -> HashMap<u8, f64> {
    HashMap::from([
        (b' ', 0.13000),
        (b'e', 0.11049),
        (b't', 0.07882),
        (b'a', 0.07108),
        (b'o', 0.06533),
        (b'i', 0.06063),
        (b'n', 0.05873),
        (b's', 0.05507),
        (b'h', 0.05298),
        (b'r', 0.05211),
        (b'd', 0.03698),
        (b'l', 0.03506),
        (b'c', 0.02419),
        (b'u', 0.02401),
        (b'm', 0.02097),
        (b'w', 0.02053),
        (b'f', 0.01940),
        (b'g', 0.01757),
        (b'y', 0.01714),
        (b'p', 0.01679),
        (b'b', 0.01296),
        (b'v', 0.00853),
        (b'k', 0.00670),
        (b'j', 0.00131),
        (b'x', 0.00131),
        (b'q', 0.00087),
        (b'z', 0.00061),
    ])
}

//  Chi Squared is: for each character c, compute (observed_count[c] - expected_count[c])^2 /
//  expected_count[c], sum across all characters. Lower score = better English match.
//  non alpha-numeric or space characters have penalty applied.
fn chi_squared(text: &Bytes) -> f64 {
    let ascii = text.0.to_ascii_lowercase();
    let mut total_count = 0.0;
    let penalty = 0.3;

    let letter_score: f64 = english_frequencies()
        .iter()
        .map(|(&b, &f)| {
            let count = ascii.iter().filter(|&&c| c == b).count() as f64;
            total_count += count;
            let expected = ascii.len() as f64 * f;
            ((count - expected) * (count - expected)) / expected
        })
        .sum();

    letter_score + (penalty * (ascii.len() as f64 - total_count))
}

pub(crate) fn crack_single_byte_xor(ciphertext: &Bytes) -> ScoredCandidate {
    let mut scores_vec: Vec<ScoredCandidate> = Vec::new();

    for u in 0..=255u8 {
        let plaintext = Bytes(ciphertext.0.iter().map(|&b| b ^ u).collect::<Vec<u8>>());
        let score = chi_squared(&plaintext);
        let sc = ScoredCandidate {
            key: u,
            plaintext,
            score,
        };
        scores_vec.push(sc);
    }
    scores_vec.sort_by(|a, b| a.score.total_cmp(&b.score));

    scores_vec[0].clone()
}
