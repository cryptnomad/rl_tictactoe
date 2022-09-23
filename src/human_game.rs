#[derive(Debug)]
pub struct HumanGame {
    current_state: State,
    current_symbol: isize,
    ai_player: AiPlayer,
}