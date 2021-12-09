use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use std::string::ParseError;

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
    let test_values: Vec<Observation> = test_data
        .iter()
        .map(|&line| line.parse::<Observation>().unwrap())
        .collect();

    println!("[Test] 26 = {}", solve(&test_values));

    if let Ok(live_data) = read_lines("challenges/day_8/input/input.txt") {
        let live_values: Vec<Observation> = live_data
            .map(|line| line.unwrap())
            .map(|line| line.parse::<Observation>().unwrap())
            .collect();

        println!("[PartOne] {}", solve(&live_values));
    }
}

fn solve(data: &[Observation]) -> u64 {
    data.iter()
        .flat_map(|o| o.output.iter().map(|i| i.len() as u64))
        .filter(|&o| o == 2 || o == 3 ||o == 4 || o == 7)
        .count() as u64
}

#[derive(Debug, Clone)]
struct Observation {
    patterns: Vec<String>,
    output: Vec<String>,
}

impl Observation {
    fn new(patterns: Vec<&str>, output: Vec<&str>) -> Self {
        let p = patterns.iter().map(|s| s.to_string()).collect();
        let o = output.iter().map(|s| s.to_string()).collect();
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

        let patterns: Vec<&str> = parts[0].split(" ").collect();
        let output: Vec<&str> = parts[1].split(" ").collect();

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
