use std::collections::HashMap;

use aoc2023::process_lines;

#[derive(Debug)]
struct Day9 {
    m: Vec<Vec<i32>>,
}

impl Day9 {
    fn from_file(file: &str) -> Day9 {
        let mut m = vec![];
        process_lines(file, |line| {
            m.push(
                line.split(' ')
                    .filter(|v| !v.is_empty())
                    .map(|v| v.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>(),
            );
        });

        Day9 { m }
    }

    fn extrap(m: &Vec<i32>) -> i32 {
        let mut is_zero = true;

        let mut diff = vec![];
        diff.reserve(m.len() - 1);
        for (i, v) in m.iter().enumerate().skip(1) {
            let ve = v - m[i-1];
            diff.push(ve);
            if ve != 0 {
                is_zero = false;
            }
        }

        // println!("{:?}", diff);
        if !is_zero {
            return m.last().unwrap() + Self::extrap(&diff);
        }

        *m.last().unwrap()
    }

    fn extrap_left(m: &Vec<i32>) -> i32 {
        let mut is_zero = true;

        let mut diff = vec![];
        diff.reserve(m.len() - 1);
        for (i, v) in m.iter().enumerate().skip(1) {
            let ve = v - m[i-1];
            diff.push(ve);
            if ve != 0 {
                is_zero = false;
            }
        }

        if !is_zero {
            return m.first().unwrap() - Self::extrap_left(&diff);
        }

        *m.first().unwrap()
    }
}

fn main() {
    let day9 = Day9::from_file("inputs/day09/input.txt");

    let mut sum_r = 0;
    let mut sum_l = 0;
    for m in day9.m.iter() {
        let er = Day9::extrap(m);
        let el = Day9::extrap_left(m);
        sum_r  += er;
        sum_l  += el;
        println!("+ {}", er);
        println!("- {}", el);
    }
    println!();
    println!("{}", sum_r);
    println!("{}", sum_l);
}
