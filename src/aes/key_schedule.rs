use crate::aes::block::{make_empty_block, Block};
use crate::aes::sub_bytes::get_substitute_byte;

pub fn generate_key_schedule_old(key: &Block) -> Vec<Block> {
    let mut key_words = vec![];
    key_words.push(key[0].clone());
    key_words.push(key[1].clone());
    key_words.push(key[2].clone());
    key_words.push(key[3].clone());

    for i in 0..10 {
        let prev_i_start = i * 4;
        let malformed_prev = malform_vector(&key_words[prev_i_start + 3], i + 1);
        let w0 = xor_vec(&key_words[prev_i_start], &malformed_prev);
        let w1 = xor_vec(&key_words[prev_i_start + 1], &w0);
        let w2 = xor_vec(&key_words[prev_i_start + 2], &w1);
        let w3 = xor_vec(&key_words[prev_i_start + 3], &w2);
        key_words.push(w0);
        key_words.push(w1);
        key_words.push(w2);
        key_words.push(w3);
    }

    let mut return_blocks = vec![];
    for i in 0..11 {
        let block_start_i = i * 4;
        return_blocks.push(vec![
            key_words[block_start_i].clone(),
            key_words[block_start_i + 1].clone(),
            key_words[block_start_i + 2].clone(),
            key_words[block_start_i + 3].clone(),
        ])
    }
    return_blocks
}

pub fn generate_key_schedule(key: &Block) -> Vec<Block> {
    let mut key_words = vec![];

    for i in 0..key.len() {
        let mut new_row = vec![];
        for j in 0..key[i].len() {
            new_row.push(key[j][i]);
        }
        key_words.push(new_row);
    }

    for i in 0..10 {
        let prev_i_start = i * 4;
        let malformed_prev = malform_vector(&key_words[prev_i_start + 3], i + 1);
        let w0 = xor_vec(&key_words[prev_i_start], &malformed_prev);
        let w1 = xor_vec(&key_words[prev_i_start + 1], &w0);
        let w2 = xor_vec(&key_words[prev_i_start + 2], &w1);
        let w3 = xor_vec(&key_words[prev_i_start + 3], &w2);
        key_words.push(w0);
        key_words.push(w1);
        key_words.push(w2);
        key_words.push(w3);
    }

    let mut return_blocks = vec![];
    for i in 0..11 {
        let block_start_i = i * 4;
        let untrasposed = vec![
            key_words[block_start_i].clone(),
            key_words[block_start_i + 1].clone(),
            key_words[block_start_i + 2].clone(),
            key_words[block_start_i + 3].clone(),
        ];
        let mut transposed_block = vec![];

        for i in 0..untrasposed.len() {
            let mut new_row = vec![];
            for j in 0..untrasposed[i].len() {
                new_row.push(untrasposed[j][i]);
            }
            transposed_block.push(new_row);
        }
        return_blocks.push(transposed_block);
    }
    return_blocks
}

fn xor_vec(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter()
        .zip(b.iter())
        .map(|(a, b)| a ^ b)
        .collect::<Vec<u8>>()
}

fn malform_vector(prev: &[u8], round_number: usize) -> Vec<u8> {
    // One byte left circular rotation and subBytes
    let new_vector = vec![
        get_substitute_byte(&prev[1]),
        get_substitute_byte(&prev[2]),
        get_substitute_byte(&prev[3]),
        get_substitute_byte(&prev[0]),
    ];

    xor_vec(&new_vector, &get_rcon(round_number))
}

fn get_rcon(round_number: usize) -> Vec<u8> {
    match round_number {
        1 => vec![0x01, 0x0, 0x0, 0x0],
        2 => vec![0x02, 0x0, 0x0, 0x0],
        3 => vec![0x04, 0x0, 0x0, 0x0],
        4 => vec![0x08, 0x0, 0x0, 0x0],
        5 => vec![0x10, 0x0, 0x0, 0x0],
        6 => vec![0x20, 0x0, 0x0, 0x0],
        7 => vec![0x40, 0x0, 0x0, 0x0],
        8 => vec![0x80, 0x0, 0x0, 0x0],
        9 => vec![0x1B, 0x0, 0x0, 0x0],
        10 => vec![0x36, 0x0, 0x0, 0x0],
        _ => {
            panic!("Round constant not stored for round {}", round_number)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_xor() {
        let data_a: Vec<u8> = vec![0b10011001, 0b01011101, 0b00011000, 0b11111111];
        let data_b: Vec<u8> = vec![0b10100111, 0b01011101, 0b10001001, 0b00000000];
        let exp_result: Vec<u8> = vec![0b00111110, 0b00000000, 0b10010001, 0b11111111];
        let result = xor_vec(&data_a, &data_b);

        assert_eq!(result, exp_result);
    }

    #[test]
    fn test_key_expansion() {
        let mut key = vec![
            vec![0x2b, 0x28, 0xab, 0x09],
            vec![0x7e, 0xae, 0xf7, 0xcf],
            vec![0x15, 0xd2, 0x15, 0x4f],
            vec![0x16, 0xa6, 0x88, 0x3c],
        ];
        let first_expanded_key = vec![
            vec![0x2b, 0x28, 0xab, 0x09],
            vec![0x7e, 0xae, 0xf7, 0xcf],
            vec![0x15, 0xd2, 0x15, 0x4f],
            vec![0x16, 0xa6, 0x88, 0x3c],
        ];
        let second_expanded_key = vec![
            vec![0xa0, 0x88, 0x23, 0x2a],
            vec![0xfa, 0x54, 0xa3, 0x6c],
            vec![0xfe, 0x2c, 0x39, 0x76],
            vec![0x17, 0xb1, 0x39, 0x05],
        ];
        let third_expanded_key = vec![
            vec![0xf2, 0x7a, 0x59, 0x73],
            vec![0xc2, 0x96, 0x35, 0x59],
            vec![0x95, 0xb9, 0x80, 0xf6],
            vec![0xf2, 0x43, 0x7a, 0x7f],
        ];
        let keys_generated = generate_key_schedule(&mut key);
        assert_eq!(keys_generated[0], first_expanded_key);
        assert_eq!(keys_generated[1], second_expanded_key);
        assert_eq!(keys_generated[2], third_expanded_key);
    }
}
