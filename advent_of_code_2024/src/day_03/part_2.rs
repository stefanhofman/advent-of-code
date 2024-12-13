use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead},
};

pub(crate) fn main(input: &str) -> io::Result<()> {
    let file = File::open(input)?;
    let reader = io::BufReader::new(file);

    let reg = Regex::new(r"don't\(\).*?(?:do\(\)|$)|mul\((\d+),(\d+)\)").unwrap();

    let mut text = String::new();
    for line in reader.lines() {
        text += &line?;
    }

    let mut sum = 0;

    for cap in reg.captures_iter(&text).filter(|cap| !cap.get(1).is_none()) {
        let num1: i32 = cap[1].parse().unwrap();
        let num2: i32 = cap[2].parse().unwrap();
        sum += num1 * num2;
    }

    println!("The result of save multiplication is: {}", sum);

    Ok(())
}
