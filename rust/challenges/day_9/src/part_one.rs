use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let test_data = vec![
        "2199943210",
        "3987894921",
        "9856789892",
        "8767896789",
        "9899965678",
    ];
    let test_values: Vec<Vec<u64>> = test_data
        .iter()
        .map(|&line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect()
        })
        .collect();

    println!("[Test] 15 = {}", solve(&test_values));

    if let Ok(live_data) = read_lines("challenges/day_9/input/input.txt") {
        let live_values: Vec<Vec<u64>> = live_data
            .map(|line| line.unwrap())
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u64)
                    .collect()
            })
            .collect();

        println!("[PartOne] {}", solve(&live_values));
    }
}

fn solve(data: &Vec<Vec<u64>>) -> u64 {
    let mut low_point_values = vec![];
    for i_idx in 0..data.len() as i64 {
        for j_idx in 0..data[i_idx as usize].len() as i64 {
            let (i, j) = (i_idx as usize, j_idx as usize);
            let val = data[i][j];

            let surrounding_pos = [
                (i_idx - 1, j_idx),
                (i_idx + 1, j_idx),
                (i_idx, j_idx - 1),
                (i_idx, j_idx + 1),
            ];

            let valid_positions: Vec<(usize, usize)> = surrounding_pos
                .into_iter()
                .filter(|(x, y)| {
                    x >= &0 && x < &(data.len() as i64) && y >= &0 && y < &(data[i].len() as i64)
                })
                .map(|(x, y)| (x as usize, y as usize))
                .collect();

            if valid_positions.iter().all(|(x, y)| data[*x][*y] > val) {
                low_point_values.push(val);
            }
        }
    }

    low_point_values.iter().map(|v| v + 1).sum()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
