use crate::utils::input;

pub fn run(extra: bool, test: bool) -> String {
    let content = input::read_raw("5", test);

    let (_, instrs) = utils::sections(&content);
    let state = utils::state(test);

    let parsed_state = utils::State::new(state);
    let parsed_instrs = utils::InstructionSet::from(instrs);

    match extra {
        false => p1::run(parsed_state, parsed_instrs),
        true => p2::run(parsed_state, parsed_instrs)
    }
}

mod utils {
    use regex::Regex;

    pub fn state(test: bool) -> Vec<Vec<String>> {
        if test {
            vec![
                vec!["", "D", ""],
                vec!["N", "C", ""],
                vec!["Z", "M", "P"]
            ]
        } else {
            vec![
                vec!["", "", "", "", "", "", "Z", "W", "Z"],
                vec!["", "", "D", "M", "", "", "L", "P", "G"],
                vec!["", "S", "N", "R", "", "", "S", "F", "N"],
                vec!["", "N", "J", "W", "", "J", "F", "D", "F"],
                vec!["N", "H", "G", "J", "", "H", "Q", "H", "P"],
                vec!["V", "J", "T", "F", "H", "Z", "R", "L", "M"],
                vec!["C", "M", "C", "D", "F", "T", "P", "S", "S"],
                vec!["S", "Z", "M", "T", "P", "C", "D", "C", "D"]
            ]
        }.iter().map(|r| r.iter().map(|&c| c.to_string()).collect()).collect::<Vec<Vec<String>>>()
    }

    pub struct State {
        state: Vec<Vec<String>>
    }

    impl State {
        pub fn new(state: Vec<Vec<String>>) -> Self {
            Self { state }
        }

        pub fn mv(&mut self, from: usize, to: usize, amount: usize) {
            let mut acc: Vec<String> = Vec::new();
            let mut curr_amount = 0;

            for row in self.state.iter_mut() {
                if !row[from].is_empty() {
                    acc.push(row[from].clone());
                    row[from] = "".to_string();

                    curr_amount += 1;

                    if curr_amount == amount {
                        break;
                    }
                }
            }

            acc = acc.into_iter().rev().collect();
            let mut found = false;
            for el in acc {
                for row_indx in 0..self.state.len() {
                    let row = &self.state[row_indx];
                    if !row[to].is_empty() {
                        if row_indx == 0 {
                            let mut new_row = vec!["".to_string(); row.len()];
                            new_row[to] = el.clone();
                            self.state.insert(0, new_row);
                        } else {
                            self.state[row_indx - 1][to] = el.clone();
                        }
                        found = true;
                        break;
                    }
                }

                if !found {
                    let last_row = self.state.len() - 1;
                    self.state[last_row][to] = el;
                }
            }
        }

        pub fn peak_top(&self, col: usize) -> Option<String> {
            for row in self.state.iter() {
                if !row[col].is_empty() {
                    return Some(row[col].clone());
                }
            }

            None
        }

        pub fn heads(&self) -> String {
            let mut acc: Vec<String> = Vec::new();
            for col in 0..self.state[0].len() {
                let res = self.peak_top(col).unwrap();
                acc.push(res);
            }

            acc.join("")
        }
    }

    pub struct Instruction {
        pub amount: usize,
        pub from: usize,
        pub to: usize
    }

    impl From<&str> for Instruction {
        fn from(instr: &str) -> Self {
            let re = Regex::new(r"move (.+) from (.+) to (.+)").unwrap();
            let groups = re.captures(instr).unwrap();

            Self {
                amount: groups.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                from: groups.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                to: groups.get(3).unwrap().as_str().parse::<usize>().unwrap()
            }
        }
    }

    pub struct InstructionSet {
        pub instructions: Vec<Instruction>
    }

    impl From<&str> for InstructionSet {
        fn from(inp: &str) -> Self {
            let instrs: Vec<&str> = inp.split('\n').collect();

            let mut acc: Vec<Instruction> = Vec::with_capacity(instrs.len());

            for instr in instrs {
                acc.push(Instruction::from(instr));
            }

            Self { instructions: acc }
        }
    }

    pub fn sections(content: &str) -> (&str, &str) {
        let sections: Vec<&str> = content.split("\n\n").collect();

        (sections[0], sections[1])
    }
}

mod p1 {
    use super::utils::{State, InstructionSet};

    pub fn run(mut state: State, instructions: InstructionSet) -> String {
        for instruction in instructions.instructions {
            for _ in 0..instruction.amount {
                state.mv(instruction.from - 1, instruction.to - 1, 1);
            }
        }

        state.heads()
    }
}

mod p2 {
    use super::utils::{State, InstructionSet};

    pub fn run(mut state: State, instructions: InstructionSet) -> String {
        for instruction in instructions.instructions {
            state.mv(instruction.from - 1, instruction.to - 1, instruction.amount)
        }

        state.heads()
    }
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    pub fn test_p1_test() {
        assert_eq!(run(false, true), "CMZ");
    }

    #[test]
    pub fn test_p1_real() {
        assert_eq!(run(false, false), "CNSZFDVLJ");
    }

    #[test]
    pub fn test_p2_test() {
        assert_eq!(run(true, true), "MCD");
    }

    #[test]
    pub fn test_p2_real() {
        assert_eq!(run(true, false), "QNDWLMGNS");
    }
}