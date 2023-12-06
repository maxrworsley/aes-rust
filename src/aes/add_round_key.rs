use crate::aes::block::Block;

pub fn add_round_key(block: &mut Block, key: &Block) {
    for (row_data, row_key) in block.iter_mut().zip(key) {
        row_data
            .iter_mut()
            .zip(row_key.iter())
            .for_each(|(elem_data, elem_key)| {
                *elem_data ^= elem_key;
            });
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simpl_add_key() {
        let mut data: Block = vec![
            vec![0x32, 0x88, 0x31, 0xe0],
            vec![0x43, 0x5a, 0x31, 0x37],
            vec![0xf6, 0x30, 0x98, 0x07],
            vec![0xa8, 0x8d, 0xa2, 0x34],
        ];
        let key: Block = vec![
            vec![0x2b, 0x28, 0xab, 0x09],
            vec![0x7e, 0xae, 0xf7, 0xcf],
            vec![0x15, 0xd2, 0x15, 0x4f],
            vec![0x16, 0xa6, 0x88, 0x3c],
        ];

        let expected_result: Block = vec![
            vec![0x19, 0xa0, 0x9a, 0xe9],
            vec![0x3d, 0xf4, 0xc6, 0xf8],
            vec![0xe3, 0xe2, 0x8d, 0x48],
            vec![0xbe, 0x2b, 0x2a, 0x08],
        ];
        add_round_key(&mut data, &key);
        assert_eq!(data, expected_result);
    }
}
