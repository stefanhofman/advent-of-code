use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{self, BufRead},
};

pub(crate) fn main(input: &str) -> io::Result<()> {
    let file = File::open(input)?;
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
            let original_vec = line.split(',').collect::<Vec<&str>>();
            if is_valid(&line, &dependencies) {
                if let Some(mid_element) = original_vec.get(original_vec.len() / 2) {
                    if let Ok(parsed_value) = mid_element.parse::<i32>() {
                        sum += parsed_value;
                    }
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
