use std::{
    collections::{HashMap, HashSet},
    vec,
};

use aoc2023::process_lines;

#[derive(Debug)]
struct Day10 {
    map: Vec<Vec<char>>,
    pipes: HashMap<char, Pipe>,
    s_x: usize,
    s_y: usize,
    n_x: usize,
    n_y: usize,
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Dir {
    Top,
    Right,
    Bottom,
    Left,
    None,
}

impl Dir {
    fn reverse(&self) -> Dir {
        match &self {
            Dir::Top => Dir::Bottom,
            Dir::Right => Dir::Left,
            Dir::Bottom => Dir::Top,
            Dir::Left => Dir::Right,
            Dir::None => Dir::None,
        }
    }
}

#[derive(Debug)]
struct Pipe {
    dirs: [Dir; 2],
}

impl Pipe {
    fn new(dir1: Dir, dir2: Dir) -> Self {
        Pipe { dirs: [dir1, dir2] }
    }

    fn next_dir(&self, prev_dir: Dir) -> Option<Dir> {
        let prev_dir = prev_dir.reverse();
        if self.dirs[0] == prev_dir {
            Some(self.dirs[1])
        } else if self.dirs[1] == prev_dir {
            Some(self.dirs[0])
        } else {
            None
        }
    }
}

impl Day10 {
    fn from_file(file: &str) -> Day10 {
        let mut map: Vec<Vec<char>> = vec![];

        process_lines(file, |line| {
            map.push(line.chars().collect());
        });

        for (x, line) in map.iter().enumerate() {
            for (y, c) in line.iter().enumerate() {
                if *c == 'S' {
                    return Day10 {
                        map: map.clone(),
                        pipes: Self::pipes(),
                        s_x: x,
                        s_y: y,
                        n_x: map.len(),
                        n_y: line.len(),
                    };
                }
            }
        }
        panic!("Starting position not found!");
    }

    fn pipes() -> HashMap<char, Pipe> {
        let mut h = HashMap::new();

        h.insert('|', Pipe::new(Dir::Top, Dir::Bottom));
        h.insert('-', Pipe::new(Dir::Left, Dir::Right));

        h.insert('F', Pipe::new(Dir::Bottom, Dir::Right));
        h.insert('J', Pipe::new(Dir::Top, Dir::Left));
        h.insert('L', Pipe::new(Dir::Top, Dir::Right));
        h.insert('7', Pipe::new(Dir::Bottom, Dir::Left));
        h.insert('.', Pipe::new(Dir::None, Dir::None));

        h
    }

    fn find_loop(&mut self) -> (u32, u32) {
        let dirs = [
            (Dir::Top, (self.s_x - 1, self.s_y)),
            (Dir::Bottom, (self.s_x + 1, self.s_y)),
            (Dir::Left, (self.s_x, self.s_y - 1)),
            (Dir::Right, (self.s_x, self.s_y + 1)),
        ];

        for (prev_dir, (x, y)) in dirs {
            if let Some((count, path, dir)) = self.follow_pipe(prev_dir, x, y) {
                let start = (prev_dir, dir.reverse());
                for (k, v) in self.pipes.iter() {
                    if v.dirs.contains(&start.0) && v.dirs.contains(&start.1) {
                        // Also updates the starting position with the correct pipe
                        self.map[self.s_x][self.s_y] = *k;
                    }
                }
                return (count, self.calc_area(path));
            }
        }

        panic!("No loop found!");
    }

    fn follow_pipe(
        &self,
        mut prev_dir: Dir,
        mut x: usize,
        mut y: usize,
    ) -> Option<(u32, HashSet<(usize, usize)>, Dir)> {
        let mut steps = 0;
        let mut path: HashSet<(usize, usize)> = HashSet::new();

        while prev_dir != Dir::None {
            let p = self.map[x][y];

            path.insert((x, y));

            if p == 'S' {
                return Some((steps, path, prev_dir));
            }

            let pipe = self.pipes.get(&p).unwrap();
            if let Some(next) = pipe.next_dir(prev_dir) {
                steps += 1;
                prev_dir = match next {
                    Dir::Top => {
                        if x > 0 {
                            x -= 1;
                            next
                        } else {
                            Dir::None
                        }
                    }
                    Dir::Bottom => {
                        if x < self.n_x - 1 {
                            x += 1;
                            next
                        } else {
                            Dir::None
                        }
                    }
                    Dir::Left => {
                        if y > 0 {
                            y -= 1;
                            next
                        } else {
                            Dir::None
                        }
                    }
                    Dir::Right => {
                        if y < self.n_y - 1 {
                            y += 1;
                            next
                        } else {
                            Dir::None
                        }
                    }
                    _ => Dir::None,
                }
            } else {
                break;
            }
        }

        None
    }

    fn calc_area(&self, path: HashSet<(usize, usize)>) -> u32 {
        let mut area = 0;
        for (x, line) in self.map.iter().enumerate() {
            let mut inside = false;
            let mut prev_dir: Dir = Dir::None;

            for (y, char) in line.iter().enumerate() {
                if path.contains(&(x, y)) {
                    let mut toggle = false;
                    match char {
                        '|' => toggle = true,
                        '-' => {}
                        _ => {
                            let dir = self.pipes.get(char).unwrap().dirs[0];
                            if prev_dir == Dir::None {
                                prev_dir = dir;
                            } else {
                                toggle = prev_dir != dir;
                            }
                        }
                    }
                    if toggle {
                        inside = !inside;
                    }
                } else if inside {
                    println!("{:?}", (x, y));
                    area += 1;
                }
            }
        }

        area
    }
}

fn main() {
    let mut day10 = Day10::from_file("inputs/day10/input.txt");

    let (count, area) = day10.find_loop();
    println!("{}", (count + 1) / 2);
    println!("{}", area);
}
