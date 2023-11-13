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
    while turn_complete == false {
        // Get input from player.
        print!("Player {}, make your move.", game.player_id);
        let mut input = &mut String::new();
        // Read input into input buffer. TODO: Sanitise input here.
        stdin().read_line(&mut input).unwrap();
        // Parse input into coordinates. TODO: Make this better.
        // X coord.
        let x_coordinate = letter_to_number(input.chars().nth(0).unwrap()).unwrap();
        // Y coord.
        let y_coordinate = input.chars().nth(1).unwrap().to_digit(10).unwrap() as u8;
        // Place tile on board.
        // Check tile is not occupied already, if it is, log it and restart the turn.
        if game.board[y_coordinate as usize][x_coordinate as usize] == 0 {
            game.board[y_coordinate as usize][x_coordinate as usize] = game.player_id;
            turn_complete = true;
        } else {
            println!("That space is already occupied!");
        }
    }
    debug_info(game);
    // Send win signal if win condition is met. TODO: Check for winning patterns.
}

fn letter_to_number(letter: char) -> Option<u8> {
    match letter {
        'a'..='z' => Some(letter as u8 - 97),
        'A'..='Z' => Some(letter as u8 - 65),
        _ => None,
    }
}
// Minmax algorithm subsection of the game section
/*

*/

fn minmax(depth: usize) {}

// UI section
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};
use gtk4 as gtk;

fn gui() -> glib::ExitCode {
    let app = Application::builder().application_id("Gomoku").build();
    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Gomoku")
            .default_width(400)
            .default_height(400)
            .build();

        let button = Button::with_label("Click me!");
        button.connect_clicked(|_| {
            eprintln!("Clicked!");
        });
        window.set_child(Some(&button));

        window.present();
    });

    app.run()
}
