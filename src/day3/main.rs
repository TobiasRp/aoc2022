use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

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

fn item_priority(item: char) -> u32 {
    let item_nr = item as u32;

    if (item_nr >= 'A' as u32) && (item_nr <= 'Z' as u32) {
        return item_nr - ('A' as u32) + 27;
    } else if (item_nr >= 'a' as u32) && (item_nr <= 'z' as u32) {
        return item_nr - ('a' as u32) + 1;
    } else {
        panic!("Can't prioritize unexpected item!");
    }
}

fn find_duplicated(left: &[char], right: &[char]) -> char {
    let mut map_left = HashMap::new();

    for item in left {
        map_left.insert(item, true);
    }

    for item in right {
        if map_left.contains_key(item) {
            return item.clone();
        }
    }
    panic!("No key found that exists in both sides!");
}

fn solve_pt1(file: &str) -> u32
{
    let mut total_sum = 0;

    let lines = read_lines(file).unwrap();
    for rline in lines {
        if let Ok(line) = rline {
            let all_chars: Vec<char> = line.chars().collect();
            assert!(all_chars.len() % 2 == 0);


            let (head, tail) = all_chars.split_at(all_chars.len() / 2);
            let item = find_duplicated(head, tail);
            total_sum += item_priority(item);
        }
    }
    total_sum
}

fn find_threeway_duplicated(left: &[char], mid: &[char], right: &[char]) -> char {
    let mut map_left = HashMap::new();
    let mut map_mid = HashMap::new();

    for item in left {
        map_left.insert(item, true);
    }

    for item in mid {
        map_mid.insert(item, true);
    }

    for item in right {
        if map_left.contains_key(item) && map_mid.contains_key(item) {
            return item.clone();
        }
    }

    panic!("No key found that exists in all three!");
}

fn solve_pt2(file: &str) -> u32
{
    let mut total_sum = 0;

    let lines: Vec<String> = read_lines(file).unwrap().collect::<Result<_, _>>().unwrap();

    for window in lines.windows(3).step_by(3) {
        let chars0 = &window[0].chars().collect::<Vec<char>>();
        let chars1 = &window[1].chars().collect::<Vec<char>>();
        let chars2 = &window[2].chars().collect::<Vec<char>>();

        let item = find_threeway_duplicated(&chars0, &chars1, &chars2);

        total_sum += item_priority(item);
    }
    total_sum
}

fn main() {
    let solution_pt1 = solve_pt1("data/day3/input");
    println!("The solution to part one is {}", solution_pt1);

    let solution_pt2: u32 = solve_pt2("data/day3/input");
    println!("The solution to part two is {}", solution_pt2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_priority() {
        assert_eq!(item_priority('b'), 2);
        assert_eq!(item_priority('t'), 20);
        assert_eq!(item_priority('B'), 28);
        assert_eq!(item_priority('L'), 38);
    }

    #[test]
    fn test_find_duplicated() {
        let left = vec!['a', 't', 'x'];
        let right = vec!['c', 'b', 't'];
        assert_eq!(find_duplicated(&left, &right), 't');
    }

    #[test]
    fn test_pt1() {
        // test data and result from the task description
        assert_eq!(solve_pt1("data/day3/test"), 157);
    }

    #[test]
    fn test_find_threeway_duplicated() {
        let left = vec!['a', 't', 'x', 'x', 'x'];
        let mid = vec!['z', 't', 'a', 'x', 'c'];
        let right = vec!['c', 'b', 't', 'b', 'b'];
        assert_eq!(find_threeway_duplicated(&left, &mid, &right), 't');
    }

    #[test]
    fn test_pt2() {
        // test data and result from the task description
        assert_eq!(solve_pt2("data/day3/test"), 70);
    }
}