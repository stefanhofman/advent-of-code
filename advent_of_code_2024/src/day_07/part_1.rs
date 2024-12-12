use std::{
    fs::File,
    io::{self, BufRead},
};

macro_rules! calculate {
    ($numbers:expr, $operators:expr) => {{
        let mut result = $numbers[0];
        for i in 0..$operators.len() {
            match $operators[i] {
                "+" => result += $numbers[i + 1],
                "*" => result *= $numbers[i + 1],
                _ => panic!("Unsupported operator"),
            }
        }
        result
    }};
}

pub(crate) fn main() -> io::Result<()> {
    let file = File::open("./advent_of_code_2024/src/day_07/input.txt")?;
    let reader = io::BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        let line = line?;

        let mut parts = line.split(":").flat_map(|n| n.split_whitespace());
        let answer = parts.next().unwrap().parse::<i64>().unwrap();
        let numbers: Vec<i64> = parts.map(|n| n.parse::<i64>().unwrap()).collect();
        let mut operators = vec!["*"; numbers.len() - 1];

        if is_possible(&numbers, &mut operators, &answer) {
            sum += answer;
        }
    }

    println!("The sum of the possible equations is: {:?}", sum);

    Ok(())
}

fn is_possible(numbers: &[i64], operators: &mut [&str], answer: &i64) -> bool {
    while calculate!(numbers, operators) != *answer {
        if operators.iter().all(|o| *o == "+") {
            return false;
        }

        for operator in operators.iter_mut() {
            if *operator == "*" {
                *operator = "+";
                break;
            } else {
                *operator = "*";
            }
        }
    }
    true
}
