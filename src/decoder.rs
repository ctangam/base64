use std::{io, vec};

use crate::alphabet::{Alphabet, Classic};

pub fn decode(data: &String) -> Result<Vec<u8>, io::Error> {
    let alphabet = Classic;
    decode_using_alphabet(alphabet, data)
}

pub fn decode_using_alphabet<T: Alphabet>(
    alphabet: T,
    data: &String,
) -> Result<Vec<u8>, io::Error> {
    if data.chars().count() % 4 != 0 {
        return Err(io::Error::from(io::ErrorKind::InvalidInput));
    }

    let result = data
        .chars()
        .collect::<Vec<char>>()
        .chunks(4)
        .map(|chunk| original(&alphabet, chunk))
        .flat_map(stitch)
        .collect();

    Ok(result)
}

fn original<T: Alphabet>(alphabet: &T, chunk: &[char]) -> Vec<u8> {
    chunk
        .iter()
        .filter(|c| **c != alphabet.get_padding_char())
        .map(|c| {
            alphabet
                .get_index_for_char(*c)
                .expect("unable to find char in alphabet")
        })
        .collect()
}

fn stitch(bytes: Vec<u8>) -> Vec<u8> {
    let out = match bytes.len() {
        2 => vec![
            (bytes[0] & 0b111111) << 2 | bytes[1] >> 4,
            (bytes[1] & 0b1111) << 4,
        ],
        3 => vec![
            (bytes[0] & 0b111111) << 2 | bytes[1] >> 4,
            (bytes[1] & 0b1111) << 4 | bytes[2] >> 2,
            (bytes[2] & 0b11) << 6
        ],
        4 => vec![
            (bytes[0] & 0b111111) << 2 | bytes[1] >> 4,
            (bytes[1] & 0b1111) << 4 | bytes[2] >> 2,
            (bytes[2] & 0b11) << 6 | bytes[3] & 0b111111,
        ],
        _ => unreachable!(),
    };

    out.into_iter().filter(|&x| x > 0).collect()
}