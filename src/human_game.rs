#[derive(Debug)]
pub struct HumanGame {
    current_state: State,
    current_symbol: isize,
    player1: AiPlayer,
}