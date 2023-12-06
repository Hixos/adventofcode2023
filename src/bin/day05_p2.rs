use std::{
    cmp::{max, min},
    ops::Range,
    vec,
};

use aoc2023::process_lines;

const SEEDS: &str = "seeds: ";

type RangeMap = (Range<u64>, Range<u64>);

fn intersection<T: Ord + Copy>(a: &Range<T>, b: &Range<T>) -> Range<T> {
    let start = max(a.start, b.start);
    let end = min(a.end, b.end);

    Range { start, end }
}

fn difference<T: PartialOrd + Copy>(a: &Range<T>, b: &Range<T>) -> Vec<Range<T>> {

    if a.start < b.start && a.end >= b.start {
        if a.end > b.end {
            return vec![
                Range {
                    start: a.start,
                    end: b.start,
                },
                Range {
                    start: b.end,
                    end: a.end,
                },
            ];
        } else {
            return vec![Range {
                start: a.start,
                end: b.start,
            }];
        }
    } else if b.start <= a.start && b.end >= a.start {
        if b.end >= a.end {
            return vec![];
        }else{
            return vec![Range {
                start: b.end,
                end: a.end,
            }];
        }
    }
    vec![]
}

#[derive(Debug)]
struct Map {
    ranges: Vec<RangeMap>,
}

impl Map {
    fn map(&self, input: &Range<u64>) -> Vec<Range<u64>> {
        let mut input = vec![input.clone()];
        let mut out = vec![];

        for (src, dst) in self.ranges.iter() {
            let mut new_input = vec![];
            for r in input {
                let int = intersection(&r, src);
                if !int.is_empty() {
                    let start = dst.start + (int.start - src.start);
                    let end = dst.start + (int.end - src.start);
                    out.push(Range { start, end });
                    
                    new_input.append(&mut difference(&r, src));
                }else{
                    new_input.push(r);
                }
            }
            input = new_input;
        }
        out.append(&mut input);
        out
    }

    fn map_vec(&self, input: &Vec<Range<u64>>) -> Vec<Range<u64>> {
        let mut out = vec![];
        for vi in input.iter() {
            out.append(&mut self.map(vi));
        }

        out
    }
}

#[derive(Debug, Default)]
struct Day5 {
    seeds: Vec<Range<u64>>,
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
            let seeds_nums: Vec<u64> = line
                .strip_prefix(SEEDS)
                .unwrap()
                .split(' ')
                .map(|v| v.parse::<u64>().unwrap())
                .collect();
            self.seeds = seeds_nums
                .chunks(2)
                .map(|v| Range {
                    start: v[0],
                    end: v[0] + v[1],
                })
                .collect();
        } else if !line.is_empty() && line.chars().next().unwrap().is_alphabetic() {
            self.maps.push(Map { ranges: vec![] });
        } else if !line.is_empty() {
            let data: Vec<u64> = line.split(' ').map(|v| v.parse::<u64>().unwrap()).collect();
            let dest = Range {
                start: data[0],
                end: data[0] + data[2],
            };
            let src = Range {
                start: data[1],
                end: data[1] + data[2],
            };
            self.maps.last_mut().unwrap().ranges.push((src, dest));
        }
    }

    fn locations(&self) -> Vec<Range<u64>> {
        let mut val = self.seeds.clone();

        for m in self.maps.iter() {
            val = m.map_vec(&val)
        }

        val
    }
}

