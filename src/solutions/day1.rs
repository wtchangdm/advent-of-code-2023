fn transform_words_to_digits(line: &str) -> String {
    let mut s = String::new();

    for (i, _) in line.chars().enumerate() {
        let letter = line.chars().nth(i).unwrap();
        if letter.is_numeric() {
            s.push(letter);
            continue;
        }

        match &line[i..] {
            x if x.starts_with("one") => s.push('1'),
            x if x.starts_with("two") => s.push('2'),
            x if x.starts_with("three") => s.push('3'),
            x if x.starts_with("four") => s.push('4'),
            x if x.starts_with("five") => s.push('5'),
            x if x.starts_with("six") => s.push('6'),
            x if x.starts_with("seven") => s.push('7'),
            x if x.starts_with("eight") => s.push('8'),
            x if x.starts_with("nine") => s.push('9'),
            _ => (),
        }
    }

    s
}

fn get_number(line: &str) -> u32 {
    let digits: Vec<char> = line.chars().filter(|c| c.is_numeric()).collect();
    let left = digits.first().unwrap();
    let right = digits.last().unwrap();
    let mut s = String::new();
    s.push(*left);
    s.push(*right);

    s.parse::<u32>().unwrap()
}

pub fn solve_part1(input: &[String]) -> u32 {
    input.iter().map(|s| get_number(s)).sum()
}

pub fn solve_part2(input: &[String]) -> u32 {
    input
        .iter()
        .map(|s| transform_words_to_digits(s))
        .map(|s| get_number(&s))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: Vec<String> = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]
            .iter()
            .map(|x| x.to_string())
            .collect();

        assert_eq!(solve_part1(&input), 142);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = [
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect();

        assert_eq!(solve_part2(&input), 281);
    }
}
