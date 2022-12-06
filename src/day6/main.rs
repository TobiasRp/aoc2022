use std::collections::HashSet;
use std::fs;

fn is_start_of_packet(window: &[char]) -> bool {
    let mut set = HashSet::new();
    for c in window {
        if set.contains(c) {
            return false;
        } else {
            set.insert(c);
        }
    }
    return true;
}

fn solve_day6(stream: &Vec<char>, window_size: usize) -> usize {
    for (idx, c) in stream.windows(window_size).enumerate() {
        if is_start_of_packet(c) {
            return idx + window_size;
        }
    }
    return stream.len();
}

fn main() {
    let file_str = fs::read_to_string("data/day6/input")
                                .expect("Unable to read file");
    let stream = file_str.chars().collect::<Vec<char>>();

    let solution_pt1 = solve_day6(&stream, 4);
    println!("The solution to part one is {}", solution_pt1);

    let solution_pt2 = solve_day6(&stream, 14);
    println!("The solution to part two is {}", solution_pt2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day6_examples() {
        let ex1: Vec<char> = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".chars().collect();
        assert_eq!(solve_day6(&ex1, 4), 7);
        assert_eq!(solve_day6(&ex1, 14), 19);

        let ex2: Vec<char> = "bvwbjplbgvbhsrlpgdmjqwftvncz".chars().collect();
        assert_eq!(solve_day6(&ex2, 4), 5);
        assert_eq!(solve_day6(&ex2, 14), 23);

        let ex3: Vec<char> = "nppdvjthqldpwncqszvftbrmjlhg".chars().collect();
        assert_eq!(solve_day6(&ex3, 4), 6);
        assert_eq!(solve_day6(&ex3, 14), 23);

        let ex4: Vec<char> = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".chars().collect();
        assert_eq!(solve_day6(&ex4, 4), 10);
        assert_eq!(solve_day6(&ex4, 14), 29);

        let ex5: Vec<char> = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".chars().collect();
        assert_eq!(solve_day6(&ex5, 4), 11);
        assert_eq!(solve_day6(&ex5, 14), 26);
    }
}