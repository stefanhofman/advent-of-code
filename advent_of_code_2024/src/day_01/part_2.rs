use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

pub(crate) fn main() -> io::Result<()> {
    let file = File::open("./advent_of_code_2024/src/day_01/input.txt")?;
    let reader = io::BufReader::new(file);

    let mut hash_map: HashMap<i32, i32> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse().expect("Failed to parse number"))
            .collect();

        if numbers.len() != 2 {
            continue; // Skip lines that don't have exactly two numbers
        }

        let (num1, num2) = (numbers[0], numbers[1]);

        // Update the first number in the hash map
        hash_map
            .entry(num1)
            .and_modify(|v| *v = v.abs())
            .or_insert(0);

        // Update the second number in the hash map
        hash_map
            .entry(num2)
            .and_modify(|v| {
                *v = v.abs();
                *v += 1;
            })
            .or_insert(-1);
    }

    let similarity_score: i32 = hash_map
        .iter()
        .filter(|(_, &count)| count > 0)
        .map(|(&key, &count)| key * count)
        .sum();

    println!("The similarity score is: {}", similarity_score);

    Ok(())
}
