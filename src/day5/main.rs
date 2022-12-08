use std::fs;

#[derive(Clone, Copy, Debug)]
struct Crate {
    name: char,
}

impl Crate {
    fn build(text: &[char]) -> Crate {
        Crate { name: text[1] }
    }
}

fn parse_num_stacks(line: &str) -> usize {
    line.split(' ')
        .map(|s| s.parse::<usize>())
        .filter(|i| i.is_ok())
        .count()
}

fn parse_stacks(text: &str) -> Vec<Vec<Crate>> {
    let lines: Vec<&str> = text.lines().collect();

    let last_line = lines.last().unwrap();

    let num_stacks = parse_num_stacks(last_line);
    let mut stacks = Vec::new();

    for _ in 0..num_stacks {
        stacks.push(Vec::new());
    }

    for line in lines {
        let chars: Vec<char> = line.chars().collect();

        for i in 0..num_stacks {
            let start_idx = i * 4;
            let end_idx = (i + 1) * 4 - 1;

            if end_idx <= chars.len() && chars[start_idx] == '[' {
                stacks[i].insert(0, Crate::build(&chars[start_idx..end_idx]));
            }
        }
    }

    stacks
}

#[derive(Clone, Copy)]
enum MoveType {
    Pt1,
    Pt2,
}

struct Move {
    src: usize,
    dst: usize,
    num: usize,
    kind: MoveType,
}

impl Move {
    pub fn build(line: &str, kind: MoveType) -> Move {
        let mut it = line.split(' ');
        it.next().unwrap(); // move
        let num: usize = it.next().unwrap().parse().unwrap();
        it.next().unwrap(); // from
        let src: usize = it.next().unwrap().parse().unwrap();
        it.next().unwrap(); // to
        let dst: usize = it.next().unwrap().parse().unwrap();

        // assumes that the index is one and not zero-based
        // so subtract one for src and dst
        Move {
            src: src - 1,
            dst: dst - 1,
            num: num,
            kind: kind,
        }
    }

    fn apply_pt1(&self, stacks: &mut Vec<Vec<Crate>>) {
        for _ in 0..self.num {
            let c = stacks[self.src].pop();
            if let Some(c) = c {
                stacks[self.dst].push(c);
            }
        }
    }

    fn apply_pt2(&self, stacks: &mut Vec<Vec<Crate>>) {
        let mut temp: Vec<Crate> = Vec::new();
        for _ in 0..self.num {
            let c = stacks[self.src].pop();
            if let Some(c) = c {
                temp.push(c);
            }
        }

        for c in temp.into_iter().rev() {
            stacks[self.dst].push(c.clone());
        }
    }

    pub fn apply(&self, stacks: &mut Vec<Vec<Crate>>) {
        match self.kind {
            MoveType::Pt1 => {
                self.apply_pt1(stacks);
            }
            MoveType::Pt2 => {
                self.apply_pt2(stacks);
            }
        }
    }
}

fn apply_moves(stacks: &mut Vec<Vec<Crate>>, text: &str, kind: MoveType) {
    for line in text.lines() {
        let mov = Move::build(line, kind.clone());
        mov.apply(stacks);
    }
}

fn get_top_crates(stacks: &Vec<Vec<Crate>>) -> String {
    stacks
        .iter()
        .filter_map(|s| s.last())
        .map(|c| c.name)
        .collect::<String>()
}

fn split_file(file_str: &str) -> (&str, &str) {
    let mut part_it = file_str.split("\n\n");
    let stack_str = part_it.next().expect("unable to parse file");
    let move_str = part_it.next().expect("unable to parse file");
    (stack_str, move_str)
}

fn solve(file: &str, kind: MoveType) -> String {
    let file_str = fs::read_to_string(file).expect("Unable to read file");
    let (stack_str, move_str) = split_file(file_str.as_str());

    let mut stacks = parse_stacks(stack_str);

    apply_moves(&mut stacks, move_str, kind);

    get_top_crates(&stacks)
}

fn main() {
    let solution_pt1 = solve("data/day5/input", MoveType::Pt1);
    println!("The solution to part one is {}", solution_pt1);

    let solution_pt2 = solve("data/day5/input", MoveType::Pt2);
    println!("The solution to part two is {}", solution_pt2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_num_stacks() {
        let text = " 1 21 42    5  60 ";
        assert_eq!(parse_num_stacks(&text), 5);
    }

    #[test]
    fn test_parse_stacks() {
        let text = "[D]        \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 ";

        let stacks = parse_stacks(text);
        assert_eq!(stacks.len(), 3);
        assert_eq!(stacks[0].len(), 3);
        assert_eq!(stacks[1].len(), 2);
        assert_eq!(stacks[2].len(), 1);
    }

    fn create_test_data() -> (String, Vec<Vec<Crate>>) {
        let mut stacks = Vec::new();
        stacks.push(vec![Crate { name: 'A' }, Crate { name: 'B' }]);
        stacks.push(vec![Crate { name: 'X' }]);

        let move_text =
            String::from("move 1 from 2 to 1\nmove 3 from 1 to 2\nmove 1 from 2 to 1\n");
        (move_text, stacks)
    }

    #[test]
    fn test_apply_moves_pt1() {
        let (text, mut stacks) = create_test_data();
        apply_moves(&mut stacks, text.as_str(), MoveType::Pt1);

        assert_eq!(stacks[0].len(), 1);
        assert_eq!(stacks[0][0].name, 'A');
        assert_eq!(stacks[1].len(), 2);
        assert_eq!(stacks[1][0].name, 'X');
        assert_eq!(stacks[1][1].name, 'B');
    }

    #[test]
    fn test_apply_moves_pt2() {
        let (text, mut stacks) = create_test_data();

        apply_moves(&mut stacks, text.as_str(), MoveType::Pt2);

        assert_eq!(stacks[0].len(), 1);
        assert_eq!(stacks[0][0].name, 'X');
        assert_eq!(stacks[1].len(), 2);
        assert_eq!(stacks[1][0].name, 'A');
        assert_eq!(stacks[1][1].name, 'B');
    }

    #[test]
    fn test_day5() {
        let file_str = fs::read_to_string("data/day5/test").expect("Unable to read file");
        let (stack_str, move_str) = split_file(file_str.as_str());
        assert!(!stack_str.contains("move"));
        assert!(!move_str.contains("Z"));

        let mut stacks = parse_stacks(stack_str);

        assert_eq!(stacks[0].len(), 2);
        assert_eq!(stacks[1].len(), 3);
        assert_eq!(stacks[2].len(), 1);

        let move_lines = move_str.lines().collect::<Vec<&str>>();
        assert_eq!(move_lines.len(), 4);

        apply_moves(&mut stacks, &move_lines[0], MoveType::Pt1);
        apply_moves(&mut stacks, &move_lines[1], MoveType::Pt1);
        apply_moves(&mut stacks, &move_lines[2], MoveType::Pt1);
        apply_moves(&mut stacks, &move_lines[3], MoveType::Pt1);

        let solution = get_top_crates(&stacks);

        assert_eq!(solution, String::from("CMZ"));
        assert_eq!(solve("data/day5/test", MoveType::Pt1), String::from("CMZ"));

        assert_eq!(solve("data/day5/test", MoveType::Pt2), String::from("MCD"));
    }
}
