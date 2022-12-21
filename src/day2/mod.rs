use std::collections::HashMap;

pub fn solve1(hashes: Vec<&str>) -> u32 {
    let (no_of_2s, no_of_3s) = hashes.iter().fold((0, 0), |(no_of_2s, no_of_3s), hash| {
        let (contains_2, contains_3) = count_letters(hash);

        (
            no_of_2s + if contains_2 { 1 } else { 0 },
            no_of_3s + if contains_3 { 1 } else { 0 },
        )
    });

    no_of_3s * no_of_2s
}

fn count_letters(id: &str) -> (bool, bool) {
    let mut letters_count: HashMap<char, u32> = HashMap::new();

    for letter in id.chars() {
        *letters_count.entry(letter).or_insert(0) += 1;
    }

    let contains_2 = letters_count.values().any(|&v| v == 2);
    let contains_3 = letters_count.values().any(|&v| v == 3);

    (contains_2, contains_3)
}

pub fn solve2(hashes: Vec<&str>) -> String {
    for h1 in hashes.iter() {
        for h2 in hashes.iter() {
            if distance(h1, h2) == 1 {
                return common_chars(h1, h2);
            }
        }
    }
    panic!("No words of distance 1 found")
}

fn distance(str1: &str, str2: &str) -> u32 {
    str1.chars()
        .zip(str2.chars())
        .map(|(ch1, ch2)| if ch1 == ch2 { 0 } else { 1 })
        .sum()
}

fn common_chars(str1: &str, str2: &str) -> String {
    str1.chars()
        .zip(str2.chars())
        .filter_map(|(c1, c2)| if c1 == c2 { Some(c1) } else { None })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::*;

    #[test]
    fn it_returns_a_boolean_tuple() {
        let _result: (bool, bool) = count_letters("abcdef");
    }

    #[test]
    fn it_returns_true_if_string_contains_2_of_a_letter() {
        let (result, _) = count_letters("abbcde");
        assert!(result)
    }

    #[test]
    fn it_returns_false_if_string_doesnt_contain_2_of_a_letter() {
        let (result, _) = count_letters("abcdef");
        assert!(!result)
    }

    #[test]
    fn it_returns_true_if_string_contains_3_of_a_letter() {
        let (_, result) = count_letters("abcccd");
        assert!(result)
    }

    #[test]
    fn it_returns_false_if_string_doesnt_contain_3_of_a_letter() {
        let (_, result) = count_letters("abcdef");
        assert!(!result)
    }

    #[test]
    fn it_returns_true_if_string_contains_both_2_and_3_of_a_letter() {
        let (contains_2, contains_3) = count_letters("bababc");
        assert!(contains_2 && contains_3)
    }

    #[test]
    fn it_returns_the_product_of_no_of_2s_and_no_of_3s() {
        let strings = vec![
            "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
        ];

        let product = solve1(strings);

        assert_eq!(product, 12)
    }

    #[test]
    fn it_returns_the_product_of_no_of_2s_and_no_of_3s_for_test_input() {
        let strings = map_lines_to_strings("./src/day2/input");
        let strings = strings.iter().map(|s| s.as_str()).collect();

        let product = solve1(strings);

        assert_eq!(product, 5434)
    }

    #[test]
    fn it_counts_distance_strings_that_differ_by_2() {
        let result = distance("abcde", "axcye");
        assert_eq!(result, 2)
    }

    #[test]
    fn it_counts_distance_strings_that_differ_by_1() {
        let result = distance("fghij", "fguij");
        assert_eq!(result, 1)
    }

    #[test]
    fn it_prints_the_common_chars() {
        let result = common_chars("fghij", "fguij");
        assert_eq!(result, "fgij")
    }

    #[test]
    fn it_finds_the_two_strings_that_have_distance_1_for_example_input() {
        let strs = vec![
            "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
        ];

        let result = solve2(strs);
        assert_eq!(result, "fgij")
    }

    #[test]
    fn it_finds_the_two_strings_that_have_distance_1_for_real_input() {
        let strings = map_lines_to_strings("./src/day2/input");
        let strings = strings.iter().map(|s| s.as_str()).collect();

        let result = solve2(strings);
        assert_eq!(result, "agimdjvlhedpsyoqfzuknpjwt")
    }
}
