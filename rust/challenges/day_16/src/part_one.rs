use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use std::string::ParseError;

fn main() {
    let test_data = vec![
        // ("D2FE28", 0),
        // ("38006F45291200", 0),
        // ("EE00D40C823060", 0),
        ("8A004A801A8002F478", 16),
        ("620080001611562C8802118E34", 12),
        ("C0015000016115A2E0802F182340", 23),
        ("A0016C880162017C3686B18A3D4780", 31),
    ];

    for (test_value, test_result) in test_data {
        println!("[Test] {} = {}", test_result, solve(test_value));
    }

    if let Ok(live_data) = read_lines("challenges/day_16/input/input.txt") {
        let live_values: Vec<String> = live_data.map(|line| line.unwrap()).collect();

        println!("[PartOne] {}", solve(live_values.first().unwrap()));
    }
}

fn solve<S: Into<String>>(data: S) -> u64 {
    let raw_data: String = data.into();
    let packet: Packet = raw_data.parse().unwrap();

    packet.versions().iter().sum()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Packet {
    LiteralValue {
        version: u8,
        value: u64,
    },
    Operator {
        version: u8,
        sub_packets: Vec<Packet>,
    },
}

impl FromStr for Packet {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let binary_data: String = s
            .chars()
            .map(|c| match c {
                '0' => "0000",
                '1' => "0001",
                '2' => "0010",
                '3' => "0011",
                '4' => "0100",
                '5' => "0101",
                '6' => "0110",
                '7' => "0111",
                '8' => "1000",
                '9' => "1001",
                'A' => "1010",
                'B' => "1011",
                'C' => "1100",
                'D' => "1101",
                'E' => "1110",
                'F' => "1111",
                _ => "XXXX",
            })
            .collect();
        Ok(Self::parse(&binary_data).0)
    }
}

impl Packet {
    fn parse(bin_data: &str) -> (Self, usize) {
        let version = u8::from_str_radix(&bin_data[..3], 2).unwrap();
        let type_id = u8::from_str_radix(&bin_data[3..6], 2).unwrap();
        let (packet, consumed) = match type_id {
            4 => Packet::parse_literal(version, &bin_data[6..]),
            _ => Packet::parse_operator(version, &bin_data[6..]),
        };
        (packet, consumed + 6)
    }

    fn parse_literal(version: u8, bin_data: &str) -> (Self, usize) {
        let all_chunks = bin_data
            .chars()
            .collect::<Vec<char>>()
            .chunks(5)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>();

        let mut value_chunks = Vec::new();
        for chunk in all_chunks {
            if &chunk[..1] == "0" {
                value_chunks.push(chunk);
                break;
            }
            value_chunks.push(chunk);
        }

        let raw_value = value_chunks.iter().map(|s| &s[1..]).collect::<String>();

        let value = u64::from_str_radix(&raw_value, 2).unwrap();
        let consumed: usize = value_chunks.iter().map(|s| s.len()).sum();
        (Packet::LiteralValue { version, value }, consumed)
    }

    fn parse_operator(version: u8, bin_data: &str) -> (Self, usize) {
        let (length, consumed) = OperatorLength::parse(bin_data);
        let mut sub_packets = Vec::new();
        let mut total_bits_consumed = consumed;

        match length {
            OperatorLength::Bits(n) => {
                let mut bits_consumed = 0;
                while bits_consumed < n {
                    let (p, c) = Packet::parse(&bin_data[total_bits_consumed..]);
                    sub_packets.push(p);
                    bits_consumed += c as u64;
                    total_bits_consumed += c;
                }
            }
            OperatorLength::Count(n) => {
                let mut packets_recieved = 0;
                while packets_recieved < n {
                    let (p, c) = Packet::parse(&bin_data[total_bits_consumed..]);
                    sub_packets.push(p);
                    packets_recieved += 1;
                    total_bits_consumed += c;
                }
            }
        }

        (
            Packet::Operator {
                version,
                sub_packets,
            },
            total_bits_consumed,
        )
    }

    fn versions(&self) -> Vec<u64> {
        match self {
            Self::LiteralValue { version, value: _ } => vec![*version as u64],
            Self::Operator {
                version,
                sub_packets,
            } => {
                let mut versions = vec![*version as u64];
                versions.extend(sub_packets.iter().flat_map(|p| p.versions()));

                versions
            }
        }
    }
}

enum OperatorLength {
    Bits(u64),
    Count(u64),
}

impl OperatorLength {
    fn parse(bin_data: &str) -> (Self, usize) {
        let length_type = bin_data.chars().nth(0).unwrap();

        match length_type {
            '0' => (
                Self::Bits(u64::from_str_radix(&bin_data[1..16], 2).unwrap()),
                16,
            ),
            '1' => (
                Self::Count(u64::from_str_radix(&bin_data[1..12], 2).unwrap()),
                12,
            ),
            _ => (Self::Bits(0), 16),
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
