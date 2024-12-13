use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

pub(crate) fn main(input: &str) -> io::Result<()> {
    let file = File::open(input)?;
    let reader = io::BufReader::new(file);

    let mut nodes: HashMap<char, Vec<Point>> = HashMap::new();

    let mut max_y = 0;
    let mut max_x = 0;

    for (y, line) in reader.lines().enumerate() {
        let line = line?;

        max_y += 1;
        max_x = line.len() as i32;

        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                nodes.entry(c).or_insert_with(Vec::new).push(Point {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }

    let mut unique_locations: HashMap<Point, bool> = HashMap::new();

    for node in nodes.iter() {
        let pairs = get_all_pairs(&node.1);
        for pair in pairs {
            let x_diff_one = pair.0.x - pair.1.x;
            let y_diff_one = pair.0.y - pair.1.y;

            if pair.0.x + x_diff_one >= 0
                && pair.0.x + x_diff_one < max_x
                && pair.0.y + y_diff_one >= 0
                && pair.0.y + y_diff_one < max_y
            {
                unique_locations.insert(
                    Point {
                        x: pair.0.x + x_diff_one,
                        y: pair.0.y + y_diff_one,
                    },
                    true,
                );
            }

            let x_diff_two = pair.1.x - pair.0.x;
            let y_diff_two = pair.1.y - pair.0.y;
            if pair.1.x + x_diff_two >= 0
                && pair.1.x + x_diff_two < max_x
                && pair.1.y + y_diff_two >= 0
                && pair.1.y + y_diff_two < max_y
            {
                unique_locations.insert(
                    Point {
                        x: pair.1.x + x_diff_two,
                        y: pair.1.y + y_diff_two,
                    },
                    true,
                );
            }
        }
    }

    println!(
        "The sum of the possible spots is: {:?}",
        unique_locations.len()
    );

    Ok(())
}

fn get_all_pairs<T: Clone>(arr: &[T]) -> Vec<(T, T)> {
    let mut pairs = Vec::new();

    for i in 0..arr.len() {
        for j in i + 1..arr.len() {
            pairs.push((arr[i].clone(), arr[j].clone()));
        }
    }

    pairs
}
