#[allow(dead_code)]
mod solutions;
use std::{fmt::Display, fs::File, io::Read};
use solutions::*;

fn solve<T, F>(puzzle_id: &str, input_path: &str, func: F)
where
    F: Fn(&[&str]) -> T,
    T: Display,
{
    let mut file = File::open(input_path).unwrap_or_else(|_| panic!("can't open file: {}", input_path));
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let lines = content.lines().collect::<Vec<&str>>();

    println!("Answer of {} is: {}", puzzle_id, func(&lines));
}

fn main() {
    solve("1-1", "./inputs/1-1.txt", day1::solve_part1);
    solve("1-2", "./inputs/1-1.txt", day1::solve_part2);
    solve("2-1", "./inputs/2-1.txt", day2::solve_part1);
    solve("2-2", "./inputs/2-1.txt", day2::solve_part2);
    solve("3-1", "./inputs/3-1.txt", day3::solve_part1);
    solve("3-2", "./inputs/3-1.txt", day3::solve_part2);
    solve("4-1", "./inputs/4-1.txt", day4::solve_part1);
    solve("4-2", "./inputs/4-1.txt", day4::solve_part2);
    solve("5-1", "./inputs/5-1.txt", day5::solve_part1);
    // solve("5-2", "./inputs/5-1.txt", day5::solve_part2);
    solve("6-1", "./inputs/6-1.txt", day6::solve_part1);
    solve("6-2", "./inputs/6-1.txt", day6::solve_part2);
    solve("7-1", "./inputs/7-1.txt", day7::solve_part1);
    solve("7-2", "./inputs/7-1.txt", day7::solve_part2);
    solve("8-1", "./inputs/8-1.txt", day8::solve_part1);
    solve("8-2", "./inputs/8-1.txt", day8::solve_part2);
    solve("9-1", "./inputs/9-1.txt", day9::solve_part1);
    solve("9-2", "./inputs/9-1.txt", day9::solve_part2);
    solve("10-1", "./inputs/10-1.txt", day10::solve_part1);
    // solve("10-2", "./inputs/10-1.txt", day10::solve_part2);
}
