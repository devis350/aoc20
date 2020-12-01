#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    input.iter()
        .enumerate()
        .find_map(|(i, &num)| {
            input.iter()
                .skip(i)
                .find(|&num1| num + num1 == 2020)
                .map(|&num1| num * num1)
        }).unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {
    input.iter()
        .enumerate()
        .find_map(|(i, &num)| {
            input.iter()
                .skip(i)
                .enumerate()
                .find_map(|(j, &num1)| {
                    input.iter()
                        .skip(i+j)
                        .find(|&num2| num + num1 + num2 == 2020)
                        .map(|&num2| num * num1 * num2)
                })
        })
        .unwrap()
}
