use std::collections::HashSet;
use std::fs;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Move {
    dir: Direction,
    num: i32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Move {
    fn build(line: &str) -> Move {
        let line_vec: Vec<&str> = line.split(" ").collect();
        assert_eq!(line_vec[0].len(), 1);
        let dir_char = line_vec[0].chars().next().unwrap();

        let dir = match dir_char {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => Direction::Up,
        };
        let num = line_vec[1].parse::<i32>().unwrap();

        Move { dir: dir, num: num }
    }

    fn move_head(&self, p: &mut Point) {
        match self.dir {
            Direction::Up => p.y += 1,
            Direction::Down => p.y -= 1,
            Direction::Left => p.x -= 1,
            Direction::Right => p.x += 1,
        }
    }

    fn move_tail(h: &Point, t: &mut Point) {
        let d_x = h.x - t.x;
        let d_y = h.y - t.y;

        if d_x.abs() + d_y.abs() >= 3 {
            t.x += d_x.signum();
            t.y += d_y.signum();
        } else if d_x.abs() >= 2 {
            t.x += d_x.signum();
        } else if d_y.abs() >= 2 {
            t.y += d_y.signum();
        }
    }
}

fn build_moves(input: &str) -> Vec<Move> {
    input.lines().map(|l| Move::build(l)).collect()
}

fn solve(moves: &[Move], num_knots: usize) -> usize {
    let mut tails: HashSet<Point> = HashSet::new();
    tails.insert(Point { x: 0, y: 0 });

    let mut heads = Vec::new();
    for _ in 0..num_knots {
        heads.push(Point { x: 0, y: 0 });
    }

    for mov in moves {
        for _ in 0..mov.num {
            mov.move_head(&mut heads[0]);
            for i in 1..num_knots {
                let (left, right) = heads.split_at_mut(i);
                Move::move_tail(left.last().unwrap(), right.first_mut().unwrap());
            }
            tails.insert(heads.last().unwrap().clone());
        }
    }
    tails.len()
}

fn main() {
    let input = fs::read_to_string("data/day9/input").expect("Unable to read file");
    let moves = build_moves(input.as_str());

    let solution_pt1 = solve(&moves, 2);
    println!("The solution to part one is {}", solution_pt1);

    let solution_pt2 = solve(&moves, 10);
    println!("The solution to part two is {}", solution_pt2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day9_move() {
        assert_eq!(
            Move::build("U 4"),
            Move {
                dir: Direction::Up,
                num: 4
            }
        );
        assert_eq!(
            Move::build("D 12"),
            Move {
                dir: Direction::Down,
                num: 12
            }
        );
    }

    #[test]
    fn test_day9() {
        let test_input = fs::read_to_string("data/day9/test").expect("Unable to read file");
        let moves = build_moves(test_input.as_str());

        assert_eq!(moves.len(), 8);
        assert_eq!(moves[0].dir, Direction::Right);
        assert_eq!(moves[6].dir, Direction::Left);

        let solution = solve(&moves, 2);
        assert_eq!(solution, 13);

        let solution = solve(&moves, 10);
        assert_eq!(solution, 1);
    }

    #[test]
    fn test_day9_pt2() {
        let test_input = fs::read_to_string("data/day9/test2").expect("Unable to read file");
        let moves = build_moves(test_input.as_str());

        let solution = solve(&moves, 10);
        assert_eq!(solution, 36);
    }
}
