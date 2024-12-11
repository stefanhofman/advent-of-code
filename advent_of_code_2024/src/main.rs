mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

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
        (1, 1) => day_1::part_1::main(),
        (1, 2) => day_1::part_2::main(),
        (2, 1) => day_2::part_1::main(),
        (2, 2) => day_2::part_2::main(),
        (3, 1) => day_3::part_1::main(),
        (3, 2) => day_3::part_2::main(),
        (4, 1) => day_4::part_1::main(),
        (4, 2) => day_4::part_2::main(),
        (5, 1) => day_5::part_1::main(),
        (5, 2) => day_5::part_2::main(),
        (6, 1) => day_6::part_1::main(),
        (6, 2) => day_6::part_2::main(),
        (7, 1) => day_7::part_1::main(),
        (7, 2) => day_7::part_2::main(),
        (8, 1) => day_8::part_1::main(),
        (8, 2) => day_8::part_2::main(),
        (9, 1) => day_9::part_1::main(),
        (9, 2) => day_9::part_2::main(),
        _ => panic!("Invalid day or part"),
    }
}
