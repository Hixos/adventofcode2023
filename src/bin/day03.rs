use std::ops::Range;

use aoc2023::process_lines;
use regex::Regex;

struct Day3 {
    lines: Vec<String>,
}

impl Day3 {
    fn from_file(file: &str) -> Day3 {
        let mut lines = vec![];
        process_lines(file, |line| lines.push(line.clone()));

        Day3 { lines }
    }

    fn get_codes(&self) -> Vec<u32> {
        let mut codes = vec![];
        let r_num = Regex::new(r"\.*(\d+)").unwrap();

        for (i, l) in self.lines.iter().enumerate() {
            let mut search_i = 0usize;
            while let Some(cap) = r_num.captures_at(&l, search_i) {
                let c = cap.get(1).unwrap();
                let code = c.as_str().parse::<u32>().unwrap();

                search_i = c.range().end;

                let mut v_range = c.range();
                // Check left side
                if v_range.start > 0 {
                    v_range.start -= 1;
                    if Self::contains_symbol(
                        &l,
                        &Range {
                            start: v_range.start,
                            end: v_range.start + 1,
                        },
                    ) {
                        codes.push(code);
                        continue;
                    }
                }

                // Right side
                if v_range.end < l.len() {
                    v_range.end += 1;
                    if Self::contains_symbol(
                        &l,
                        &Range {
                            start: v_range.end - 1,
                            end: v_range.end,
                        },
                    ) {
                        codes.push(code);
                        continue;
                    }
                }

                // Top side
                if i > 0 && Self::contains_symbol(&self.lines[i - 1], &v_range) {
                    codes.push(code);
                    continue;
                }

                // Bottom side
                if i < self.lines.len() - 1 && Self::contains_symbol(&self.lines[i + 1], &v_range) {
                    codes.push(code);
                    continue;
                }
            }
        }

        codes
    }

    fn contains_symbol(line: &str, range: &Range<usize>) -> bool {
        for (i, c) in line.chars().enumerate().skip(range.start) {
            if i >= range.end {
                return false;
            }
            if !c.is_digit(10) && c != '.' {
                return true;
            }
        }

        false
    }
}

fn main() {
    let day3 = Day3::from_file("inputs/day03/input.txt");
    let codes = day3.get_codes();
    println!("Sum code: {:?}", codes.iter().sum::<u32>());
}

#[cfg(test)]
mod tests {
    use crate::Day3;

    #[test]
    fn test_has_symbol() {
        let cases = [
            ("...*...", 0, 7, true),
            ("......*", 0, 7, true),
            ("*......", 0, 7, true),
            (".......", 0, 7, false),
            ("..123..", 0, 7, false),
            (".*123..", 0, 7, true),
            (".*123..", 0, 7, true),
            (".....*.", 5, 7, true),
            ("......*", 5, 7, true),
            ("......*", 6, 7, true),
            ("......*", 0, 6, false),
            (".*.....", 5, 7, false),
            (".*.....", 1, 2, true),
            (".*.....", 1, 1, false),
        ];

        for (str, start, end, result) in cases {
            assert_eq!(
                result,
                Day3::contains_symbol(str, &std::ops::Range::<usize> { start, end })
            );
        }
    }
}
