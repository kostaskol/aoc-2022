use crate::utils::input;

pub fn run(extra: bool, test: bool) -> String {
    let lines = input::read_file("1", test);
    let input = split_on_empty_and_make_int(&lines);

    format!(
        "{}",
        match extra {
            false => p1::run(input),
            true => p2::run(input)
        }
    )
}

fn split_on_empty_and_make_int(lines: &[String]) -> Vec<Vec<u32>> {
    let mut acc: Vec<Vec<u32>> = Vec::new();
    let mut partial_acc: Vec<u32> = Vec::new();

    for line in lines {
        if line.is_empty() {
            acc.push(partial_acc);
            partial_acc = Vec::new();
        } else {
            partial_acc.push(line.parse::<u32>().unwrap_or_else(|_| panic!("Not an integer!")));
        }
    }
    acc.push(partial_acc);

    acc
}

fn sums(input: &[Vec<u32>]) -> Vec<u32> {
    input.iter().map(|e| e.iter().sum() ).collect()
}

mod p1 {
    use crate::days::day1::sums;

    pub fn run(input: Vec<Vec<u32>>) -> u32 {
        match sums(&input).iter().max() {
            Some(&m) => m,
            None => 0
        }
    }
}

mod p2 {
    use crate::days::day1::sums;

    pub fn run(input: Vec<Vec<u32>>) -> u32 {
        let mut sums = sums(&input);
        // descending order
        sums.sort_by(|a, b| b.cmp(a));

        sums[0..=2].iter().sum()
    }
}