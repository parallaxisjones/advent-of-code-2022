const ALPHA_POSITION: u8 = 0b00011111;
const ASCII_LOWER_A: u8 = 97;
const UPPER_PRIORITY_OFFSET: u8 = 26;
type Input<'a> = Vec<&'a str>;

fn parse(input: &str) -> Input {
    input.lines().collect()
}

fn score_char(code: u8) -> u8 {
    code % 32 + (26 * (code <= 90) as u8)
}

fn priority(char_code: u8) -> u8 {
    if char_code < ASCII_LOWER_A {
        (char_code & ALPHA_POSITION) + UPPER_PRIORITY_OFFSET
    } else {
        char_code & ALPHA_POSITION
    }
}


fn bucket_line(l: &str) -> Option<u32> {
    let (a, b) = l.split_at(l.len() / 2);
    let a = a.as_bytes();
    let b = b.as_bytes();
    a.iter()
        .find(|byte| b.contains(byte))
        .map(|&byte| priority(byte) as u32)
}

fn filter_sum_buckets(input: &str) -> u32 {
    input.lines().filter_map(bucket_line).sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(filter_sum_buckets(input))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(parse(input)
    .chunks(3)
    .filter_map(|chunks| {
        let mut chunks = chunks.iter();
        let a = chunks.next()?.as_bytes();
        let b = chunks.next()?.as_bytes();
        let c = chunks.next()?.as_bytes();
        a.iter()
            .find(|byte| b.contains(byte) && c.contains(byte))
            .map(|&byte| score_char(byte) as u32)
    })
    .sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
