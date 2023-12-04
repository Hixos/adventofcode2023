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

    fn from_lines(lines: &Vec<&str>) -> Day3 {
        let lines = lines.iter().map(|&v| v.to_owned()).collect::<Vec<String>>();
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

    fn get_gear_ratios(&self) -> Vec<u32> {
        let mut ratios = vec![];

        for (i, line) in self.lines.iter().enumerate() {
            let mut slice_idx = 0usize;
            while let Some(star_idx) = line[slice_idx..].find("*") {
                let star_idx = star_idx + slice_idx;
                slice_idx = star_idx + 1;
                if let Some(ratio) = self.get_gear_ratio(i, star_idx) {
                    ratios.push(ratio);
                }
            }
        }

        ratios
    }

    fn get_gear_ratio(&self, line_idx: usize, star_index: usize) -> Option<u32> {
        // Assumption: There are at most adjacent numbers to the provided asterisk

        let line = &self.lines[line_idx];

        let mut num_range = Range {
            start: star_index,
            end: star_index + 1,
        };

        let mut nums: Vec<u32> = vec![];

        if num_range.start > 0 {
            num_range.start -= 1;
        }
        if num_range.end < line.len() {
            num_range.end += 1;
        }

        if let Some(mut num) = Self::get_full_number_from_range(&self.lines[line_idx], &num_range) {
            nums.append(&mut num);
        }

        if line_idx > 0 {
            if let Some(mut num) =
                Self::get_full_number_from_range(&self.lines[line_idx - 1], &num_range)
            {
                nums.append(&mut num);
            }
        }

        if line_idx < self.lines.len() - 1 {
            if let Some(mut num) =
                Self::get_full_number_from_range(&self.lines[line_idx + 1], &num_range)
            {
                nums.append(&mut num);
            }
        }

        assert!(nums.len() <= 2, "More than two gears!");

        if nums.len() == 2 {
            println!(
                "Line: {}, star: {}, {:?}",
                line_idx + 1,
                star_index + 1,
                nums
            );
            Some(nums.iter().product::<u32>())
        } else {
            None
        }
    }

    fn get_full_number_from_range(line: &str, range: &Range<usize>) -> Option<Vec<u32>> {
        let mut nums = vec![];

        enum State {
            Num,
            Sep,
        }

        let mut state = State::Num;

        for (i, _) in line[range.clone()].chars().enumerate() {
            let oi = i + range.start;
            match state {
                State::Num => {
                    if let Some(num) = Day3::get_full_number(line, oi) {
                        nums.push(num);
                        state = State::Sep;
                    }
                }
                State::Sep => {
                    if !line.chars().nth(oi).unwrap().is_digit(10) {
                        state = State::Num;
                    }
                }
            }
        }

        if nums.len() > 0 {
            Some(nums)
        } else {
            None
        }
    }
    fn get_full_number(line: &str, digit_idx: usize) -> Option<u32> {
        if !line.chars().nth(digit_idx).unwrap().is_digit(10) {
            return None;
        }

        let mut num_range = Range {
            start: digit_idx,
            end: digit_idx + 1,
        };

        while num_range.start > 0 && line.chars().nth(num_range.start - 1).unwrap().is_digit(10) {
            num_range.start -= 1;
        }

        while num_range.end < line.len() && line.chars().nth(num_range.end).unwrap().is_digit(10) {
            num_range.end += 1;
        }

        Some(line[num_range].parse::<u32>().unwrap())
    }
}

fn main() {
    let day3 = Day3::from_file("inputs/day03/input.txt");
    let codes = day3.get_codes();
    println!("Sum code: {}", codes.iter().sum::<u32>());

    let ratios = day3.get_gear_ratios();
    println!("Ratios: {:?}", ratios);
    println!(
        "Sum ratios: {}",
        ratios.iter().map(|&v| v as u64).sum::<u64>()
    );
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

    #[test]
    fn test_full_number() {
        let cases = [
            ("..123..", 0usize, None),
            ("..123..", 2usize, Some(123)),
            ("..123..", 3usize, Some(123)),
            ("..123..", 4usize, Some(123)),
            ("..123..", 5usize, None),
            ("123", 0usize, Some(123)),
            ("123", 2usize, Some(123)),
            ("...", 2usize, None),
        ];

        for (str, index, result) in cases {
            assert_eq!(result, Day3::get_full_number(str, index));
        }
    }

    #[test]
    fn test_full_number_range() {
        let cases = [
            (".123.123.", (3usize, 6usize), Some(vec![123, 123])),
            (".123.....", (0usize, 2usize), Some(vec![123])),
            (".....123.", (7usize, 9usize), Some(vec![123])),
            (".123.....", (3usize, 6usize), Some(vec![123])),
            ("...123...", (3usize, 6usize), Some(vec![123])),
            (".....123.", (3usize, 6usize), Some(vec![123])),
            (".........", (3usize, 6usize), None),
            ];

        use std::ops::Range;

        for (str, range, result) in cases {
            assert_eq!(
                result,
                Day3::get_full_number_from_range(
                    str,
                    &Range {
                        start: range.0,
                        end: range.1
                    }
                )
            );
        }
    }

    #[test]
    fn test_gears() {
        let cases = [
            (vec!["..123..", "..*....", "...1..."], vec![123]),
            (vec!["..123..", "...*...", "...1..."], vec![123]),
            (vec!["..123..", "....*..", "...1..."], vec![123]),
            (vec!["..123..", "....*..", "...1..."], vec![123]),
            (vec!["..123..", "....*1."], vec![123]),
            (vec!["..123.", "....*.", ".....1"], vec![123]),
            (vec!["..123..", "...1*..", "......."], vec![123]),
            (vec![".123.1.", "....*..", "......."], vec![123]),
            (vec![".......", "..*....", ".1.123."], vec![123]),
        ];

        for (lines, result) in cases {
            let day3 = Day3::from_lines(&lines);
            assert_eq!(result, day3.get_gear_ratios());
        }
    }
}
