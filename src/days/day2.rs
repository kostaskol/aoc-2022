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
    use phf::phf_map;

    static SCORES: phf::Map<&'static str, u8> = phf_map! {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        "LOSS" => 0,
        "DRAW" => 3,
        "WIN" => 6
    };

    static CIRCLE: phf::Map<&'static str, &'static str> = phf_map! {
        "A" => "C",
        "B" => "A",
        "C" => "B"
    };

    static WIN_CIRCLE: phf::Map<&'static str, &'static str> = phf_map! {
        "A" => "B",
        "B" => "C",
        "C" => "A"
    };

    static PLAYER_TO_OP: phf::Map<&'static str, &'static str> = phf_map! {
        "X" => "A",
        "Y" => "B",
        "Z" => "C"
    };

    static OUTCOMES: phf::Map<&'static str, &'static str> = phf_map! {
        "X" => "LOSS",
        "Y" => "DRAW",
        "Z" => "WIN"
    };

    pub fn round(op: &str, me: &str) -> u8 {
        let player_hand = *PLAYER_TO_OP.get(me).unwrap();
        let hand_score = SCORES.get(player_hand).unwrap();

        let game_score =
            if op == player_hand {
                SCORES.get("DRAW").unwrap()
            } else if *CIRCLE.get(op).unwrap() == player_hand {
                SCORES.get("LOSS").unwrap()
            } else {
                SCORES.get("WIN").unwrap()
            };

        game_score + hand_score
    }

    pub fn inverse_round(op: &str, outcome: &str) -> u8 {
        let outcome = *OUTCOMES.get(outcome).unwrap();
        let game_score = SCORES.get(outcome).unwrap();

        let hand =
            if outcome == "DRAW" {
                op
            } else if outcome == "LOSS" {
                CIRCLE.get(op).unwrap()
            } else {
                WIN_CIRCLE.get(op).unwrap()
            };

        let hand_score = SCORES.get(hand).unwrap();

        game_score + hand_score
    }

    pub fn play(input: Vec<String>, f: fn(&str, &str) -> u8) -> u32 {
        let mut acc: u32 = 0;

        for line in input {
            let moves = line.split(' ').collect::<Vec<&str>>();

            acc += f(moves[0], moves[1]) as u32;
        }

        acc
    }
}

mod p1 {
    use crate::day2::game;

    pub fn run(input: Vec<String>) -> u32 {
        game::play(input, game::round)
    }
}

mod p2 {
    use crate::day2::game;

    pub fn run(input: Vec<String>) -> u32 {
        game::play(input, game::inverse_round)
    }
}