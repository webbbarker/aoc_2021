use std::{str::FromStr, fmt, num::ParseIntError};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let test_data = vec![
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ];
    let test_values: Vec<Action> = test_data.iter()
        .map(|&s| s.parse().unwrap())
        .collect();

    println!("[Test] 150 = {}", solve(&test_values));

    if let Ok(live_data) = read_lines("challenges/day_2/input/input.txt") {
        let live_values: Vec<Action> = live_data
            .map(|line| line.unwrap().parse().unwrap())
            .collect();
        
        println!("[PartOne] {}", solve(&live_values));
    }
}

fn solve(data: &[Action]) -> u64 {
    let mut depth = 0;
    let mut pos = 0;

    for action in data {
        match action {
            Action::Forward(amt) => pos += amt,
            Action::Up(amt) => depth -= amt,
            Action::Down(amt) => depth += amt,
        }
    }

    depth * pos
}

enum Action {
    Forward(u64),
    Up(u64),
    Down(u64),
}

#[derive(Debug, Clone)]
struct ActionParseError;

impl fmt::Display for ActionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unable to parse action")
    }
}

impl From<ParseIntError> for ActionParseError {
    fn from(_: ParseIntError) -> Self {
        ActionParseError
    }
}

impl FromStr for Action {
    type Err = ActionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        let amount = parts[1].parse::<u64>()?;
        match parts[0] {
            "forward" => Ok(Action::Forward(amount)),
            "up" => Ok(Action::Up(amount)),
            "down" => Ok(Action::Down(amount)),
            _ => Err(ActionParseError),
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