use std::{
    collections::BinaryHeap,
    fs::File,
    io::{self, BufRead},
};

pub(crate) fn main() -> io::Result<()> {
    let file_contents = File::open("./advent_of_code_2024/src/day_1/input.txt")?;
    let reader = io::BufReader::new(file_contents);

    let mut heap_one: BinaryHeap<i32> = BinaryHeap::new();
    let mut heap_two: BinaryHeap<i32> = BinaryHeap::new();

    for line in reader.lines() {
        let number_line: Vec<i32> = line?
            .clone()
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        heap_one.push(number_line[0].clone());
        heap_two.push(number_line[1].clone());
    }

    let mut total_distance = 0;
    while let Some(element) = heap_one.pop() {
        total_distance += (element - heap_two.pop().unwrap()).abs();
    }

    println!("The total distance is: {}", total_distance);

    Ok(())
}
