use std::collections::BTreeMap;
use structopt::StructOpt;

mod day01;
mod day02;
mod common;
mod day03;
mod day04;

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
        (1, Box::new(day01::run_day) as Box<_>),
        (2, Box::new(day02::run_day) as Box<_>),
        (3, Box::new(day03::run_day) as Box<_>),
        (4, Box::new(day04::run_day) as Box<_>)
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


