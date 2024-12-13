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

        if is_safe(number_line.clone(), false) {
            safe_count += 1;
        }
    }

    println!("The amount of save sequences is: {}", safe_count);

    Ok(())
}

fn is_safe(numbers: Vec<i32>, problem: bool) -> bool {
    let first_difference = numbers[1] - numbers[0];
    for i in 0..numbers.len() - 1 {
        let difference = numbers[i + 1] - numbers[i];
        if (difference.abs() < 1 || difference.abs() > 3)
            || (first_difference > 0 && difference < 0)
            || (first_difference < 0 && difference > 0)
        {
            if problem {
                return false;
            }

            if i > 0 {
                let mut numbers_zero = numbers.clone();
                numbers_zero.remove(i - 1);
                if is_safe(numbers_zero, true) {
                    return true;
                }
            }

            let mut numbers_one = numbers.clone();
            numbers_one.remove(i);

            if is_safe(numbers_one, true) {
                return true;
            }

            let mut numbers_two = numbers.clone();
            numbers_two.remove(i + 1);

            if is_safe(numbers_two, true) {
                return true;
            }

            return false;
        }
    }

    true
}
