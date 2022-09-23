/*
    Author: @cryptnomad
*/
use lazy_static::lazy_static;

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub const BOARD_SIZE: usize = 9;

/*
    Load all states as a static
*/
lazy_static! {
    pub static ref ALL_STATES: HashMap<isize, (bool, isize)> = get_all_states();
}


/*
    Get all possible states of the board
*/
pub fn get_all_states() -> HashMap<isize, (bool, isize)>{
    let mut all_states = HashMap::new();
    let cur_symbol: isize = 1;
    let first_state = State::new();

    all_states.insert(first_state.hash, (first_state.end, first_state.winner));
    get_all_states_impl(&first_state, cur_symbol, &mut all_states);
    all_states
}

/*
    Loop through every position on the board and recursively add
    every possible state
*/
fn get_all_states_impl(
    cur_state: &State, 
    cur_symbol: isize, 
    all_states: &mut HashMap<isize, (bool, isize)>
){
    for (i, symbol) in cur_state.board.iter().enumerate(){
        if *symbol == 0isize{
            let new_state = cur_state.get_next_state(i, cur_symbol);
            if !all_states.contains_key(&new_state.hash){
                all_states.insert(new_state.hash, (new_state.end, new_state.winner));
                if !new_state.end{
                    get_all_states_impl(&new_state, -cur_symbol, all_states)
                }
            }
        }
    }
}

/*
    State struct holds 1 possible configuration of the board
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)] // enable copy semantics, make it printable and comparable
pub struct State{
    board: [isize; BOARD_SIZE], // Board has 1 for X, 0 for none, and -1 for O
    winner: isize,
    hash: isize,
    end: bool,
}

impl State {
    /*
        A new state is initialized with no moves and no winner
    */
    pub fn new() -> Self{
        let mut state = Self{
            board: [0; BOARD_SIZE],
            winner: 0,
            hash: 0,
            end: false,
        };
        state.update();
        state
    }

    /*
        Print the board as X and O
    */
    pub fn display(&self){
        let mut char_arr: [char; BOARD_SIZE] = [' '; BOARD_SIZE];
        for i in 0..BOARD_SIZE{
            char_arr[i] = match self.board[i]{
                1 => 'X',
                0 => ' ',
                -1 => 'O',
                _ => ' ',
            };
        }
        println!("{} | {} | {}\n---------\n{} | {} | {}\n---------\n{} | {} | {}\n", 
            char_arr[0], 
            char_arr[1], 
            char_arr[2], 
            char_arr[3], 
            char_arr[4], 
            char_arr[5], 
            char_arr[6], 
            char_arr[7], 
            char_arr[8]
        );
    }
    
    /*
        Public access to hash
    */
    pub fn get_hash(&self) -> isize{
        self.hash
    }

    /*
        Public access to a specific tile on the board
    */
    pub fn get_tile(&self, i: usize) -> isize{
        self.board[i]
    }

     /*
        Public access to the winner of the game
    */
    pub fn get_winner(&self) -> isize{
        self.winner
    }

    /*
        Whether the game is over or not
    */
    pub fn is_end(&self) -> bool{
        self.end
    }

    /*
        Return the next state, after a specific move is made
    */
    pub fn get_next_state(&self, i: usize, symbol: isize) -> Self {
        let mut new_state = self.clone();
        new_state.board[i] = symbol;
        new_state.update();
        new_state
    }

    /*
        A simple hash function that will be unique for each 
    */
    fn compute_hash(&mut self){
        let mut hash: isize = 0;
        for (_, val) in self.board.iter().enumerate(){
            hash = hash * 3 + (*val) + 1;
        }
        self.hash = hash;
    }

    /*
        Check whether the game is over, and who has won
    */
    fn check_end(&mut self){
        let mut score: isize;
        // Check verticals and horizontals
        for i in 0..3 {
            //vertical
            score = self.board[i] + self.board[i+3] + self.board[i+6];
            if score == -3{
                self.winner = -1;
                self.end = true;
                return
            }else if score == 3{
                self.winner = 1;
                self.end = true;
                return;
            }

            // horizontal
            score = self.board[i*3]  + self.board[(i*3)+1]  + self.board[(i*3)+2];
            if score == -3{
                self.winner = -1;
                self.end = true;
                return
            }else if score == 3{
                self.winner = 1;
                self.end = true;
                return
            }
        }

        // Check diagonals
        score = self.board[0] + self.board[4] + self.board[8];
        if score == -3{
            self.winner = -1;
            self.end = true;
            return
        }else if score == 3{
            self.winner = 1;
            self.end = true;
            return
        }

        score = self.board[2] + self.board[4] + self.board[6];

        if score == -3{
            self.winner = -1;
            self.end = true;
            return
        }else if score == 3{
            self.winner = 1;
            self.end = true;
            return
        }

        //check a tie
        score = self.board.iter().map(|&x| x.abs()).sum();

        if score == BOARD_SIZE as isize {
            self.winner = 0;
            self.end = true;
            return
        }

        self.end = false;
        
    }

    /*
        Update the hash and the end members
            - this is called when a new state is made or a move is made
    */
    fn update(&mut self){
        self.compute_hash();
        self.check_end();
    }

}