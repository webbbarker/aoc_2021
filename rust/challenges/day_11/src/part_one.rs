use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let test_data = vec![
        "5483143223",
        "2745854711",
        "5264556173",
        "6141336146",
        "6357385478",
        "4167524645",
        "2176841721",
        "6882881134",
        "4846848554",
        "5283751526",
    ];
    let test_values: Vec<Vec<u64>> = test_data
        .iter()
        .map(|&line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect()
        })
        .collect();

    println!("[Test] 1656 = {}", solve(&test_values, 100));

    if let Ok(live_data) = read_lines("challenges/day_11/input/input.txt") {
        let live_values: Vec<Vec<u64>> = live_data
            .map(|line| line.unwrap())
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u64)
                    .collect()
            })
            .collect();

        println!("[PartOne] {}", solve(&live_values, 100));
    }
}

fn solve(data: &Vec<Vec<u64>>, step_count: u64) -> u64 {
    let mut grid = data.to_vec();
    let mut flashes: u64 = 0;
    for _ in 0..step_count {
        increment_all(&mut grid);

        let mut flashed = HashSet::new();
        while grid.iter().any(|row| row.iter().any(|&v| v > 9)) {
            let mut updates = vec![];
            for x in 0..10 {
                for y in 0..10 {
                    if grid[x][y] > 9 {
                        updates.append(&mut around((x, y)));
                        flashes += 1;

                        grid[x][y] = 0;
                        flashed.insert((x, y));
                    }
                }
            }

            updates
                .iter()
                .filter(|&pos| !flashed.contains(pos))
                .for_each(|&(x, y)| grid[x][y] += 1);
        }

        // if i == 1 {
        //     println!("{:?}", grid);
        //     break;
        // }
    }

    flashes
}

fn increment_all(grid: &mut Vec<Vec<u64>>) {
    for x in 0..10 {
        for y in 0..10 {
            grid[x][y] += 1;
        }
    }
}

fn around(pos: (usize, usize)) -> Vec<(usize, usize)> {
    let x = pos.0 as i32;
    let y = pos.1 as i32;
    let transforms = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    transforms
        .iter()
        .map(|(dx, dy)| (x + dx, y + dy))
        .filter(|&(px, py)| px >= 0 && py >= 0 && px < 10 && py < 10)
        .map(|(px, py)| (px as usize, py as usize))
        .collect()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
