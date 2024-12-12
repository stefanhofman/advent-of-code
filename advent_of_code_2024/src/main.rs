mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;

use clap::Parser;
use std::io;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The day of the challange to run
    #[arg(short, long)]
    day: i32,

    /// Which part of the day to run
    #[arg(short, long)]
    part: i32,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    match (args.day, args.part) {
        (1, 1) => day_01::part_1::main(),
        (1, 2) => day_01::part_2::main(),
        (2, 1) => day_02::part_1::main(),
        (2, 2) => day_02::part_2::main(),
        (3, 1) => day_03::part_1::main(),
        (3, 2) => day_03::part_2::main(),
        (4, 1) => day_04::part_1::main(),
        (4, 2) => day_04::part_2::main(),
        (5, 1) => day_05::part_1::main(),
        (5, 2) => day_05::part_2::main(),
        (6, 1) => day_06::part_1::main(),
        (6, 2) => day_06::part_2::main(),
        (7, 1) => day_07::part_1::main(),
        (7, 2) => day_07::part_2::main(),
        (8, 1) => day_08::part_1::main(),
        (8, 2) => day_08::part_2::main(),
        (9, 1) => day_09::part_1::main(),
        (9, 2) => day_09::part_2::main(),
        _ => panic!("Invalid day or part"),
    }
}
