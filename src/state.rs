use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

pub const BOARD_SIZE: usize = 9;

lazy_static! {
    pub static ref ALL_STATES: HashMap<i32, (bool, TileState)> = get_all_states();
}

///  Get all possible states of the board
pub fn get_all_states() -> HashMap<i32, (bool, TileState)> {
    let mut all_states = HashMap::new();
    let cur_symbol: TileState = TileState::X;
    let first_state = State::new();

    all_states.insert(first_state.hash, (first_state.end, first_state.winner));
    get_all_states_impl(&first_state, cur_symbol, &mut all_states);
    all_states
}

///Loop through every position on the board and recursively add
/// every possible state
fn get_all_states_impl(
    cur_state: &State,
    cur_symbol: TileState,
    all_states: &mut HashMap<i32, (bool, TileState)>,
) {
    for (i, symbol) in cur_state.board.iter().enumerate() {
        if *symbol == TileState::Empty {
            let new_state = cur_state.get_next_state(i, cur_symbol);
            if let std::collections::hash_map::Entry::Vacant(e) = all_states.entry(new_state.hash) {
                e.insert((new_state.end, new_state.winner));
                if !new_state.end {
                    get_all_states_impl(&new_state, opposite_tile_state(cur_symbol), all_states)
                }
            }
        }
    }
}

/// Holds the state of 1 tile on the board
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TileState {
    O = -1,
    Empty,
    X,
}

pub fn opposite_tile_state(symbol: TileState) -> TileState {
    match symbol {
        TileState::O => TileState::X,
        TileState::Empty => TileState::Empty,
        TileState::X => TileState::O,
    }
}

impl fmt::Display for TileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TileState::X => write!(f, "X"),
            TileState::O => write!(f, "O"),
            TileState::Empty => write!(f, " "),
        }
    }
}

/// State struct holds 1 possible configuration of the board
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    board: [TileState; BOARD_SIZE],
    winner: TileState,
    hash: i32,
    end: bool,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> Self {
        let mut state = Self {
            board: [TileState::Empty; BOARD_SIZE],
            winner: TileState::Empty,
            hash: 0,
            end: false,
        };
        state.update();
        state
    }

    pub fn display(&self) {
        println!(
            "{} | {} | {}\n---------\n{} | {} | {}\n---------\n{} | {} | {}\n",
            self.board[0],
            self.board[1],
            self.board[2],
            self.board[3],
            self.board[4],
            self.board[5],
            self.board[6],
            self.board[7],
            self.board[8]
        );
    }

    pub fn get_hash(&self) -> i32 {
        self.hash
    }

    pub fn get_tile(&self, i: usize) -> TileState {
        self.board[i]
    }

    pub fn get_winner(&self) -> TileState {
        self.winner
    }

    pub fn is_end(&self) -> bool {
        self.end
    }

    pub fn get_next_state(&self, i: usize, symbol: TileState) -> Self {
        let mut new_state = *self;
        new_state.board[i] = symbol;
        new_state.update();
        new_state
    }

    fn compute_hash(&mut self) {
        let mut hash: i32 = 0;
        for (_, val) in self.board.iter().enumerate() {
            hash = hash * 3 + (*val as i32) + 1;
        }
        self.hash = hash;
    }

    fn check_end(&mut self) {
        let mut score: i32;
        // Check verticals and horizontals
        for i in 0..3 {
            //vertical
            score = self.board[i] as i32 + self.board[i + 3] as i32 + self.board[i + 6] as i32;
            if score == -3 {
                self.winner = TileState::O;
                self.end = true;
                return;
            } else if score == 3 {
                self.winner = TileState::X;
                self.end = true;
                return;
            }

            // horizontal
            score = self.board[i * 3] as i32
                + self.board[(i * 3) + 1] as i32
                + self.board[(i * 3) + 2] as i32;
            if score == -3 {
                self.winner = TileState::O;
                self.end = true;
                return;
            } else if score == 3 {
                self.winner = TileState::X;
                self.end = true;
                return;
            }
        }

        // Check diagonals
        score = self.board[0] as i32 + self.board[4] as i32 + self.board[8] as i32;
        if score == -3 {
            self.winner = TileState::O;
            self.end = true;
            return;
        } else if score == 3 {
            self.winner = TileState::X;
            self.end = true;
            return;
        }

        score = self.board[2] as i32 + self.board[4] as i32 + self.board[6] as i32;

        if score == -3 {
            self.winner = TileState::O;
            self.end = true;
            return;
        } else if score == 3 {
            self.winner = TileState::X;
            self.end = true;
            return;
        }

        score = self.board.iter().map(|&x| (x as i32).abs()).sum();

        if score == BOARD_SIZE as i32 {
            self.winner = TileState::Empty;
            self.end = true;
            return;
        }

        self.end = false;
    }

    fn update(&mut self) {
        self.compute_hash();
        self.check_end();
    }
}
