use crate::alphabet::{Alphabet, Classic};

fn split(chunk: &[u8]) -> Vec<u8> {
    match chunk.len() {
        1 => vec![&chunk[0] >> 2, (&chunk[0] & 0b11) << 4],
        2 => vec![
            &chunk[0] >> 2,
            (&chunk[0] & 0b11) << 4 | (&chunk[1] >> 4),
            (&chunk[1] & 0b1111) << 2,
        ],
        3 => vec![
            &chunk[0] >> 2,
            (&chunk[0] & 0b11) << 4 | (&chunk[1] >> 4),
            (&chunk[1] & 0b1111) << 2 | (&chunk[2] >> 6),
            &chunk[2] & 0b111111,
        ],
        _ => unreachable!(),
    }
}

pub fn encode(data: &[u8]) -> String {
    let alphabet = Classic;
    encode_using_alphabet(&alphabet, data)
}

pub fn encode_using_alphabet<T: Alphabet>(alphabet: &T, data: &[u8]) -> String {
    let encoded = data
        .chunks(3)
        .map(split)
        .flat_map(|chunk| encode_chunk(alphabet, chunk));

    String::from_iter(encoded)
}

fn encode_chunk<T: Alphabet>(alphabet: &T, chunk: Vec<u8>) -> Vec<char> {
    let mut out = vec![alphabet.get_padding_char(); 4];

    for i in 0..chunk.len() {
        if let Some(c) = alphabet.get_char_for_index(chunk[i]) {
            out[i] = c;
        }
    }

    out
}
