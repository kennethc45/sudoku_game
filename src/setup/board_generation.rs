use rand::Rng;
use crate::setup::utilities::{valid, box_compatible, column_compatible, determine_quad, produce_indexes, check_spot_occupied, every_spot_full, valid_board, print_board};
use crate::setup::solvability_check::generate_solve_board;

//Starts the search for the clues
fn generate_eighteen_clues() -> Vec<Vec<u32>> {
    let mut clues: Vec<((usize, usize), u32)> = Vec::new();
    let numbers_to_place: Vec<u32> = 
        vec![
            1,2,3,4,5,6,7,8,9,
            1,2,3,4,5,6,7,8,9,
        ];

    let final_board: Vec<Vec<u32>> = clues_recursive_helper(1, &mut clues, numbers_to_place, 18);

    final_board
}

pub fn generate_solvable_clues() -> (Vec<Vec<u32>>, Vec<Vec<u32>>)  {
    let mut board_found = false;
    let mut clues_for_display: Vec<Vec<u32>> = Vec::new();
    let mut solved_board: &Vec<Vec<u32>>  = &Vec::new();
    let mut generated_clues: Vec<Vec<u32>> = Vec::new();

    //Goes until it a valid & solvable board has been produced
    while !board_found {
        //Generates the hints
        generated_clues = generate_eighteen_clues();
        //Keeps a copy for displaying before handing it off to the solveable function
        clues_for_display = generated_clues.clone();
        //Attempts to solve the board based on the clues
        solved_board = generate_solve_board(&mut generated_clues);
        //If it gives back a fully filled and valid board its done
        if every_spot_full(solved_board) && valid_board(solved_board) {
            board_found = true;
        }
    }
    (clues_for_display, solved_board.clone())
}

fn clues_recursive_helper (quad: u32, clues: &mut Vec<((usize, usize), u32)>, mut remaining_nums: Vec<u32>, num_clues: usize) -> Vec<Vec<u32>> {
    //Recursion stops once there are enough clues generated
    if clues.len() < num_clues {
        //Creation of board and picks for random positions and random values
        let current_board = create_board(&clues);
        let cells_in_quad = produce_indexes(quad);
        let (row_pos, col_pos) = pick_index(&cells_in_quad, &current_board);
        let index = pick_number(&remaining_nums);
        let number_choice = remaining_nums[index];

        //Determining whether the location and value work
        let ((proposed_row, proposed_col), same_value) = fill_spot(&current_board, (row_pos, col_pos), number_choice);

        //Number choice does not work in that quad/box
        if ((proposed_row, proposed_col), same_value) == ((100,100),100) {

            //Determines backtracking versus retrying
            if backtracking_needed_check(&remaining_nums, &current_board, quad) {
                //Reducing the quad/box by one, remove the most recent clue, and put its value back in remainings nums
                let ((_popped_row, _popped_col), popped_val) = clues.pop().unwrap();
                remaining_nums.push(popped_val);

                //Handles moving to the previous quad/box
                if quad == 1 {clues_recursive_helper(9, clues, remaining_nums, num_clues);}
                else {clues_recursive_helper(quad - 1, clues, remaining_nums, num_clues);}
            }
            else {
                //Retrying but staying in the current quad because backtracking is not neccessary yet
                clues_recursive_helper(quad, clues, remaining_nums, num_clues);
            }
        }
        //Number choice works in that quad/box
        else {
            //Adding the location and value to the list of clues and removing the value from the list of options
            clues.push(((proposed_row, proposed_col), same_value));
            remaining_nums.remove(index);
            
            //Handles moving to the next quad/box
            if quad == 9 {clues_recursive_helper(1, clues, remaining_nums, num_clues);}
            else {clues_recursive_helper(quad + 1, clues, remaining_nums, num_clues);}
        }
    }

    create_board(&clues)
}

//Takes a list of clues and returns them represented in board format
fn create_board (clues: &Vec<((usize, usize), u32)>) -> Vec<Vec<u32>> {
    let mut board: Vec<Vec<u32>> = vec![
        vec![ 0,0,0,  0,0,0,  0,0,0],
        vec![ 0,0,0,  0,0,0,  0,0,0],
        vec![ 0,0,0,  0,0,0,  0,0,0],

        vec![ 0,0,0,  0,0,0,  0,0,0],
        vec![ 0,0,0,  0,0,0,  0,0,0],
        vec![ 0,0,0,  0,0,0,  0,0,0],

        vec![ 0,0,0,  0,0,0,  0,0,0],
        vec![ 0,0,0,  0,0,0,  0,0,0],
        vec![ 0,0,0,  0,0,0,  0,0,0]
    ];

    for i in clues {
        board[i.0.0][i.0.1] = i.1;
    }

    board
}

