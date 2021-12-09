use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use std::string::ParseError;

use itertools::Itertools;

fn main() {
    let test_data = vec![
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
        "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
    ];
    // let test_data = vec!["acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"];
    let test_values: Vec<Observation> = test_data
        .iter()
        .map(|&line| line.parse::<Observation>().unwrap())
        .collect();

    println!("[Test] 61229 = {}", solve(&test_values));

    if let Ok(live_data) = read_lines("challenges/day_8/input/input.txt") {
        let live_values: Vec<Observation> = live_data
            .map(|line| line.unwrap())
            .map(|line| line.parse::<Observation>().unwrap())
            .collect();

        println!("[PartTwo] {}", solve(&live_values));
    }
}

fn solve(data: &[Observation]) -> u64 {
    let mut readings = vec![];
    for observation in data {
        let mut translator = HashMap::new();
        let mut reverse_translator = HashMap::new();
        let values: Vec<&String> = observation
            .patterns
            .iter()
            .chain(observation.output.iter())
            .unique()
            .sorted_by(|&a, &b| Ord::cmp(&a.len(), &b.len()))
            .collect();
        for value in values {
            if translator.contains_key(value) {
                continue;
            }
            if value.len() == 2 {
                translator.insert(value, 1);
                reverse_translator.insert(1, value);
                continue;
            }
            if value.len() == 3 {
                translator.insert(value, 7);
                reverse_translator.insert(7, value);
                continue;
            }
            if value.len() == 4 {
                translator.insert(value, 4);
                reverse_translator.insert(4, value);
                continue;
            }
            if value.len() == 7 {
                translator.insert(value, 8);
                reverse_translator.insert(8, value);
                continue;
            }

            if value.len() == 5 {
                let one_value = reverse_translator.get(&1).unwrap();
                let four_value = reverse_translator.get(&4).unwrap();
                // if value.contains(&one_value.chars()) {
                if contains_all(value, one_value) {
                    translator.insert(value, 3);
                    reverse_translator.insert(3, value);
                    continue;
                } else if value.chars().filter(|&c| !four_value.contains(c)).count() == 2 {
                    translator.insert(value, 5);
                    reverse_translator.insert(5, value);
                    continue;
                } else {
                    translator.insert(value, 2);
                    reverse_translator.insert(2, value);
                    continue;
                }
            }

            if value.len() == 6 {
                let one_value = reverse_translator.get(&1).unwrap();
                let four_value = reverse_translator.get(&4).unwrap();

                // if !value.contains(*one_value) {
                if !contains_all(value, one_value) {
                    translator.insert(value, 6);
                    reverse_translator.insert(6, value);
                    continue;
                } else if value.chars().filter(|&c| !four_value.contains(c)).count() == 2 {
                    translator.insert(value, 9);
                    reverse_translator.insert(9, value);
                    continue;
                } else {
                    translator.insert(value, 0);
                    reverse_translator.insert(0, value);
                    continue;
                }
            }
        }

        let reading = observation
            .output
            .iter()
            .map(|o| translator.get(o).unwrap().to_string())
            .join("")
            .parse::<u64>()
            .unwrap();

        // println!("{:?} = {}", observation.output, reading);
        // println!("{:?}", translator);
        readings.push(reading);
    }

    readings.iter().sum()
}

fn contains_all(string: &str, substring: &str) -> bool {
    substring.chars().all(|c| string.contains(c))
}

#[derive(Debug, Clone)]
struct Observation {
    patterns: Vec<String>,
    output: Vec<String>,
}

impl Observation {
    fn new<S>(patterns: Vec<S>, output: Vec<S>) -> Self
    where
        S: Into<String>,
    {
        let p = patterns.into_iter().map(|s| s.into()).collect();
        let o = output.into_iter().map(|s| s.into()).collect();
        Observation {
            patterns: p,
            output: o,
        }
    }
}

impl FromStr for Observation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<String> = s.split(" | ").map(|s| s.to_string()).collect();

        let patterns: Vec<String> = parts[0]
            .split(" ")
            .map(|p| {
                let mut chars: Vec<char> = p.chars().collect();
                chars.sort();
                chars.into_iter().collect::<String>()
            })
            .collect();
        let output: Vec<String> = parts[1]
            .split(" ")
            .map(|p| {
                let mut chars: Vec<char> = p.chars().collect();
                chars.sort();
                chars.into_iter().collect::<String>()
            })
            .collect();

        Ok(Observation::new(patterns, output))
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
