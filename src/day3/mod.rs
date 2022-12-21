use std::collections::HashMap;
use std::collections::HashSet;

type Coord = (u32, u32);

pub fn solve1(claims: Vec<(u32, Coord, Coord)>) -> u32 {
    let grid = fill_grid(&claims);

    grid.values()
        .map(|list| if list.len() > 1 { 1 } else { 0 })
        .sum()
}

pub fn solve2(claims: Vec<(u32, Coord, Coord)>) -> u32 {
    let grid = fill_grid(&claims);

    let (max_id, _, _) = *claims.iter().max_by_key(|(id, _, _)| *id).unwrap();

    let mut set: HashSet<u32> = (1..=max_id).collect();

    for (_, list) in grid.iter() {
        if list.len() > 1 {
            for id in list.iter() {
                set.remove(id);
            }
        }
    }

    set.into_iter().next().unwrap()
}

fn fill_grid(claims: &[(u32, Coord, Coord)]) -> HashMap<Coord, Vec<u32>> {
    let mut grid: HashMap<_, _> = HashMap::new();

    for &(id, (x, y), (w, h)) in claims {
        for i in x..(x + w) {
            for j in y..(y + h) {
                grid.entry((i, j)).or_insert(vec![]).push(id);
            }
        }
    }

    grid
}

fn debug_print_map(grid: &HashMap<Coord, Vec<u32>>) {
    for i in 0..=8 {
        for j in 0..=7 {
            match grid.get(&(j, i)) {
                Some(arr) => if arr.len() > 1 {
                    print!("X")
                } else {
                    print!("{}", arr.first().unwrap())
                },
                None => print!("."),
            }
        }
        println!();
    }
}

fn parse_line(line: &str) -> (u32, (u32, u32), (u32, u32)) {
    let splits: Vec<_> = line
        .split(&['#', '@', ',', ':', 'x'])
        .filter(|&c| !c.is_empty())
        .flat_map(|s| s.trim().parse())
        .collect();

    (splits[0], (splits[1], splits[2]), (splits[3], splits[4]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::*;

    #[test]
    fn it_returns_num_of_overlapping_claims() {
        let claims = vec![
            (1, (1, 3), (4, 4)),
            (2, (3, 1), (4, 4)),
            (3, (5, 5), (2, 2)),
        ];

        assert_eq!(solve1(claims), 4)
    }

    #[test]
    fn it_parses_a_line_correctly() {
        assert_eq!(parse_line("#1 @ 1,3: 4x4"), (1, (1, 3), (4, 4)))
    }

    #[test]
    fn it_parses_another_line_correctly() {
        assert_eq!(parse_line("#123 @ 3,2: 5x4"), (123, (3, 2), (5, 4)))
    }

    #[test]
    fn it_handles_input_for_star_1() {
        let strings = map_lines_to_strings("./src/day3/input");
        let claims = strings.iter().map(|s| s.as_str()).map(parse_line).collect();

        assert_eq!(solve1(claims), 119551)
    }

    #[test]
    fn it_handles_input_for_star_2() {
        let strings = map_lines_to_strings("./src/day3/input");
        let claims = strings.iter().map(|s| s.as_str()).map(parse_line).collect();

        assert_eq!(solve2(claims), 1124)
    }

}
