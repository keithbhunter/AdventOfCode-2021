use input;
use regex::Regex;
use std::collections::HashMap;
use std::error;
use std::path::Path;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }
}

#[derive(Debug, PartialEq)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(start: Point, end: Point) -> Line {
        Line { start, end }
    }

    fn parse(string: String) -> Result<Line, Box<dyn error::Error>> {
        let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
        for cap in re.captures_iter(&string) {
            if cap.len() == 5 {
                return Ok(Line::new(
                    Point::new(
                        isize::from_str_radix(&cap[1], 10)?,
                        isize::from_str_radix(&cap[2], 10)?,
                    ),
                    Point::new(
                        isize::from_str_radix(&cap[3], 10)?,
                        isize::from_str_radix(&cap[4], 10)?,
                    ),
                ));
            }
        }

        let error_string = format!("could not parse string: {}", string);
        let error: Box<dyn error::Error> = String::from(error_string).into();
        Err(error)
    }

    fn slope_components(&self) -> (isize, isize) {
        ((self.end.y - self.start.y), (self.end.x - self.start.x))
    }

    fn slope(&self) -> f64 {
        let (y_slope, x_slope) = self.slope_components();
        y_slope as f64 / x_slope as f64
    }

    fn is_horizontal(&self) -> bool {
        let (y_slope, _) = self.slope_components();
        y_slope == 0
    }

    fn is_vertical(&self) -> bool {
        let (_, x_slope) = self.slope_components();
        x_slope == 0
    }

    fn points(&self) -> Vec<Point> {
        let mut points: Vec<Point> = vec![self.start];
        let mut point = self.start;

        let mut increment = 1;
        if (self.is_vertical() && self.start.y > self.end.y)
            || (self.is_horizontal() && self.start.x > self.end.x)
        {
            increment = -1
        }

        while point != self.end {
            if self.is_horizontal() {
                point.x += increment;
                points.push(point);
            } else if self.is_vertical() {
                point.y += increment;
                points.push(point);
            }
        }

        points
    }
}

fn read_lines<P>(filename: P) -> Vec<Line>
where
    P: AsRef<Path>,
{
    let str_lines = input::read_lines(filename);
    str_lines
        .into_iter()
        .map(|line| Line::parse(line).unwrap())
        .collect()
}

fn count_overlaps(lines: Vec<Line>) -> usize {
    let mut counts: HashMap<Point, isize> = HashMap::new();
    for line in lines {
        if line.is_horizontal() || line.is_vertical() {
            for point in line.points() {
                counts
                    .entry(point)
                    .and_modify(|entry| *entry += 1)
                    .or_insert(1);
            }
        }
    }

    let (_, max_count) = counts
        .clone()
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .unwrap();
    counts
        .into_iter()
        .filter(|(_, count)| *count == max_count)
        .count()
}

fn main() {
    // 18 is wrong
    println!("{}", count_overlaps(read_lines("input.txt")))
}

#[test]
fn test_parse_coordinate_string() {
    let start = Point::new(0, 9);
    let end = Point::new(5, 9);
    let line = Line::parse(String::from("0,9 -> 5,9")).unwrap();
    assert_eq!(line, Line::new(start, end));

    let start = Point::new(63, 865);
    let end = Point::new(63, 407);
    let line = Line::parse(String::from("63,865 -> 63,407")).unwrap();
    assert_eq!(line, Line::new(start, end));
}

#[test]
fn test_slope() {
    let line = Line::parse(String::from("0,9 -> 5,9")).unwrap();
    assert_eq!(line.slope(), 0f64);

    let line = Line::parse(String::from("9,0 -> 9,5")).unwrap();
    assert_eq!(line.slope(), f64::INFINITY);

    let line = Line::parse(String::from("1,2 -> 2,4")).unwrap();
    assert_eq!(line.slope(), 2f64);

    let line = Line::parse(String::from("2,1 -> 4,2")).unwrap();
    assert_eq!(line.slope(), 0.5);
}

#[test]
fn test_is_horizontal() {
    let line = Line::parse(String::from("0,9 -> 5,9")).unwrap();
    assert_eq!(line.is_horizontal(), true);

    let line = Line::parse(String::from("9,0 -> 9,5")).unwrap();
    assert_eq!(line.is_horizontal(), false);
}

#[test]
fn test_is_vertical() {
    let line = Line::parse(String::from("9,0 -> 9,5")).unwrap();
    assert_eq!(line.is_vertical(), true);

    let line = Line::parse(String::from("0,9 -> 5,9")).unwrap();
    assert_eq!(line.is_vertical(), false);
}

#[test]
fn test_points() {
    let line = Line::parse(String::from("0,9 -> 5,9")).unwrap();
    let points = vec![
        Point::new(0, 9),
        Point::new(1, 9),
        Point::new(2, 9),
        Point::new(3, 9),
        Point::new(4, 9),
        Point::new(5, 9),
    ];
    assert_eq!(line.points(), points);
}

#[test]
fn check_part_1_example() {
    assert_eq!(count_overlaps(read_lines("example.txt")), 5);
}
