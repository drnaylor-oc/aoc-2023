use std::collections::BTreeMap;
use structopt::StructOpt;

mod day01;
mod day02;
mod common;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;

#[derive(StructOpt, Debug)]
#[structopt(name = "aoc")]
struct Opts {

    #[structopt(short, long("day"))]
    days: Vec<u8>

}

fn main() {
    let args: Opts = Opts::from_args();

    // Add day numbers to functions here
    let days: BTreeMap<u8, Box<dyn Fn() -> ()>> = BTreeMap::from([
        (1,  Box::new(day01::run_day) as Box<_>),
        (2,  Box::new(day02::run_day) as Box<_>),
        (3,  Box::new(day03::run_day) as Box<_>),
        (4,  Box::new(day04::run_day) as Box<_>),
        (5,  Box::new(day05::run_day) as Box<_>),
        (6,  Box::new(day06::run_day) as Box<_>),
        (7,  Box::new(day07::run_day) as Box<_>),
        (8,  Box::new(day08::run_day) as Box<_>),
        (9,  Box::new(day09::run_day) as Box<_>),
        (10, Box::new(day10::run_day) as Box<_>),
        (11, Box::new(day11::run_day) as Box<_>),
        (12, Box::new(day12::run_day) as Box<_>),
        (13, Box::new(day13::run_day) as Box<_>),
        (14, Box::new(day14::run_day) as Box<_>),
    ]);
    let no_of_days: u8 = days.len().try_into().unwrap();

    // Chooses the days to run
    let days_to_run: Vec<u8> = if args.days.is_empty() {
        // No entries = run all days
        days.keys().map(|x| x.clone()).collect()
    } else if args.days.iter().all(|entry| *entry <= no_of_days && *entry > 0) {
        // The days to run
        args.days
    } else {
        let days: Vec<String> = days.keys().map(|s| format!("{}", *s)).collect();
        let asked: Vec<String> = args.days.iter().map(|s| format!("{}", *s)).collect();
        panic!("A day was specified that does not exist (specified {}, allowed days are {})!", asked.join(", "), days.join(", "))
    };

    for day in days_to_run {
        println!("Day {}", day);
        println!("---");
        days.get(&day).unwrap()();
        println!("---");
    }
}


