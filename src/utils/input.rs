
use std::fs::read_to_string;

pub fn read_file(day: &str, test: bool) -> Vec<String> {
    read_lines(&inp_file(day, test))
}

pub fn read_raw(day: &str, test: bool) -> String {
    let filename = inp_file(day, test);

    read_to_string(&filename).unwrap_or_else(|_| panic!("Put something in {} first", filename))
}

pub fn read_lines(filename: &str) -> Vec<String> {
    let content =
        read_to_string(filename).unwrap_or_else(|_| panic!("Put something in {} first", filename));

    raw_to_lines(&content)
}

pub fn raw_to_lines(file: &str) -> Vec<String> {
    file.split('\n')
        .map(|s| s.to_string())
        .filter(|s| !s.starts_with('#'))
        .collect()
}

#[allow(dead_code)]
pub fn read_day_as_ints(day: &str, test: bool) -> Vec<i32> {
    convert_to_ints(&read_file(day, test))
}

#[allow(dead_code)]
pub fn convert_to_ints(lines: &[String]) -> Vec<i32> {
    lines.
        iter().
        map(|s| {
            s.parse::<i32>()
                .unwrap_or_else(|_| panic!("{} is not a number!", s))
        })
        .collect()
}

fn inp_file(day: &str, test: bool) -> String {
    if test {
        format!("input/test/day{}.in", day)
    } else {
        format!("input/day{}.in", day)
    }
}