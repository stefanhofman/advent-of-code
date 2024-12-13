use std::{
    fs::File,
    io::{self, BufRead},
};

pub(crate) fn main(input: &str) -> io::Result<()> {
    let file = File::open(input)?;
    let reader = io::BufReader::new(file);

    let mut safe_count = 0;

    for line in reader.lines() {
        let number_line: Vec<i32> = line?
            .clone()
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        if is_safe(number_line.clone()) {
            safe_count += 1;
        }
    }

    println!("The amount of save sequences is: {}", safe_count);

    Ok(())
}

fn is_safe(numbers: Vec<i32>) -> bool {
    let first_difference = numbers[1] - numbers[0];
    for i in 0..numbers.len() - 1 {
        let difference = numbers[i + 1] - numbers[i];
        if (difference.abs() < 1 || difference.abs() > 3)
            || (first_difference > 0 && difference < 0)
            || (first_difference < 0 && difference > 0)
        {
            return false;
        }
    }

    true
}
