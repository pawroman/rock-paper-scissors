use hands::{Hand, HandResult, HANDS_NAMES, HANDS_CODES, play_hand, random_hand};

use rand::{NewRng, StdRng};
use std::io;
use std::io::Write;


pub struct Game {
    rng: StdRng,
    score: isize,
}


impl Game {
    pub fn new() -> Game {
        Game {
            rng: StdRng::new(),
            score: 0,
        }
    }

    pub fn game_loop(&mut self) {
        println!("=== {} Game ===\n", HANDS_NAMES.join(" "));

        while let Some(player_hand) = self.choose_hand() {
            let (cpu_hand, result) = self.play_hand(player_hand);

            println!("You play : {}", player_hand.to_string());
            println!("CPU plays: {}\n", cpu_hand.to_string());
            println!("Result: you {}!", result.to_string());
            println!("Current score: {}\n", self.score);
        }

        println!("Game over.");
    }

    fn choose_hand(&self) -> Option<Hand> {
        let hand_moves = HANDS_NAMES.join(" / ");

        print!("Your move ({} / Quit)? >> ", hand_moves);
        let _ = io::stdout().flush();

        let mut input = String::new();

        if let Err(_) = io::stdin().read_line(&mut input) {
            None
        } else {
            let code = input.trim().to_uppercase();

            HANDS_CODES.get(&code).map(|&hand| hand)
        }
    }

    fn play_hand(&mut self, hand: Hand) -> (Hand, HandResult) {
        let cpu_hand = random_hand(&mut self.rng);
        let result = play_hand(hand, cpu_hand);
        let score_delta = match &result {
            &HandResult::Win => 1,
            &HandResult::Lose => -1,
            _ => 0,
        };

        self.score += score_delta;

        (cpu_hand, result)
    }
}
