use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let test_data = vec![
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
        "",
        "22 13 17 11  0",
        " 8  2 23  4 24",
        "21  9 14 16  7",
        " 6 10  3 18  5",
        " 1 12 20 15 19",
        "",
        " 3 15  0  2 22",
        " 9 18 13 17  5",
        "19  8  7 25 23",
        "20 11 10 24  4",
        "14 21 16 12  6",
        "",
        "14 21 17 24  4",
        "10 16 15  9 19",
        "18  8 23 26 20",
        "22 11 13  6  5",
        " 2  0 12  3  7",
    ];

    let mut test_values = Game::parse(test_data);
    println!("[Test] 4512 = {}", solve(&mut test_values));

    if let Ok(live_data) = read_lines("challenges/day_4/input/input.txt") {
        let raw_data: Vec<String> = live_data.map(|line| line.unwrap()).collect();
        let mut live_values = Game::parse(raw_data.iter().map(AsRef::as_ref).collect());

        println!("[PartOne] {}", solve(&mut live_values));
    }
}

fn solve(game: &mut Game) -> u64 {
    game.play()
}

#[derive(Debug, Clone)]
struct Board {
    hit_values: HashSet<u64>,
    rows: Vec<Vec<u64>>,
    cols: Vec<Vec<u64>>,
}

impl Board {
    fn new(numbers: Vec<Vec<u64>>) -> Self {
        let rows = numbers.clone();
        let len = rows[0].len();
        let mut iters: Vec<_> = numbers.into_iter().map(|n| n.into_iter()).collect();
        let cols = (0..len)
            .map(|_| {
                iters
                    .iter_mut()
                    .map(|n| n.next().unwrap())
                    .collect::<Vec<u64>>()
            })
            .collect();

        Board {
            hit_values: HashSet::new(),
            rows: rows,
            cols: cols,
        }
    }

    fn mark_draw(&mut self, n: u64) -> bool {
        for row in &self.rows {
            if row.contains(&n) {
                self.hit_values.insert(n);
            }
        }
        self.is_winner()
    }

    fn is_winner(&self) -> bool {
        for row in &self.rows {
            if row.iter().all(|v| self.hit_values.contains(v)) {
                return true;
            }
        }
        for col in &self.cols {
            if col.iter().all(|v| self.hit_values.contains(v)) {
                return true;
            }
        }

        return false;
    }

    fn unmatch_numbers(&self) -> Vec<u64> {
        self.rows
            .iter()
            .flat_map(|r| r)
            .filter(|&v| !self.hit_values.contains(v))
            .copied()
            .collect()
    }
}

#[derive(Debug, Clone)]
struct Game {
    draws: Vec<u64>,
    boards: Vec<Board>,
}

impl Game {
    fn parse(values: Vec<&str>) -> Self {
        let draws: Vec<u64> = values[0]
            .split(",")
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        let boards: Vec<Board> = values
            .split(|&line| line.is_empty())
            .skip(1)
            .map(|rows| {
                let numbers: Vec<Vec<u64>> = rows
                    .iter()
                    .map(|&row| {
                        row.split(" ")
                            .filter(|&s| !s.is_empty())
                            .map(|s| s.parse::<u64>().unwrap())
                            .collect::<Vec<u64>>()
                    })
                    .collect();
                Board::new(numbers)
            })
            .collect();

        Game {
            draws: draws,
            boards: boards,
        }
    }

    fn play(&mut self) -> u64 {
        for draw in &self.draws {
            for board in &mut self.boards {
                if board.mark_draw(*draw) {
                    return board.unmatch_numbers().iter().sum::<u64>() * draw;
                }
            }
        }

        0
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
