use crate::aes::block::{Block, make_empty_block};


pub fn generate_key_schedule(key: &Block) -> Vec<Block> {
    todo!();
    let mut key_words = vec![];
    key_words.push(key[0].clone());
    key_words.push(key[1].clone());
    key_words.push(key[2].clone());
    key_words.push(key[3].clone());
    
    for i in 0..10 {
        let w1 = vec![];
        let w2 = vec![];
        let w3 = vec![];
        let w4 = vec![];
        key_words.push(w1);
        key_words.push(w2);
        key_words.push(w3);
        key_words.push(w4);
    }
    vec![make_empty_block()]
}