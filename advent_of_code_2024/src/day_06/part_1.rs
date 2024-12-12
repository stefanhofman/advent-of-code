use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug)]
enum Position {
    Up { x: usize, y: usize },
    Down { x: usize, y: usize },
    Left { x: usize, y: usize },
    Right { x: usize, y: usize },
}

impl Position {
    fn new(x: usize, y: usize, c: char) -> Self {
        match c {
            '^' => Position::Up { x, y },
            'v' => Position::Down { x, y },
            '<' => Position::Left { x, y },
            '>' => Position::Right { x, y },
            _ => panic!("Invalid character"),
        }
    }

    fn move_pos(&self, matrix: &Vec<Vec<char>>) -> Self {
        match self {
            Position::Up { x, y } => {
                if matrix[y - 1][*x] == '#' {
                    Position::Right { x: x + 1, y: *y }
                } else {
                    Position::Up { x: *x, y: y - 1 }
                }
            }
            Position::Down { x, y } => {
                if matrix[y + 1][*x] == '#' {
                    Position::Left { x: x - 1, y: *y }
                } else {
                    Position::Down { x: *x, y: y + 1 }
                }
            }
            Position::Left { x, y } => {
                if matrix[*y][x - 1] == '#' {
                    Position::Up { x: *x, y: y - 1 }
                } else {
                    Position::Left { x: x - 1, y: *y }
                }
            }
            Position::Right { x, y } => {
                if matrix[*y][x + 1] == '#' {
                    Position::Down { x: *x, y: y + 1 }
                } else {
                    Position::Right { x: x + 1, y: *y }
                }
            }
        }
    }

    fn can_go_next(&self, matrix: &Vec<Vec<char>>) -> bool {
        match self {
            Position::Up { y, .. } => y > &0,
            Position::Down { y, .. } => y < &(matrix.len() - 1),
            Position::Left { x, .. } => x > &0,
            Position::Right { x, .. } => x < &(matrix[0].len() - 1),
        }
    }

    fn get_coords(&self) -> (usize, usize) {
        match self {
            Position::Up { x, y } => (*x, *y),
            Position::Down { x, y } => (*x, *y),
            Position::Left { x, y } => (*x, *y),
            Position::Right { x, y } => (*x, *y),
        }
    }
}

pub(crate) fn main() -> io::Result<()> {
    let file = File::open("./advent_of_code_2024/src/day_06/input.txt")?;
    let reader = io::BufReader::new(file);

    let mut explored: HashMap<(usize, usize), bool> = HashMap::new();
    let mut position: Option<Position> = None;

    let matrix: Vec<Vec<char>> = reader
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let Ok(line) = line else {
                panic!("Invalid line");
            };
            if let Some(pos) = line
                .chars()
                .position(|c| c == '^' || c == 'v' || c == '<' || c == '>')
            {
                position = Some(Position::new(pos, i, line.chars().nth(pos).unwrap()));
            }
            line.chars().collect()
        })
        .collect();

    let Some(mut position) = position else {
        panic!("No position found");
    };

    explored.insert(position.get_coords(), true);

    while position.can_go_next(&matrix) {
        position = position.move_pos(&matrix);
        explored.insert(position.get_coords(), true);
    }

    println!("The number of explored tiles is: {}", explored.len());

    Ok(())
}
