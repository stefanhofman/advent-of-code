use std::{
    collections::VecDeque,
    fs::File,
    io::{self, BufRead},
};

pub(crate) fn main() -> io::Result<()> {
    let file = File::open("./advent_of_code_2024/src/day_10/input.txt")?;
    let reader = io::BufReader::new(file);

    let mut current_mem: VecDeque<i64> = VecDeque::new();

    for line in reader.lines() {
        let line = line?;
        let mut is_file = true;
        let mut index = 0;
        line.chars().for_each(|c| {
            if is_file {
                current_mem.extend(vec![index; c.to_string().parse::<usize>().unwrap()]);
                index += 1;
            } else {
                current_mem.extend(vec![-1; c.to_string().parse::<usize>().unwrap()]);
            }
            is_file = !is_file;
        });
    }

    let filled_mem: Vec<i64> = current_mem.iter().copied().collect();

    let mut stop = filled_mem.len();
    let final_mem: Vec<i64> = filled_mem
        .iter()
        .enumerate()
        .map(|(i, c)| {
            if c == &-1 {
                while let Some(last) = current_mem.pop_back() {
                    if last != -1 {
                        stop -= 1;
                        return last * i as i64;
                    }
                }
            }
            return c * i as i64;
        })
        .collect();

    let final_sum: i64 = final_mem.iter().take(stop).sum();
    println!("{:?}", final_sum);

    Ok(())
}
