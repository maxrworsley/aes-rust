use crate::aes::block::Block;

pub fn sub_bytes(block: &mut Block) {
    for row in block {
        for element in row {
            *element = get_substitute_byte(element);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_substitution() {
        let mut data: Block = vec![
            vec![0x19, 0xa0, 0x9a, 0xe9],
            vec![0x3d, 0xf4, 0xc6, 0xf8],
            vec![0xe3, 0xe2, 0x8d, 0x48],
            vec![0xbe, 0x2b, 0x2a, 0x08],
        ];
        
        let expected_result: Block = vec![
            vec![0xd4, 0xe0, 0xb8, 0x1e],
            vec![0x27, 0xbf, 0xb4, 0x41],
            vec![0x11, 0x98, 0x5d, 0x52],
            vec![0xae, 0xf1, 0xe5, 0x30],
        ];
        sub_bytes(&mut data);
        assert_eq!(data, expected_result);
    }
}

fn get_substitute_byte(byte: &u8) -> u8 {
    match byte {
        0x00 => 0x63,
        0x10 => 0xca,
        0x20 => 0xb7,
        0x30 => 0x04,
        0x40 => 0x09,
        0x50 => 0x53,
        0x60 => 0xd0,
        0x70 => 0x51,
        0x80 => 0xcd,
        0x90 => 0x60,
        0xa0 => 0xe0,
        0xb0 => 0xe7,
        0xc0 => 0xba,
        0xd0 => 0x70,
        0xe0 => 0xe1,
        0xf0 => 0x8c,
        0x01 => 0x7c,
        0x11 => 0x82,
        0x21 => 0xfd,
        0x31 => 0xc7,
        0x41 => 0x83,
        0x51 => 0xd1,
        0x61 => 0xef,
        0x71 => 0xa3,
        0x81 => 0x0c,
        0x91 => 0x81,
        0xa1 => 0x32,
        0xb1 => 0xc8,
        0xc1 => 0x78,
        0xd1 => 0x3e,
        0xe1 => 0xf8,
        0xf1 => 0xa1,
        0x02 => 0x77,
        0x12 => 0xc9,
        0x22 => 0x93,
        0x32 => 0x23,
        0x42 => 0x2c,
        0x52 => 0x00,
        0x62 => 0xaa,
        0x72 => 0x40,
        0x82 => 0x13,
        0x92 => 0x4f,
        0xa2 => 0x3a,
        0xb2 => 0x37,
        0xc2 => 0x25,
        0xd2 => 0xb5,
        0xe2 => 0x98,
        0xf2 => 0x89,
        0x03 => 0x7b,
        0x13 => 0x7d,
        0x23 => 0x26,
        0x33 => 0xc3,
        0x43 => 0x1a,
        0x53 => 0xed,
        0x63 => 0xfb,
        0x73 => 0x8f,
        0x83 => 0xec,
        0x93 => 0xdc,
        0xa3 => 0x0a,
        0xb3 => 0x6d,
        0xc3 => 0x2e,
        0xd3 => 0x66,
        0xe3 => 0x11,
        0xf3 => 0x0d,
        0x04 => 0xf2,
        0x14 => 0xfa,
        0x24 => 0x36,
        0x34 => 0x18,
        0x44 => 0x1b,
        0x54 => 0x20,
        0x64 => 0x43,
        0x74 => 0x92,
        0x84 => 0x5f,
        0x94 => 0x22,
        0xa4 => 0x49,
        0xb4 => 0x8d,
        0xc4 => 0x1c,
        0xd4 => 0x48,
        0xe4 => 0x69,
        0xf4 => 0xbf,
        0x05 => 0x6b,
        0x15 => 0x59,
        0x25 => 0x3f,
        0x35 => 0x96,
        0x45 => 0x6e,
        0x55 => 0xfc,
        0x65 => 0x4d,
        0x75 => 0x9d,
        0x85 => 0x97,
        0x95 => 0x2a,
        0xa5 => 0x06,
        0xb5 => 0xd5,
        0xc5 => 0xa6,
        0xd5 => 0x03,
        0xe5 => 0xd9,
        0xf5 => 0xe6,
        0x06 => 0x6f,
        0x16 => 0x47,
        0x26 => 0xf7,
        0x36 => 0x05,
        0x46 => 0x5a,
        0x56 => 0xb1,
        0x66 => 0x33,
        0x76 => 0x38,
        0x86 => 0x44,
        0x96 => 0x90,
        0xa6 => 0x24,
        0xb6 => 0x4e,
        0xc6 => 0xb4,
        0xd6 => 0xf6,
        0xe6 => 0x8e,
        0xf6 => 0x42,
        0x07 => 0xc5,
        0x17 => 0xf0,
        0x27 => 0xcc,
        0x37 => 0x9a,
        0x47 => 0xa0,
        0x57 => 0x5b,
        0x67 => 0x85,
        0x77 => 0xf5,
        0x87 => 0x17,
        0x97 => 0x88,
        0xa7 => 0x5c,
        0xb7 => 0xa9,
        0xc7 => 0xc6,
        0xd7 => 0x0e,
        0xe7 => 0x94,
        0xf7 => 0x68,
        0x08 => 0x30,
        0x18 => 0xad,
        0x28 => 0x34,
        0x38 => 0x07,
        0x48 => 0x52,
        0x58 => 0x6a,
        0x68 => 0x45,
        0x78 => 0xbc,
        0x88 => 0xc4,
        0x98 => 0x46,
        0xa8 => 0xc2,
        0xb8 => 0x6c,
        0xc8 => 0xe8,
        0xd8 => 0x61,
        0xe8 => 0x9b,
        0xf8 => 0x41,
        0x09 => 0x01,
        0x19 => 0xd4,
        0x29 => 0xa5,
        0x39 => 0x12,
        0x49 => 0x3b,
        0x59 => 0xcb,
        0x69 => 0xf9,
        0x79 => 0xb6,
        0x89 => 0xa7,
        0x99 => 0xee,
        0xa9 => 0xd3,
        0xb9 => 0x56,
        0xc9 => 0xdd,
        0xd9 => 0x35,
        0xe9 => 0x1e,
        0xf9 => 0x99,
        0x0a => 0x67,
        0x1a => 0xa2,
        0x2a => 0xe5,
        0x3a => 0x80,
        0x4a => 0xd6,
        0x5a => 0xbe,
        0x6a => 0x02,
        0x7a => 0xda,
        0x8a => 0x7e,
        0x9a => 0xb8,
        0xaa => 0xac,
        0xba => 0xf4,
        0xca => 0x74,
        0xda => 0x57,
        0xea => 0x87,
        0xfa => 0x2d,
        0x0b => 0x2b,
        0x1b => 0xaf,
        0x2b => 0xf1,
        0x3b => 0xe2,
        0x4b => 0xb3,
        0x5b => 0x39,
        0x6b => 0x7f,
        0x7b => 0x21,
        0x8b => 0x3d,
        0x9b => 0x14,
        0xab => 0x62,
        0xbb => 0xea,
        0xcb => 0x1f,
        0xdb => 0xb9,
        0xeb => 0xe9,
        0xfb => 0x0f,
        0x0c => 0xfe,
        0x1c => 0x9c,
        0x2c => 0x71,
        0x3c => 0xeb,
        0x4c => 0x29,
        0x5c => 0x4a,
        0x6c => 0x50,
        0x7c => 0x10,
        0x8c => 0x64,
        0x9c => 0xde,
        0xac => 0x91,
        0xbc => 0x65,
        0xcc => 0x4b,
        0xdc => 0x86,
        0xec => 0xce,
        0xfc => 0xb0,
        0x0d => 0xd7,
        0x1d => 0xa4,
        0x2d => 0xd8,
        0x3d => 0x27,
        0x4d => 0xe3,
        0x5d => 0x4c,
        0x6d => 0x3c,
        0x7d => 0xff,
        0x8d => 0x5d,
        0x9d => 0x5e,
        0xad => 0x95,
        0xbd => 0x7a,
        0xcd => 0xbd,
        0xdd => 0xc1,
        0xed => 0x55,
        0xfd => 0x54,
        0x0e => 0xab,
        0x1e => 0x72,
        0x2e => 0x31,
        0x3e => 0xb2,
        0x4e => 0x2f,
        0x5e => 0x58,
        0x6e => 0x9f,
        0x7e => 0xf3,
        0x8e => 0x19,
        0x9e => 0x0b,
        0xae => 0xe4,
        0xbe => 0xae,
        0xce => 0x8b,
        0xde => 0x1d,
        0xee => 0x28,
        0xfe => 0xbb,
        0x0f => 0x76,
        0x1f => 0xc0,
        0x2f => 0x15,
        0x3f => 0x75,
        0x4f => 0x84,
        0x5f => 0xcf,
        0x6f => 0xa8,
        0x7f => 0xd2,
        0x8f => 0x73,
        0x9f => 0xdb,
        0xaf => 0x79,
        0xbf => 0x08,
        0xcf => 0x8a,
        0xdf => 0x9e,
        0xef => 0xdf,
        0xff => 0x16
    }
}
