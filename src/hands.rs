extern crate rand;

use rand::{Rng, StdRng};
use strum::IntoEnumIterator;

use self::Hand::*;
use self::HandResult::*;


#[derive(ToString, Eq, Debug, PartialEq)]
pub enum HandResult {
    Win,
    Lose,
    Draw,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter, ToString)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

lazy_static! {
    pub static ref HANDS: Vec<Hand> = Hand::iter().collect();
    pub static ref HANDS_NAMES: Vec<String> = Hand::iter()
                                              .map(|hand| hand.to_string())
                                              .collect();
}

pub trait Beats {
    fn beats(&self) -> Self;
}

impl Beats for Hand {
    fn beats(&self) -> Self {
        match *self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }
}

pub fn play_hand(own_hand: Hand, other_hand: Hand) -> HandResult {
    let (own_beats, other_beats) = (own_hand.beats(), other_hand.beats());

    match (own_beats, other_beats) {
        _ if own_beats == other_hand => Win,
        _ if other_beats == own_hand => Lose,
        _                            => Draw,
    }
}

pub fn random_hand(rng: &mut StdRng) -> Hand {
    *rng.choose(&HANDS).unwrap()
}


#[cfg(test)]
mod tests {
    use super::{HANDS, HANDS_NAMES, play_hand};
    use super::{HandResult::*, Hand::*};
    use std::collections::HashSet;

    #[test]
    fn test_unique_names() {
        assert_eq!(HANDS_NAMES.len(), HANDS.len());
        assert_eq!(HANDS_NAMES.iter().collect::<HashSet<_>>().len(),
                   HANDS.len());
    }

    #[test]
    fn test_play_hand() {
        assert_eq!(play_hand(Rock, Scissors), Win);
        assert_eq!(play_hand(Rock, Paper), Lose);
        assert_eq!(play_hand(Rock, Rock), Draw);

        assert_eq!(play_hand(Paper, Rock), Win);
        assert_eq!(play_hand(Paper, Scissors), Lose);
        assert_eq!(play_hand(Paper, Paper), Draw);

        assert_eq!(play_hand(Scissors, Paper), Win);
        assert_eq!(play_hand(Scissors, Rock), Lose);
        assert_eq!(play_hand(Scissors, Scissors), Draw);
    }
}
