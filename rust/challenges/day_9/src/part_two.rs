use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use itertools::Itertools;

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

    println!("[Test] 1134 = {}", solve(&test_values));

    if let Ok(live_data) = read_lines("challenges/day_9/input/input.txt") {
        let live_values: Vec<Vec<u64>> = live_data
            .map(|line| line.unwrap())
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u64)
                    .collect()
            })
            .collect();

        println!("[PartTwo] {}", solve(&live_values));
    }
}

fn solve(data: &Vec<Vec<u64>>) -> u64 {
    let low_points = find_low_points(data);

    let mut basins = vec![];
    for low_point in low_points {
        let basin_positions = find_basin_positions(data, low_point);
        basins.push(basin_positions.len());
    }

    basins
        .into_iter()
        .sorted_by(|a, b| Ord::cmp(b, a))
        .take(3)
        .map(|x| x as u64)
        .product()
}

fn find_basin_positions(data: &Vec<Vec<u64>>, low_point: (usize, usize)) -> Vec<(usize, usize)> {
    let mut visited = HashSet::new();
    let mut candidates = vec![];
    let mut basin_positions = vec![];
    let x_max = data.len();
    let y_max = data[0].len();

    visited.insert(low_point);
    basin_positions.push(low_point);
    let mut surrounding: Vec<(usize, usize)> = get_surrounding_pos(low_point)
        .into_iter()
        .filter(|(x, y)| *x < x_max && *y < y_max)
        .collect();
    candidates.append(&mut surrounding);

    while let Some(candidate) = candidates.pop() {
        let (x, y) = candidate;
        let value = data[x][y];

        if !visited.insert(candidate) || value == 9 {
            continue;
        }

        basin_positions.push(candidate);

        candidates.append(
            &mut get_surrounding_pos(candidate)
                .into_iter()
                .filter(|(x, y)| *x < x_max && *y < y_max)
                .collect(),
        );
    }

    basin_positions
}

fn get_surrounding_pos(pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut positions = vec![];
    let (x, y) = pos;
    if x >= 1 {
        positions.push((x - 1, y));
    }
    if y >= 1 {
        positions.push((x, y - 1));
    }

    positions.push((x + 1, y));
    positions.push((x, y + 1));

    positions
}

fn find_low_points(data: &Vec<Vec<u64>>) -> Vec<(usize, usize)> {
    let mut low_point_positions = vec![];
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
                low_point_positions.push((i, j));
            }
        }
    }

    low_point_positions
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
