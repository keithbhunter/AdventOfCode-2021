use std::convert::From;
use input;

#[derive(Debug)]
enum Direction {
    Forward,
    Down,
    Up,
}

impl From<&str> for Direction {
    fn from(direction: &str) -> Self {
        match direction {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => panic!("unexpected direction {:?}", direction),
        }
    }
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    value: i32,
}

impl From<&str> for Command {
    fn from(item: &str) -> Self {
        let split: Vec<&str> = item.split(" ").collect();
        if split.len() != 2 {
            panic!("unexpected command format {:?}", item);
        }

        Command {
            direction: split[0].into(),
            value: split[1].parse::<i32>().unwrap(),
        }
    }
}

fn calculate_position(commands: &[String]) -> i32 {
    let commands: Vec<Command> = commands.iter().map(|command| Command::from(command.as_str())).collect();
    let mut horizontal = 0;
    let mut vertical = 0;

    for command in commands {
        match command.direction {
            Direction::Forward => horizontal += command.value,
            Direction::Down => vertical += command.value,
            Direction::Up => vertical -= command.value
        }
    }

    horizontal * vertical
}

fn main() {}

#[test]
fn check_part_1_example() {
    let input = [
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ].map(|s| s.to_string());
    assert_eq!(calculate_position(&input), 150);
}

#[test]
fn check_part_1() {
    let input = input::read_lines("input.txt");
    assert_eq!(calculate_position(&input), 1962940);
}
