use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let test_data = vec!["16,1,2,0,4,2,7,1,2,14"];
    let test_values: Vec<i64> = test_data
        .first()
        .unwrap()
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    println!("[Test] 37 = {}", solve(&test_values));

    if let Ok(mut live_data) = read_lines("challenges/day_7/input/input.txt") {
        let live_values: Vec<i64> = live_data
            .next()
            .unwrap()
            .unwrap()
            .split(",")
            .map(|line| line.parse::<i64>().unwrap())
            .collect();

        println!("[PartOne] {}", solve(&live_values));
    }
}

fn solve(data: &[i64]) -> i64 {
    let mut sorted = data.to_vec();
    sorted.sort();
    let mid_value = sorted[sorted.len() / 2];

    let fuel_cost = sorted.iter().map(|&v| (v - mid_value).abs()).sum();
    fuel_cost
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
