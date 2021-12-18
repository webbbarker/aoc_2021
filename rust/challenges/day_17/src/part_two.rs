use std::convert::Infallible;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

fn main() {
    let test_data = vec!["target area: x=20..30, y=-10..-5"];

    let test_value = test_data.first().unwrap().parse::<Target>().unwrap();
    println!("[Test] 112 = {}", solve(test_value));

    if let Ok(live_data) = read_lines("challenges/day_17/input/input.txt") {
        let live_values: Vec<String> = live_data.map(|line| line.unwrap()).collect();

        println!(
            "[PartTwo] {}",
            solve(live_values.first().unwrap().parse::<Target>().unwrap())
        );
    }
}

fn solve(target: Target) -> usize {
    let mut heights = Vec::new();

    for x in 0..(target.br.0 + 1) {
        for y in (target.br.1 - 1)..1000 {
            let visited: Vec<Point> = paths((x, y))
                .take_while(|&(px, py)| px <= target.br.0 && py >= target.br.1)
                .collect();
            if target.contains_any(&visited) {
                heights.push(visited.iter().map(|&(_, py)| py).max().unwrap());
            }
        }
    }

    heights.len()
}

fn paths(velocity: Point) -> impl std::iter::Iterator<Item = Point> {
    let mut dx = velocity.0;
    let mut dy = velocity.1;
    let mut position = (0, 0);

    std::iter::from_fn(move || {
        position = (position.0 + dx, position.1 + dy);

        if dx > 0 {
            dx -= 1;
        }
        dy -= 1;

        Some(position)
    })
}

type Point = (i64, i64);

#[derive(Debug)]
struct Target {
    tl: Point,
    br: Point,
}

impl Target {
    fn contains_any(&self, points: &[Point]) -> bool {
        points
            .iter()
            .any(|&(x, y)| x >= self.tl.0 && x <= self.br.0 && y <= self.tl.1 && y >= self.br.1)
    }
}

impl FromStr for Target {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //"target area: x=20..30, y=-10..-5"
        //           0  1  2 3 45  6  7 8 9
        let parts: Vec<&str> = s.split(&[':', '=', '.', ','][..]).collect();
        let xs: Vec<i64> = vec![parts[2], parts[4]]
            .iter()
            .map(|&s| s.parse::<i64>().unwrap())
            .collect();
        let ys: Vec<i64> = vec![parts[6], parts[8]]
            .iter()
            .map(|&s| s.parse::<i64>().unwrap())
            .collect();

        let tlx = xs.iter().min().unwrap();
        let brx = xs.iter().max().unwrap();
        let tly = ys.iter().max().unwrap();
        let bry = ys.iter().min().unwrap();

        Ok(Self {
            tl: (*tlx, *tly),
            br: (*brx, *bry),
        })
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
