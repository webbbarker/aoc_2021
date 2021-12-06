use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let test_data = vec![
        "0,9 -> 5,9",
        "8,0 -> 0,8",
        "9,4 -> 3,4",
        "2,2 -> 2,1",
        "7,0 -> 7,4",
        "6,4 -> 2,0",
        "0,9 -> 2,9",
        "3,4 -> 1,4",
        "0,0 -> 8,8",
        "5,5 -> 8,2",
    ];

    let test_values: Vec<((u64, u64), (u64, u64))> = test_data
        .iter()
        .map(|&s| {
            let parts: Vec<&str> = s.split(" -> ").collect();
            let points: Vec<u64> = parts
                .iter()
                .flat_map(|&s| {
                    s.split(",")
                        .map(|v| v.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>()
                })
                .collect();

            ((points[0], points[1]), (points[2], points[3]))
        })
        .collect();

    println!("[Test] 5 = {}", solve(&test_values));

    if let Ok(live_data) = read_lines("challenges/day_5/input/input.txt") {
        let raw_data:Vec<String> = live_data.map(|line| line.unwrap()).collect();
        let live_values: Vec<((u64,u64),(u64,u64))> = raw_data.iter()
            .map(|s| {
                let parts: Vec<&str> = s.split(" -> ").collect();
                let points: Vec<u64> = parts
                    .iter()
                    .flat_map(|&s| {
                        s.split(",")
                            .map(|v| v.parse::<u64>().unwrap())
                            .collect::<Vec<u64>>()
                    })
                    .collect();

                ((points[0], points[1]), (points[2], points[3]))
            })
            .collect();

        println!("[PartOne] {}", solve(&live_values));
    }
}

fn solve(data: &[((u64, u64), (u64, u64))]) -> u64 {
    let mut covered_points = HashMap::new();
    for range in data {
        let (x1, y1) = range.0;
        let (x2, y2) = range.1;

        if x1 != x2 && y1 != y2 {
            continue
        }

        if x2 > x1 {
            (x1..=x2).for_each(|x| {
                *covered_points.entry((x,y1)).or_insert(0) += 1;
            });
        } else if x1 > x2 {
            (x2..=x1).for_each(|x| {
                *covered_points.entry((x,y1)).or_insert(0) += 1;
            });
        } else if y2 > y1 {
            (y1..=y2).for_each(|y| {
                *covered_points.entry((x1,y)).or_insert(0) += 1;
            });
        } else {
            (y2..=y1).for_each(|y| {
                *covered_points.entry((x1,y)).or_insert(0) += 1;
            });
        }
    }
    
    covered_points.values().filter(|&&v| v >= 2 ).count() as u64
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
