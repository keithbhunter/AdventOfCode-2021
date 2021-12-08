use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn number_of_increases(measurements: &[i32]) -> i32 {
    let mut increases = 0i32;
    for i in 0..measurements.len()-3 {
        let first_window = &measurements[i..i+3];
        let second_window = &measurements[i+1..i+4];

        let first_sum = first_window.iter().fold(0, |sum, x| sum + x);
        let second_sum = second_window.iter().fold(0, |sum, x| sum + x);

        if second_sum > first_sum {
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
fn check_part_2_example() {
    let input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(number_of_increases(&input), 5);
}

#[test]
fn check_part_2() {
    let input = read_i32_lines("input.txt");
    assert_eq!(number_of_increases(&input), 1608);
}
