use std::collections::HashSet;

use aoc2023::process_lines;

struct Day4 {
    cards: Vec<Card>,
}

struct Card {
    index: usize,
    copies: u32,
    winning_set: HashSet<u32>,
    winning_vec: Vec<u32>,
    chosen_numbers: Vec<u32>,
}

impl Card {
    fn prize_from_vec(&self) -> u32 {
        let mut win = 1u32;
        for n in self.chosen_numbers.iter() {
            for w in self.winning_vec.iter() {
                if n == w {
                    win <<= 1;
                    break;
                }
            } 
        }

        win >> 1
    }

    fn prize_from_set(&self) -> u32 {
        let mut win = 1u32;
        for n in self.chosen_numbers.iter() {
            if self.winning_set.contains(n) {
                win <<= 1;
            }
        }

        win >> 1
    }

    fn wins_vec(&self) -> u32 {
        let mut win = 0u32;
        for n in self.chosen_numbers.iter() {
            for w in self.winning_vec.iter() {
                if n == w {
                    win += 1;
                    break;
                }
            } 
        }

        win
    }

    fn wins_set(&self) -> u32 {
        let mut win = 0u32;
        for n in self.chosen_numbers.iter() {
            if self.winning_set.contains(n) {
                win += 1;
            }
        }

        win
    }
}

impl Day4 {
    fn from_file(file: &str) -> Day4 {
        let mut cards = vec![];

        process_lines(file, |line| {
            let card = Day4::card_from_line(line);
            cards.push(card);
        });

        Day4 { cards }
    }

    fn from_lines(lines: &Vec<&str>) -> Day4 {
        let mut cards = vec![];
        for &l in lines {
            let card = Day4::card_from_line(l.to_owned());
            cards.push(card);
        }
        Day4 { cards }
    }
    
    fn total_points_vec(&self) -> u32 {
        let mut sum = 0u32;
        for c in self.cards.iter() {
            sum += c.prize_from_vec();
        }
        sum
    }

    fn total_points_set(&self) -> u32 {
        let mut sum = 0u32;
        for c in self.cards.iter() {
            sum += c.prize_from_set();
        }
        sum
    }

    fn count_copies_vec(&self) -> u32 {
        let mut copies = vec![1u32; self.cards.len()];
        let mut num_cards = 0u32;

        for (i, card) in self.cards.iter().enumerate() {
            let n = copies[i];
            num_cards += n;
            let w = card.wins_vec() as usize;

            for j in i+1..std::cmp::min(self.cards.len(), i+1+w) {
                copies[j] += n;
            }
        }

        num_cards
    }

    fn count_copies_set(&self) -> u32 {
        let mut copies = vec![1u32; self.cards.len()];
        let mut num_cards = 0u32;

        for (i, card) in self.cards.iter().enumerate() {
            let n = copies[i];
            num_cards += n;
            let w = card.wins_set() as usize;

            for j in i+1..std::cmp::min(self.cards.len(), i+1+w) {
                copies[j] += n;
            }
        }

        num_cards
    }

    fn card_from_line(line: String) -> Card {
        let mut split_card = line.split(": ");
        let index = split_card
            .next()
            .unwrap()
            .split(" ")
            .filter(|v|!v.is_empty())
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let mut split_numbers = split_card.next().unwrap().split(" | ");
        let winning_vec = split_numbers
            .next()
            .unwrap()
            .split(" ")
            .filter(|v| !v.is_empty())
            .map(|v| v.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let chosen_numbers = split_numbers
            .next()
            .unwrap()
            .split(" ")
            .filter(|v| !v.is_empty())
            .map(|v| v.trim().parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let winning_set: HashSet<u32> = HashSet::from_iter(winning_vec.iter().cloned());
        Card {
            index,
            copies: 1,
            winning_set,
            winning_vec,
            chosen_numbers,
        }
    }

}
fn main() {
    let day4 = Day4::from_file("inputs/day04/input.txt");

    println!("Total vec: {}", day4.total_points_vec());
    println!("Total vec: {}", day4.total_points_set());


    println!("Copies vec: {}", day4.count_copies_vec());
    println!("Copies set: {}", day4.count_copies_set());
}
