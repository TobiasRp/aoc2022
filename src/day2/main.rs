use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Helper function from:
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
//
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum HandShape {
    Rock = 0,
    Paper = 1,
    Scissors = 2
}

fn shape_idx(shape: HandShape) -> usize {
    shape as usize
}

fn score(shape: HandShape) -> i32 {
    shape_idx(shape) as i32 + 1
}

fn score_round(elf_shape: HandShape, my_shape: HandShape) -> i32 {
    let my_score: i32 = score(my_shape);
    let my_idx: usize = shape_idx(my_shape);
    let elf_idx: usize = shape_idx(elf_shape);

    // win/draw/lose scores for all combinatios of elf and my shapes
    // row: elf
    // col: my shape
    let scores = [3, 6, 0,
                            0, 3, 6,
                            6, 0, 3];

    my_score + scores[elf_idx * 3 + my_idx]
}

fn new_shape(c: char) -> Option<HandShape> {
    match c {
        'A' | 'X' => Some(HandShape::Rock),
        'B' | 'Y' => Some(HandShape::Paper),
        'C' | 'Z' => Some(HandShape::Scissors),
        _ => None
    }
}

fn parse_chars(line: String) -> (char, char) {
    // No error handling, expect char - whitespace - char in each line!
    let mut chars = line.chars();

    let elf_char = chars.next().expect("Expected A/B/C char!");

    chars.next();

    let my_char = chars.next().expect("Expected X/Y/Z char!");
    (elf_char, my_char)
}

enum ExpectedResult {
    Lose = 0,
    Draw = 1,
    Win = 2
}

fn expected_result(c: char) -> Option<ExpectedResult> {
    match c {
        'X' => Some(ExpectedResult::Lose),
        'Y' => Some(ExpectedResult::Draw),
        'Z' => Some(ExpectedResult::Win),
        _ => None
    }
}

fn determine_my_shape(elf_shape: HandShape, result: ExpectedResult) -> HandShape {
    let result_idx: usize = result as usize;
    let elf_idx: usize = shape_idx(elf_shape);

    // row: elf
    // col: expected result
    let my_shapes = [HandShape::Scissors, HandShape::Rock, HandShape::Paper,
                                     HandShape::Rock, HandShape::Paper, HandShape::Scissors,
                                     HandShape::Paper, HandShape::Scissors, HandShape::Rock];

    my_shapes[elf_idx * 3 + result_idx]
}

fn score_part1(elf_char: char, my_char: char) -> i32 {
    let elf_shape = new_shape(elf_char).unwrap();
    let my_shape = new_shape(my_char).unwrap();

    score_round(elf_shape, my_shape)
}

fn score_part2(elf_char: char, my_char: char) -> i32 {
    let elf_shape = new_shape(elf_char).unwrap();

    let result = expected_result(my_char).unwrap();
    let my_shape = determine_my_shape(elf_shape, result);

    score_round(elf_shape, my_shape)
}

fn solve<F: Fn(char, char) -> i32>(file: &str, score_fn: F) -> i32 {
    let mut total_score = 0;

    let lines = read_lines(file).unwrap();
    for rline in lines {
        if let Ok(line) = rline {
            let (elf_char, my_char) = parse_chars(line);

            total_score += score_fn(elf_char, my_char);
        }
    }
    total_score
}

fn main() {
    let score_pt1 = solve("data/day2/input", score_part1);
    let score_pt2 = solve("data/day2/input", score_part2);

    println!("Total score in Pt. 1 is {}", score_pt1);
    println!("Total score in Pt. 2 is {}", score_pt2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scores() {
        // draw
        assert_eq!(4, score_round(HandShape::Rock, HandShape::Rock));
        assert_eq!(5, score_round(HandShape::Paper, HandShape::Paper));
        assert_eq!(6, score_round(HandShape::Scissors, HandShape::Scissors));

        // win
        assert_eq!(7, score_round(HandShape::Scissors, HandShape::Rock));
        assert_eq!(8, score_round(HandShape::Rock, HandShape::Paper));
        assert_eq!(9, score_round(HandShape::Paper, HandShape::Scissors));

        // lose
        assert_eq!(1, score_round(HandShape::Paper, HandShape::Rock));
        assert_eq!(2, score_round(HandShape::Scissors, HandShape::Paper));
        assert_eq!(3, score_round(HandShape::Rock, HandShape::Scissors));
    }

    #[test]
    fn test_results() {
        // lose
        assert_eq!(HandShape::Rock, determine_my_shape(HandShape::Paper, ExpectedResult::Lose));
        assert_eq!(HandShape::Paper, determine_my_shape(HandShape::Scissors, ExpectedResult::Lose));
        assert_eq!(HandShape::Scissors, determine_my_shape(HandShape::Rock, ExpectedResult::Lose));

        // draw
        assert_eq!(HandShape::Paper, determine_my_shape(HandShape::Paper, ExpectedResult::Draw));
        assert_eq!(HandShape::Scissors, determine_my_shape(HandShape::Scissors, ExpectedResult::Draw));
        assert_eq!(HandShape::Rock, determine_my_shape(HandShape::Rock, ExpectedResult::Draw));

        // win
        assert_eq!(HandShape::Scissors, determine_my_shape(HandShape::Paper, ExpectedResult::Win));
        assert_eq!(HandShape::Rock, determine_my_shape(HandShape::Scissors, ExpectedResult::Win));
        assert_eq!(HandShape::Paper, determine_my_shape(HandShape::Rock, ExpectedResult::Win));
    }

    #[test]
    fn test_after_completion() {
        assert_eq!(14069, solve("data/day2/input", score_part1));
        assert_eq!(12411, solve("data/day2/input", score_part2));
    }
}