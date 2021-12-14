use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use std::string::ParseError;

use itertools::Itertools;

fn main() {
    let test_data = vec![
        "6,10",
        "0,14",
        "9,10",
        "0,3",
        "10,4",
        "4,11",
        "6,0",
        "6,12",
        "4,1",
        "0,13",
        "10,12",
        "3,4",
        "3,0",
        "8,4",
        "1,10",
        "2,14",
        "8,10",
        "9,0",
        "",
        "fold along y=7",
        "fold along x=5",
    ];
    let test_values: Manual = Manual::parse(&test_data);

    println!("[Test] 17 = {}", solve(&test_values));

    if let Ok(live_data) = read_lines("challenges/day_13/input/input.txt") {
        let raw_data: Vec<String> = live_data.map(|line| line.unwrap()).collect();
        let live_values: Manual = Manual::parse(&raw_data);

        println!("[PartOne] {}", solve(&live_values));
    }
}

fn solve(data: &Manual) -> usize {
    data.points
        .iter()
        .map(|p| p.translate(&data.folds[0]))
        .unique()
        .count()
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    fn new(x: u64, y: u64) -> Self {
        Point { x, y }
    }

    fn translate(&self, p: &Point) -> Point {
        match p {
            &Point { x: 0, y } => {
                if self.y < y {
                    self.clone()
                } else {
                    let dy = self.y - y;
                    Point {
                        x: self.x,
                        y: y - dy,
                    }
                }
            }
            &Point { x, y: 0 } => {
                if self.x > x {
                    self.clone()
                } else {
                    let dx = x - self.x;
                    Point {
                        x: x + dx,
                        y: self.y,
                    }
                }
            }
            _ => Point { x: 0, y: 0 },
        }
    }
}

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<u64> = s.split(",").map(|s| s.parse::<u64>().unwrap()).collect();

        Ok(Point {
            x: parts[0],
            y: parts[1],
        })
    }
}

#[derive(Debug, Clone)]
struct Manual {
    points: Vec<Point>,
    folds: Vec<Point>,
}

impl Manual {
    fn parse<S: AsRef<str>>(input: &[S]) -> Self {
        let mut points = vec![];
        let mut folds = vec![];
        let mut points_complete = false;
        for line in input {
            if points_complete {
                let instruction: Vec<&str> = line
                    .as_ref()
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .split("=")
                    .collect();
                let value = instruction[1].parse::<u64>().unwrap();
                if instruction[0] == "x" {
                    folds.push(Point::new(value, 0));
                } else {
                    folds.push(Point::new(0, value));
                }
            } else if !line.as_ref().is_empty() {
                points.push(line.as_ref().parse::<Point>().unwrap());
            } else {
                points_complete = true;
            }
        }

        Manual {
            points: points,
            folds: folds,
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
