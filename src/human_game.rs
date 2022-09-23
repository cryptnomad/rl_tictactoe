use std::io;
use std::ptr;

use crate::state::State;
use crate::ai_player::AiPlayer;

#[derive(Debug)]
pub struct HumanGame {
    current_state: State,
    current_symbol: isize,
    ai_player: AiPlayer,
    human_symbol: isize,
}

impl HumanGame {
    pub fn new(human_symbol: isize) -> Self{
        let ai_player = AiPlayer::competitor(human_symbol*-1);
        Self{
            current_state: State::new(),
            current_symbol: 1,
            ai_player: ai_player,
            human_symbol: human_symbol
        }
    }

    pub fn play(&mut self) -> isize{
        let mut input_str = String::new();
        self.ai_player.set_state(&self.current_state);
        println!("\nYou will use the number keys to select your move.\n");
        println!("The board is set up like so:");
        println!("0 | 1 | 2\n---------\n3 | 4 | 5\n---------\n6 | 7 | 8\n",);
        println!("Press enter when ready\n");
        io::stdin().read_line(ptr::null_mut());
        
        while !self.current_state.is_end(){
            self.current_state.display();
            let index = if self.current_symbol == self.human_symbol{
                self.get_move()
            }else{
                self.ai_player.act()
            };
            self.current_state = self.current_state
                .get_next_state(index, self.current_symbol);
            self.ai_player.set_state(&self.current_state);
            self.alternate();
        }
        self.current_state.display();
        return self.current_state.get_winner();
    }

    fn get_move(&self) -> usize{
        let mut move_str: String;
        let mut move_index: usize = 9;
        while !self.move_is_valid(move_index){
            move_str = "".to_string();
            io::stdin()
                .read_line(&mut move_str)
                .expect("Failed to read the line");

            move_index = move_str.trim().parse().expect("Please type a number");
        }
        move_index
    }

    fn move_is_valid(&self, index: usize) -> bool{
        if index > 8{
            return false;
        }
        self.current_state.get_tile(index) == 0
    }

    fn alternate(&mut self){
        self.current_symbol = self.current_symbol*-1;
    }
}