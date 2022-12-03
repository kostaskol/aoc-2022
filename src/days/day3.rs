use crate::utils::input;

pub fn run(extra: bool, test: bool) -> String {
    let input = input::read_file("3", test);

    format!(
        "{}",
        match extra {
            false => p1::run(input),
            true => p2::run(input)
        }
    )
}

mod utils {
    use std::hash::Hash;
    use std::collections::HashSet;

    pub fn n_way_intersection<T: Clone + Copy + Eq + Hash>(vs: &Vec<Vec<T>>) -> Vec<T> {
        let mut acc: Vec<T> = vs.first().unwrap().clone();

        for tv in vs {
            let hs: HashSet<T> = tv.iter().copied().collect();
            acc = hs.
                intersection(&acc.into_iter().collect()).
                copied().
                collect::<Vec<T>>();
        }

        acc
    }

    pub fn char_to_priority(c: &char) -> u8 {
        let uval = *c as u8;
        let offset = if c.is_lowercase() { 96 } else { 38 };

        uval - offset
    }
}

mod p1 {
    use super::utils;

    pub fn run(input: Vec<String>) -> u32 {
        let mut acc: u32 = 0;

        for line in input {
            let arr: Vec<char> = line.chars().collect();

            let compartments = arr.chunks(arr.len() / 2);
            let c: Vec<Vec<char>> = compartments.map(Vec::<char>::from).collect();

            let intersection = utils::n_way_intersection(&c);
            acc += intersection.iter().map(utils::char_to_priority).sum::<u8>() as u32;
        }
        
        acc
    }
}

mod p2 {
    use super::utils;

    pub fn run(input: Vec<String>) -> u32 {
        let mut acc = 0;
        let groups: Vec<&[String]> = input.chunks(3).collect();

        for group in groups {
            let cgroups: Vec<Vec<char>> = group.iter().map(|g| g.chars().collect()).collect();
            let intersection = utils::n_way_intersection(&cgroups);
            let nums = intersection.iter().map(utils::char_to_priority);
            acc += nums.sum::<u8>() as u32;
        }
        acc
    }
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    pub fn test_p1_test() {
        assert_eq!(run(false, true), "157")
    }

    #[test]
    pub fn test_p1_real() {
        assert_eq!(run(false, false), "7793")
    }

    #[test]
    pub fn test_p2_test() {
        assert_eq!(run(true, true), "70")
    }

    #[test]
    pub fn test_p2_real() {
        assert_eq!(run(true, false), "2499")
    }
}