//
fn fill_spot (board: &Vec<Vec<u32>>, loc_row_col: (u32, u32), value: u32) -> ((usize, usize), u32) {
    //Check whether that number can go there and if it can, remove it from the list
    let validity = valid(&board, value, (loc_row_col.0, loc_row_col.1));
    if validity {
        ((loc_row_col.0 as usize, loc_row_col.1 as usize), value)
    }
    else {
        let can_place_in_box = box_compatible(&board, value, (loc_row_col.0, loc_row_col.1));
        
        //Number is compatible in this quadrant but maybe not that row or column
        if can_place_in_box {
            //Checking whether the row or the column is the problem
            let can_place_in_col = column_compatible(&board, value, (loc_row_col.0, loc_row_col.1));
            let alternative_spots = produce_alt_index_options(loc_row_col.0, loc_row_col.1, can_place_in_col, &board);

            for i in alternative_spots {
                let is_viable = valid(&board, value, (i.0, i.1));
                if is_viable {return ((i.0 as usize, i.1 as usize), value)}
            }
        }
        return ((100,100),100)
    }
}

//Called when a value works in a quad/box but the original coords didn't because there was a row or column conflict
fn produce_alt_index_options(row: u32, col: u32, is_col_conflict: bool, board_state: &Vec<Vec<u32>>) -> Vec<(u32, u32)> {
    let quad = determine_quad(row, col);
    let indexes = produce_indexes(quad);
    let mut compatible_indexes = Vec::new();

    for i in indexes {
        //Produces coords that aren't in that column and are not occupied
        if is_col_conflict && i.1 != col && !check_spot_occupied(row, col, &board_state) {compatible_indexes.push((i.0,i.1));}
        //Produces coords that aren't in that row and are not occupied
        if !is_col_conflict && i.0 != row && !check_spot_occupied(row, col, &board_state) {compatible_indexes.push((i.0,i.1));}
    }
    compatible_indexes
}


//Picks a random *index* from the list provided and returns it
fn pick_number (number_collection: &Vec<u32>) -> usize {
    if number_collection.is_empty() {
        100
    }
    else {
        //Generating random index based on current length of vector
        let number_of_options = number_collection.len();
        let index = rand::thread_rng().gen_range(0..number_of_options);
        index
    }
}

//Takes a list of indexes in the current quad and returns one set of coords that could work
fn pick_index (index_collection: &Vec<(u32, u32)>, board: &Vec<Vec<u32>>) -> (u32, u32) {
    //Picks coords based on random index
    let number_of_indexes = index_collection.len();
    let index = rand::thread_rng().gen_range(0..number_of_indexes);
    let (rand_index_choice_row, rand_index_choice_col) = index_collection[index];

    //Handles error due to empty
    if index_collection.is_empty() {
        (100,100)
    }
    //Handles if the spot choosen is already occupied
    else if board[rand_index_choice_row as usize][rand_index_choice_col as usize] != 0 {
        let mut modified_collection = index_collection.clone();
        modified_collection.remove(index);
        pick_index(&modified_collection, &board)
    }
    else {
        (rand_index_choice_row, rand_index_choice_col)
    }
}

//Determines based on the values left to place whether backtracking should occur
fn backtracking_needed_check (remaining_nums: &Vec<u32>, board: &Vec<Vec<u32>>, quad: u32) -> bool {
    let quad_indexes = produce_indexes(quad);
    let mut valid_nums:Vec<u32> = Vec::new();

    //Goes through the nums
    for i in remaining_nums {
        //Goes through all of the spots it can be placed
        for j in &quad_indexes {
            //Determines whether the number is valid in that spot
            if valid(board, *i, (j.0, j.1)) {
                //Only records numbers that are not duplicates
                if !valid_nums.contains(i) {
                    valid_nums.push(*i);
                }
            }
        }
    }

    //Makes the decision based on how many unique and valid numbers are remaining in that quad
    if valid_nums.len() < 2 {true} //The 2 is arbitrary can be changed to a different value
    else {false}
}