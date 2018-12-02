use std::collections::HashSet;

pub fn solve1(numbers: &Vec<i32>) -> i32 {
    return numbers.iter().sum::<i32>();
}

pub fn solve2(numbers: &Vec<i32>) -> i32 {
    let mut map_of_counts: HashSet<i32> = HashSet::new();
    map_of_counts.insert(0);
    let mut sum = 0;
    for num in numbers.iter().cycle() {
        sum += num;
        if !map_of_counts.insert(sum) {
            return sum;
        }
    }
    panic!("numbers.cycle() ended?");
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::*;

    #[test]
    fn it_sums_the_array() {
        let result: i32 = solve1(&vec![1, 2, 3]);
        assert_eq!(result, 1 + 2 + 3)
    }

    #[test]
    fn it_handles_example_input_1() {
        let result: i32 = solve1(&vec![1, 1, 1]);
        assert_eq!(result, 3)
    }

    #[test]
    fn it_handles_example_input_2() {
        let result: i32 = solve1(&vec![1, 1, -2]);
        assert_eq!(result, 0)
    }

    #[test]
    fn it_handles_example_input_3() {
        let result: i32 = solve1(&vec![-1, -2, -3]);
        assert_eq!(result, -6)
    }

    #[test]
    fn it_handles_example_input_4() {
        let result: i32 = solve1(&vec![1, -2, 3, 1]);
        assert_eq!(result, 3)
    }

    #[test]
    fn it_handles_real_input() {
        let numbers = map_lines_to_int32("./src/day1/input");
        let result: i32 = solve1(&numbers);
        assert_eq!(result, 470);
    }

    #[test]
    fn it_handles_example_input_4_star_2() {
        let result: i32 = solve2(&vec![1, -2, 3, 1]);
        assert_eq!(result, 2)
    }

    #[test]
    fn it_handles_real_input_star_2() {
        let numbers = map_lines_to_int32("./src/day1/input");
        let result: i32 = solve2(&numbers);
        assert_eq!(result, 790);
    }
}
