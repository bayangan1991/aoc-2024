use std::collections::{HashMap, HashSet};

type Coord = (usize, usize);
fn parse_map(input: &str) -> (HashMap<Coord, Option<u32>>, HashSet<Coord>) {
    let mut map = HashMap::new();
    let mut starts = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let height = ch.to_digit(10);
            map.insert((y, x), height);
            if height == Some(0) {
                starts.insert((y, x));
            }
        }
    }

    (map, starts)
}

fn count_trailheads_simple(
    map: &HashMap<Coord, Option<u32>>,
    y: usize,
    x: usize,
    max: usize,
    trailheads: &mut HashSet<Coord>,
) {
    let current_value = map[&(y, x)].unwrap_or(0);
    if current_value == 9 {
        trailheads.insert((y, x));
    } else {
        if y > 0 {
            if map[&(y - 1, x)] == Some(current_value + 1) {
                count_trailheads_simple(map, y - 1, x, max, trailheads);
            }
        };
        if y < max {
            if map[&(y + 1, x)] == Some(current_value + 1) {
                count_trailheads_simple(map, y + 1, x, max, trailheads);
            }
        };
        if x > 0 {
            if map[&(y, x - 1)] == Some(current_value + 1) {
                count_trailheads_simple(map, y, x - 1, max, trailheads);
            }
        };
        if x < max {
            if map[&(y, x + 1)] == Some(current_value + 1) {
                count_trailheads_simple(map, y, x + 1, max, trailheads);
            }
        };
    };
}

fn count_trailheads_advanced(
    map: &HashMap<Coord, Option<u32>>,
    y: usize,
    x: usize,
    max: usize,
) -> usize {
    let current_value = map[&(y, x)].unwrap_or(0);
    if current_value == 9 {
        1
    } else {
        let up = if y > 0 && map[&(y - 1, x)] == Some(current_value + 1) {
            count_trailheads_advanced(map, y - 1, x, max)
        } else {
            0
        };
        let down = if y < max && map[&(y + 1, x)] == Some(current_value + 1) {
            count_trailheads_advanced(map, y + 1, x, max)
        } else {
            0
        };
        let left = if x > 0 && map[&(y, x - 1)] == Some(current_value + 1) {
            count_trailheads_advanced(map, y, x - 1, max)
        } else {
            0
        };
        let right = if x < max && map[&(y, x + 1)] == Some(current_value + 1) {
            count_trailheads_advanced(map, y, x + 1, max)
        } else {
            0
        };

        up + down + left + right
    }
}

pub fn exec(input: &str) -> (usize, usize) {
    let size = input.lines().count() - 1;
    let (map, starts) = parse_map(input);

    let part_1 = starts
        .iter()
        .map(|start| {
            let mut trailheads = HashSet::new();
            count_trailheads_simple(&map, start.0, start.1, size, &mut trailheads);
            trailheads.len()
        })
        .sum();
    let part_2 = starts
        .iter()
        .map(|start| count_trailheads_advanced(&map, start.0, start.1, size))
        .sum();

    (part_1, part_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_sample() {
        let input = utils::read_input("10_sample");
        let result = exec(&input);

        assert_eq!(result, (36, 81));
    }

    #[test]
    fn test_sample_2() {
        let input = utils::read_input("10_sample_2");
        let result = exec(&input);

        assert_eq!(result, (2, 0));
    }

    #[test]
    fn test_simple_1() {
        let input = "0123\n7654\n8911\n1111";
        let result = exec(&input);

        assert_eq!(result, (1, 0));
    }
    #[test]
    fn test_simple_2() {
        let input = "0123\n7654\n8911\n9111";
        let result = exec(&input);

        assert_eq!(result, (2, 0));
    }
}
