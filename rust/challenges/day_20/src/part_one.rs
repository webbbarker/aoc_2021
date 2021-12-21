use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let test_data = vec![
        "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#",
        "",
        "#..#.",
        "#....",
        "##..#",
        "..#..",
        "..###",
    ];

    let test_algo: Vec<char> = test_data[0].chars().collect();
    let test_values: Vec<Point> = test_data[2..]
        .iter()
        .enumerate()
        .flat_map(|(x, &line)| {
            line.chars()
                .enumerate()
                .filter_map(|(y, c)| {
                    if c == '#' {
                        Some((x as i64, y as i64))
                    } else {
                        None
                    }
                })
                .collect::<Vec<Point>>()
        })
        .collect();
    let test_dimensions = (test_data[2..].len() as i64, test_data[2].len() as i64);

    println!(
        "[Test] 35 = {}",
        solve(&test_values, test_dimensions, &test_algo, 2)
    );

    if let Ok(live_data) = read_lines("challenges/day_20/input/input.txt") {
        let raw_data: Vec<String> = live_data.map(|line| line.unwrap()).collect();
        let live_algo: Vec<char> = raw_data[0].chars().collect();
        let live_values: Vec<Point> = raw_data[2..]
            .iter()
            .enumerate()
            .flat_map(|(x, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(y, c)| {
                        if c == '#' {
                            Some((x as i64, y as i64))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<Point>>()
            })
            .collect();
        let live_dimensions = (raw_data[2..].len() as i64, raw_data[2].len() as i64);

        println!(
            "[PartOne] {}",
            solve(&live_values, live_dimensions, &live_algo, 2)
        );
    }
}

// NOTES:
// This is probably a problem where part 2 asks you to expand the image _a lot_.  Storing vectors is wasteful, better to just store pixels that are "on"
// and navigate a "view" relative to that.  You can then compute the algo lookup just by doing list.contains((x,y)), no is always 0 yes is a 1.

// NOTES:
// Thanks AoC meme for pointing out that full empty cells toggle to enabled cells.  Need to figure out how to compute given that context... :cry:

fn solve(data: &[Point], dim: Point, algo: &[char], count: usize) -> usize {
    // println!("dim = {:?}", dim);
    // println!("algo = {:?}", algo);
    // println!("data = {:?}", data);
    let mut image: HashSet<Point> = HashSet::new();
    for p in data {
        image.insert(*p);
    }
    let count = count as i64;

    // Note:  This is uh, counterintuitive.  Basically we start from a much larger area shrinking
    //  down towards the true target which is 2*count+dim.  The reason being is that in the larger area
    //  we can play with the "flashing" empty space will only ever examining inner (ie smaller) regions
    //  on subsequent steps.  This gives us good behavior around the edges without thinking too hard.
    for i in (1..=count).rev() {
        let mut new_image: HashSet<Point> = HashSet::new();
        for x in (0 - i * 9)..(dim.0 + i * 9) {
            for y in (0 - i * 9)..(dim.1 + i * 9) {
                let idx = pos_to_idx((x, y), &image, i);
                if algo[idx] == '#' {
                    new_image.insert((x, y));
                }
            }
        }
        image = new_image;
    }

    image.len()
}

type Point = (i64, i64);

fn pos_to_idx(pos: Point, image: &HashSet<Point>, step: i64) -> usize {
    let relative_pos = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 0),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let bin_num = relative_pos
        .iter()
        .map(|(dx, dy)| {
            let test_location = (dx + pos.0, dy + pos.1);
            if image.contains(&test_location) {
                "1"
            } else {
                "0"
            }
        })
        .collect::<String>();

    usize::from_str_radix(&bin_num, 2).unwrap()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
