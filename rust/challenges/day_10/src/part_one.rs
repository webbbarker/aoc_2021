use std::collections::HashSet;
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

    println!("[Test] 26397 = {}", solve(&test_values));

    if let Ok(live_data) = read_lines("challenges/day_10/input/input.txt") {
        let live_values: Vec<Vec<char>> = live_data
            .map(|line| line.unwrap())
            .map(|line| line.chars().collect())
            .collect();

        println!("[PartOne] {}", solve(&live_values));
    }
}

fn solve(data: &Vec<Vec<char>>) -> u64 {
    let open_tokens = HashSet::from(['(', '[', '{', '<']);
    let close_tokens = HashSet::from([')', ']', '}', '>']);
    let mut scores = vec![];
    for line in data {
        let mut pending_close_tokens = vec![];
        for v in line {
            match v {
                '(' => pending_close_tokens.push(')'),
                '[' => pending_close_tokens.push(']'),
                '{' => pending_close_tokens.push('}'),
                '<' => pending_close_tokens.push('>'),
                token => {
                    if let Some(expected_token) = pending_close_tokens.pop() {
                        if *token != expected_token {
                            match token {
                                ')' => scores.push(3),
                                ']' => scores.push(57),
                                '}' => scores.push(1197),
                                '>' => scores.push(25137),
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }

    scores.iter().sum()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
