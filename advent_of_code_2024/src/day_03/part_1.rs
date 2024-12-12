use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead},
};

pub(crate) fn main() -> io::Result<()> {
    let file = File::open("./advent_of_code_2024/src/day_03/input.txt")?;
    let reader = io::BufReader::new(file);

    let reg = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut sum = 0;

    for line in reader.lines() {
        let line = line?;

        for cap in reg.captures_iter(&line) {
            let num1: i32 = cap[1].parse().unwrap();
            let num2: i32 = cap[2].parse().unwrap();
            sum += num1 * num2;
        }
    }

    println!("The result of save multiplication is: {}", sum);

    Ok(())
}
