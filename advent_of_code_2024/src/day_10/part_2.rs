use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Vec<usize>>,
}

pub(crate) fn main(input: &str) -> io::Result<()> {
    let file = File::open(input)?;
    let reader = io::BufReader::new(file);

    let mut numbers: Vec<Node> = Vec::new();
    let mut starting_points: Vec<usize> = Vec::new();

    for (y, line) in reader.lines().enumerate() {
        let line = line?;

        line.chars().into_iter().enumerate().for_each(|(x, n)| {
            let value = n.to_digit(10).unwrap() as i32;
            numbers.push(Node { value, next: None });

            if value == 0 {
                starting_points.push(get_index(x, y, line.len()));
            }

            check_neighbours(&mut numbers, x, y, line.len());
        });
    }

    let mut sum = 0;
    starting_points.iter().for_each(|sp| {
        walk_path(&numbers[*sp], &numbers, &mut sum);
    });

    println!("There are {:?} trails", sum);

    Ok(())
}

fn walk_path<'a>(node: &Node, numbers: &Vec<Node>, count: &'a mut i32) -> &'a i32 {
    if node.value == 9 {
        *count += 1;
        return count;
    }

    if let Some(next) = &node.next {
        for n in next {
            walk_path(&numbers[*n], numbers, count);
        }
    }

    return count;
}

fn check_neighbours(numbers: &mut Vec<Node>, x: usize, y: usize, length: usize) {
    let current_index = get_index(x, y, length);
    let current_value = numbers[current_index].value;

    // Collect indices to update
    let mut updates = Vec::new();

    // check left
    if x > 0 {
        let left_index = get_index(x - 1, y, length);
        let left_value = numbers[left_index].value;
        if current_value - left_value == 1 {
            updates.push((left_index, current_index));
        }
        if left_value - current_value == 1 {
            updates.push((current_index, left_index));
        }
    }

    // check up
    if y > 0 {
        let up_index = get_index(x, y - 1, length);
        let up_value = numbers[up_index].value;
        if current_value - up_value == 1 {
            updates.push((up_index, current_index));
        }
        if up_value - current_value == 1 {
            updates.push((current_index, up_index));
        }
    }

    // Apply updates
    for (from, to) in updates {
        if numbers[from].next.is_none() {
            numbers[from].next = Some(vec![to]);
        } else {
            numbers[from].next.as_mut().unwrap().push(to);
        }
    }
}

fn get_index(x: usize, y: usize, length: usize) -> usize {
    y * length + x
}
