use std::collections::HashSet;

use aoc2023::process_lines;

struct Day6 {
    time: Vec<u64>,
    distance: Vec<u64>,
}

impl Day6 {
    fn from_file(file: &str) -> Day6 {
        let mut time: Vec<u64> = vec![];
        let mut distance: Vec<u64> = vec![];

        process_lines(file, |line| {
            if line.starts_with("Time:") {
                time = line
                    .split(':')
                    .nth(1)
                    .unwrap()
                    .split(' ')
                    .filter(|v| !v.is_empty())
                    .map(|v| v.parse::<u64>().unwrap())
                    .collect();
            }

            if line.starts_with("Distance:") {
                distance = line
                    .split(':')
                    .nth(1)
                    .unwrap()
                    .split(' ')
                    .filter(|v| !v.is_empty())
                    .map(|v| v.parse::<u64>().unwrap())
                    .collect();
            }

        });

        Day6 { time, distance }
    }

    fn from_file_p2(file: &str) -> Day6 {
        let mut time: Vec<u64> = vec![];
        let mut distance: Vec<u64> = vec![];
        

        process_lines(file, |line| {
            let line = line.replace(' ', "");

            if line.starts_with("Time:") {
                time = line
                    .split(':')
                    .nth(1)
                    .unwrap()
                    .split(' ')
                    .filter(|v| !v.is_empty())
                    .map(|v| v.parse::<u64>().unwrap())
                    .collect();
            }

            if line.starts_with("Distance:") {
                distance = line
                    .split(':')
                    .nth(1)
                    .unwrap()
                    .split(' ')
                    .filter(|v| !v.is_empty())
                    .map(|v| v.parse::<u64>().unwrap())
                    .collect();
            }

        });

        Day6 { time, distance }
    }

    fn day6(&self) -> u64 {
        let mut res = 1u64;
        for (i, race_time) in self.time.iter().enumerate() {
            let dist = self.distance[i];
            let mut n = 0;
            for t in 1..*race_time {
                if t * (race_time - t) > dist {
                    n += 1;
                }
            }
            res *= n;
        }
        res
    }
}

fn main() {
    let day6 = Day6::from_file("inputs/day06/input.txt");
    println!("Number of ways to beat records: {}", day6.day6());


    let day6 = Day6::from_file_p2("inputs/day06/input.txt");
    println!("Number of ways to beat records: {}", day6.day6());
}
