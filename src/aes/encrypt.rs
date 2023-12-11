use crate::aes::{add_round_key, block, key_schedule, mix_columns, shift_rows, sub_bytes};

pub fn encrypt(data: &mut block::Block, key: &block::Block) {
    let keys = key_schedule::generate_key_schedule(key);
    let mut key_iter = keys.iter();

    if let Some(key_block) = key_iter.next() {
        add_round_key::add_round_key(data, key_block);
    } else { panic!("Not enough keys were generated") }

    for _ in 0..9 {
        sub_bytes::sub_bytes(data);
        shift_rows::shift_rows(data);
        mix_columns::mix_columns(data);
        if let Some(key_block) = key_iter.next() {
            add_round_key::add_round_key(data, key_block);
        } else { panic!("Not enough keys were generated") }
    }

    sub_bytes::sub_bytes(data);
    shift_rows::shift_rows(data);
    
    if let Some(key) = key_iter.next() {
        add_round_key::add_round_key(data, key);
    } else { panic!("Not enough keys were generated") }

}