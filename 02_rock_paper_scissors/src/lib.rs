pub mod parse;

use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn value(&self) -> i32 {
        match *self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    pub fn beats(&self) -> Self {
        match *self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

    pub fn beat_by(&self) -> Self {
        match *self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }
}

impl Ord for Shape {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if &self.beats() == other {
            Ordering::Greater
        } else if &self.beat_by() == other {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn game_score(me: &Shape, opponent: &Shape) -> i32 {
    let outcome = match me.cmp(opponent) {
        Ordering::Less => 0,
        Ordering::Equal => 3,
        Ordering::Greater => 6,
    };
    outcome + me.value()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordering_ne() {
        assert!(Shape::Rock > Shape::Scissors);
        assert!(Shape::Paper > Shape::Rock);
        assert!(Shape::Scissors > Shape::Paper);
    }

    #[test]
    fn ordering_eq() {
        assert!(Shape::Rock == Shape::Rock);
        assert!(Shape::Paper == Shape::Paper);
        assert!(Shape::Scissors == Shape::Scissors);
    }

    #[test]
    fn game_score_win() {
        assert_eq!(8, game_score(&Shape::Paper, &Shape::Rock));
    }

    #[test]
    fn game_score_loss() {
        assert_eq!(1, game_score(&Shape::Rock, &Shape::Paper));
    }

    #[test]
    fn game_score_draw() {
        assert_eq!(6, game_score(&Shape::Scissors, &Shape::Scissors));
    }
}

