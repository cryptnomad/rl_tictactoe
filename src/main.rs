
use rl_tictactoe::{TileState, GameState, display_board, update_state, move_is_valid};
use std::io;

fn main() {
    let mut board: [TileState; 9] = [TileState::Empty; 9];
    let mut game_state: GameState = GameState::P1Turn;
    
    let mut move_index: usize;

    while game_state == GameState::P1Turn || game_state == GameState::P2Turn{
        display_board(&board);

        println!("{}", game_state);
        println!("Input the index of the next move");

        let mut move_str  = String::new();

        io::stdin()
            .read_line(&mut move_str)
            .expect("Failed to read the line");

        move_index = match move_str.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        while !move_is_valid(&board, move_index){
            println!("Invalid move. Valid moves are on empty tiles between 0 and 8");
            let mut move_str  = String::new();

            io::stdin()
                .read_line(&mut move_str)
                .expect("Failed to read the line");
    
            move_index = move_str.trim().parse().expect("Please type a number");
        }

        if game_state == GameState::P1Turn{
            board[move_index] = TileState::X;
        }else{
            board[move_index] = TileState::O;
        }

        update_state(&board, &mut game_state);
    }
    display_board(&board);
    println!("{}", game_state);
}
