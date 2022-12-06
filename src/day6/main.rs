use std::collections::VecDeque;
use std::collections::HashSet;
use std::fs;

fn is_start_of_packet(window: &VecDeque<char>) -> bool {
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

fn solve_day6(stream: &str, window_size: usize) -> usize {
    let mut window = VecDeque::new();

    for (idx, c) in stream.chars().into_iter().enumerate() {
        if window.len() >= window_size {
            window.pop_back();
        }
        window.push_front(c);

        if window.len() == window_size && is_start_of_packet(&window) {
            return idx + 1;
        }
    }

    return stream.len();
}

fn main() {
    let file_str = fs::read_to_string("data/day6/input")
                                .expect("Unable to read file");

    let solution_pt1 = solve_day6(file_str.as_str(), 4);
    println!("The solution to part one is {}", solution_pt1);

    let solution_pt2 = solve_day6(file_str.as_str(), 14);
    println!("The solution to part two is {}", solution_pt2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day6_examples() {
        let ex1 = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(solve_day6(ex1, 4), 7);
        assert_eq!(solve_day6(ex1, 14), 19);

        let ex2 = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(solve_day6(ex2, 4), 5);
        assert_eq!(solve_day6(ex2, 14), 23);

        let ex3 = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(solve_day6(ex3, 4), 6);
        assert_eq!(solve_day6(ex3, 14), 23);

        let ex4 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(solve_day6(ex4, 4), 10);
        assert_eq!(solve_day6(ex4, 14), 29);

        let ex5 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(solve_day6(ex5, 4), 11);
        assert_eq!(solve_day6(ex5, 14), 26);
    }
}