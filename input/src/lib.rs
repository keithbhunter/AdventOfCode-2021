use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> Vec<String> where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines.map(|line| line.unwrap()).collect()
}

pub fn read_i32_lines<P>(filename: P) -> Vec<i32> where P: AsRef<Path>, {
    read_lines(filename).iter().map(|line| line.parse::<i32>().unwrap()).collect()
}
