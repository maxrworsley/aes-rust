use crate::aes::block::Block;

pub fn shift_rows(block: &mut Block) {
    for (offset, row) in block.iter_mut().enumerate() {
        let row_0 = row[(offset) % 4];
        let row_1 = row[(1 + offset) % 4];
        let row_2 = row[(2 + offset) % 4];
        let row_3 = row[(3 + offset) % 4];
        row[0] = row_0;
        row[1] = row_1;
        row[2] = row_2;
        row[3] = row_3;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_shift() {
        let mut data: Block = vec![
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
        ];
        let expected_result: Block = vec![
            vec![1, 2, 3, 4],
            vec![2, 3, 4, 1],
            vec![3, 4, 1, 2],
            vec![4, 1, 2, 3],
        ];
        shift_rows(&mut data);
        assert_eq!(data, expected_result);
    }
}
