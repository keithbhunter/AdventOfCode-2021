use input;
use std::fmt;
use std::path::Path;

#[derive(Copy, Clone, Debug)]
struct Square {
    number: usize,
    marked: bool,
}

fn new_square(number: usize) -> Square {
    Square {
        number,
        marked: false,
    }
}

#[derive(Debug)]
struct Board {
    squares: [[Square; 5]; 5],
}

impl Board {
    fn new(rows: Vec<Vec<usize>>) -> Option<Board> {
        if rows.len() != 5 {
            return None;
        }
        let mut squares: [[Square; 5]; 5] = [[new_square(0); 5]; 5];
        for (row, row_values) in rows.iter().enumerate() {
            if row_values.len() != 5 {
                return None;
            }
            for (col, col_value) in row_values.iter().enumerate() {
                squares[row][col] = new_square(*col_value);
            }
        }
        Some(Board { squares })
    }

    fn mark(&mut self, number: usize) {
        for row in 0..self.squares.len() {
            for col in 0..self.squares.len() {
                if self.squares[row][col].number == number {
                    self.squares[row][col].marked = true;
                }
            }
        }
    }

    fn has_bingo(&self) -> bool {
        for row in 0..self.squares.len() {
            if self.squares[row]
                .iter()
                .fold(true, |acc, sq| acc && sq.marked)
            {
                return true;
            }
        }

        for col in 0..self.squares.len() {
            if self
                .squares
                .iter()
                .fold(true, |acc, row| acc && row[col].marked)
            {
                return true;
            }
        }

        false
    }

    fn sum_of_unmarked_squares(&self) -> usize {
        let mut sum = 0;
        for row in self.squares {
            for square in row {
                if !square.marked {
                    sum += square.number;
                }
            }
        }
        sum
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.squares {
            for square in row {
                if square.marked {
                    write!(f, "X ");
                } else {
                    write!(f, "{} ", square.number);
                }
            }
            write!(f, "\n");
        }
        Ok(())
    }
}

fn build_bingo_board<P>(filename: P) -> (Vec<usize>, Vec<Board>)
where
    P: AsRef<Path>,
{
    let lines = input::read_lines(filename);
    let mut lines_iter = lines.iter().peekable();
    let borrowed_iter = lines_iter.by_ref();

    let numbers: Vec<usize> = borrowed_iter
        .next()
        .unwrap()
        .split(",")
        .map(|i| usize::from_str_radix(i, 10).unwrap())
        .collect();
    let mut boards: Vec<Board> = Vec::new();

    while let Some(_) = borrowed_iter.peek() {
        if let Some(next) = borrowed_iter.peek() {
            if *next == "" {
                borrowed_iter.next();
            }
        }

        let board_lines: Vec<&String> = borrowed_iter
            .take_while(|line| line.as_str() != "")
            .collect();
        let board_numbers: Vec<Vec<usize>> = board_lines
            .iter()
            .map(|line| {
                line.trim()
                    .split(" ")
                    .filter(|c| *c != "")
                    .map(|i| usize::from_str_radix(i, 10).unwrap())
                    .collect()
            })
            .collect();
        boards.push(Board::new(board_numbers).unwrap());
    }

    (numbers, boards)
}

fn play_bingo(numbers: &[usize], boards: &mut [Board]) -> usize {
    for number in numbers {
        for board in &mut *boards {
            board.mark(*number);
            if board.has_bingo() {
                return board.sum_of_unmarked_squares() * number;
            }
        }
    }

    panic!("should've found a board");
}

fn play_bingo_until_last_board(numbers: &[usize], boards: Vec<Board>) -> usize {
    let mut remaining = boards;

    for number in numbers {
        remaining = remaining
            .into_iter()
            .filter(|board| !board.has_bingo())
            .collect();

        for board in remaining.iter_mut() {
            board.mark(*number);
        }

        if remaining.len() == 1 && remaining.get(0).unwrap().has_bingo() {
            return remaining.get(0).unwrap().sum_of_unmarked_squares() * number;
        }
    }
    panic!("should've found a board");
}

fn main() {
    let (numbers, mut boards) = build_bingo_board("input.txt");
    println!("part 1 {}", play_bingo(&numbers, &mut *boards));

    let (numbers, boards) = build_bingo_board("input.txt");
    println!("part 2 {}", play_bingo_until_last_board(&numbers, boards));
}

#[test]
fn check_part_1_example() {
    let (numbers, mut boards) = build_bingo_board("example.txt");
    assert_eq!(play_bingo(&numbers, &mut *boards), 4512);
}

#[test]
fn check_part_2_example() {
    let (numbers, boards) = build_bingo_board("example.txt");
    assert_eq!(play_bingo_until_last_board(&numbers, boards), 1924);
}
