use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let test_data = vec!["3,4,3,1,2"];
    let test_values: Vec<u64> = test_data.first().unwrap()
        .split(",").map(|s| s.parse::<u64>().unwrap()).collect();
    
    println!("[Test] 26 = {}", solve(&test_values, 18));
    println!("[Test] 5934 = {}", solve(&test_values, 80));

    if let Ok(mut live_data) = read_lines("challenges/day_6/input/input.txt") {
        let live_values: Vec<u64> = live_data.next().unwrap().unwrap().split(",")
            .map(|line| line.parse::<u64>().unwrap())
            .collect();
        
        println!("[PartOne] {}", solve(&live_values, 80));
    }
}

fn solve(data: &[u64], days: u64) -> u64 {
    let mut counts: [u64;9] = [0;9];
    data.iter().for_each(|&v| {
        counts[v as usize] += 1;
    });
    for _day in 0..days {
        let to_grow = counts[0];
        counts[0..7].rotate_left(1);
        counts[6] += counts[7];
        counts[7..9].rotate_left(1);
        counts[8] = to_grow;
    }
    
    return counts.iter().sum()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}