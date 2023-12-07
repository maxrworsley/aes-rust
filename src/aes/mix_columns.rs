use crate::aes::block::{make_empty_block, Block};

pub fn mix_columns(block: &mut Block) {
    let mut temp_block = make_empty_block();

    for j in 0..=3 {
        temp_block[0][j] = field_multiply_2(block[0][j])
            ^ field_multiply_3(block[1][j])
            ^ block[2][j]
            ^ block[3][j];
        temp_block[1][j] = block[0][j]
            ^ field_multiply_2(block[1][j])
            ^ field_multiply_3(block[2][j])
            ^ block[3][j];
        temp_block[2][j] = block[0][j]
            ^ block[1][j]
            ^ field_multiply_2(block[2][j])
            ^ field_multiply_3(block[3][j]);
        temp_block[3][j] = field_multiply_3(block[0][j])
            ^ block[1][j]
            ^ block[2][j]
            ^ field_multiply_2(block[3][j]);
    }
    *block = temp_block;
}

fn field_multiply_2(byte: u8) -> u8 {
    if byte & 0x80 == 0x80 {
        (byte << 1) ^ 0x1B
    } else {
        byte << 1
    }
}

fn field_multiply_3(byte: u8) -> u8 {
    field_multiply_2(byte) ^ byte
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_mix() {
        let mut data: Block = vec![
            vec![0xd4, 0xe0, 0xb8, 0x1e],
            vec![0xbf, 0xb4, 0x41, 0x27],
            vec![0x5d, 0x52, 0x11, 0x98],
            vec![0x30, 0xae, 0xf1, 0xe5],
        ];
        let expected_result: Block = vec![
            vec![0x04, 0xe0, 0x48, 0x28],
            vec![0x66, 0xcb, 0xf8, 0x06],
            vec![0x81, 0x19, 0xd3, 0x26],
            vec![0xe5, 0x9a, 0x7a, 0x4c],
        ];
        mix_columns(&mut data);
        assert_eq!(data, expected_result);
    }

    #[test]
    fn test_field_multiply() {
        let byte1 = 0xd4;
        assert_eq!(field_multiply_2(byte1), 0xb3);

        let byte2 = 0xbf;
        assert_eq!(field_multiply_3(byte2), 0xda);
    }
}
