use itertools::Itertools;
use std::collections::HashSet;

pub fn solve1(string: &str) -> String {
    react(string)
}

fn react_tuple(c1: char, c2: char) -> bool {
    c1.to_ascii_lowercase() == c2.to_ascii_lowercase() && c1 != c2
}

fn react(string: &str) -> String {
    let mut index = 0usize;
    let mut chars: Vec<_> = string.chars().collect();

    while chars.len() != 0 && index < chars.len() - 1 {
        let should_remove = react_tuple(chars[index], chars[index + 1]);

        if should_remove {
            chars.remove(index);
            chars.remove(index);
            if index != 0 {
                index -= 1;
            }
        } else {
            index += 1;
        }
    }

    chars.iter().collect()
}

fn unique_chars(string: &str) -> String {
    string
        .chars()
        .map(|c| c.to_ascii_lowercase())
        .unique()
        .collect()
}

fn remove_char_from_string(ch: char, string: &str) -> String {
    let lower_char = ch.to_ascii_lowercase();
    string
        .chars()
        .filter(|c| c.to_ascii_lowercase() != lower_char)
        .collect()
}

pub fn solve2(string: &str) -> usize {
    let reacted_input = solve1(string);
    let polymers = unique_chars(string);

    polymers
        .chars()
        .map(|c| solve1(&remove_char_from_string(c, &reacted_input)).len())
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::*;

    #[test]
    fn it_eliminates_two_equal_elements_with_different_polarity() {
        assert_eq!(react_tuple('a', 'A'), true)
    }

    #[test]
    fn it_keeps_two_equal_elements_with_same_polarity() {
        assert_eq!(react_tuple('A', 'A'), false)
    }

    #[test]
    fn it_eliminates_two_equal_elements_in_one_order() {
        assert_eq!(react_tuple('a', 'A'), true)
    }

    #[test]
    fn it_eliminates_two_equal_elements_in_another_order() {
        assert_eq!(react_tuple('A', 'a'), true)
    }

    #[test]
    fn it_keeps_two_unequal_elements() {
        assert_eq!(react_tuple('a', 'b'), false)
    }

    #[test]
    fn it_works_on_a_string_with_one_reaction() {
        assert_eq!(react("aA"), "")
    }

    #[test]
    fn it_works_on_a_string_with_no_reactions() {
        assert_eq!(react("abAB"), "abAB")
    }

    #[test]
    fn it_works_on_the_first_pass_through_a_sequence_with_no_reactions() {
        assert_eq!(react("aabAAB"), "aabAAB")
    }
    #[test]
    fn it_works_on_a_larger_string() {
        assert_eq!(react("dabAcCaCBAcCcaDA"), "dabCBAcaDA")
    }

    #[test]
    fn it_counts_the_chars_in_a_larger_string() {
        assert_eq!(solve1("dabAcCaCBAcCcaDA").len(), 10)
    }

    #[test]
    fn it_works_on_the_input() {
        let string = read_file_to_string("./src/day5/input");
        assert_eq!(solve1(&string).len(), 10762)
    }

    #[test]
    fn it_uniqs_a_string() {
        assert_eq!(unique_chars("dabAcCaCBAcCcaDA"), "dabc")
    }

    #[test]
    fn it_removes_all_chars_from_string_lowercase() {
        assert_eq!(
            remove_char_from_string('a', "dabAcCaCBAcCcaDA"),
            "dbcCCBcCcD"
        )
    }

    #[test]
    fn it_removes_all_chars_from_string_uppercase() {
        assert_eq!(
            remove_char_from_string('A', "dabAcCaCBAcCcaDA"),
            "dbcCCBcCcD"
        )
    }

    #[test]
    fn it_finds_the_length_of_the_most_efficient_polymer_for_a_string() {
        assert_eq!(solve2("dabAcCaCBAcCcaDA"), 4)
    }

    #[test]
    fn it_finds_the_length_of_the_most_efficient_polymer_for_the_input() {
        let string = read_file_to_string("./src/day5/input");
        assert_eq!(solve2(&string), 6946)
    }
}
