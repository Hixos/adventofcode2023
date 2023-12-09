use std::collections::HashMap;

use aoc2023::process_lines;

struct Day8 {
    dirs: Vec<usize>,
    network: HashMap<String, Vec<String>>,
}

impl Day8 {
    fn from_file(file: &str) -> Day8 {
        let mut parsed_directions = false;
        let mut dirs: Vec<usize> = vec![];
        let mut network: HashMap<String, Vec<String>> = HashMap::new();

        process_lines(file, |line| {
            if line.is_empty() {
                return;
            }

            if !parsed_directions {
                for c in line.chars() {
                    match c {
                        'L' => dirs.push(0),
                        'R' => dirs.push(1),
                        _ => panic!("Unexpected direction!"),
                    }
                }
                parsed_directions = true;
            } else {
                let mut key_val = line.split(" = ");
                let key = key_val.next().unwrap().to_owned();

                let vals: Vec<String> = key_val
                    .next()
                    .unwrap()
                    .split(", ")
                    .map(|v| v.replace("(", "").replace(")", ""))
                    .collect();

                network.insert(key, vals);
            }
        });

        Day8 { dirs, network }
    }

    fn count_steps(&self) -> usize {
        let mut key = &"AAA".to_string();
        let mut steps = 0usize;

        while key != "ZZZ" {
            let dir = self.dirs[steps % self.dirs.len()];
            key = &self.network.get(key).unwrap()[dir];

            steps += 1;
        }

        steps
    }

    fn count_steps_sigle_p2(&self, start_key: &String) -> usize {
        let mut key = start_key;
        let mut steps = 0usize;

        while !key.ends_with('Z') {
            let dir = self.dirs[steps % self.dirs.len()];
            key = &self.network.get(key).unwrap()[dir];

            steps += 1;
        }

        steps
    }

    fn count_steps_p2(&self) -> usize {
        use num::integer::lcm;

        let keys = self
            .network
            .keys()
            .filter(|&k| k.ends_with('A'))
            .collect::<Vec<&String>>();

        println!("{:?}", keys);

        let steps_vec: Vec<usize> = keys
            .iter()
            .map(|&k| {
                println!("{}", k);
                self.count_steps_sigle_p2(k)
            })
            .collect();

        println!("{:?}", steps_vec);

        let mut steps = steps_vec[0];

        for s in steps_vec.iter().skip(1) {
            steps = lcm(steps, *s);
        }

        steps
    }
}

fn main() {
    let day8 = Day8::from_file("inputs/day08/input.txt");

    println!("Part 1: {}", day8.count_steps());

    println!("Part 2: {}", day8.count_steps_p2());
}
