use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let test_data = vec![
        "199", "200", "208", "210", "200", "207", "240", "269", "260", "263",
    ];
    let test_values: Vec<u64> = test_data
        .iter()
        .map(|&s| s.parse::<u64>().unwrap())
        .collect();

    println!("[Test] 7 = {}", solve(&test_values));

    if let Ok(live_data) = read_lines("challenges/day_1/input/input.txt") {
        let live_values: Vec<u64> = live_data
            .map(|line| line.unwrap().parse::<u64>().unwrap())
            .collect();

        println!("[PartOne] {}", solve(&live_values));
    }
}

fn solve(data: &[u64]) -> u64 {
    data.windows(2).filter(|vs| vs[1] > vs[0]).count() as u64
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
