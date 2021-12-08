use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn number_of_increases(measurements: &[i32]) -> i32 {
    let mut increases = 0i32;
    for i in 1..measurements.len() {
        if measurements[i] > measurements[i-1] {
            increases += 1;
        }
    }
    increases
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>> where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

fn read_i32_lines<P>(filename: P) -> Vec<i32> where P: AsRef<Path>, {
    read_lines(filename).map(|line| line.unwrap().parse::<i32>().unwrap()).collect()
}

#[test]
fn check_part_1_example() {
    let input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(number_of_increases(&input), 7);
}

#[test]
fn check_part_1() {
    let input = read_i32_lines("input.txt");
    assert_eq!(number_of_increases(&input), 1557);
}
