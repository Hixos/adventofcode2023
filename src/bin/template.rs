use std::collections::HashMap;

use aoc2023::process_lines;

struct DayX {

}

impl DayX {
    fn from_file(file: &str) -> DayX {

        process_lines(file, |line| {

        });

        DayX {  }
    }

}

fn main() {
    let dayx = DayX::from_file("inputs/day07/input.txt");
}
