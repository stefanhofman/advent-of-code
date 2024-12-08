use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

pub(crate) fn main() -> io::Result<()> {
    let file_contents = File::open("./advent_of_code_2024/src/day_1/input.txt")?;
    let reader = io::BufReader::new(file_contents);

    let mut hash_map: HashMap<i32, i32> = HashMap::new();

    for line in reader.lines() {
        let number_line: Vec<i32> = line?
            .clone()
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        // If the value is negative, make it positive
        if let Some(value) = hash_map.get_mut(&number_line[0]) {
            if *value < 0 {
                *value = value.abs();
            }
        } else {
            // If the value is not in the hash map, add it
            hash_map.insert(number_line[0], 0);
        }

        // If the second number is in the hash map, increment the value by 1
        if let Some(value) = hash_map.get_mut(&number_line[1]) {
            if *value < 0 {
                *value = value.abs();
            }
            *value += 1;
        } else {
            // If the second number is not in the hash map, add it to the list as a negative number
            hash_map.insert(number_line[1], -1);
        }
    }

    let mut similarity_score = 0;

    for key in hash_map.keys() {
        let count = hash_map.get(key).unwrap();

        if *count > 0 {
            similarity_score += key * count;
        }
    }

    println!("The similarity score is: {}", similarity_score);

    Ok(())
}
