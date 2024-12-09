use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{self, BufRead},
};

pub(crate) fn main() -> io::Result<()> {
    let file = File::open("./advent_of_code_2024/src/day_5/input.txt")?;
    let reader = io::BufReader::new(file);

    let mut dependencies: HashMap<String, Vec<String>> = HashMap::new();

    let mut read_pages = false;
    let mut sum = 0;

    for line in reader.lines() {
        let line = line?;
        if line == "" {
            read_pages = true;
            continue;
        }

        if !read_pages {
            if let [first, second] = line.split('|').collect::<Vec<&str>>()[..] {
                dependencies
                    .entry(second.to_string().clone())
                    .or_default()
                    .push(first.to_string().clone());
            } else {
                eprintln!("Unexpected number of elements in line: {}", line);
            }
        } else {
            if !is_valid(&line, &dependencies) {
                let (_, original_vec) = re_order_and_validate(&line, &dependencies);
                if let Some(mid_element) = original_vec.get(original_vec.len() / 2) {
                    sum += mid_element;
                }
            }
        }
    }

    println!("The sum of the valid pages is: {}", sum);

    Ok(())
}

fn is_valid(line: &str, dependencies: &HashMap<String, Vec<String>>) -> bool {
    let mut line = line.split(',').collect::<VecDeque<&str>>();
    while let Some(page) = line.pop_front() {
        if let Some(page_dependencies) = dependencies.get(page) {
            if line
                .iter()
                .any(|item| page_dependencies.contains(&item.to_string()))
            {
                return false;
            }
        }
    }
    true
}

fn re_order_and_validate(
    line: &str,
    dependencies: &HashMap<String, Vec<String>>,
) -> (bool, Vec<i32>) {
    let mut new_line: Vec<i32> = Vec::new();
    let mut skip: Vec<i32> = Vec::new();

    let mut line = line.split(',').collect::<VecDeque<&str>>();
    while let Some(page) = line.pop_front() {
        if let Some(page_dependencies) = dependencies.get(page) {
            let deps: Vec<i32> = line
                .iter()
                .filter(|item| page_dependencies.contains(&item.to_string()))
                .map(|item| item.parse::<i32>().unwrap())
                .collect();
            if deps.len() > 0 {
                skip.push(page.parse::<i32>().unwrap());
            } else {
                new_line.push(page.parse::<i32>().unwrap());
                new_line = [new_line, skip.clone()].concat();
                skip = Vec::new();
            }
        } else {
            new_line.push(page.parse::<i32>().unwrap());
            new_line = [new_line, skip.clone()].concat();
        }
    }

    let string_line = new_line
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .join(",");

    if is_valid(&string_line, dependencies) {
        return (true, new_line);
    }

    return re_order_and_validate(&string_line, dependencies);
}
