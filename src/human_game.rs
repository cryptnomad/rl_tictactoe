use std::io;

use crate::ai_player::AiPlayer;
use crate::state::{State, TileState, opposite_tile_state};

#[derive(Debug)]
pub struct HumanGame {
    current_state: State,
    current_symbol: TileState,
    ai_player: AiPlayer,
    human_symbol: TileState,
}

impl HumanGame {
    pub fn new(human_symbol: TileState) -> Self {
        let ai_player = AiPlayer::competitor(opposite_tile_state(human_symbol));
        Self {
            current_state: State::new(),
            current_symbol: TileState::X,
            ai_player: ai_player,
            human_symbol: human_symbol,
        }
    }

    pub fn play(&mut self) -> TileState {
        let mut input_str = String::new();
        self.ai_player.set_state(&self.current_state);
        println!("\nYou will use the number keys to select your move.\n");
        println!("The board is set up like so:");
        println!("0 | 1 | 2\n---------\n3 | 4 | 5\n---------\n6 | 7 | 8\n",);
        println!("Press enter when ready\n");
        io::stdin()
            .read_line(&mut input_str)
            .expect("Failed to read input");
        input_str.clear();

        while !self.current_state.is_end() {
            self.current_state.display();
            let index = if self.current_symbol == self.human_symbol {
                self.get_move()
            } else {
                self.ai_player.act()
            };
            self.current_state = self
                .current_state
                .get_next_state(index, self.current_symbol);
            self.ai_player.set_state(&self.current_state);
            self.alternate();
        }
        self.current_state.display();
        return self.current_state.get_winner();
    }

    fn get_move(&self) -> usize {
        let mut move_str: String;
        let mut move_index: usize = 9;
        while !self.move_is_valid(move_index) {
            move_str = "".to_string();
            io::stdin()
                .read_line(&mut move_str)
                .expect("Failed to read the line");

            move_index = move_str.trim().parse().expect("Please type a number");
        }
        move_index
    }

    fn move_is_valid(&self, index: usize) -> bool {
        if index > 8 {
            return false;
        }
        self.current_state.get_tile(index) == TileState::Empty
    }

    fn alternate(&mut self) {
        self.current_symbol = opposite_tile_state(self.current_symbol);
    }
}
