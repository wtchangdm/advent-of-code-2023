#[allow(dead_code)]
mod solutions;
use solutions::*;

use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead};

struct Puzzle<T: Display> {
    name: String,
    input: String,
    func: fn(input: &[String]) -> T,
}

fn get_content(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap_or_else(|_| panic!("can't open file: {}", path));

    io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect()
}

fn main() {
    let puzzles = vec![
        Puzzle { name: "1-1".into(), input: "./inputs/1-1.txt".into(), func: day1::solve_part1 },
        Puzzle { name: "1-2".into(), input: "./inputs/1-1.txt".into(), func: day1::solve_part2 },
        Puzzle { name: "2-1".into(), input: "./inputs/2-1.txt".into(), func: day2::solve_part1 },
        Puzzle { name: "2-2".into(), input: "./inputs/2-1.txt".into(), func: day2::solve_part2 },
        Puzzle { name: "3-1".into(), input: "./inputs/3-1.txt".into(), func: day3::solve_part1 },
        Puzzle { name: "3-2".into(), input: "./inputs/3-1.txt".into(), func: day3::solve_part2 },
    ];

    for puzzle in puzzles {
        let content = get_content(&puzzle.input);
        println!(
            "Answer of {} ({}) is: {}",
            puzzle.name,
            puzzle.input,
            (puzzle.func)(&content)
        );
    }
}
