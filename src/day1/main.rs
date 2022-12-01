use std::fs::File;
use std::io::{self, BufRead};

fn read_calories(file: &str) -> io::Result<Vec<u32>> {
    let f = File::open(file)?;

    let mut count = 0u32;
    let mut calories = Vec::new();
    for lres in io::BufReader::new(f).lines() {
        if let Ok(line) = lres {
            let trimmed_line = line.trim();

            if trimmed_line.is_empty() {
                calories.push(count);
                count = 0;
            } else {
                count += trimmed_line.parse::<u32>().expect("Expected integer!");
            }
        }
    }
    Ok(calories)
}

fn solve_pt1(calories: &[u32]) -> u32 {
    *calories.iter().max().unwrap_or(&0)
}

fn solve_pt2(calories: &[u32]) -> u32 {
    calories.iter().rev().take(3).sum()
}

fn main() {
    let calories = read_calories("data/day1/input").unwrap();

    let solution_pt1 = solve_pt1(&calories);
    println!("The solution to pt1 is {} calories!", solution_pt1);

    let solution_pt2 = solve_pt2(&calories);
    println!("The solution to pt2 is {} calories!", solution_pt2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let calories = vec![6000u32, 4000u32, 11000u32, 24000u32, 10000u32];

        assert_eq!(solve_pt1(&calories), 24000);
        assert_eq!(solve_pt2(&calories), 45000);
    }

    #[test]
    fn read_test() {
        let calories = vec![6000u32, 4000u32, 11000u32, 24000u32, 10000u32];
        assert_eq!(calories, read_calories("data/day1/test").unwrap());
    }

    #[test]
    fn edge_case_test() {
        assert_eq!(solve_pt1(&[]), 0);
        assert_eq!(solve_pt2(&[]), 0);
    }
}
