use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    if let Ok(count) = part1() {
        println!("{}", count);
    }
    if let Ok(count) = part2() {
        println!("{}", count);
    }
}

fn part1() -> io::Result<i32> {
    let lines = read_lines()?;

    let count = lines
        .map_while(Result::ok)
        .filter(|line| {
            line.split_whitespace()
                .map(str::parse::<i32>)
                .collect::<Result<Vec<_>, _>>()
                .map_or(false, |row| is_safe(&row))
        })
        .count();

    Ok(count as i32)
}

fn part2() -> io::Result<i32> {
    let lines = read_lines()?;

    let count = lines
        .map_while(Result::ok)
        .filter(|line| {
            line.split_whitespace()
                .map(str::parse::<i32>)
                .collect::<Result<Vec<_>, _>>()
                .map_or(false, |row| {
                    (0..row.len()).any(|i| {
                        let new_row = remove_element(&row, i);
                        is_safe(&new_row)
                    })
                })
        })
        .count();

    Ok(count as i32)
}

fn is_safe(line: &[i32]) -> bool {
    match line.first().zip(line.get(1)) {
        Some((first, second)) => {
            let increasing = first < second;
            line.windows(2).all(|w| {
                if increasing {
                    w[0] < w[1] && w[1] - w[0] <= 3
                } else {
                    w[0] > w[1] && w[0] - w[1] <= 3
                }
            })
        }
        None => true,
    }
}

fn remove_element(line: &[i32], index: usize) -> Vec<i32> {
    let mut new_line = line.to_vec();
    if index < new_line.len() {
        new_line.remove(index);
    }
    new_line
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open("./input.txt")?;
    Ok(io::BufReader::new(file).lines())
}
