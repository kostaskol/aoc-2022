use crate::utils::input;

pub fn run(extra: bool, test: bool) -> String {
    let input = input::read_raw("6", test);

    format!(
        "{}",
        match extra {
            false => p1::run(input),
            true => p2::run(input)
        }
    )
}

mod utils {
    pub fn count_unique(inp: String, in_groups_of: usize) -> u32 {
        let mut indx = (in_groups_of - 1) as u32;
        let chars: Vec<char> = inp.chars().collect();

        for window in chars.windows(in_groups_of) {
            indx += 1;
            let mut v = Vec::from(window);
            let initial_len = v.len();
            v.sort_unstable();
            v.dedup();
            if v.len() == initial_len {
                break;
            }
        }

        indx
    }
}

mod p1 {
    use super::utils;

    pub fn run(input: String) -> u32 {
        utils::count_unique(input, 4)
    }
}

mod p2 {
    use super::utils;

    pub fn run(input: String) -> u32 {
        utils::count_unique(input, 14)
    }
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    pub fn test_p1_test() {
        assert_eq!(run(false, true), "11");
    }

    #[test]
    pub fn test_p1_real() {
        assert_eq!(run(false, false), "1723");
    }

    #[test]
    pub fn test_p2_test() {
        assert_eq!(run(true, true), "26");
    }

    #[test]
    pub fn test_p2_real() {
        assert_eq!(run(true, false), "3708");
    }
}