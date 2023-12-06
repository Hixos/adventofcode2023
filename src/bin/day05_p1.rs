use std::{ops::Range, vec};

use aoc2023::process_lines;

const SEEDS: &str = "seeds: ";

type RangeMap = (Range<u64>, Range<u64>);

#[derive(Debug)]
struct Map {
    key: String,
    ranges: Vec<RangeMap>
}

impl Map {
    fn map(&self, src: u64) -> u64 {
        for r in self.ranges.iter() {
            if r.0.contains(&src) {
                return r.1.start + (src - r.0.start)
            }
        }

        src
    }
}

#[derive(Debug, Default)]
struct Day5 {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}


impl Day5 {
    fn from_file(file: &str) -> Day5 {
        let mut day5 = Day5::default();
        process_lines(file, |line| {
            day5.parse_line(line);
        });

        day5
    }

    fn from_lines(lines: &Vec<&str>) -> Day5 {
        let mut day5 = Day5::default();
        for &l in lines {
            day5.parse_line(l.to_owned());
        }
        day5
    }

    fn parse_line(&mut self, line: String) {
        if line.starts_with(SEEDS) {
            self.seeds = line.strip_prefix(SEEDS).unwrap()
                .split(' ')
                .map(|v| v.parse::<u64>().unwrap())
                .collect();
        } else if !line.is_empty() && line.chars().next().unwrap().is_alphabetic() {
            let key = line.split(' ').next().unwrap().to_owned();
            self.maps.push(Map {key, ranges: vec![]});
        } else if !line.is_empty() {
            let data: Vec<u64> = line.split(' ').map(|v| v.parse::<u64>().unwrap()).collect();
            let dest = Range {start: data[0], end: data[0] + data[2]};
            let src = Range {start: data[1], end: data[1] + data[2]};
            self.maps.last_mut().unwrap().ranges.push((src, dest));
        }
    }

    fn map(&self, seed: u64) -> u64 {
        let mut val = seed;
        for m in self.maps.iter() {
            val = m.map(val)
        }

        val
    }
}

fn main() {
    let day5 = Day5::from_file("inputs/day05/input.txt");

    let min = day5.seeds.iter().map(|v| day5.map(*v)).min().unwrap();
    println!("{}", min);
}
