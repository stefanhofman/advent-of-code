mod day_1;
mod day_2;

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

    if args.day == 1 && args.part == 1 {
        return day_1::part_1::main();
    } else if args.day == 1 && args.part == 2 {
        return day_1::part_2::main();
    } else if args.day == 2 && args.part == 1 {
        return day_2::part_1::main();
    } else if args.day == 2 && args.part == 2 {
        return day_2::part_2::main();
    }

    Ok(())
}
