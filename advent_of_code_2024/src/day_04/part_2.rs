use std::{
    fs::File,
    io::{self, BufRead},
};

pub(crate) fn main(input: &str) -> io::Result<()> {
    let file = File::open(input)?;
    let reader = io::BufReader::new(file);

    let matrix: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let count = matrix.iter().enumerate().fold(0, |acc, (y, row)| {
        acc + row
            .iter()
            .enumerate()
            .filter(|&(x, _)| mas_hor_right(&matrix, x, y) && mas_hor_left(&matrix, x + 2, y))
            .count()
    });

    println!("The X-MAS count is: {}", count);

    Ok(())
}

fn has_xmas<'a, I>(matrix: I) -> bool
where
    I: Iterator<Item = &'a char>,
{
    let col: String = matrix.collect();
    return col.contains("MAS") || col.contains("SAM");
}

fn mas_hor_right(matrix: &[Vec<char>], x: usize, y: usize) -> bool {
    if let (Some(_), Some(slice)) = (matrix.get(x + 2), matrix.get(y..=y + 2)) {
        return has_xmas(slice.iter().enumerate().map(|(i, row)| &row[x + i]));
    }
    false
}

fn mas_hor_left(matrix: &[Vec<char>], x: usize, y: usize) -> bool {
    if x < 2 {
        return false;
    }
    if let Some(slice) = matrix.get(y..=y + 2) {
        return has_xmas(slice.iter().enumerate().map(|(i, row)| &row[x - i]));
    }
    false
}
