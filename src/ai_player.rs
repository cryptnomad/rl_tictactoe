use crate::state::{State, ALL_STATES, BOARD_SIZE};
use rand::prelude::*;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct AiPlayer {
    estimations: HashMap<isize, f64>,
    step_size: f64,
    epsilon: f64,
    states: Vec<State>,
    greedy: Vec<bool>,
    symbol: isize,
}

impl AiPlayer {
    pub fn default() -> Self {
        Self {
            estimations: HashMap::new(),
            step_size: 0.1,
            epsilon: 0.01,
            states: vec![],
            greedy: vec![],
            symbol: 0,
        }
    }

    pub fn custom(step_size: f64, epsilon: f64) -> Self {
        Self {
            estimations: HashMap::new(),
            step_size: step_size,
            epsilon: epsilon,
            states: vec![],
            greedy: vec![],
            symbol: 0,
        }
    }

    pub fn competitor(symbol: isize) -> Self {
        let mut player = Self {
            estimations: HashMap::new(),
            step_size: 0.0,
            epsilon: 0.0,
            states: vec![],
            greedy: vec![],
            symbol: symbol,
        };
        player.load_policy();
        player
    }

    pub fn reset(&mut self) {
        self.states = vec![];
        self.greedy = vec![];
    }

    pub fn set_state(&mut self, state: &State) {
        self.states.push(state.clone());
        self.greedy.push(true);
    }

    pub fn set_symbol(&mut self, symbol: isize) {
        self.symbol = symbol;
        for (&hash_val, end) in ALL_STATES.iter() {
            let is_end = end.0;
            if is_end {
                let winner = end.1;
                if winner == symbol {
                    self.estimations.insert(hash_val, 1.0);
                } else if winner == 0isize {
                    self.estimations.insert(hash_val, 0.5);
                } else {
                    self.estimations.insert(hash_val, 0.0);
                }
            } else {
                self.estimations.insert(hash_val, 0.5);
            }
        }
    }

    pub fn backup(&mut self) {
        let mut state_hashes = vec![];

        for s in &self.states {
            state_hashes.push(s.get_hash());
        }

        for i in (0..state_hashes.len() - 1).rev() {
            let state_hash = state_hashes[i];
            let td_error = ((self.greedy[i] as isize) as f64)
                * (self.estimations.get(&state_hashes[i + 1]).unwrap()
                    - self.estimations.get(&state_hash).unwrap());
            *self.estimations.get_mut(&state_hash).unwrap() += self.step_size * td_error;
        }
    }

    pub fn act(&mut self) -> usize {
        let cur_state = self.states[self.states.len() - 1].clone();
        let mut next_states = vec![];
        let mut next_positions = vec![];

        for i in 0..BOARD_SIZE {
            if cur_state.get_tile(i) == 0 {
                next_positions.push(i);
                next_states.push(cur_state.get_next_state(i, self.symbol).get_hash());
            }
        }

        let mut rng = rand::thread_rng();

        if rng.gen::<f64>() < self.epsilon {
            let length = self.greedy.len();
            self.greedy[length - 1] = false;
            return next_positions[rng.gen_range(0..next_positions.len())];
        }

        let mut values = vec![];
        for (hash_val, pos) in next_states.iter().zip(&next_positions) {
            values.push((self.estimations.get(hash_val), pos));
        }

        values.shuffle(&mut rng);
        values.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        *values[values.len() - 1].1
    }

    pub fn save_policy(&self) {
        let filename = if self.symbol == 1 {
            "policy_first.bin"
        } else {
            "policy_second.bin"
        }
        .to_string();

        let mut outfile = File::create(filename).expect("failed creating file");

        let serialized = serde_json::to_string(&self.estimations).unwrap();
        outfile
            .write_all(serialized.as_bytes())
            .expect("Failed to write file");
    }

    /*
        Load and deserialize the policy
    */
    pub fn load_policy(&mut self) {
        let filename = if self.symbol == 1 {
            "policy_first.bin"
        } else {
            "policy_second.bin"
        }
        .to_string();

        let mut infile = File::open(filename).expect("Failed to open file");
        let mut serialized = String::new();
        infile
            .read_to_string(&mut serialized)
            .expect("Failed to read file contents");
        let deserialized = serde_json::from_str(&serialized).unwrap();

        self.estimations = deserialized;
    }
}
