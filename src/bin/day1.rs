use std::fs::File;
use std::io::{self, BufReader, prelude::*};
use std::path::Path;

const DIGIT_WORDS: [&str; 20] = [ "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn get_digit(line_chars: impl Iterator<Item = char> + Clone, rev: bool) -> u32 {
    let mut digit_progress = [0usize; 20];

    let word_iter =  if rev{
        | word_i: usize | -> Box<dyn Iterator<Item = char>> { Box::new(DIGIT_WORDS[word_i].chars().rev()) }
    }else{
        | word_i: usize | -> Box<dyn Iterator<Item = char>> { Box::new(DIGIT_WORDS[word_i].chars()) }
    };

    let main_iter = line_chars.clone();

    for (i, c) in main_iter.enumerate() {
        for (word_i, word_progress) in digit_progress.iter_mut().enumerate() {
            if word_iter(word_i).nth(*word_progress).unwrap() == c {
                *word_progress += 1;
            }else{
                let mut i_back = i - *word_progress + 1;
                *word_progress = 0;
                while i_back <= i {
                    let c_back = line_chars.clone().nth(i_back).unwrap();
                    if word_iter(word_i).nth(*word_progress).unwrap() == c_back {
                        *word_progress += 1;
                    }
                    i_back += 1;
                }
            }

            if *word_progress == DIGIT_WORDS[word_i].len() {
                return (word_i % 10) as u32;
            }
        }
    }
    panic!("No digit found !");
}

fn get_calibration(line: &String) -> u32 {
    get_digit(line.chars(), false) * 10 + get_digit(line.chars().rev(), true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forwards() {
        let cases = [
            ("six0", 6), 
            ("60", 6),
            ("onine0", 9),
            ("ninine0", 9),
            ("ninininionnine0", 9),
            ("nin6", 6),
        ];

        for (s, r) in cases {
            assert_eq!(r, get_digit(s.chars(), false));
        }
    }

    #[test]
    fn test_backwards() {
        let cases = [
            ("0six", 6), 
            ("06", 6),
            ("0onine", 9),
            ("0nine", 9),
            ("0ninenenenene", 9),
            ("6nin", 6),
        ];

        for (s, r) in cases {
            assert_eq!(r, get_digit(s.chars().rev(), true));
        }
    }

    #[test]
    fn test_calibration() {
        let cases = [
            ("1abc2", 12), 
            ("pqr3stu8vwx", 38),
            ("a1b2c3d4e5f", 15),
            ("treb7uchet", 77),
            ("nine561six", 96),
            ("six", 66),
            ("6", 66),
            ("nininininenenenene", 99),
        ];

        for (s, r) in cases {
            assert_eq!(r, get_calibration(&s.to_owned()));
        }
    }
}

fn main() {

    if let Ok(lines) = read_lines("inputs/day1.txt") {
        let mut sum = 0u32;

        for l in lines {
            if let Ok(l) = l {
                sum += get_calibration(&l);
            }
        }

        println!("{}", sum);
    }
}

fn read_lines<P>(path: P) -> io::Result<io::Lines<BufReader<File>>> 
where P: AsRef<Path> {
    let reader = BufReader::new(File::open(path)?);
    Ok(reader.lines())
}