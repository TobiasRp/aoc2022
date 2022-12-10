use std::fs;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Op {
    NoOp,
    AddX(i32),
}

impl Op {
    fn build(line: &str) -> Op {
        if line.contains("addx") {
            let mut line_it = line.split(" ");
            line_it.next();
            let v = line_it.next().unwrap().parse::<i32>().unwrap();
            Op::AddX(v)
        } else {
            Op::NoOp
        }
    }
}

fn parse_ops(input: &str) -> Vec<Op> {
    input.lines().map(|l| Op::build(l)).collect()
}

fn run_ops(ops: Vec<Op>) -> Vec<i32> {
    let mut x = 1;
    let mut xs = vec![x];
    for op in ops {
        match op {
            Op::AddX(v) => {
                xs.push(x);
                xs.push(x);
                x += v;
            }
            _ => xs.push(x),
        }
    }
    xs
}

fn signal_strength(xs: &[i32], cycle: usize) -> i32 {
    xs[cycle] * (cycle as i32)
}

fn total_signal_strength(xs: &[i32]) -> i32 {
    let cycles = [20, 60, 100, 140, 180, 220];
    cycles
        .iter()
        .map(|c| signal_strength(xs, *c as usize))
        .sum()
}

fn draw(xs: &[i32]) {
    const WIDTH: usize = 40;
    const HEIGHT: usize = 6;

    assert_eq!(xs.len(), 1 + HEIGHT * WIDTH);
    for y in 0..HEIGHT {
        let line: String = (0..WIDTH)
            .map(|x| {
                (xs[y * WIDTH + x + 1] - 1) <= (x as i32)
             && (xs[y * WIDTH + x + 1] + 1) >= (x as i32)
            })
            .map(|f| if f { '#' } else { '.' })
            .collect();
        println!("{}", line);
    }
}

fn main() {
    let input = fs::read_to_string("data/day10/input").expect("Unable to read file");
    let ops = parse_ops(input.as_str());
    let xs = run_ops(ops);
    let solution_pt1 = total_signal_strength(&xs);
    println!("The solution to part one is {}", solution_pt1);

    println!("The solution to part two is");
    draw(&xs);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day10_ops() {
        assert_eq!(Op::build("addx 42"), Op::AddX(42));
        assert_eq!(Op::build("noop"), Op::NoOp);
    }

    #[test]
    fn test_day10() {
        let input = fs::read_to_string("data/day10/test").expect("Unable to read file");
        let ops = parse_ops(input.as_str());
        let xs = run_ops(ops);
        assert_eq!(signal_strength(&xs, 20), 420);
        assert_eq!(signal_strength(&xs, 60), 1140);
        assert_eq!(signal_strength(&xs, 100), 1800);
        assert_eq!(signal_strength(&xs, 140), 2940);
        assert_eq!(signal_strength(&xs, 180), 2880);
        assert_eq!(signal_strength(&xs, 220), 3960);

        assert_eq!(total_signal_strength(&xs), 13140);
    }
}
