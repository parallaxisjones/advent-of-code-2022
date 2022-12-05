type Input = Vec<(u32, u32)>;

fn parse(input: &str) -> Input {
    input
        .lines()
        .filter_map(|l| {
            // normalize to index 0 with ASCII table. (A -> 65 | X -> 88)
            let bytes = l.as_bytes();
            Some(((bytes.first()? - 65) as u32, (bytes.last()? - 88) as u32))
        })
        .collect()
}

fn score_distance(theirs: u32, ours: u32) -> u32 {
    //The score for a single round is the score for the *shape you selected* (1 for Rock, 2 for Paper, and 3 for Scissors)
    //plus the score for the *outcome of the round* (0 if you lost, 3 if the round was a draw, and 6 if you won).
    //
    // game result can be determined by calculating wrapped distance.
    // use modulo to map indexes (0, 1, 2) to (loss, draw, win) for multiplication with 3.
    let score = (3 - (2 + theirs - ours) % 3) % 3 * 3;
    score + ours + 1
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse(input);
    // Your *total score* is the sum of your scores for each round.
    //
    // Left column is "their move": A means Rock, B means Paper, C means Scissors.
    // Right column is "our move": X means Rock, Y means Paper, Z means Scissors.
    // Each line corresponds to a turn, and we must calculate the total score we get.
    // Picking "Rock" gives 1 point, "Paper" gives 2 points, and "Scissors" gives 3.
    // Losing the round gives 0 points, drawing gives 3, winning it gives 6.

    // Line A Y means they picked "Rock", we picked "Paper" (2 points), and we won (6 points), so our score goes up by 8.
    // Line B X means they picked "Paper", we picked "Rock" (1 point), and we lost, so our score goes up by 1.
    // Line C Z means we both picked "Scissors" (3 points) and it's a draw (3 points),
    // so our score goes up by 6, for a grand total of 8 + 1 + 6 = 15.
    Some(
        input
            .iter()
            .map(|&(theirs, ours)| score_distance(theirs, ours))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse(input);
    let score = input
        .iter()
        .map(|&(theirs, result)| {
            // reverse scoring method to find our strategy for round.
            let ours = match result {
                0 => (theirs + 2) % 3,
                1 => theirs,
                2 => (theirs + 1) % 3,
                _ => unreachable!(),
            };

            score_distance(theirs, ours)
        })
        .sum();

    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::read_file("examples", 2));
        assert_eq!(result, Some(15));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::read_file("examples", 2));
        assert_eq!(result, Some(12));
    }
}
