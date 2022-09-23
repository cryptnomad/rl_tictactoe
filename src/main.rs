/*
    Author: @cryptnomad
*/
pub mod state;
pub mod ai_player;
pub mod ai_game;

use std::io;
use std::{thread, time::Duration};

fn main() {
    ai_game::train(1000, 100);
    ai_game::compete(500);

    let mut input_str = String::new();
    println!("\n\nDo you want to play against the models? (y/n)");

    io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read the line");

    if input_str.trim() == "Y" || input_str.trim() == "y"{
        loop{
            println!("\nYou will use the number keys to select your move.\n");
            println!("The board is set up like so:");
            println!("1 | 2 | 3\n---------\n4 | 5 | 6\n---------\n7 | 8 | 9\n",);
            thread::sleep(Duration::from_millis(3000));

            let mut symbol_select = String::new();
            while symbol_select.trim() != "X" && symbol_select.trim() != "O" {
                println!("Would you like to play X (first) or O (second)? (X/O)");
                io::stdin()
                    .read_line(&mut symbol_select)
                    .expect("Failed to read the line");
            }

            if input_str == "1"{

            }
    
        }
    }

    println!("Goodbye");

}
