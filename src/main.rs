extern crate clap;
use clap::{App, Arg};

mod days;
mod utils;

use crate::days::*;

fn main() {
    let matches = App::new("Advent of code!")
        .arg(
            Arg::with_name("extra")
                .short("e")
                .long("extra")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("problem_number")
                .short("p")
                .long("problem")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("test_input")
                .short("t")
                .long("test")
                .takes_value(false),
        )
        .get_matches();

    let extra = matches.is_present("extra");
    let test = matches.is_present("test_input");

    let problem = matches.value_of("problem_number").unwrap_or("12");
    let answer: String = match problem {
        "1" => day1::run(extra, test),
        "2" => day2::run(extra, test),
        "3" => day3::run(extra, test),
        "4" => day4::run(extra, test),
        "5" => day5::run(extra, test),
        "6" => day6::run(extra, test),
        "12" => day12::run(extra, test),
        &_ => format!("Only know how to solve #{:?} for now :(", (1..=12)),
    };

    println!("{}", answer);
}