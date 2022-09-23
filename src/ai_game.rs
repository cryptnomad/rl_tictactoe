use crate::ai_player::AiPlayer;
use crate::state::State;

pub fn train(epochs: usize, print_every_n: usize) {
    let player1 = AiPlayer::default();
    let player2 = AiPlayer::default();
    let mut game = AiGame::new(player1, player2);
    let mut p1_winrate = 0.0;
    let mut p2_winrate = 0.0;

    for i in 1..epochs + 1 {
        let winner = game.play(false);
        if winner == 1 {
            p1_winrate += 1.0;
        } else if winner == -1 {
            p2_winrate += 1.0;
        }

        if i % print_every_n == 0 {
            println!(
                "Epoch {}: Player 1 winrate: {}, Player 2 winrate: {}",
                i,
                p1_winrate / i as f64,
                p2_winrate / i as f64
            );
        }
        game.backup();
        game.reset();
    }
    game.save_policies();
}

pub fn compete(num_games: usize) {
    let player1 = AiPlayer::custom(0.1, 0.0);
    let player2 = AiPlayer::custom(0.1, 0.0);
    let mut game = AiGame::new(player1, player2);
    game.load_policies();
    let mut p1_winrate = 0.0;
    let mut p2_winrate = 0.0;

    for _ in 1..num_games + 1 {
        let winner = game.play(false);
        if winner == 1 {
            p1_winrate += 1.0;
        } else if winner == -1 {
            p2_winrate += 1.0;
        }
        game.reset();
    }
    println!(
        "{} Games: Player 1 winrate: {}, Player 2 winrate: {}",
        num_games,
        p1_winrate / num_games as f64,
        p2_winrate / num_games as f64
    );
}

#[derive(Debug)]
pub struct AiGame {
    current_state: State,
    current_symbol: isize,
    player1: AiPlayer,
    player2: AiPlayer,
}

impl AiGame {
    pub fn new(player1: AiPlayer, player2: AiPlayer) -> Self {
        let mut game = Self {
            current_state: State::new(),
            current_symbol: 1,
            player1: player1,
            player2: player2,
        };

        game.player1.set_symbol(1);
        game.player2.set_symbol(-1);
        game.player1.set_state(&game.current_state);
        game.player2.set_state(&game.current_state);
        game
    }

    //this could be optimized fo sho
    pub fn play(&mut self, print: bool) -> isize {
        while !self.current_state.is_end() {
            if print {
                self.current_state.display();
            }
            let index = if self.current_symbol == 1 {
                self.player1.act()
            } else {
                self.player2.act()
            };
            self.current_state = self
                .current_state
                .get_next_state(index, self.current_symbol);
            self.player1.set_state(&self.current_state);
            self.player2.set_state(&self.current_state);
            self.alternate();
        }

        if print {
            self.current_state.display();
        }
        self.current_state.get_winner()
    }

    pub fn reset(&mut self) {
        self.current_state = State::new();
        self.current_symbol = 1;
        self.player1.reset();
        self.player2.reset();
        self.player1.set_state(&self.current_state);
        self.player2.set_state(&self.current_state);
    }

    pub fn backup(&mut self) {
        self.player1.backup();
        self.player2.backup();
    }

    pub fn save_policies(&mut self) {
        self.player1.save_policy();
        self.player2.save_policy();
    }

    pub fn load_policies(&mut self) {
        self.player1.load_policy();
        self.player2.load_policy();
    }

    fn alternate(&mut self) {
        self.current_symbol = self.current_symbol * -1;
    }
}
