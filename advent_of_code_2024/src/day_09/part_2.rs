use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(Clone, Copy, Debug)]
struct Mem {
    size: usize,
    id: Option<usize>,
    free: bool,
}

pub(crate) fn main() -> io::Result<()> {
    let file = File::open("./advent_of_code_2024/src/day_09/input.txt")?;
    let reader = io::BufReader::new(file);

    let mut current_mem: Vec<Mem> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut is_file = true;
        let mut index = 0;
        line.chars().for_each(|c| {
            if is_file {
                current_mem.push(Mem {
                    size: c.to_string().parse::<usize>().unwrap(),
                    id: Some(index),
                    free: false,
                });
                index += 1;
            } else {
                current_mem.push(Mem {
                    size: c.to_string().parse::<usize>().unwrap(),
                    id: None,
                    free: true,
                });
            }
            is_file = !is_file;
        });
    }

    let mut final_mem: Vec<Mem> = current_mem.iter().copied().collect();

    current_mem
        .iter_mut()
        .rev()
        .filter(|c| !c.free)
        .for_each(|c| {
            if let Some(spot) = final_mem.iter().position(|s| s.free && s.size >= c.size) {
                let fs = final_mem[spot].clone();
                let to_remove = final_mem.iter().position(|p| !p.free && p.id == c.id);
                if to_remove.unwrap() > spot {
                    final_mem[spot] = Mem {
                        size: c.size,
                        id: c.id,
                        free: false,
                    };
                    final_mem[to_remove.unwrap()] = Mem {
                        size: c.size,
                        id: None,
                        free: true,
                    };

                    if fs.size > c.size {
                        final_mem.insert(
                            spot + 1,
                            Mem {
                                size: fs.size - c.size,
                                id: None,
                                free: true,
                            },
                        );
                    }
                }
            }
        });

    let final_sum: i64 = final_mem
        .iter()
        .flat_map(|f| std::iter::repeat(f).take(f.size))
        .enumerate()
        .map(|(i, c)| {
            if !c.free {
                return c.id.unwrap() as i64 * i as i64;
            }
            return 0;
        })
        .sum();

    println!("{:?}", final_sum);

    Ok(())
}
