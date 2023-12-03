use std::fs::File;
use std::io::{self, BufReader, prelude::*};
use std::path::Path;

fn read_lines<P>(path: P) -> io::Result<io::Lines<BufReader<File>>> 
where P: AsRef<Path> {
    let reader = BufReader::new(File::open(path)?);
    Ok(reader.lines())
}

pub fn process_lines(file: &str, mut fun: impl FnMut(&String)) {
    if let Ok(lines) = read_lines(file) {
        for l in lines {
            if let Ok(l) = l {
                fun(&l);
            }
        }
    }
}