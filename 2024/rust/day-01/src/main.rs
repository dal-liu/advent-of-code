use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    if let Ok(dist) = part1() {
        println!("{}", dist);
    }

    if let Ok(dist) = part2() {
        println!("{}", dist);
    }
}

fn part1() -> io::Result<i32> {
    let lines = read_lines()?;

    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for line in lines.map_while(Result::ok) {
        let line: Vec<&str> = line.split_whitespace().collect();
        vec1.push(line[0].parse::<i32>().unwrap());
        vec2.push(line[1].parse::<i32>().unwrap());
    }

    vec1.sort();
    vec2.sort();

    Ok(vec1
        .iter()
        .zip(vec2.iter())
        .map(|(x, y)| (x - y).abs())
        .sum::<i32>())
}

fn part2() -> io::Result<i32> {
    let lines = read_lines()?;

    let mut vec = Vec::new();
    let mut hash_map = HashMap::new();

    for line in lines.map_while(Result::ok) {
        let line: Vec<&str> = line.split_whitespace().collect();
        vec.push(line[0].parse::<i32>().unwrap());

        let key = line[1].parse::<i32>().unwrap();
        if let Some(count) = hash_map.get_mut(&key) {
            *count += 1;
        } else {
            hash_map.insert(key, 1);
        }
    }

    Ok(vec
        .iter()
        .map(|x| {
            if let Some(count) = hash_map.get(&x) {
                *count * x
            } else {
                0
            }
        })
        .sum::<i32>())
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open("./input.txt")?;
    Ok(io::BufReader::new(file).lines())
}
