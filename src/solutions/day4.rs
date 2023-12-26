use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Card {
    id: u32,
    won_copies: u32,
}

fn parse_numbers(s: &str) -> HashSet<u32> {
    s.split_whitespace()
        .map(|c| c.parse::<u32>().unwrap())
        .collect()
}

impl Card {
    fn from(line: &str) -> Self {
        let (meta, numbers) = line.split_once(": ").unwrap();
        let id: u32 = meta.replace("Card ", "").trim().parse().unwrap();
        let (winning, owned) = numbers
            .split_once(" | ")
            .map(|(a, b)| (parse_numbers(a), parse_numbers(b)))
            .unwrap();
        let won_copies = winning.intersection(&owned).count() as u32;

        Self { id, won_copies }
    }

    fn points(&self) -> u32 {
        match self.won_copies().checked_sub(1) {
            None => 0, // negative, like -1
            Some(v) => 2_u32.pow(v),
        }
    }

    fn won_copies(&self) -> u32 {
        self.won_copies
    }
}

pub fn solve_part1(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|line| Card::from(line))
        .map(|card| card.points())
        .sum()
}

pub fn solve_part2(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|line| Card::from(line))
        .fold(HashMap::new(), |mut count, card| {
            *count.entry(card.id).or_insert(0) += 1;

            // inspired by https://github.com/rust-tw/advent-of-code/tree/main/2023/04
            (1..=card.won_copies())
                .map(|i| i + card.id)
                .for_each(|id| *count.entry(id).or_insert(0) += *count.get(&card.id).unwrap());

            count
        })
        .values()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];

        assert_eq!(solve_part1(&input), 13);
    }

    #[test]
    fn test_part2() {
        let input = [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];

        assert_eq!(solve_part2(&input), 30);
    }
}
