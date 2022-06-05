use hands::{Hand, HandResult, HANDS, HANDS_NAMES, play_hand, random_hand};

use rand::prelude::*;
use std::collections::HashMap;
use std::io;
use std::io::Write;


struct HandInput {
    hand: Hand,
    label: String,
    number: usize,
}

lazy_static! {
    static ref HAND_INPUTS: Vec<HandInput> = {
        HANDS.iter().enumerate().map(
            |(num, &hand)| {
                let number = num + 1;
                let label = format!("[{}] {}", number, hand);

                HandInput { hand, label, number }
            }
        ).collect()
    };

    static ref INPUT_MAP: HashMap<usize, &'static HandInput> = {
        let mut input_map = HashMap::new();

        for hand_input in HAND_INPUTS.iter() {
            input_map.insert(hand_input.number, hand_input);
        }

        input_map
    };

    static ref HAND_INPUT_PROMPT: String = {
        HAND_INPUTS
        .iter()
        .map(|hand_input| hand_input.label.clone())
        .collect::<Vec<_>>()
        .join(" / ")
    };
}

pub struct Game {
    rng: ThreadRng,
    score: isize,
}

impl Game {
    pub fn new() -> Game {
        Game {
            rng: thread_rng(),
            score: 0,
        }
    }

    pub fn game_loop(&mut self) {
        println!("=== {} Game ===\n", HANDS_NAMES.join(" "));
        println!("Any non-move input quits.\n");

        while let Some(player_hand) = self.choose_hand() {
            let (cpu_hand, result) = self.play_hand(player_hand);

            println!("You play : {}", player_hand);
            println!("CPU plays: {}\n", cpu_hand);
            println!("Result: you {}!", result);
            println!("Current score: {}\n", self.score);
        }

        println!("Game over.");
    }

    fn choose_hand(&self) -> Option<Hand> {
        print!("Your move ({})? >> ", *HAND_INPUT_PROMPT);

        let _ = io::stdout().flush();
        let mut input = String::new();

        if io::stdin().read_line(&mut input).is_err() {
            None
        } else {
            self.parse_input(input)
        }
    }

    fn parse_input(&self, input: String) -> Option<Hand> {
        let number: usize = input.trim().parse().ok()?;

        INPUT_MAP.get(&number).map(|hand_input| hand_input.hand)
    }

    fn play_hand(&mut self, hand: Hand) -> (Hand, HandResult) {
        let cpu_hand = random_hand(&mut self.rng);
        let result = play_hand(hand, cpu_hand);
        let score_delta = match result {
            HandResult::Win => 1,
            HandResult::Lose => -1,
            _ => 0,
        };

        self.score += score_delta;

        (cpu_hand, result)
    }
}
