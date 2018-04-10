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
}

lazy_static! {
    pub static ref HANDS_NAMES: Vec<String> = Hand::iter()
                                              .map(|hand| hand.to_string())
                                              .collect();
}


pub fn play_hand(own_hand: Hand, other_hand: Hand) -> HandResult {
    match (own_hand, other_hand) {
        | (Rock, Scissors)
        | (Scissors, Paper)
        | (Paper, Rock)             => Win,

        _ if own_hand == other_hand => Draw,

        _                           => Lose,
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