fn main() {
    let day5 = Day5::from_file("inputs/day05/input.txt");
    println!("{:.2?}", day5.seeds);
    println!();

    use std::time::Instant;
    let now = Instant::now();
    let locs = day5.locations();
    let min = locs.iter().map(|r| r.start).min().unwrap();
    let elapsed = now.elapsed();
    
    println!("{:?}", locs);
    println!();
    
    println!("{}", min);
    println!("Elapsed: {:?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersect() {
        let cases = [
            (
                (Range { start: 0, end: 0 }, Range { start: 0, end: 4 }),
                (false, None),
            ),
            (
                (Range { start: 0, end: 0 }, Range { start: 0, end: 0 }),
                (false, None),
            ),
            (
                (Range { start: 0, end: 4 }, Range { start: 0, end: 0 }),
                (false, None),
            ),
            (
                (Range { start: 0, end: 4 }, Range { start: 0, end: 4 }),
                (true, Some(Range { start: 0, end: 4 })),
            ),
            (
                (Range { start: 0, end: 4 }, Range { start: 2, end: 4 }),
                (true, Some(Range { start: 2, end: 4 })),
            ),
            (
                (Range { start: 0, end: 4 }, Range { start: 0, end: 2 }),
                (true, Some(Range { start: 0, end: 2 })),
            ),
            (
                (Range { start: 1, end: 2 }, Range { start: 0, end: 4 }),
                (true, Some(Range { start: 1, end: 2 })),
            ),
            (
                (Range { start: 0, end: 4 }, Range { start: 1, end: 2 }),
                (true, Some(Range { start: 1, end: 2 })),
            ),
            (
                (Range { start: 0, end: 4 }, Range { start: 2, end: 6 }),
                (true, Some(Range { start: 2, end: 4 })),
            ),
            (
                (Range { start: 0, end: 4 }, Range { start: -2, end: 2 }),
                (true, Some(Range { start: 0, end: 2 })),
            ),
            (
                (Range { start: 0, end: 4 }, Range { start: 1, end: 3 }),
                (true, Some(Range { start: 1, end: 3 })),
            ),
            (
                (Range { start: 0, end: 4 }, Range { start: 6, end: 8 }),
                (false, None),
            ),
        ];

        for (case, res) in cases {
            let int = intersection(&case.0, &case.1);
            assert_eq!(res.0, !int.is_empty());
            if res.0 {
                assert_eq!(res.1.unwrap(), int);
            }
        }
    }

    #[test]
    fn test_diff() {
        fn r(start: i32, end: i32) -> Range<i32> {
            Range { start, end }
        }

        let cases = [
            (
                (Range { start: 0, end: 0 }, Range { start: 0, end: 4 }),
                vec![],
            ),
            (
                (Range { start: 2, end: 2 }, Range { start: 0, end: 4 }),
                vec![],
            ),
            (
                (Range { start: 0, end: 0 }, Range { start: 0, end: 0 }),
                vec![],
            ),
            (
                (Range { start: 0, end: 4 }, Range { start: 0, end: 0 }),
                vec![r(0, 4)],
            ),
            (
                (Range { start: 0, end: 4 }, Range { start: 0, end: 4 }),
                vec![],
            ),
            (
                (Range { start: 0, end: 4 }, Range { start: 2, end: 4 }),
                vec![r(0, 2)],
            ),
            (
                (Range { start: 0, end: 4 }, Range { start: 0, end: 2 }),
                vec![r(2, 4)],
            ),
            (
                (Range { start: 1, end: 2 }, Range { start: 0, end: 4 }),
                vec![],
            ),
            (
                (Range { start: 0, end: 4 }, Range { start: 1, end: 2 }),
                vec![r(0, 1), r(2, 4)],
            ),
            (
                (Range { start: 0, end: 4 }, Range { start: 2, end: 6 }),
                vec![r(0, 2)],
            ),
            (
                (Range { start: 0, end: 4 }, Range { start: -2, end: 2 }),
                vec![r(2, 4)],
            ),
            (
                (Range { start: 0, end: 4 }, Range { start: 1, end: 3 }),
                vec![r(0, 1), r(3, 4)],
            ),
            (
                (Range { start: 0, end: 4 }, Range { start: 6, end: 8 }),
                vec![],
            ),
        ];

        for (_i, (case, res)) in cases.iter().enumerate() {
            let diff = difference(&case.0, &case.1);
            assert_eq!(*res, diff)
        }
    }
}
