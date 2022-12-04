use crate::utils::input;

pub fn run(extra: bool, test: bool) -> String {
    let input = input::read_file("4", test);

    format!(
        "{}",
        match extra {
            false => p1::run(input),
            true => p2::run(input)
        }
    )
}

mod utils {
    use std::ops::RangeInclusive;

    fn parse_pair_range(pair: &str) -> RangeInclusive<u16> {
        let r: Vec<u16> = pair.split('-').map(|p| p.parse::<u16>().unwrap()).collect();

        r[0]..=r[1]
    }

    pub fn parse_pairs(line: &str) -> (RangeInclusive<u16>, RangeInclusive<u16>) {
        let pairs: Vec<&str> = line.split(',').collect();

        (parse_pair_range(pairs[0]), parse_pair_range(pairs[1]))
    }
}

mod p1 {
    use std::cmp::Ordering;
    use std::ops::RangeInclusive;
    use super::utils;

    pub fn run(input: Vec<String>) -> u32 {
        let mut overlapping: u32 = 0;

        for line in input {
            let ranges = utils::parse_pairs(&line);
            let range_compare = |r1: RangeInclusive<u16>, r2: RangeInclusive<u16>| -> bool {
                r1.start() >= r2.start() && r1.end() <= r2.end()
            };

            // A range can be entirely included in another only if has a smaller length
            // If they're the same length they should be exactly equal
            match ranges.0.len().cmp(&ranges.1.len()) {
                Ordering::Equal => {
                    if ranges.0 == ranges.1 {
                        overlapping += 1;
                    }
                },
                Ordering::Less => {
                    if range_compare(ranges.0, ranges.1) {
                        overlapping += 1
                    }
                },
                Ordering::Greater => {
                    if range_compare(ranges.1, ranges.0) {
                        overlapping += 1
                    }
                }
            };
        }

        overlapping
    }
}

mod p2 {
    use super::utils;

    pub fn run(input: Vec<String>) -> u32 {
        let mut overlapping: u32 = 0;

        for line in input {
            let ranges = utils::parse_pairs(&line);

            if ranges.0.start() <= ranges.1.end() && ranges.0.end() >= ranges.1.start() {
                overlapping += 1;
            }
        }

        overlapping
    }
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    pub fn test_p1_test() {
        assert_eq!(run(false, true), "2");
    }

    #[test]
    pub fn test_p1_real() {
        assert_eq!(run(false, false), "450");
    }

    #[test]
    pub fn test_p2_test() {
        assert_eq!(run(true, true), "4");
    }

    #[test]
    pub fn test_p2_real() {
        assert_eq!(run(true, false), "837");
    }
}
