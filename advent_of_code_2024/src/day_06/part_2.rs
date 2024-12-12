use std::{
    collections::HashMap,
    fs::File,
    hash::Hash,
    io::{self, BufRead, Write},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    fn rotate(&self) -> Self {
        match self {
            Position::Up { x, y } => Position::Right { x: *x, y: *y },
            Position::Down { x, y } => Position::Left { x: *x, y: *y },
            Position::Left { x, y } => Position::Up { x: *x, y: *y },
            Position::Right { x, y } => Position::Down { x: *x, y: *y },
        }
    }

    fn move_pos(&self, matrix: &Vec<Vec<char>>) -> Self {
        match self {
            Position::Up { x, y } => {
                if matrix[y - 1][*x] == '#' {
                    self.rotate()
                } else {
                    Position::Up { x: *x, y: y - 1 }
                }
            }
            Position::Down { x, y } => {
                if matrix[y + 1][*x] == '#' {
                    self.rotate()
                } else {
                    Position::Down { x: *x, y: y + 1 }
                }
            }
            Position::Left { x, y } => {
                if matrix[*y][x - 1] == '#' {
                    self.rotate()
                } else {
                    Position::Left { x: x - 1, y: *y }
                }
            }
            Position::Right { x, y } => {
                if matrix[*y][x + 1] == '#' {
                    self.rotate()
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
    let mut stdout = io::stdout();
    let file = File::open("./advent_of_code_2024/src/day_06/input.txt")?;
    let reader = io::BufReader::new(file);

    let mut explored: HashMap<Position, bool> = HashMap::new();
    let mut possible_tiles: HashMap<(usize, usize), bool> = HashMap::new();
    let mut position: Option<Position> = None;

    let mut matrix: Vec<Vec<char>> = reader
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let line = line.expect("Invalid line");
            if let Some(pos) = line.chars().position(|c| "^v<>".contains(c)) {
                position = Some(Position::new(pos, i, line.chars().nth(pos).unwrap()));
            }
            line.chars().collect()
        })
        .collect();

    let mut position = position.expect("No position found");
    let original_position = position.clone();

    // Get all tiles that the guard goes to
    while position.can_go_next(&matrix) {
        position = position.move_pos(&matrix);
        possible_tiles.insert(position.get_coords(), true);
    }

    // Remove the starting position
    possible_tiles.remove(&original_position.get_coords());

    let mut loop_count = 0;
    let original_matrix = matrix.clone();
    for (i, possible_tile) in possible_tiles.keys().enumerate() {
        print!("\r{}/{}", i, possible_tiles.len() - 1);
        stdout.flush().unwrap();

        // Reset position, matrix, and explored
        position = original_position.clone();
        matrix = original_matrix.clone();
        explored.clear();

        matrix[possible_tile.1][possible_tile.0] = '#';
        explored.insert(position.clone(), true);

        while position.can_go_next(&matrix) {
            position = position.move_pos(&matrix);
            if let Some(_) = explored.get(&position) {
                loop_count += 1;
                break;
            }
            explored.insert(position.clone(), true);
        }
    }
    println!();
    println!("The number of loop options is: {}", loop_count);

    Ok(())
}
