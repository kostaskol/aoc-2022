use crate::utils::input;

pub fn run(extra: bool, test: bool) -> String {
    let input = input::read_file("2", test);

    format!(
        "{}",
        match extra {
            false => p1::run(input),
            true => p2::run(input)
        }
    )
}

mod game {
    use enum_iterator::{previous_cycle, next_cycle, Sequence};

    #[derive(PartialEq, Eq)]
    pub enum Outcome {
        Loss,
        Draw,
        Win
    }

    impl Outcome {
        pub fn score(&self) -> u8 {
            match self {
                Self::Loss => 0,
                Self::Draw => 3,
                Self::Win => 6
            }
        }
    }

    impl From<&str> for Outcome {
        fn from(outcome: &str) -> Self {
            match outcome {
                "X" | "Loss" => Self::Loss,
                "Y" | "Draw" => Self::Draw,
                "Z" | "Win" => Self::Win,
                _ => panic!("Unknown outcome {}", outcome)
            }
        }
    }

    impl From<(&Hand, &Hand)> for Outcome {
        fn from((hand1, hand2): (&Hand, &Hand)) -> Self {
            if hand1 == hand2 {
                Self::from("Draw")
            } else if hand1.beats() == *hand2 {
                Self::from("Loss")
            } else {
                Self::from("Win")
            }
        }
    }

    #[derive(Sequence, PartialEq, Eq, Copy, Clone)]
    pub enum Hand {
        Rock,
        Paper,
        Scissors
    }

    impl Hand {
        pub fn beats(&self) -> Self {
            previous_cycle::<Self>(self).unwrap()
        }

        pub fn loses_to(&self) -> Self {
            next_cycle::<Self>(self).unwrap()
        }

        pub fn draw(&self) -> Self {
            *self
        }

        pub fn score(&self) -> u8 {
            match self {
                Self::Rock => 1,
                Self::Paper => 2,
                Self::Scissors => 3
            }
        }
    }

    impl From<&str> for Hand {
        fn from(hand: &str) -> Self {
            match hand {
                "A" | "X" => Self::Rock,
                "B" | "Y" => Self::Paper,
                "C" | "Z" => Self::Scissors,
                _ => panic!("Unknown move {}", hand)
            }
        }
    }
}

mod p1 {
    use super::game;

    pub fn run(input: Vec<String>) -> u32 {
        let mut acc: u32 = 0;

        for line in input {
            let moves = line.split(' ').collect::<Vec<&str>>();
            let op_hand = game::Hand::from(moves[0]);
            let player_hand = game::Hand::from(moves[1]);

            acc += play(&op_hand, &player_hand) as u32;
        }

        acc
    }

    fn play(op_hand: &game::Hand, player_hand: &game::Hand) -> u8 {
        game::Outcome::from((op_hand, player_hand)).score() + player_hand.score()
    }
}

mod p2 {
    use super::game;

    pub fn run(input: Vec<String>) -> u32 {
        let mut acc: u32 = 0;

        for line in input {
            let moves = line.split(' ').collect::<Vec<&str>>();
            let op_hand = game::Hand::from(moves[0]);
            let outcome = game::Outcome::from(moves[1]);

            acc += play(&op_hand, &outcome) as u32;
        }

        acc
    }

    fn play(op: &game::Hand, outcome: &game::Outcome) -> u8 {
        let hand = match outcome {
            game::Outcome::Loss => op.beats(),
            game::Outcome::Draw => op.draw(),
            game::Outcome::Win => op.loses_to()
        };

        outcome.score() + hand.score()
    }
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_p1_test() {
        assert_eq!(run(false, true), "15")
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(run(false, false), "11063");
    }

    #[test]
    fn test_p2_test() {
        assert_eq!(run(true, true), "12")
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(run(true, false), "10349")
    }
}