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
    let test_rules: HashMap<(String, String), String> = test_data
        .iter()
        .skip(2)
        .map(|&s| {
            let parts: Vec<&str> = s.split(" -> ").collect();
            let key: Vec<String> = parts[0].chars().map(|c| c.to_string()).collect();
            let value = parts[1].to_string();

            ((key[0].to_string(), key[1].to_string()), value)
        })
        .collect();

    println!(
        "[Test] 2188189693529 = {}",
        solve(&test_template, &test_rules, 40)
    );

    if let Ok(live_data) = read_lines("challenges/day_14/input/input.txt") {
        let raw_data: Vec<String> = live_data.map(|line| line.unwrap()).collect();
        let live_template: Vec<String> = raw_data
            .first()
            .unwrap()
            .chars()
            .map(|c| c.to_string())
            .collect();
        let live_rules: HashMap<(String, String), String> = raw_data
            .iter()
            .skip(2)
            .map(|s| {
                let parts: Vec<&str> = s.split(" -> ").collect();
                let key: Vec<String> = parts[0].chars().map(|c| c.to_string()).collect();
                let value = parts[1].to_string();

                ((key[0].to_string(), key[1].to_string()), value)
            })
            .collect();

        println!("[PartTwo] {}", solve(&live_template, &live_rules, 40));
    }
}

fn solve(template: &[String], rules: &HashMap<(String, String), String>, iterations: usize) -> u64 {
    let mut pair_counts: Vec<((String, String), u64)> = Vec::new();
    template.windows(2).for_each(|w| {
        pair_counts.push(((w[0].clone(), w[1].clone()), 1));
    });
    for _i in 0..iterations {
        let new_pair_counts: Vec<((String, String), u64)> = pair_counts
            .iter()
            .flat_map(|((e1, e2), ct)| {
                let e3 = rules.get(&(e1.to_string(), e2.to_string())).unwrap();
                vec![
                    ((e1.clone(), e3.clone()), *ct),
                    ((e3.clone(), e2.clone()), *ct),
                ]
            })
            .collect();

        let mut combined = HashMap::new();
        for (key, value) in new_pair_counts {
            *combined.entry(key).or_insert(0) += value;
        }

        pair_counts = combined.into_iter().map(|(k, v)| (k, v)).collect();
    }

    println!("{:?}", pair_counts);

    let mut counts = HashMap::new();
    for ((e1, e2), ct) in pair_counts {
        *counts.entry(e1).or_insert(0) += ct / 2;
        *counts.entry(e2).or_insert(0) += ct / 2;
    }

    println!("{:?}", counts);

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
