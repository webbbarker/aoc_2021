use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let test_data = vec![
        "NNCB", "", "CH -> B", "HH -> N", "CB -> H", "NH -> C", "HB -> C", "HC -> B", "HN -> C",
        "NN -> C", "BH -> H", "NC -> B", "NB -> B", "BN -> B", "BB -> N", "BC -> B", "CC -> N",
        "CN -> C",
    ];
    let test_template: Vec<String> = test_data
        .first()
        .unwrap()
        .chars()
        .map(|c| c.to_string())
        .collect();
    let test_rules: HashMap<Vec<String>, String> = test_data
        .iter()
        .skip(2)
        .map(|&s| {
            let parts: Vec<&str> = s.split(" -> ").collect();
            let key = parts[0].chars().map(|c| c.to_string()).collect();
            let value = parts[1].to_string();

            (key, value)
        })
        .collect();

    println!("[Test] 1588 = {}", solve(&test_template, &test_rules, 10));

    if let Ok(live_data) = read_lines("challenges/day_14/input/input.txt") {
        let raw_data: Vec<String> = live_data.map(|line| line.unwrap()).collect();
        let live_template: Vec<String> = raw_data
            .first()
            .unwrap()
            .chars()
            .map(|c| c.to_string())
            .collect();
        let live_rules: HashMap<Vec<String>, String> = raw_data
            .iter()
            .skip(2)
            .map(|s| {
                let parts: Vec<&str> = s.split(" -> ").collect();
                let key = parts[0].chars().map(|c| c.to_string()).collect();
                let value = parts[1].to_string();

                (key, value)
            })
            .collect();

        println!("[PartOne] {}", solve(&live_template, &live_rules, 10));
    }
}

fn solve(template: &[String], rules: &HashMap<Vec<String>, String>, iterations: usize) -> u64 {
    let mut polymer: Vec<String> = template.to_vec();

    for _i in 0..iterations {
        let last_element = polymer.last().unwrap().to_string();
        polymer = polymer
            .windows(2)
            .flat_map(|w| {
                let insert = rules.get(w).unwrap();
                let prefix = &w[0];
                vec![prefix.to_string(), insert.to_string()]
            })
            .collect::<Vec<String>>();
        polymer.push(last_element);
    }

    let mut counts = HashMap::new();
    for element in polymer {
        *counts.entry(element).or_insert(0) += 1;
    }

    let min_element = counts.values().min().unwrap();
    let max_element = counts.values().max().unwrap();

    max_element - min_element
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
