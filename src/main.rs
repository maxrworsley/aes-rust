mod aes;
use aes::{
    key_schedule,
    add_round_key,
    shift_rows,
    mix_columns,
    sub_bytes,
    block
};

pub fn encrypt(data: &mut block::Block, key: &block::Block) {
    let keys = key_schedule::generate_key_schedule(&key);
    let mut key_iter = keys.iter();
    
    if let Some(key_block) = key_iter.next() {
        add_round_key::add_round_key(data, &key_block);
    }

    for _ in 0..9 {
        sub_bytes::sub_bytes(data);
        shift_rows::shift_rows(data);
        mix_columns::mix_columns(data);
        if let Some(key_block) = key_iter.next() {
            add_round_key::add_round_key(data, &key_block);
        }
        print_as_block(data);
    }
    sub_bytes::sub_bytes(data);
    shift_rows::shift_rows(data);
    if let Some(key) = key_iter.next() {
        add_round_key::add_round_key(data, &key);
    }
}

fn print_as_block(data: &block::Block) {
    for row in data {
        for element in row {
            print!("{:x} ", element)
        }
        println!();
    }
    println!();
}

pub fn main() {
    let mut data = vec![
        vec![0x32, 0x88, 0x31, 0xe0],
        vec![0x43, 0x5a, 0x31, 0x37],
        vec![0xf6, 0x30, 0x98, 0x07],
        vec![0xa8, 0x8d, 0xa2, 0x34],
    ];

    let key = vec![
            vec![0x2b, 0x28, 0xab, 0x09],
            vec![0x7e, 0xae, 0xf7, 0xcf],
            vec![0x15, 0xd2, 0x15, 0x4f],
            vec![0x16, 0xa6, 0x88, 0x3c],
        ];

    encrypt(&mut data, &key);
    print_as_block(&data)
}
