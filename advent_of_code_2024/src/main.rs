mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;

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

fn run_day(day: i32, part: i32) -> io::Result<()> {
    let input_path = format!("./advent_of_code_2024/src/day_{:02}/input.txt", day);

    match (day, part) {
        (1, 1) => day_01::part_1::main(&input_path),
        (1, 2) => day_01::part_2::main(&input_path),
        (2, 1) => day_02::part_1::main(&input_path),
        (2, 2) => day_02::part_2::main(&input_path),
        (3, 1) => day_03::part_1::main(&input_path),
        (3, 2) => day_03::part_2::main(&input_path),
        (4, 1) => day_04::part_1::main(&input_path),
        (4, 2) => day_04::part_2::main(&input_path),
        (5, 1) => day_05::part_1::main(&input_path),
        (5, 2) => day_05::part_2::main(&input_path),
        (6, 1) => day_06::part_1::main(&input_path),
        (6, 2) => day_06::part_2::main(&input_path),
        (7, 1) => day_07::part_1::main(&input_path),
        (7, 2) => day_07::part_2::main(&input_path),
        (8, 1) => day_08::part_1::main(&input_path),
        (8, 2) => day_08::part_2::main(&input_path),
        (9, 1) => day_09::part_1::main(&input_path),
        (9, 2) => day_09::part_2::main(&input_path),
        (10, 1) => day_10::part_1::main(&input_path),
        (10, 2) => day_10::part_2::main(&input_path),
        _ => panic!("Invalid day or part"),
    }
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    run_day(args.day, args.part)
}
