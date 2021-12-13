use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use std::string::ParseError;

fn main() {
    let test_data = vec![
        "fs-end", "he-DX", "fs-he", "start-DX", "pj-DX", "end-zg", "zg-sl", "zg-pj", "pj-he",
        "RW-he", "fs-DX", "pj-RW", "zg-RW", "start-pj", "he-WI", "zg-he", "pj-fs", "start-RW",
    ];
    let test_values: Graph = Graph::parse(test_data.iter().map(|s| s.to_string()).collect());

    println!("[Test] 226 = {}", solve(&test_values));

    if let Ok(live_data) = read_lines("challenges/day_12/input/input.txt") {
        let live_values: Graph = Graph::parse(live_data.map(|line| line.unwrap()).collect());

        println!("[PartOne] {}", solve(&live_values));
    }
}

fn solve(data: &Graph) -> u64 {
    let paths = data.paths();
    let path_count = paths.len() as u64;

    // for path in paths {
    //     println!("{:?}", path);
    // }

    path_count
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Node {
    Start,
    Big(String),
    Little(String),
    End,
}

impl FromStr for Node {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "start" => Ok(Self::Start),
            "end" => Ok(Self::End),
            a if a.chars().last().unwrap().is_uppercase() => Ok(Self::Big(a.to_string())),
            b => Ok(Self::Little(b.to_string())),
        }
    }
}

impl Node {
    fn is_little(&self) -> bool {
        match self {
            Node::Big(_) => false,
            _ => true,
        }
    }
}

#[derive(Debug, Clone)]
struct Graph {
    nodes: HashSet<Node>,
    edges: HashMap<Node, Vec<Node>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: HashSet::new(),
            edges: HashMap::new(),
        }
    }

    fn parse(raw_input: Vec<String>) -> Self {
        let mut nodes = HashSet::new();
        let mut edges = HashMap::new();
        for input in raw_input {
            let parts: Vec<&str> = input.split("-").collect();
            let start: Node = parts[0].parse().unwrap();
            let end: Node = parts[1].parse().unwrap();

            nodes.insert(start.clone());
            nodes.insert(end.clone());

            edges
                .entry(start.clone())
                .or_insert(Vec::new())
                .push(end.clone());
            edges.entry(end).or_insert(Vec::new()).push(start);
        }

        Graph {
            nodes: nodes,
            edges: edges,
        }
    }

    fn paths(&self) -> Vec<Vec<Node>> {
        let mut paths = Vec::new();

        self.visit_node(&mut Vec::new(), &Node::Start, &mut paths);

        paths
    }

    fn visit_node(&self, path: &mut Vec<Node>, n: &Node, results: &mut Vec<Vec<Node>>) {
        path.push(n.clone());
        if *n == Node::End {
            results.push(path.to_vec());
        } else {
            let adj_nodes = self.edges.get(n).unwrap();
            for adj in adj_nodes {
                if adj.is_little() && path.contains(adj) {
                    continue;
                }
                self.visit_node(path, adj, results);
            }
        }
        path.pop();
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
