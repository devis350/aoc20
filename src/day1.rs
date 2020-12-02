use std::num::ParseIntError;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.lines().into_iter().map(str::parse).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> Option<u32> {
    input.iter()
        .enumerate()
        .find_map(|(i, &num)| {
            input.iter()
                .skip(i)
                .find_map(|&num1| if num + num1 == 2020 { Some(num * num1) } else { None })
        })
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u32]) -> Option<u32> {
    input.iter()
        .enumerate()
        .find_map(|(i, &num)| {
            input.iter()
                .skip(i)
                .enumerate()
                .find_map(|(j, &num1)| {
                    input.iter()
                        .skip(i + j)
                        .find_map(|&num2| if num + num1 + num2 == 2020 { Some(num * num1 * num2) } else { None })
                })
        })
}
