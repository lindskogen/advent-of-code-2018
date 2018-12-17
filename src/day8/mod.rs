use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Debug)]
enum Tree<T> {
    Node(Box<Vec<T>>, Box<Vec<Tree<T>>>),
}

pub fn solve1(input: &str) -> i32 {
    let tree = parse_line(input);
    sum_tree(&tree)
}

pub fn solve2(input: &str) -> i32 {
    let tree = parse_line(input);
    sum_tree_by_reference(&tree)
}

fn sum_tree(tree: &Tree<i32>) -> i32 {
    let Tree::Node(data, children) = tree;

    let sum: i32 = data.iter().map(|x| *x).sum();
    let child_sum: i32 = children.iter().map(sum_tree).sum();

    sum + child_sum
}

fn sum_tree_by_reference(tree: &Tree<i32>) -> i32 {
    let Tree::Node(data, children) = tree;

    if children.is_empty() {
        data.iter().map(|x| *x).sum()
    } else {
        data.iter()
            .map(|&i| children.get((i - 1) as usize).map(sum_tree_by_reference).unwrap_or(0))
            .sum()
    }
}

fn parse_line(input: &str) -> Tree<i32> {
    let mut numbers: VecDeque<i32> = input
        .split_whitespace()
        .map(|s| i32::from_str(s).unwrap())
        .collect();

    return parse_node(&mut numbers);
}

fn parse_node(numbers: &mut VecDeque<i32>) -> Tree<i32> {
    let num_children = numbers.pop_front().unwrap();
    let num_metadata = numbers.pop_front().unwrap();

    let children: Vec<_> = (0..num_children).map(|_| parse_node(numbers)).collect();

    let metadata: Vec<_> = (0..num_metadata)
        .map(|_| numbers.pop_front().unwrap())
        .collect();

    return Tree::Node(Box::new(metadata), Box::new(children));
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::*;

    #[test]
    fn it_parses_line_into_tree() {
        let sum = solve1("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
        assert_eq!(sum, 138)
    }

    #[test]
    fn it_handles_input_for_star1() {
        let input = read_file_to_string("./src/day8/input");
        let sum = solve1(&input);
        assert_eq!(sum, 44338)
    }

    #[test]
    fn it_sums_by_references() {
        let sum = solve2("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
        assert_eq!(sum, 66)
    }

    #[test]
    fn it_handles_input_for_star2() {
        let input = read_file_to_string("./src/day8/input");
        let sum = solve2(&input);
        assert_eq!(sum, 37560)
    }
}
