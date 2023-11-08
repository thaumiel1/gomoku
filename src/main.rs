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
    gui();
    loop {
        advance_turn(&mut game);
    }
}

use std::io::stdin;

fn advance_turn(game: &mut Game) {
    // Advance turn counter
    game.turn_number += 1;
    // Advance player id, if 0, initialise as player 1. If 1, go to player 2. If 2, go to player 1.
    // TODO: Find a better way to do this.
    if game.player_id == 0 {
        game.player_id = 1;
    } else if game.player_id == 1 {
        game.player_id = 2;
    } else if game.player_id == 2 {
        game.player_id = 1;
    }
    // Get input from player.
    print!("Player {}, make your move.", game.player_id);
    let mut input = &mut String::new();
    // Read input into input buffer.
    stdin().read_line(&mut input).unwrap();

    // Send win signal if win condition is met.

}

// Minmax algorithm subsection of the game section
/*

*/

fn minmax(depth: usize) {}


// UI section
use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};


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

