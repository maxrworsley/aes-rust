mod aes;

pub fn main() {
    let key = aes::block::make_empty_block();
    let mut data = aes::block::make_empty_block();
    data[0][1] = 0x01;

    aes::add_round_key::add_round_key(&mut data, &key);
    aes::mix_columns::mix_columns(&mut data);
    aes::shift_rows::shift_rows(&mut data);
    aes::sub_bytes::sub_bytes(&mut data);
    println!("{:?}", data);
}
