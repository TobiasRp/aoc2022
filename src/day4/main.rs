use aoc::read_lines;

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn contains(&self, r: &Range) -> bool {
        return self.start <= r.start && self.end >= r.end;
    }

    fn build(substr: &str) -> Result<Range, &'static str> {
        let mut number_it = substr.split('-');
        let first = number_it.next().ok_or("invalid range")?;
        let second = number_it.next().ok_or("invalid range")?;

        Ok(Range {
            start: first.parse::<u32>().unwrap(),
            end: second.parse::<u32>().unwrap(),
        })
    }
}

fn is_contained(lhs: &Range, rhs: &Range) -> bool {
    return lhs.contains(rhs) || rhs.contains(lhs);
}

fn has_overlap(lhs: &Range, rhs: &Range) -> bool {
    return (lhs.start >= rhs.start && lhs.start <= rhs.end)
        || (rhs.start >= lhs.start && rhs.start <= lhs.end);
}

fn parse_line(line: &str) -> Vec<Range> {
    let mut result = Vec::new();

    for rstr in line.split(',') {
        let range = Range::build(rstr).expect("Expected valid range!");
        result.push(range)
    }
    result
}

fn solve<F>(file: &str, decision_fn: F) -> u32
where
    F: Fn(&Range, &Range) -> bool,
{
    let mut total_score = 0;

    let lines = read_lines(file).unwrap();
    for rline in lines {
        if let Ok(line) = rline {
            let ranges = parse_line(line.as_str());
            assert_eq!(ranges.len(), 2);

            total_score += if decision_fn(&ranges[0], &ranges[1]) {
                1
            } else {
                0
            };
        }
    }
    total_score
}

fn main() {
    let solution_pt1 = solve("data/day4/input", is_contained);
    println!("The solution to part one is {}", solution_pt1);

    let solution_pt2 = solve("data/day4/input", has_overlap);
    println!("The solution to part two is {}", solution_pt2);
}

#[cfg(test)]
mod tests_day4 {
    use super::*;

    #[test]
    fn test_contains_other() {
        let r1 = Range { start: 2, end: 4 };
        let r2 = Range { start: 2, end: 8 };
        let r3 = Range { start: 5, end: 8 };
        assert!(is_contained(&r1, &r2));
        assert!(!is_contained(&r1, &r3));
    }

    #[test]
    fn test_has_overlap() {
        let r1 = Range { start: 2, end: 4 };
        let r2 = Range { start: 3, end: 8 };
        let r3 = Range { start: 5, end: 8 };
        assert!(has_overlap(&r1, &r2));
        assert!(!has_overlap(&r1, &r3));
    }

    #[test]
    fn test_parse_line() {
        let line = "2-4,12-42";
        let ranges = parse_line(line);
        assert_eq!(ranges.len(), 2);

        let lhs = &ranges[0];
        let rhs = &ranges[1];

        assert_eq!(lhs.start, 2);
        assert_eq!(lhs.end, 4);
        assert_eq!(rhs.start, 12);
        assert_eq!(rhs.end, 42);
    }

    #[test]
    fn test_day4() {
        // test data and result from the task description
        assert_eq!(solve("data/day4/test", is_contained), 2);
        assert_eq!(solve("data/day4/test", has_overlap), 4);
    }
}
