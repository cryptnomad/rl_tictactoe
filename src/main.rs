/*
    Author: @cryptnomad
*/
pub mod ai_game;
pub mod ai_player;
pub mod human_game;
pub mod state;

use crate::state::TileState;
use std::io;
use std::time::{Duration, Instant};

fn main() {
    println!("Let's teach some ai tic tac toe.");
    let start = Instant::now();
    ai_game::train(100000, 10000);
    let duration: Duration = start.elapsed();
    println!("Training took {:?}", duration);

    // Should be a draw every time if sufficient training has occured
    ai_game::compete(500);

    let mut input_str = String::new();
    println!("\n\nDo you want to play against the models? (y/n)");

    io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read the line");

    if input_str.trim() == "Y" || input_str.trim() == "y" {
        loop {
            let mut symbol_select = String::new();
            while symbol_select.trim() != "X" && symbol_select.trim() != "O" {
                symbol_select = "".to_string();
                println!("Would you like to play X (first) or O (second)? (X/O)");
                io::stdin()
                    .read_line(&mut symbol_select)
                    .expect("Failed to read the line");
            }

            let user_symbol: TileState = match symbol_select.trim() {
                "X" => TileState::X,
                "O" => TileState::O,
                _ => TileState::Empty,
            };
            let mut game = human_game::HumanGame::new(user_symbol);
            let winner = game.play();

            if winner == user_symbol {
                println!("You win!");
            } else if winner == TileState::Empty {
                println!("Draw!");
            } else {
                println!("You lose! ")
            }
        }
    }

    println!("\nGoodbye!");
}
