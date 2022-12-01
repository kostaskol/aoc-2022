extern crate clap;
use clap::{App, Arg};

mod days;
mod utils;

use crate::days::*;

const LAST_DAY: & str = "1";

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

    let problem = matches.value_of("problem_number").unwrap_or(LAST_DAY);
    let answer: String = match problem {
        "1" => day1::run(extra, test),
        &_ => format!("Only know how to solve #{:?} for now :(", (1..=LAST_DAY.parse::<i8>().unwrap())),
    };

    println!("{}", answer);
}
