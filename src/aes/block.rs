pub type Block = Vec<Vec<u8>>;

pub fn make_empty_block() -> Block {
    vec![vec![0; 4]; 4]
}