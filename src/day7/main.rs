use std::{collections::BTreeMap, fs};

#[derive(Debug)]
enum Node {
    File { size: usize },
    Dir { name: String },
}
type Nodes = Vec<Node>;

impl Node {
    fn build(line: &str) -> Node {
        let mut it = line.split(' ');
        let first = it.next().unwrap();
        let name = it.next().unwrap();
        if first == "dir" {
            return Node::Dir {
                name: name.to_string(),
            };
        } else {
            let size: usize = first.parse().unwrap();
            return Node::File { size: size };
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
enum CdDst {
    Up,
    Root,
    Dir(String),
}

impl CdDst {
    fn build(line: &str) -> CdDst {
        if line.contains("..") {
            CdDst::Up
        } else if line.contains("/") {
            CdDst::Root
        } else {
            let mut line_it = line.split(' ');
            line_it.next();
            let dir = line_it.next().unwrap();
            CdDst::Dir(dir.to_string())
        }
    }
}

#[derive(Debug)]
enum Op {
    Ls(Nodes),
    Cd(CdDst),
}

impl Op {
    fn build(block: &str) -> Op {
        let mut block_lines = block.lines();

        let cmd = block_lines.next().unwrap().trim();
        if cmd.starts_with("cd") {
            let cd_dst = CdDst::build(cmd);
            return Op::Cd(cd_dst);
        } else if cmd.starts_with("ls") {
            let mut nodes: Nodes = Vec::new();
            for line in block_lines {
                nodes.push(Node::build(line));
            }
            Op::Ls(nodes)
        } else {
            assert!(false);
            Op::Cd(CdDst::Root)
        }
    }
}

fn build_path(dir_stack: &Vec<String>) -> String {
    let mut path = String::from("");
    for dir in dir_stack {
        path += dir;
        if dir != "/" {
            path += "/";
        }
    }
    path
}

fn compute_dir_size(
    total_sizes: &BTreeMap<String, usize>,
    dir_path: &str,
    dir_nodes: &Nodes,
) -> Option<usize> {
    let mut dir_size: usize = 0;
    for node in dir_nodes {
        match node {
            Node::Dir { name } => {
                let node_path = dir_path.to_string() + name.as_str() + "/";
                if total_sizes.contains_key(&node_path) {
                    dir_size += total_sizes[&node_path];
                } else {
                    // We have a dir for which no size exists (yet)
                    return None;
                }
            }
            Node::File { size } => {
                dir_size += size;
            }
        }
    }
    Some(dir_size)
}

fn compute_dir_sizes(dirs: BTreeMap<String, Nodes>) -> BTreeMap<String, usize> {
    let mut sizes: BTreeMap<String, usize> = BTreeMap::new();
    let num_dirs = dirs.len();
    while sizes.len() < num_dirs {
        for (dir_path, nodes) in dirs.iter() {
            if !sizes.contains_key(dir_path) {
                let size = compute_dir_size(&sizes, dir_path, nodes);
                if let Some(dir_size) = size {
                    sizes.insert(String::from(dir_path), dir_size);
                }
            }
        }
    }
    sizes
}

fn collect_dir_structure(ops: Vec<Op>) -> BTreeMap<String, Nodes> {
    let mut dirs: BTreeMap<String, Nodes> = BTreeMap::new();
    let mut stack = vec!["/".to_string()];

    for op in ops {
        match op {
            Op::Cd(CdDst::Root) => {
                stack.truncate(1);
            }
            Op::Cd(CdDst::Up) => {
                stack.pop();
            }
            Op::Cd(CdDst::Dir(dir)) => {
                stack.push(dir);
            }
            Op::Ls(nodes) => {
                let path = build_path(&stack);
                dirs.insert(path, nodes);
            }
        }
    }
    dirs
}

fn process_directories(input: &str) -> BTreeMap<String, usize> {
    // Build list of op's from input
    let ops: Vec<Op> = input
        .trim()
        .split('$')
        .filter(|s| !s.is_empty())
        .map(|b| Op::build(b))
        .collect();

    // collect directory structure into a tree map
    let dirs = collect_dir_structure(ops);

    // compute sizes of directories
    compute_dir_sizes(dirs)
}

fn solve_pt1(total_sizes: &BTreeMap<String, usize>) -> usize {
    total_sizes.values().filter(|size| **size < 100000).sum()
}

fn solve_pt2(total_sizes: &BTreeMap<String, usize>) -> usize {
    const REQUIRED_FREE_SPACE: usize = 30000000;
    const TOTAL_SPACE: usize = 70000000;
    let used_space = total_sizes["/"];

    *total_sizes
        .iter()
        .map(|(_, s)| s)
        .filter(|s| TOTAL_SPACE + **s - used_space >= REQUIRED_FREE_SPACE)
        .min()
        .unwrap()
}

fn main() {
    let input = fs::read_to_string("data/day7/input").expect("Unable to read file");

    let dir_sizes = process_directories(input.as_str());

    let solution_pt1 = solve_pt1(&dir_sizes);
    println!("The solution to part one is {}", solution_pt1);

    let solution_pt2 = solve_pt2(&dir_sizes);
    println!("The solution to part two is {}", solution_pt2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day7() {
        let test_input = fs::read_to_string("data/day7/test").expect("Unable to read file");
        let dir_sizes = process_directories(test_input.as_str());
        assert_eq!(dir_sizes["/"], 48381165);
        assert_eq!(solve_pt1(&dir_sizes), 95437);
        assert_eq!(solve_pt2(&dir_sizes), 24933642);
    }

    #[test]
    fn test_build_node() {
        let node = Node::build("10442123 test.file");
        match node {
            Node::File { size } => {
                assert_eq!(size, 10442123);
            }
            Node::Dir { name: _ } => assert!(false),
        }

        let node = Node::build("dir test");
        match node {
            Node::File { size: _ } => assert!(false),
            Node::Dir { name } => assert_eq!(name, "test"),
        }
    }

    #[test]
    fn test_build_op() {
        assert_eq!(CdDst::build("cd .."), CdDst::Up);
        assert_eq!(CdDst::build("cd /"), CdDst::Root);
        let cd_dir = CdDst::build("cd dir");
        match cd_dir {
            CdDst::Dir(s) => assert_eq!(s, "dir"),
            _ => assert!(false),
        }

        match Op::build("cd ..") {
            Op::Cd(cd) => assert_eq!(cd, CdDst::Up),
            Op::Ls(_) => {
                assert!(false);
            }
        }

        match Op::build("ls\n42 file.txt\ndir test_dir") {
            Op::Cd(_) => assert!(false),
            Op::Ls(nodes) => {
                match &nodes[0] {
                    Node::File { size } => {
                        assert_eq!(*size, 42);
                    }
                    Node::Dir { name: _ } => assert!(false),
                }
                match &nodes[1] {
                    Node::File { size: _ } => assert!(false),
                    Node::Dir { name } => assert_eq!(name, "test_dir"),
                }
            }
        }
    }
}
