use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let test_data = vec![
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];

    let test_values: Vec<u64> = test_data
        .iter()
        .map(|&s| usize::from_str_radix(s, 2).unwrap() as u64)
        .collect();
    let test_bit_length = test_data[0].len();

    println!("[Test] 198 = {}", solve(&test_values, test_bit_length));

    if let Ok(live_data) = read_lines("challenges/day_3/input/input.txt") {
        let raw_data: Vec<String> = live_data.map(|line| line.unwrap()).collect();
        let live_values: Vec<u64> = raw_data
            .iter()
            .map(|s| usize::from_str_radix(&s, 2).unwrap() as u64)
            .collect();
        let live_bit_length = raw_data[0].len();

        println!("[PartOne] {}", solve(&live_values, live_bit_length));
    }
}

fn solve(data: &[u64], bit_length: usize) -> u64 {
    let bit_counts: Vec<u64> = data.iter().fold(vec![0; bit_length], |mut acc, item| {
        for i in 0..bit_length {
            acc[i] += get_bit(*item, i);
        }
        acc
    });

    let majority = (data.len() / 2) as u64;

    let mut gamma: u64 = 0;
    let mut epsilon: u64 = 0;

    for i in 0..bit_length {
        if bit_counts[i] > majority {
            gamma |= 1 << i;
            epsilon &= !(1 << i);
        } else {
            gamma &= !(1 << i);
            epsilon |= 1 << i;
        }
    }

    gamma * epsilon
}

fn get_bit(n: u64, pos: usize) -> u64 {
    (n >> pos) & 1
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
