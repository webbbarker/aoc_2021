use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let test_data = vec![
        "[({(<(())[]>[[{[]{<()<>>",
        "[(()[<>])]({[<{<<[]>>(",
        "{([(<{}[<>[]}>{[]{[(<()>",
        "(((({<>}<{<{<>}{[]{[]{}",
        "[[<[([]))<([[{}[[()]]]",
        "[{[{({}]{}}([{[{{{}}([]",
        "{<[[]]>}<{[{[{[]{()[[[]",
        "[<(<(<(<{}))><([]([]()",
        "<{([([[(<>()){}]>(<<{{",
        "<{([{{}}[<[[[<>{}]]]>[]]",
    ];
    let test_values: Vec<Vec<char>> = test_data
        .iter()
        .map(|&line| line.chars().collect())
        .collect();

    println!("[Test] 288957 = {}", solve(&test_values));

    if let Ok(live_data) = read_lines("challenges/day_10/input/input.txt") {
        let live_values: Vec<Vec<char>> = live_data
            .map(|line| line.unwrap())
            .map(|line| line.chars().collect())
            .collect();

        println!("[PartTwo] {}", solve(&live_values));
    }
}

fn solve(data: &Vec<Vec<char>>) -> u64 {
    let mut scores = vec![];
    for line in data {
        let mut pending_close_tokens = vec![];
        let mut corrupted = false;
        for v in line {
            match v {
                '(' => pending_close_tokens.push(')'),
                '[' => pending_close_tokens.push(']'),
                '{' => pending_close_tokens.push('}'),
                '<' => pending_close_tokens.push('>'),
                token => {
                    if let Some(expected_token) = pending_close_tokens.pop() {
                        if *token != expected_token {
                            corrupted = true;
                            continue;
                        }
                    }
                }
            }
        }

        if corrupted {
            continue;
        }

        pending_close_tokens.reverse();
        scores.push(pending_close_tokens.iter().fold(0, |acc, val| {
            let points = match val {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => 0,
            };

            acc * 5 + points
        }));
    }

    scores.sort();
    scores[scores.len() / 2]
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
