use std::convert::{TryFrom, TryInto};

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Password> {
    input.lines().into_iter()
        .map(|s| {
            let parts: Vec<&str> = s.split(' ').collect();
            let mm: Vec<&str> = parts[0].split('-').collect();
            Password {
                min: mm[0].parse().unwrap(),
                max: mm[1].parse().unwrap(),
                c: parts[1].chars().next().unwrap(),
                code: parts[2].to_string(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Password]) -> usize {
    input.iter()
        .filter(|p| {
            (p.min..=p.max).contains(&u32::try_from(p.code.chars().filter(|&c| c == p.c).count()).unwrap())
        })
        .count()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Password]) -> usize {
    input.iter()
        .filter(|p| {
            (p.code.chars().nth((p.min - 1).try_into().unwrap()).unwrap() == p.c) ^
                (p.code.chars().nth((p.max - 1).try_into().unwrap()).unwrap() == p.c)
        })
        .count()
}

#[derive(Debug)]
pub struct Password {
    min: u32,
    max: u32,
    c: char,
    code: String,
}
