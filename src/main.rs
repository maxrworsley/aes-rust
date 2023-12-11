mod aes;

fn print_as_block(data: &aes::block::Block) {
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

    aes::encrypt::encrypt(&mut data, &key);
    print_as_block(&data)
}
