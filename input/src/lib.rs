use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>> where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

pub fn read_string_lines<P>(filename: P) -> Vec<String> where P: AsRef<Path>, {
    read_lines(filename).map(|line| line.unwrap()).collect()
}

pub fn read_i32_lines<P>(filename: P) -> Vec<i32> where P: AsRef<Path>, {
    read_lines(filename).map(|line| line.unwrap().parse::<i32>().unwrap()).collect()
}
