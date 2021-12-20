use std::convert::Infallible;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Add;
use std::path::Path;
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, recognize};
use nom::sequence::{delimited, separated_pair};
use nom::IResult;

use itertools::Itertools;

fn main() {
    let test_data = vec![
        "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
        "[[[5,[2,8]],4],[5,[[9,9],0]]]",
        "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
        "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
        "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
        "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
        "[[[[5,4],[7,7]],8],[[8,3],8]]",
        "[[9,3],[[9,9],[6,[4,9]]]]",
        "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
        "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
    ];

    let test_values: Vec<SFNumber> = test_data
        .iter()
        .map(|&s| s.parse::<SFNumber>().unwrap())
        .collect();

    // println!("{:?}", test_data[0]);
    // println!("{:?}", test_values[0]);
    // println!("mag = {:?}", test_values[0].magnitude());

    println!("[Test] 3993 = {}", solve(&test_values));

    if let Ok(live_data) = read_lines("challenges/day_18/input/input.txt") {
        let live_values: Vec<SFNumber> = live_data
            .map(|line| line.unwrap())
            .map(|line| line.parse::<SFNumber>().unwrap())
            .collect();

        println!("[PartTwo] {}", solve(&live_values));
    }
}

fn solve(data: &[SFNumber]) -> u64 {
    let local_data = data.to_vec();

    local_data
        .into_iter()
        .combinations(2)
        .flat_map(|c| {
            let x = c.first().unwrap();
            let y = c.last().unwrap();
            vec![x + y, y + x]
        })
        .map(|mut s| s.magnitude())
        .max()
        .unwrap()
}

#[derive(Debug, PartialEq, Clone)]
struct SFNumber(Vec<(usize, u64)>);

impl SFNumber {
    fn reduce(&mut self) {
        loop {
            if self.explode() {
                continue;
            }

            if self.split() {
                continue;
            }

            break;
        }
    }

    fn magnitude(&mut self) -> u64 {
        let &(max_depth, _) = self
            .0
            .iter()
            .max_by(|&(d1, _), &(d2, _)| d1.cmp(d2))
            .unwrap();

        // work "inside" out
        for i in (1..=max_depth).rev() {
            while let Some(idx) = self.0.iter().position(|&(d, _)| d == i) {
                let (_, left) = self.0.remove(idx);
                let (_, right) = self.0.remove(idx);
                self.0.insert(idx, (i - 1, 3 * left + 2 * right));
            }
        }

        self.0.iter().next().unwrap().1
    }

    fn explode(&mut self) -> bool {
        // pair is inside 4 others, so it's "5" deep
        if let Some(idx) = self.0.iter().position(|&(depth, _)| depth == 5) {
            // println!("exploding {}. {:?}", idx, self);
            let (_, v1) = self.0.remove(idx);
            let (_, v2) = self.0.remove(idx);
            // Replace our exploded value
            self.0.insert(idx, (4, 0));
            if let Some((_, v)) = self.0.get_mut(idx + 1) {
                *v += v2;
            }
            if idx != 0 {
                if let Some((_, v)) = self.0.get_mut(idx - 1) {
                    *v += v1;
                }
            }
            return true;
        }

        false
    }

    fn split(&mut self) -> bool {
        if let Some(idx) = self.0.iter().position(|&(_, v)| v >= 10) {
            let (depth, v) = self.0.remove(idx);
            let l = v / 2;
            let r = v - l;
            self.0.insert(idx, (depth + 1, r));
            self.0.insert(idx, (depth + 1, l));

            return true;
        }

        false
    }
}

impl<'a> Add for &'a SFNumber {
    type Output = SFNumber;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Vec::new();
        result.extend(self.0.iter().map(|&(d, v)| (d + 1, v)));
        result.extend(rhs.0.iter().map(|&(d, v)| (d + 1, v)));

        let mut sfn = SFNumber(result);
        sfn.reduce();

        sfn
    }
}

impl FromStr for SFNumber {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, v) = pair(0)(s).unwrap();

        Ok(SFNumber(v))
    }
}

fn number(depth: usize) -> impl Fn(&str) -> IResult<&str, (usize, u64)> {
    return move |input: &str| {
        let parser = recognize(digit1);
        map(parser, |s: &str| (depth, s.parse::<u64>().unwrap()))(input)
    };
}

fn value(depth: usize) -> impl Fn(&str) -> IResult<&str, Vec<(usize, u64)>> {
    return move |input: &str| alt((map(number(depth), |n| vec![n]), pair(depth)))(input);
}

fn pair(depth: usize) -> impl Fn(&str) -> IResult<&str, Vec<(usize, u64)>> {
    return move |input: &str| {
        let parser = delimited(
            tag("["),
            separated_pair(value(depth + 1), tag(","), value(depth + 1)),
            tag("]"),
        );
        map(parser, |(mut left, mut right)| {
            left.append(&mut right);
            left
        })(input)
    };
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
