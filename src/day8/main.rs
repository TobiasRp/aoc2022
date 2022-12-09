use std::cmp::max;
use std::fs;

struct Map {
    trees: Vec<u8>,
    width: usize,
    height: usize,
}

impl Map {
    fn build(input: &str) -> Map {
        let mut trees = Vec::new();

        let mut width: usize = 1;
        for (idx, line) in input.lines().enumerate() {
            for c in line.chars() {
                trees.push(c.to_digit(10).unwrap() as u8);
            }
            if idx == 0 {
                width = trees.len();
            }
        }

        let height: usize = trees.len() / width;
        Map {
            trees,
            width,
            height,
        }
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        return self.trees[y * self.width + x];
    }

    fn inside(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < (self.width as i32) && y >= 0 && y < (self.height as i32)
    }
}

fn solve_pt1(map: &Map) -> usize {
    let mut left_to_right = vec![0u8; map.width * map.height];
    for y in 0..map.height {
        let mut row_max: u8 = 0;
        for x in 0..map.width {
            left_to_right[y * map.height + x] = row_max;
            let h = map.get(x, y);
            row_max = max(row_max, h);
        }
    }

    let mut right_to_left = vec![0u8; map.width * map.height];
    for y in 0..map.height {
        let mut row_max: u8 = 0;
        for x in (0..map.width).rev() {
            right_to_left[y * map.height + x] = row_max;
            let h = map.get(x, y);
            row_max = max(row_max, h);
        }
    }

    let mut top_to_bottom = vec![0u8; map.width * map.height];
    for x in 0..map.width {
        let mut col_max: u8 = 0;
        for y in 0..map.height {
            top_to_bottom[y * map.height + x] = col_max;
            let h = map.get(x, y);
            col_max = max(col_max, h);
        }
    }

    let mut bottom_to_top = vec![0u8; map.width * map.height];
    for x in 0..map.width {
        let mut col_max: u8 = 0;
        for y in (0..map.height).rev() {
            bottom_to_top[y * map.height + x] = col_max;
            let h = map.get(x, y);
            col_max = max(col_max, h);
        }
    }

    // number of trees on the outside edges
    let num_edge_trees = 2 * (map.width - 2) + 2 * map.height;

    let mut sum_visible = num_edge_trees;
    for y in 1..map.height - 1 {
        for x in 1..map.width - 1 {
            let h = map.get(x, y);
            if (left_to_right[y * map.width + x] < h)
                || (right_to_left[y * map.width + x] < h)
                || (top_to_bottom[y * map.width + x] < h)
                || (bottom_to_top[y * map.width + x] < h)
            {
                sum_visible += 1;
            }
        }
    }
    sum_visible
}

fn score(map: &Map, p_x: i32, p_y: i32, s_x: i32, s_y: i32) -> usize {
    let h = map.get(p_x as usize, p_y as usize);

    let mut c_x = p_x;
    let mut c_y = p_y;
    let mut score = 0;
    loop {
        c_x += s_x;
        c_y += s_y;

        if !map.inside(c_x, c_y) {
            return score;
        }

        score += 1;

        if map.get(c_x as usize, c_y as usize) >= h {
            return score;
        }
    }
}

fn compute_scenic_score(map: &Map, x: i32, y: i32) -> usize {
    let score_right = score(map, x, y, 1, 0);
    let score_left = score(map, x, y, -1, 0);
    let score_up = score(map, x, y, 0, 1);
    let score_down = score(map, x, y, 0, -1);
    score_down * score_up * score_left * score_right
}

fn solve_pt2(map: &Map) -> usize {
    let mut scenic_score = 0;

    for x in 0..map.width {
        for y in 0..map.height {
            let score = compute_scenic_score(map, x as i32, y as i32);
            scenic_score = max(scenic_score, score);
        }
    }
    scenic_score
}

fn main() {
    let input = fs::read_to_string("data/day8/input").expect("Unable to read file");
    let map = Map::build(input.as_str());

    let solution_pt1 = solve_pt1(&map);
    println!("The solution to part one is {}", solution_pt1);

    let solution_pt2 = solve_pt2(&map);
    println!("The solution to part two is {}", solution_pt2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day8() {
        let test_input = fs::read_to_string("data/day8/test").expect("Unable to read file");

        let map = Map::build(&test_input.as_str());
        assert_eq!(map.width, 5);
        assert_eq!(map.height, 5);
        assert_eq!(map.trees.len(), 25);

        assert_eq!(solve_pt1(&map), 21);
        assert_eq!(solve_pt2(&map), 8);
    }
}
