use crate::setup::utilities::{valid, every_spot_full};

pub fn generate_solve_board(board: &mut Vec<Vec<u32>>) -> &Vec<Vec<u32>>{
    //Creating a list of the locations of the hints
    let mut hints: Vec<(usize, usize)> = vec![];
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] != 0 {
                hints.push((i, j));
            }
        }
    }
    return solving_recursive_helper(board,(0,0), hints, false, 0)
}

fn solving_recursive_helper(board: &mut Vec<Vec<u32>>, pos: (usize, usize), hints: Vec<(usize, usize)>, backtracking: bool, mut num_touch: u32) -> &Vec<Vec<u32>> {
    //Keeping track of how many times the recusion happens to catch infinite insolvability loops before they happen
    num_touch += 1;

    //Stops once every spot has a value or it appears to be unsolvable
    if every_spot_full(board) || num_touch > 10000 {
        return board
    }
    //Handles when the position is not a hint
    else if !hints.contains(&pos) {
        //Increments the value already at that location
        let value = board[pos.0][pos.1] + 1;

        //Checks whether that value or another is possible
        let current_value = fill_cell(board, value, pos);

        //Indicates that backtracking is needed
        if current_value == 100 {
            //Negates what was at that position and moves backwared
            board[pos.0][pos.1] = 0;
            return solving_recursive_helper(board, move_backwards(pos, 1), hints, true, num_touch)
        }
        //Backtracking not needed
        else {
            //Value update and position moves forward
            board[pos.0][pos.1] = current_value;
            return solving_recursive_helper(board, move_forwards(pos, 1), hints, false, num_touch)
        }
    }
    //Handles if the position is a hint since it can't change the hint
    else {
        //Moves back past the hint if its in backtracking mode 
        if backtracking {
            return solving_recursive_helper(board, move_backwards(pos, 1), hints, true, num_touch)
        }
        //Moves forward past the hint
        else {
            return solving_recursive_helper(board, move_forwards(pos, 1), hints, false, num_touch)
        }
    }
}

//Checks if the proposed value can go in the current position and if not finds the next possible one unless none is available in which case it returns 100
pub fn fill_cell(board: &mut Vec<Vec<u32>>, value: u32, pos:(usize,usize)) -> u32 {
    //Prevents numbers greater than 9
    if value > 9 {
        return 100
    }

    //Validity check
    let validity = valid(&board, value, (pos.0 as u32, pos.1 as u32));
    if validity {
        return value
    }
    else {
        if value == 9 {
            return 100
        }
        else {
            return fill_cell(board, value + 1, pos)
        }
    }
}

//Handles logic for moving backwards on the board (when backtracking)
fn move_backwards(pos: (usize, usize), number_of_times: u32) -> (usize, usize) {
    if pos == (0,0){
        return (0,0)
    }

    let mut new_row = pos.0 as i32;
    let mut new_col = pos.1 as i32;

    for _ in 1..=number_of_times {
        if new_col - 1 < 0 {
            new_col = 8;
            new_row = new_row - 1;
        }
        else {
            new_col -= 1;
        }
    }

    return (new_row as usize, new_col as usize)
}

//Handles logic for progressing forward on the board 
fn move_forwards(pos: (usize, usize), number_of_times: u32) -> (usize, usize) {
    if pos == (8,8){
        return (8,8)
    }

    let mut new_row = pos.0 as i32;
    let mut new_col = pos.1 as i32;

    for _ in 1..=number_of_times {
        if new_col + 1 > 8 {
            new_col = 0;
            new_row = new_row + 1;
        }
        else {
            new_col += 1;
        }
    }

    return (new_row as usize, new_col as usize)
}
