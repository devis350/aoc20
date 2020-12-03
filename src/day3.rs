#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<bool>> {
    input.lines().into_iter()
        .map(|l| {
            l.chars()
                .map(|c| c == '#')
                .collect()
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Vec<Vec<bool>>) -> usize {
    check(input, 3, 1)
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Vec<Vec<bool>>) -> usize {
    check(input, 1, 1)
        * check(input, 3, 1)
        * check(input, 5, 1)
        * check(input, 7, 1)
        * check(input, 1, 2)
}

pub fn check(input: &Vec<Vec<bool>>, right: usize, down: usize) -> usize {
    input.iter()
        .enumerate()
        .step_by(down)
        .filter(|(i, trees)| trees[(i /down *right) % trees.len()])
    .count()
}
