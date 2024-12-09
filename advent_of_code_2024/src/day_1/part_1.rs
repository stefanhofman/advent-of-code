use std::{
    collections::BinaryHeap,
    fs::File,
    io::{self, BufRead},
};

pub(crate) fn main() -> io::Result<()> {
    let file = File::open("./advent_of_code_2024/src/day_1/input.txt")?;
    let reader = io::BufReader::new(file);

    let mut heap_one = BinaryHeap::new();
    let mut heap_two = BinaryHeap::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse().expect("Failed to parse number"))
            .collect();

        if let [first, second] = numbers[..] {
            heap_one.push(first);
            heap_two.push(second);
        } else {
            eprintln!("Unexpected number of elements in line: {}", line);
        }
    }

    let mut total_distance = 0;
    while let (Some(element_one), Some(element_two)) = (heap_one.pop(), heap_two.pop()) {
        total_distance += (element_one - element_two).abs();
    }

    println!("The total distance is: {}", total_distance);

    Ok(())
}
