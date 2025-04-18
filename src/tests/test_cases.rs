use crate::setup::utilities::*;
// use std::io::{self, Write};

#[cfg(test)]
mod tests {
    //use crate::setup::utilities::print_board;
    //use crate::tests::test_cases::main2;

    use super::*;

    #[test]
    fn test_vaild_with_valid_board(){
        let complete_valid_board: Vec<Vec<u32>> = vec![
            // 0's represent empty spaces

            vec![6,3,9, 5,7,4, 1,8,2],
            vec![5,4,1, 8,2,9, 3,7,6],
            vec![7,8,2, 6,1,3, 9,5,4],

            vec![1,9,8, 4,6,7, 5,2,3],
            vec![3,6,5, 9,8,2, 4,1,7],
            vec![4,2,7, 1,3,5, 8,6,9],

            vec![9,5,6, 7,4,8, 2,3,1],
            vec![8,1,3, 2,9,6, 7,4,5],
            vec![2,7,4, 3,0,1, 6,9,8]
        ];
        assert_eq!(true, valid(&complete_valid_board,5,(8,4)));
    }

    #[test]
    fn test_vaild_board_and_update_board(){
        let mut complete_valid_board: Vec<Vec<u32>> = vec![
            // 0's represent empty spaces

            vec![6,3,9, 5,7,4, 1,8,2],
            vec![5,4,1, 8,2,9, 3,7,6],
            vec![7,8,2, 6,1,3, 9,5,4],

            vec![1,9,8, 4,6,7, 5,2,3],
            vec![3,6,5, 9,8,2, 4,1,7],
            vec![4,2,7, 1,3,5, 8,6,9],

            vec![9,5,6, 7,4,8, 2,3,1],
            vec![8,1,3, 2,9,6, 7,4,5],
            vec![2,7,4, 3,0,1, 6,9,8]
        ];
        
        update_board(&mut complete_valid_board, 5,(8,4));
        assert_eq!(true, valid_board(&complete_valid_board));
    }
    
    #[test]
    fn is_not_valid_row(){
        let repeat_row_board: Vec<Vec<u32>> = vec![
        // 0's represent empty spaces

        vec![6,3,0, 5,7,4, 1,8,2],
        vec![5,4,1, 8,2,9, 3,7,6],
        vec![7,8,2, 6,1,3, 9,5,4],

        vec![1,9,8, 4,6,7, 5,2,3],
        vec![3,6,5, 9,8,2, 4,1,7],
        vec![4,2,7, 1,3,5, 8,6,9],

        vec![9,5,6, 7,4,8, 2,3,1],
        vec![8,1,3, 2,9,6, 7,4,5],
        vec![2,7,4, 3,5,1, 6,9,8]
        ];
        assert_eq!(false, row_compatible(&repeat_row_board,3,(0,2)));
    }
    
    #[test]
    fn is_not_vaild_column(){
        let repeat_column_board: Vec<Vec<u32>> = vec![
        // 0's represent empty spaces

        vec![6,3,9, 5,7,4, 1,8,2],
        vec![0,4,1, 8,2,9, 3,7,6],
        vec![7,8,2, 6,1,3, 9,5,4],

        vec![1,9,8, 4,6,7, 5,2,3],
        vec![3,6,5, 9,8,2, 4,1,7],
        vec![4,2,7, 1,3,5, 8,6,9],

        vec![9,5,6, 7,4,8, 2,3,1],
        vec![8,1,3, 2,9,6, 7,4,5],
        vec![2,7,4, 3,5,1, 6,9,8]
        ];
        assert_eq!(false,column_compatible(&repeat_column_board,6,(1,0)));
    }
    #[test]
    fn is_not_vaild_box(){
        let repeat_box_board: Vec<Vec<u32>> = vec![
        // 0's represent empty spaces

        vec![6,3,9, 5,7,4, 1,8,2],
        vec![5,4,1, 8,2,9, 3,7,6],
        vec![7,8,0, 0,1,3, 9,5,4],

        vec![1,9,8, 4,6,7, 5,2,3],
        vec![3,6,5, 9,8,2, 4,1,7],
        vec![4,2,7, 1,3,5, 8,6,9],

        vec![9,5,6, 7,4,8, 2,3,1],
        vec![8,1,3, 2,9,6, 7,4,5],
        vec![2,7,4, 3,5,1, 6,9,8]
        ];
        assert_eq!(false,box_compatible(&repeat_box_board,6,(2,2)));
    }

    #[test]
    fn valid_row_add(){
        let complete_valid_board: Vec<Vec<u32>> = vec![
            // 0's represent empty spaces

            vec![0,3,9, 5,7,4, 1,8,2],
            vec![5,4,1, 8,2,9, 3,7,6],
            vec![7,8,2, 6,1,3, 9,5,4],

            vec![1,9,8, 4,6,7, 5,2,3],
            vec![3,6,5, 9,8,2, 4,1,7],
            vec![4,2,7, 1,3,5, 8,6,9],

            vec![9,5,6, 7,4,8, 2,3,1],
            vec![8,1,3, 2,9,6, 7,4,5],
            vec![2,7,4, 3,5,1, 6,9,8]
        ];
        assert_eq!(true,row_compatible(&complete_valid_board,6,(0,0)));
    }
    
    #[test]
    fn vaild_column_add(){
        let complete_valid_board: Vec<Vec<u32>> = vec![
            // 0's represent empty spaces

            vec![0,3,9, 5,7,4, 1,8,2],
            vec![5,4,1, 8,2,9, 3,7,6],
            vec![7,8,2, 6,1,3, 9,5,4],

            vec![1,9,8, 4,6,7, 5,2,3],
            vec![3,6,5, 9,8,2, 4,1,7],
            vec![4,2,7, 1,3,5, 8,6,9],

            vec![9,5,6, 7,4,8, 2,3,1],
            vec![8,1,3, 2,9,6, 7,4,5],
            vec![2,7,4, 3,5,1, 6,9,8]
        ];
        assert_eq!(true,column_compatible(&complete_valid_board,6,(0,0)));
    }
    
    #[test]
    fn vaild_box_add(){
        let complete_valid_board: Vec<Vec<u32>> = vec![
            // 0's represent empty spaces

            vec![6,3,9, 5,7,4, 1,8,2],
            vec![5,4,1, 8,2,9, 3,7,6],
            vec![7,8,0, 6,1,3, 9,5,4],

            vec![1,9,8, 4,6,7, 5,2,3],
            vec![3,6,5, 9,8,2, 4,1,7],
            vec![4,2,7, 1,3,5, 8,6,9],

            vec![9,5,6, 7,4,8, 2,3,1],
            vec![8,1,3, 2,9,6, 7,4,5],
            vec![2,7,4, 3,5,1, 6,9,8]
        ];
        assert_eq!(true,box_compatible(&complete_valid_board,2,(2,2)));
    }
}





