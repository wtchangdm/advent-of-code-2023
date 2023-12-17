#[allow(dead_code)]
mod solutions;
use solutions::*;

use std::fs::File;
use std::io::{self, BufRead};

fn get_content(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap_or_else(|_| panic!("can't open file: {}", path));

    io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect()
}

fn main() {
    println!("Answer of 1-1 is: {}", day1::solve_part1(&get_content("./inputs/1-1.txt")));
    println!("Answer of 1-2 is: {}", day1::solve_part2(&get_content("./inputs/1-1.txt")));
    println!("Answer of 2-1 is: {}", day2::solve_part1(&get_content("./inputs/2-1.txt")));
    println!("Answer of 2-2 is: {}", day2::solve_part2(&get_content("./inputs/2-1.txt")));
    println!("Answer of 3-1 is: {}", day3::solve_part1(&get_content("./inputs/3-1.txt")));
    println!("Answer of 3-2 is: {}", day3::solve_part2(&get_content("./inputs/3-1.txt")));
    println!("Answer of 4-1 is: {}", day4::solve_part1(&get_content("./inputs/4-1.txt")));
    println!("Answer of 4-2 is: {}", day4::solve_part2(&get_content("./inputs/4-1.txt")));
    println!("Answer of 5-1 is: {}", day5::solve_part1(&get_content("./inputs/5-1.txt")));
    // println!("Answer of 5-2 is: {}", day5::solve_part2(&get_content("./inputs/5-1.txt")));
    println!("Answer of 6-1 is: {}", day6::solve_part1(&get_content("./inputs/6-1.txt")));
    println!("Answer of 6-2 is: {}", day6::solve_part2(&get_content("./inputs/6-1.txt")));
    println!("Answer of 7-1 is: {}", day7::solve_part1(&get_content("./inputs/7-1.txt")));
    println!("Answer of 7-2 is: {}", day7::solve_part2(&get_content("./inputs/7-1.txt")));
}
