use std::{
    fs::File,
    io::{self, BufRead},
};

pub(crate) fn main() -> io::Result<()> {
    let file = File::open("./advent_of_code_2024/src/day_4/input.txt")?;
    let reader = io::BufReader::new(file);

    let matrix: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let mut count = 0;

    for (y, row) in matrix.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            count += xmas_hor(row, x);
            count += xmas_vert(&matrix, x, y);

            count += xmas_hor_right(&matrix, x, y);
            count += xmas_hor_left(&matrix, x, y);
        }
    }

    println!("The XMAS count is: {}", count);

    Ok(())
}

fn has_xmas<'a, I>(matrix: I) -> usize
where
    I: Iterator<Item = &'a char>,
{
    let col: String = matrix.collect();
    let mut count = 0;
    if col.contains("XMAS") {
        count += 1;
    }
    if col.contains("SAMX") {
        count += 1;
    }
    count
}

fn xmas_hor(row: &[char], x: usize) -> usize {
    if let Some(slice) = row.get(x..=x + 3) {
        return has_xmas(slice.iter());
    }
    0
}

fn xmas_vert(matrix: &[Vec<char>], x: usize, y: usize) -> usize {
    if let Some(slice) = matrix.get(y..=y + 3) {
        return has_xmas(slice.iter().map(|row| &row[x]));
    }
    0
}

fn xmas_hor_right(matrix: &[Vec<char>], x: usize, y: usize) -> usize {
    if let (Some(_), Some(slice)) = (matrix.get(x + 3), matrix.get(y..=y + 3)) {
        return has_xmas(slice.iter().enumerate().map(|(i, row)| &row[x + i]));
    }
    0
}

fn xmas_hor_left(matrix: &[Vec<char>], x: usize, y: usize) -> usize {
    if x < 3 {
        return 0;
    }
    if let (Some(_), Some(slice)) = (matrix.get(x - 3), matrix.get(y..=y + 3)) {
        return has_xmas(slice.iter().enumerate().map(|(i, row)| &row[x - i]));
    }
    0
}
