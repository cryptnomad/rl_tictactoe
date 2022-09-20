use std::fmt;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileState {
    X = -1,
    Empty = 0,
    O = 1,
}

impl fmt::Display for TileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TileState::Empty => write!(f, " "),
            TileState::X => write!(f, "X"),
            TileState::O => write!(f, "O"),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    P1Turn,
    P2Turn,
    P1Wins,
    P2Wins,
    Draw
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self{
            GameState::P1Turn => write!(f, "It is Player 1's turn"),
            GameState::P2Turn => write!(f, "It is Player 2's turn"),
            GameState::P1Wins => write!(f, "Player 1 wins!"),
            GameState::P2Wins => write!(f, "Player 2 wins!"),
            GameState::Draw => write!(f, "Draw"),
        }
    }
}

pub fn display_board(board: &[TileState]){
    println!("{} | {} | {}\n---------\n{} | {} | {}\n---------\n{} | {} | {}", board[0], board[1], board[2], board[3], board[4], board[5], board[6], board[7], board[8]);
}

pub fn update_state(board: &[TileState], game_state: &mut GameState){
    let mut score: i32;
    // Check verticals and horizontals
    for i in 0..3 {
        //vertical
        score = board[i] as i32 + board[i+3] as i32 + board[i+6] as i32;
        if score == -3{
            *game_state = GameState::P1Wins;
            return
        }else if score == 3{
            *game_state = GameState::P2Wins;
            return
        }

        // horizontal
        score = board[i*3] as i32 + board[(i*3)+1] as i32 + board[(i*3)+2] as i32;
        if score == -3{
            *game_state = GameState::P1Wins;
            return
        }else if score == 3{
            *game_state = GameState::P2Wins;
            return
        }
    }

    // Check diagonals
    score = board[0] as i32 + board[4] as i32 + board[8] as i32;
    if score == -3{
        *game_state = GameState::P1Wins;
        return
    }else if score == 3{
        *game_state = GameState::P2Wins;
        return
    }

    score = board[2] as i32 + board[4] as i32 + board[6] as i32;

    if score == -3{
        *game_state = GameState::P1Wins;
        return
    }else if score == 3{
        *game_state = GameState::P2Wins;
        return
    }

    // count open tiles
    score = 0;
    for (_, tile) in board.iter().enumerate(){
        if *tile == TileState::Empty{
            score += 1;
        }
    }
    if score == 0{
        *game_state = GameState::Draw;
    }else if score % 2 == 0{
        *game_state = GameState::P2Turn;
    }else{
        *game_state = GameState::P1Turn;
    }
}

pub fn move_is_valid(board: &[TileState], index: usize) -> bool{
    if index > 8{
        return false;
    }
    board[index] == TileState::Empty
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vert_win() {
        use TileState::{Empty, X, O};
        let mut game_state: GameState = GameState::P1Turn;
        let board = [X, O, O, X, Empty, O, X, Empty, Empty];
        update_state(&board, &mut game_state);
        assert_eq!(game_state, GameState::P1Wins);
    }

    #[test]
    fn test_horiz_win() {
        use TileState::{Empty, X, O};
        let mut game_state: GameState = GameState::P1Turn;
        let board = [O, O, O, X, Empty, O, X, Empty, Empty];
        update_state(&board, &mut game_state);
        assert_eq!(game_state, GameState::P2Wins);
    }

    #[test]
    fn test_diag_win() {
        use TileState::{Empty, X, O};
        let mut game_state: GameState = GameState::P1Turn;
        let board = [O, X, Empty, X, O, O, X, Empty, O];
        update_state(&board, &mut game_state);
        assert_eq!(game_state, GameState::P2Wins);
    }

    #[test]
    fn test_draw() {
        use TileState::{Empty, X, O};
        let mut game_state: GameState = GameState::P1Turn;
        let board = [O, X, O, X, X, O, X, O, X];
        update_state(&board, &mut game_state);
        assert_eq!(game_state, GameState::Draw);
    }

    #[test]
    fn test_p1_turn() {
        use TileState::{Empty, X, O};
        let mut game_state: GameState = GameState::P1Turn;
        let board = [O, X, O, X, X, O, X, O, Empty];
        update_state(&board, &mut game_state);
        assert_eq!(game_state, GameState::P1Turn);
    }

    #[test]
    fn test_p2_turn() {
        use TileState::{Empty, X, O};
        let mut game_state: GameState = GameState::Draw;
        let board = [O, X, O, X, X, O, X, Empty, Empty];
        update_state(&board, &mut game_state);
        assert_eq!(game_state, GameState::P2Turn);
    }
}