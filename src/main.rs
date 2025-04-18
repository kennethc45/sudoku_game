mod setup;
mod tests;
mod html;

use axum::error_handling::HandleErrorLayer;

use axum::http::{Method, StatusCode, Uri};
use setup::board_generation::generate_solvable_clues;
use setup::utilities::{valid_board, valid, print_board};
use html::front_end::{new_board, solution_board, start_page};

use std::cmp::Ordering;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

use axum::response::{IntoResponse, Html};
use axum::{BoxError, Json};
use axum::{routing::{get, post}, Router, extract::{State, Path}};

use minijinja::render;

use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

use tower::ServiceBuilder;

use rand::Rng;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Level {
    Easy,
    Medium,
    Hard
}

#[derive(Clone)]
struct AppState {
    play_boards: Vec<Vec<Vec<u32>>>,
    play_solutions: Vec<Vec<Vec<u32>>>,
    current_board: Arc<Mutex<usize>>,
    difficulty: Arc<Mutex<Level>>,
    board_progress: Arc<Mutex<Vec<Vec<u32>>>>
}

#[tokio::main]
async fn main() {
    //Creating the boards and updating the state to hold them and the current board the user is on
    let mut boards: Vec<Vec<Vec<u32>>> = Vec::new();
    let mut solutions: Vec<Vec<Vec<u32>>> = Vec::new();

    for _ in 0..100 {
        let generated_board = generate_solvable_clues();
        boards.push(generated_board.0);
        solutions.push(generated_board.1);
    }

    let state = AppState {
        play_boards: boards.clone(),
        play_solutions: solutions,
        current_board: Arc::new(Mutex::new(0)),
        difficulty: Arc::new(Mutex::new(Level::Hard)),
        board_progress: Arc::new(Mutex::new(boards.get(0).unwrap().to_vec()))
    };

    let address_num = "127.0.0.1:3000";

    let server_address = std::env::var("SERVER_ADRESS")
        .unwrap_or(address_num.to_owned());

    let listener = TcpListener::bind(server_address)
        .await
        .expect("Could not create TCP Listener");

    println!("Listening on {}", listener.local_addr().unwrap());

    let mid = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(handle_timeout))
        .timeout(Duration::from_secs(30));
        

    // Defined endpoints 
    let app = Router::new()
        .route("/", get(handle_start))
        .route("/new_game/:level", get(handle_new_board))
        .route("/get_hint/:x/:y", get(handle_hint))
        .route("/spot_check", post(spot_check))
        .route("/win_check", post(win_check))
        .route("/solution", get(return_solution))
        .layer(mid.into_inner())
        .with_state(state);

    // Launches the local server
    axum::serve(listener, app)
        .await
        .expect("Error serving application")
}

async fn handle_timeout(_method: Method, _uri: Uri, err: BoxError) -> (StatusCode, String) {
    if err.is::<tower::timeout::error::Elapsed>() {
        println!("timemout");
        (StatusCode::REQUEST_TIMEOUT, "Timeout".to_string())
    }
    else {
        println!("Error Type: {}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Error Type: {}", err))
    }
}


#[derive(Debug, Serialize, Deserialize)]
struct Coordinates {
    x: u32,
    y: u32
}

#[derive(Debug, Serialize, Deserialize)]
struct SudokuBoard {
    board: Vec<Vec<u32>>
}

#[derive(Debug, Serialize, Deserialize)]
struct Input {
    coordinates: Coordinates,
    value: u32,
    board: SudokuBoard
}

// Reads the users input and checks if it is valid within board
async fn spot_check(data: axum::extract::Json<Input>) -> impl IntoResponse{
    let Input {
        coordinates: Coordinates {x, y}, 
        value,
        board: SudokuBoard { board }
    } = data.0;
    
    let is_valid:bool = valid(&board, value, (x, y));

    Json(is_valid)
}

// Checks if the board is a valid solution.
async fn win_check(sudoku_board: axum::extract::Json<SudokuBoard>) -> impl IntoResponse {

    let SudokuBoard { board } = sudoku_board.0;

    // If you change one of the values in a valid board to zero it counts as true
    let is_win:bool = valid_board(&board);

    Json(is_win)
}

//Displays to the user a html page with a new board
async fn handle_new_board(Path(level): Path<u32>, State(state): State<AppState>) -> impl IntoResponse {
    //Handles updating the difficulty based on the number level passed in the URL
    let mut difficulty:MutexGuard<Level> = state.difficulty.lock().expect("Modifying difficulty.");
    
    *difficulty = match level {
        1 => Level::Easy,
        2 => Level::Medium,
        _ => Level::Hard
    };

    let mut current_board_index:MutexGuard<usize> = state.current_board.lock().expect("Modifying current board index.");

    //Handles incrementing to the next board and wrapping around when the user goes through all of them
    if current_board_index.cmp(&99) == Ordering::Equal{
        *current_board_index = 0;
    }
    else {
        *current_board_index = *current_board_index + 1;
    }

    //Looking up the current board and solution
    let mut current_board: Vec<Vec<u32>> = state.play_boards.get(*current_board_index).unwrap().to_vec();
    let current_solution: Vec<Vec<u32>> = state.play_solutions.get(*current_board_index).unwrap().to_vec();

    //Determining number of clues to add based on chosen level
    let num_additional_clues = match level {
        1 => 27,
        2 => 18,
        _ => 0
    };

    let mut valid_placement = false;
        
    //Iterating through and picking random unfilled spots from the solution to fill in on the board
    for _ in 0..num_additional_clues {
        valid_placement = false;
        while !valid_placement {
            let row = rand::thread_rng().gen_range(0..9);
            let col = rand::thread_rng().gen_range(0..9);
    
            if current_board[row][col] == 0 {
                current_board[row][col] = current_solution[row][col];
                valid_placement = true;
            }
        }
    }

    //Updating the state to reflect the board's new difficulty
    let mut board_progress:MutexGuard<Vec<Vec<u32>>> = state.board_progress.lock().expect("Modifying board progress");
    
    *board_progress = current_board.clone();

    //Rendering the page with the board
    Html(render!(new_board(), board => current_board, difficulty => level))
}

//Displays the basic html home page
async fn handle_start() -> impl IntoResponse {
    Html(render!(start_page()))
}

//Takes a row and column and returns to the user the answer for that spot
async fn handle_hint(Path((row, col)): Path<(u32, u32)>, State(state): State<AppState>) -> Json<u32> {
    //Does another check to see if the values are within the board bounds
    if row > 9 || row < 1 || col > 9 || col < 1 {
        Json(0)
    }
    else {
        //Looks up the current board index, board, and solution
        let board_index = state.current_board.lock().expect("Accessing current board index.");
        let solution = state.play_solutions.get(*board_index).unwrap().to_vec();

        //Returns the value at the requested spot from the solution
        Json(solution[(row-1) as usize][(col-1) as usize])
    }
}

async fn return_solution(State(state): State<AppState>) -> impl IntoResponse {
    let current_board_index = *state.current_board.lock().expect("Accessing current board index.");

    // Get the current board from the play_boards vector
    let solution = state.play_solutions[current_board_index].clone();

    print_board(&state.play_boards[current_board_index].clone());
    print_board(&solution);

    // Render the solution HTML
    Html(render!(solution_board(), board => solution))
}
