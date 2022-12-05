
use itertools::Itertools;
use std::cmp::Reverse;

type Input = Vec<u32>;

fn parse(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|c| c.lines().filter_map(|l| l.parse::<u32>().ok()).sum())
        .collect()
}

pub fn part_one(calorie_counts: &str) -> Option<u32> {
    let calorie_counts = parse(calorie_counts);
    calorie_counts.iter().max().copied()
}

pub fn part_two(calorie_counts: &str) -> Option<u32> {
    let calorie_counts = parse(calorie_counts);
    Some(
        calorie_counts
            .iter()
            .sorted_by_key(|x| Reverse(*x))
            .take(3)
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::read_file("examples", 1));
        assert_eq!(result, Some(24000));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::read_file("examples", 1));
        assert_eq!(result, Some(45000));
    }
}
