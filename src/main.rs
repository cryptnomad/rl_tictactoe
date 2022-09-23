/*
    Author: @cryptnomad
*/
pub mod state;
pub mod ai_player;
pub mod ai_game;
pub mod human_game;

use std::io;

fn main() {
    println!("Let's teach some ai tic tac toe.");
    ai_game::train(1000000, 100000);
    ai_game::compete(500);

    let mut input_str = String::new();
    println!("\n\nDo you want to play against the models? (y/n)");

    io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read the line");

    if input_str.trim() == "Y" || input_str.trim() == "y"{
        loop{
            let mut symbol_select = String::new();
            while symbol_select.trim() != "X" && symbol_select.trim() != "O" {
                symbol_select = "".to_string();
                println!("Would you like to play X (first) or O (second)? (X/O)");
                io::stdin()
                    .read_line(&mut symbol_select)
                    .expect("Failed to read the line");
            }

            let user_symbol: isize = match symbol_select.trim(){
                "X" => 1,
                "O" => -1,
                _ => 1
            };
            let mut game = human_game::HumanGame::new(user_symbol);
            let winner = game.play();

            if winner == user_symbol{
                println!("You win!");
            }else if winner == 0{
                println!("Draw!");
            }else{
                println!("You lose! ")
            }
    
        }
    }

    println!("\nGoodbye!");

}
