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

    println!("[Test] 230 = {}", solve(&test_values, test_bit_length));

    if let Ok(live_data) = read_lines("challenges/day_3/input/input.txt") {
        let raw_data: Vec<String> = live_data.map(|line| line.unwrap()).collect();
        let live_values: Vec<u64> = raw_data
            .iter()
            .map(|s| usize::from_str_radix(&s, 2).unwrap() as u64)
            .collect();
        let live_bit_length = raw_data[0].len();

        println!("[PartTwo] {}", solve(&live_values, live_bit_length));
    }
}

fn solve(data: &[u64], bit_length: usize) -> u64 {
    let mut ox_mask: u64 = 0;
    let mut ox_reading = 0;
    let mut ox_data: Vec<u64> = data.to_vec();
    for i in (0..bit_length).rev() {
        let bit_count = bit_count_at(&ox_data, i);
        if bit_count >= ox_data.len() - bit_count {
            ox_mask |= 1 << i;
        } else {
            ox_mask &= !(1 << i);
        }

        let values: Vec<u64> = ox_data.iter().filter(|&&v| {
            v >> i == ox_mask >> i
        }).copied().collect();

        if values.len() == 1 {
            ox_reading = *values.first().unwrap();
            break;
        }

        ox_data = values;
    }

    let mut co_mask: u64 = 0;
    let mut co_reading = 0;
    let mut co_data = data.to_vec();
    for i in (0..bit_length).rev() {
        let bit_count = bit_count_at(&co_data, i);
        if bit_count >= co_data.len() - bit_count {
            co_mask &= !(1 << i);
        } else {
            co_mask |= 1 << i;
        }

        let values: Vec<u64> = co_data.iter().filter(|&&v| {
            v >> i == co_mask >> i
        }).copied().collect();
        
        if values.len() == 1 {
            co_reading = *values.first().unwrap();
            break;
        }

        co_data = values;
    }

    ox_reading * co_reading
}

fn bit_count_at(data: &[u64], bit_idx: usize) -> usize {
    data.iter().fold(0, |mut acc, item| {
        acc += *item >> bit_idx & 1;
        acc
    }) as usize
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
