use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for line in lines.map_while(Result::ok) {
        let line: Vec<&str> = line.split_whitespace().collect();
        vec1.push(line[0].parse::<i32>().unwrap());
        vec2.push(line[1].parse::<i32>().unwrap());
    }

    vec1.sort();
    vec2.sort();

    let dist = vec1
        .iter()
        .zip(vec2.iter())
        .map(|(x, y)| (x - y).abs())
        .sum::<i32>();

    println!("{}", dist);
    Ok(())
}
