/*
    Author: @cryptnomad
*/
pub mod state;
pub mod ai_player;
pub mod ai_game;

fn main() {
    ai_game::train(1000, 100);
    ai_game::compete(500);

}
