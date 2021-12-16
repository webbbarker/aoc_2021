use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let test_data = vec![
        "1163751742",
        "1381373672",
        "2136511328",
        "3694931569",
        "7463417111",
        "1319128137",
        "1359912421",
        "3125421639",
        "1293138521",
        "2311944581",
    ];
    let test_values: Vec<Vec<usize>> = test_data
        .iter()
        .map(|&s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    println!("[Test] 40 = {}", solve(&test_values));

    if let Ok(live_data) = read_lines("challenges/day_15/input/input.txt") {
        let live_values: Vec<Vec<usize>> = live_data
            .map(|line| line.unwrap())
            .map(|s| {
                s.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();

        println!("[PartOne] {}", solve(&live_values));
    }
}

fn solve(data: &Vec<Vec<usize>>) -> usize {
    let dest = (data.len() - 1, data.last().unwrap().len() - 1);

    let graph = Graph::from_vec(data);
    let paths = graph.distances((0, 0));

    *paths.get(&dest).unwrap()
}

type Point = (usize, usize);
type Edge = (Point, usize);

#[derive(Debug)]
struct Graph {
    nodes: Vec<Point>,
    edges: HashMap<Point, Vec<Edge>>,
}

impl Graph {
    fn from_vec(data: &Vec<Vec<usize>>) -> Self {
        let mut edges = HashMap::new();
        for x in 0..data.len() {
            for y in 0..data[x].len() {
                let p = (x, y);
                let adjecent_edges: Vec<Edge> = get_adjecent_points(p, data.len(), data[x].len())
                    .iter()
                    .map(|&(ax, ay)| ((ax, ay), data[ax][ay]))
                    .collect();
                edges.insert(p, adjecent_edges);
            }
        }

        let nodes: Vec<Point> = edges.keys().map(|&p| p).collect();

        Graph { nodes, edges }
    }

    fn distances(&self, start: Point) -> HashMap<Point, usize> {
        let mut distances = HashMap::new();
        let mut visited = HashSet::new();
        let mut to_visit = BinaryHeap::new();

        distances.insert(start, 0);
        to_visit.push(Reverse((start, 0)));

        while let Some(Reverse((p, dist))) = to_visit.pop() {
            if !visited.insert(p) {
                continue;
            }

            if let Some(neighbors) = self.edges.get(&p) {
                for (neighbor, cost) in neighbors {
                    let new_distance = dist + cost;
                    let is_shorter = distances
                        .get(neighbor)
                        .map_or(true, |&cur| new_distance < cur);

                    if is_shorter {
                        distances.insert(*neighbor, new_distance);
                        to_visit.push(Reverse((*neighbor, new_distance)));
                    }
                }
            }
        }

        distances
    }
}

fn get_adjecent_points(p: Point, height: usize, width: usize) -> Vec<Point> {
    let (x, y) = p;
    let xform: [(i64, i64); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    xform
        .iter()
        .map(|(dx, dy)| ((x as i64) + dx, (y as i64) + dy))
        .filter(|&(i, j)| i >= 0 && j >= 0 && i < height as i64 && j < width as i64)
        .map(|(ax, ay)| (ax as usize, ay as usize))
        .collect()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
