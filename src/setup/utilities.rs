//Displays boards in a readable format for testing
pub fn print_board(board: &Vec<Vec<u32>>) {
    for i in 0..board.len() {
        // Separates boxes horizontally
        if (i != 0) && (i % 3 == 0) {
            println!("- - - - - - - - - - - - -");
        }

        for j in 0..board[0].len() {
            // Separates boxes vertically
            if (j != 0) && (j % 3 == 0) {
                print!(" | ");
            }

            if j == 8 {
                // Prints values for last column 
                println!("{}", board[i][j]);
            } else {
                // Prints all other values w/ a space
                // Lines values up with the horizontal dash lines 
                print!("{} ", board[i][j].to_string());
            }
        }
    }
}

//Checks overall compatability of the location and value with the current state of the board
pub fn valid(board: &Vec<Vec<u32>>, val: u32, (row, col): (u32, u32)) -> bool {
    if column_compatible(board, val, (row, col)) && row_compatible(board, val, (row, col)) && box_compatible(board, val, (row, col)) {return true}
    false
}

//Determines whether the location and value work in that column
pub fn column_compatible(board: &Vec<Vec<u32>>, val: u32, (_row, col): (u32, u32)) -> bool {
    
    //Goes through each row and determines whether there is a conflict at the column index
    for indiv_row in board {
        if indiv_row[col as usize] == val {return false}
    }
    true
}

//Determines whether the location and value work in that row
pub fn row_compatible(board: &Vec<Vec<u32>>, val: u32, (row, _col): (u32, u32)) -> bool {
    let relevant_row = board[row as usize].clone();

    //Goes through each pos in the row to see if the value is already in it
    for indiv_cell in relevant_row {
        if indiv_cell == val {return false}
    }
    true
}

//Determines whether the location and value work in the present quad/box
pub fn box_compatible(board: &Vec<Vec<u32>>, val: u32, (row, col): (u32, u32)) -> bool {
    let quad = determine_quad(row, col);
    let cells_to_check = produce_indexes(quad);

    //Checks each cell in the box to see if the value exists
    for cell in cells_to_check {
        if board[cell.0 as usize][cell.1 as usize] == val {return false}
    }
    true
}

//Takes two indexes and returns which quad they are in
pub fn determine_quad (row: u32, col: u32) -> u32 {
    if row / 3 == 0 {
        if col / 3 == 0 {1} 
        else if col / 3 == 1 {2}
        else {3}
    }
    else if row / 3 == 1 {
        if col / 3 == 0 {4}
        else if col / 3 == 1 {5}
        else {6}
    }
    else {
        if col / 3 == 0 {7}
        else if col / 3 == 1 {8}
        else {9}
    }
}

//Takes a quad and returns the indexing boundaries of it
pub fn determine_quad_coords (quad: u32) -> (u32, u32, u32, u32) {
    match quad {
    //         row  col
        1 => {(0,2, 0,2)},
        2 => {(0,2, 3,5)},
        3 => {(0,2, 6,8)},
        4 => {(3,5, 0,2)},
        5 => {(3,5, 3,5)},
        6 => {(3,5, 6,8)},
        7 => {(6,8, 0,2)},
        8 => {(6,8, 3,5)},
        _ => {(6,8, 6,8)},
    }
}

//Takes a quad and returns the 2D indexes of all of the spots inside of it
pub fn produce_indexes (quad: u32) -> Vec<(u32, u32)> {
    //Getting the bounds to inform the start and end indexes of the ranges 
    let (row_start, row_end, col_start, col_end) = determine_quad_coords(quad);

    let mut indexes: Vec<(u32, u32)> = Vec::new();

    //Iterates only through the row and column indexes that exist in that quad
    for i in row_start..=row_end {
        for j in col_start..=col_end {
            indexes.push((i,j));
        }
    }

    indexes
}

//Determines whether a number is already occupying that spot on the board
pub fn check_spot_occupied (row: u32, col: u32, board_state: &Vec<Vec<u32>>) -> bool {
    if board_state[row as usize][col as usize] != 0 {false}
    else {true}
}

//Checks to make sure each spot on the board has been given a value (ie. no zeros remaining)
pub fn every_spot_full(board: &Vec<Vec<u32>>) -> bool {
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == 0 {
                return false
            }
        }
    }
    return true
}


fn valid_board_helper(board: &Vec<Vec<u32>>, val: u32, pos: (u32, u32)) -> bool {
    // Column - pos[0]; Row - pos[1]
    // Checks if there is a repeat of the value in the position's column
    for i in 0..board.len() {
        // if (board[pos.0 as usize][i] == val) && (pos.1 != i.try_into().unwrap()) {
        if (board[pos.0 as usize][i] == val) && (pos.1 != <usize as TryInto<u32>>::try_into(i).unwrap()) {
            println!("Invalid board");
            println!("repeat in row: {}", i);
            return false;
        }
    }

    // Checks if there is a repeat of the value in the position's row
    for i in 0..board[0].len() {
        // if (board[i][pos.1 as usize] == val) && (pos.0 != i.try_into().unwrap()) {
        if (board[i][pos.1 as usize] == val) && (pos.0 != <usize as TryInto<u32>>::try_into(i).unwrap()) {
            println!("Invalid board");
            println!("repeat in column: {}", i);
            return false;
        }
    }

    // Checks if there is a repeat of the value in the position's box
    // The boxes should either be 1, 2, or 3
    let box_x = pos.0 / 3;
    let box_y = pos.1 / 3;
 
    for i in (box_x * 3)..(box_x * 3 + 3) {
        for j in (box_y * 3)..(box_y * 3 + 3) {
            if (board[i as usize][j as usize] == val) && ((i, j) != pos) {
                println!("Invalid board");
                println!("repeat in a box:{}",  box_x);
                return false;
            }
        }
    }
    return true;
}

pub fn valid_board(board: &Vec<Vec<u32>>) -> bool{
    for a in 0..board.len() {
       for b in 0..board[0].len() {
          if (board[a][b] == 0) || (board[a][b] > 9) {
            return false 
          } else {
            let result = valid_board_helper(&board, board[a][b],(a as u32, b as u32));
            if result == false {
                return false;
            }
          }
       }
    }
    return true;
}

pub fn update_board(board: &mut Vec<Vec<u32>>, val: u32, pos: (usize, usize)) -> &Vec<Vec<u32>> {
    board[pos.0][pos.1] = val;
    
    let result = valid(&board,board[pos.0][pos.1],(pos.0 as u32, pos.1 as u32));
    if result == false{
        println!("Move is not vaild,Try again");
    }
    else {
        println!("Move is vaild");
    }
    return board;
}