// Main gomoku program.

// Util section

fn debug_info(game: &Game) {
    println!("Current player: {}", game.player_id);
    println!("Turn number: {}\n", game.turn_number);
    for row in &game.board {
        println!();
        for cell in row {
            print!("{}", cell);
        }
    }
}

// Game section

/*
The board part of this game struct is a list of lists containing u8 numbers, essentially creating a grid like the one used for Gomoku.
The plan on the encoding of data in these entries is:
- 0: unoccupied by either player's tile.
- 1: occupied by player one's tile.
- 2: occupied by player two's tile.
This approach should minimise board memory size and simplify interacting with it as no external structs or enums will be used.

Doing this in a struct of this low complexity will allow the game to be easily serialised and deserialised, which will be useful for saving and loading games.

The coordinate system will work as having a..o inclusive on the x axis and 1 to 15 inclusive on the y axis. This is similar to chess and will allow for easy communication of moves.

The player_id part of the struct will act as a flag so that the simulation knows which player just had a turn when the game advances.
Follows the same naming convention as the u8 for the board does.
*/

pub struct Game {
    board: Vec<Vec<u8>>,
    player_id: u8,
    turn_number: usize,
}

fn new_game() -> Game {
    let board: Vec<Vec<u8>> = vec![vec![0; 15]; 15];
    Game {
        board,
        player_id: 0,
        turn_number: 0,
    }
}

fn main() {
    let mut game: Game = new_game();
    debug_info(&game);
    loop {
        advance_turn_2p(&mut game);
    }
}

use std::io::stdin;
use regex_lite::Regex;

fn advance_turn_2p(game: &mut Game) {
    // Advance turn counter
    game.turn_number += 1;
    // Advance player id, if 0, initialise as player 1. If 1, go to player 2. If 2, go to player 1.
    match game.player_id {
        0 => game.player_id = 1,
        1 => game.player_id = 2,
        2 => game.player_id = 1,
        _ => ()
    }

    let mut turn_complete = false;
    while !turn_complete {
        // Get input from player. TODO: Review this and make it more ergonomic for user.
        // Parse input into coordinates. TODO: Make this better.
        if game.player_id == 1 {
            println!("Player 1's turn.");
        } else {
            println!("Player 2's turn.");
        }
        println!("Enter coordinates in the format 'a1' to place a tile.");
        let input_verify = Regex::new(r"^[a-zA-Z][0-9]{1,2}$").unwrap();

        let mut input = take_user_input();
        let input = input.trim().to_string();

        // This splits the input into two halves of a tuple at the index of 1.
        let (x_coordinate, y_coordinate) = input.split_at(1);
        // TODO: Sanitise the inputs here with regex.
        // This is some necessary processing to turn the x coordinate into a grid index number.
        // Also turns y from a numeric character to an int number to be used to index the board.
        let x_coordinate = letter_to_number(x_coordinate.to_string()).expect("Failed to unwrap x coordinate.");
        let y_coordinate = y_coordinate.parse::<u8>().expect("Failed to unwrap y coordinate.")-1;

        let coord = vec![x_coordinate, y_coordinate];
        // Place tile on board.
        // Check tile is not occupied already, if it is, log it and restart the turn.
        if game.board[coord[1] as usize][coord[0] as usize] == 0 {
            game.board[coord[1] as usize][coord[0] as usize] = game.player_id;
            turn_complete = true;
        } else {
            println!("That space is already occupied!");
        }
    }
    debug_info(game);
    // Send win signal if win condition is met. TODO: Check for winning patterns.
}

fn take_user_input() -> String {
    let mut buffer = "".to_string();
    stdin().read_line(&mut buffer).unwrap();
    buffer
}

fn letter_to_number(letter: String) -> Option<u8> {
    let letter = letter.chars().nth(0).unwrap();
    match letter {
        'a'..='z' => Some(letter as u8 - 97),
        _ => None,
    }
}
// Minmax algorithm subsection of the game section
/*
The minmax (also known as minimax) algorithm is an AI algorithm that evaluates future moves for advantages and makes decisions on what to do from that analysis.
*/

fn minmax(depth: usize) {}
