use std::{ops::Range};
enum Overlaps<'a> {
    Contained(&'a Range<u32>, &'a Range<u32>),
    None
}


fn range_any<'a>(a: &'a Range<u32>, b: &'a Range<u32>) -> bool {
    a.start <= b.end && a.end >= b.start
}

fn range_overlap<'a>(a: &'a Range<u32>, b: &'a Range<u32>) -> Overlaps<'a> {
    let a_in_b = a.start <= b.start && b.end <= a.end;
    let b_in_a = b.start <= a.start && a.end <= b.end;
    if a_in_b || b_in_a {
        Overlaps::Contained(a, b)
    } else {
        Overlaps::None
    }
}

fn split_group(line: &str) -> Range<u32> {
    let line_split: Vec<&str> = line.split("-").collect();
    String::from(line_split[0]).parse().unwrap()..String::from(line_split[1]).parse().unwrap()
}

fn parse(input: &str) ->  Vec<(Range<u32>, Range<u32>)>{
    input.lines().map(|line| {
        let line_split: Vec<&str> = line.split(",").collect();
        (split_group(line_split[0]), split_group(line_split[1]))
    }).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let parsed = parse(input).into_iter().filter_map(|(a, b)| match (range_overlap(&a, &b), range_overlap(&b, &a)) {
        (Overlaps::Contained(_a, _), _) => Some(a),
        (_, Overlaps::Contained(_a, _)) => Some(b),
        _ => None
    }).count();
    println!("{}", &parsed);
    Some(parsed as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let parsed = parse(input).into_iter().filter_map(|(a, b)| match range_any(&a, &b) {
        true => Some(a),
        _ => None
    }).count();
    println!("{}", &parsed);
    Some(parsed as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
