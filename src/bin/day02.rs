use core::panic;

use aoc2023::process_lines;

struct Day2 {
    n_red: u32,
    n_green: u32,
    n_blue: u32,

    games: Vec<Game>
}

impl Day2 {
    fn new(red: u32, green: u32, blue: u32) -> Day2 {
        Day2 {n_red: red, n_blue: blue, n_green: green, games: vec![]}
    }

    fn process_input(&mut self, line: &String) {
        let game = Game::from_string(line.as_str());

        self.games.push(game);
    }

    fn sum_possible(&self) -> u32 {
        let mut id_sum: u32 = 0u32;
        'outer: for game in self.games.iter() {
            id_sum += game.id;
            for extr in game.extractions.iter() {
                if extr.n_blue > self.n_blue || extr.n_green > self.n_green || extr.n_red > self.n_red {
                    id_sum -= game.id;
                    continue 'outer;
                }
            }
        }

        id_sum
    }

    fn min_power_set(&self) -> u32 {
        let mut power_sum: u32 = 0u32;
        for game in self.games.iter() {
            let mut min_extr = Extraction::default();

            for extr in game.extractions.iter() {
                if extr.n_blue > min_extr.n_blue {
                    min_extr.n_blue = extr.n_blue;
                }
                if extr.n_green > min_extr.n_green {
                    min_extr.n_green = extr.n_green;
                }
                if extr.n_red > min_extr.n_red {
                    min_extr.n_red = extr.n_red;
                }
            }

            power_sum += min_extr.n_red * min_extr.n_blue * min_extr.n_green;
        }

        power_sum
    }
}

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    extractions: Vec<Extraction>,
}

impl Game {
    fn from_string(s: &str) -> Game {
        let mut game_data = s.split(": ");
        let id = game_data
            .by_ref()
            .next()
            .unwrap()
            .split(" ")
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let mut extractions: Vec<Extraction> = vec![];
        for ex_str in game_data.next().unwrap().split("; ") {
            extractions.push(Extraction::from_string(ex_str));
        }

        Game { id, extractions }
    }
}

#[derive(Debug, Clone, Default)]
struct Extraction {
    n_red: u32,
    n_green: u32,
    n_blue: u32,
}

impl Extraction {
    fn from_string(s: &str) -> Extraction {
        let mut extraction = Extraction {
            n_red: 0,
            n_green: 0,
            n_blue: 0,
        };

        for ex in s.split(", ") {
            let data = ex.split(" ").collect::<Vec<_>>();
            let n = data[0].parse::<u32>().unwrap();
            match data[1] {
                "red" => extraction.n_red += n,
                "green" => extraction.n_green += n,
                "blue" => extraction.n_blue += n,
                _ => {
                    panic!("Unexpected color: {}", data[1])
                }
            }
        }

        extraction
    }
}

fn main() {
    let mut d2 = Day2::new(12, 13, 14);

    process_lines("inputs/day2.txt", |line| d2.process_input(line));

    println!("Possible games: {}", d2.sum_possible());
    println!("Power set: {}", d2.min_power_set());
}
