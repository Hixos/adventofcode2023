use std::collections::{HashMap, HashSet};

use aoc2023::process_lines;

#[derive(Clone, Debug)]
struct Game {
    cards: String,
    bid: u32,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
struct Hand {
    hand_type: Type,
    cards: [u8; 5],
    game_index: usize,
}

#[derive(Clone, Debug)]
struct Day7 {
    games: Vec<Game>,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum Type {
    FiveOfKind = 7,
    FourOfKind = 6,
    FullHouse = 5,
    ThreOfKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

impl Day7 {
    fn from_file(file: &str) -> Day7 {
        let mut games: Vec<Game> = vec![];

        process_lines(file, |line| {
            let mut split = line.split(" ");

            let cards = split.next().unwrap().to_owned();
            let bid = split.next().unwrap().parse::<u32>().unwrap();

            games.push(Game { cards, bid })
        });

        Day7 { games }
    }

    fn get_hands(&self) -> Vec<Hand> {
        let mut hands: Vec<Hand> = vec![];

        for (game_index, game) in self.games.iter().enumerate() {
            let mut map = HashMap::<char, u8>::new();
            let mut hand = Hand {
                cards: [0; 5],
                hand_type: Type::HighCard,
                game_index
            };
            for (i, c) in game.cards.chars().enumerate() {
                map.insert(c, *map.get(&c).unwrap_or(&0u8) + 1);
                hand.cards[i] = Self::value(c);
            }

            match map.values().len() {
                1usize => hand.hand_type = Type::FiveOfKind,
                2usize => {
                    if map.values().any(|&v| v == 4u8) {
                        hand.hand_type = Type::FourOfKind
                    } else {
                        hand.hand_type = Type::FullHouse
                    }
                }
                3usize => {
                    if map.values().any(|&v| v == 3u8) {
                        hand.hand_type = Type::ThreOfKind
                    } else {
                        hand.hand_type = Type::TwoPair
                    }
                }
                4usize => hand.hand_type = Type::OnePair,
                5usize => hand.hand_type = Type::HighCard,
                _ => {
                    panic!("Unexpected hand type!")
                }
            }

            hands.push(hand)
        }

        hands
    }

    fn value(c: char) -> u8 {
        match c {
            'A' => 13u8,
            'K' => 12u8,
            'Q' => 11u8,
            'J' => 10u8,
            'T' => 9u8,
            '9' => 8u8,
            '8' => 7u8,
            '7' => 6u8,
            '6' => 5u8,
            '5' => 4u8,
            '4' => 3u8,
            '3' => 2u8,
            '2' => 1u8,
            _ => {
                panic!("Unrecognized char: {}", c)
            }
        }
    }
}

fn main() {
    let day7 = Day7::from_file("inputs/day07/input.txt");

    let mut hands = day7.get_hands();
    hands.sort();

    let mut out = 0;
    for (i, hand) in hands.iter().enumerate() {
        let game = day7.games.get(hand.game_index).unwrap();
        out += game.bid as u64 * (i + 1) as u64;

        println!("{:?} {:?}", game, hand);
    }

    println!("{}", out);
    println!("{}", out);

}