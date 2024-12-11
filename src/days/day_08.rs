use std::collections::{HashMap, HashSet};

type Coord = (isize, isize);

fn _print_map(map: &HashMap<Coord, char>, antinodes: &HashSet<Coord>, size: isize) {
    for y in 0..size {
        for x in 0..size {
            let coord = &(y, x);
            let is_antinode = antinodes.contains(coord);
            let map_value = match map.get(coord) {
                Some(value) => {
                    if is_antinode {
                        format!("({})", value)
                    } else {
                        format!(" {} ", value)
                    }
                }
                None => String::from(if is_antinode { " # " } else { " . " }),
            };

            print!("{}", map_value);
        }
        println!();
    }
    println!();
}

fn calculate_antinodes(a: Coord, b: Coord, max: isize, extrapolate: bool) -> HashSet<Coord> {
    let mut nodes = HashSet::new();

    let a_to_b = (b.0 - a.0, b.1 - a.1);
    let a_antinode = (a.0 - a_to_b.0, a.1 - a_to_b.1);
    if a_antinode.0 >= 0 && a_antinode.0 < max {
        if a_antinode.1 >= 0 && a_antinode.1 < max {
            nodes.insert(a_antinode);
            if extrapolate {
                let mut mult = 1;
                loop {
                    let a_harmonic = (
                        a_antinode.0 - (a_to_b.0 * mult),
                        a_antinode.1 - (a_to_b.1 * mult),
                    );
                    if a_harmonic.0 >= 0 && a_harmonic.0 < max {
                        if a_harmonic.1 >= 0 && a_harmonic.1 < max {
                            nodes.insert(a_harmonic);
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                    mult += 1;
                }
            }
        }
    }
    let b_to_a = (a.0 - b.0, a.1 - b.1);
    let b_antinode = (b.0 - b_to_a.0, b.1 - b_to_a.1);
    if b_antinode.0 >= 0 && b_antinode.0 < max {
        if b_antinode.1 >= 0 && b_antinode.1 < max {
            nodes.insert(b_antinode);
            if extrapolate {
                let mut mult = 1;
                loop {
                    let b_harmonic = (
                        b_antinode.0 - (b_to_a.0 * mult),
                        b_antinode.1 - (b_to_a.1 * mult),
                    );
                    if b_harmonic.0 >= 0 && b_harmonic.0 < max {
                        if b_harmonic.1 >= 0 && b_harmonic.1 < max {
                            nodes.insert(b_harmonic);
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                    mult += 1;
                }
            }
        }
    }

    nodes
}

fn calculate_all_nodes(points: &Vec<Coord>, max: isize, extrapolate: bool) -> HashSet<Coord> {
    let mut result = HashSet::new();

    for index in 0..points.len() {
        let a = &points[index];
        for b in points.iter().skip(index + 1) {
            result.extend(calculate_antinodes(*a, *b, max, extrapolate))
        }
        if extrapolate && points.len() > 2 {
            result.extend(points.iter().collect::<HashSet<_>>())
        }
    }

    result
}

pub fn exec(input: &str) -> (usize, usize) {
    let size = input.lines().count() as isize;
    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| line.chars().enumerate().map(move |(x, c)| (c, (y, x))))
        .fold(HashMap::new(), |mut acc, b| {
            b.for_each(|(freq, pos)| {
                if freq != '.' {
                    acc.entry(freq)
                        .or_insert(vec![])
                        .push((pos.0 as isize, pos.1 as isize));
                }
            });

            acc
        });

    let part_1_antinodes = map.iter().fold(HashMap::new(), |mut acc, (c, locs)| {
        acc.insert(c, calculate_all_nodes(locs, size, false));
        acc
    });

    let part_1 = part_1_antinodes
        .iter()
        .fold(HashSet::new(), |mut acc: HashSet<Coord>, (_, nodes)| {
            acc.extend(nodes);
            acc
        })
        .len();

    let part_2_antinodes = map
        .iter()
        .fold(HashMap::new(), |mut acc, (c, locs)| {
            acc.insert(c, calculate_all_nodes(locs, size, true));
            acc
        })
        .iter()
        .fold(HashSet::new(), |mut acc: HashSet<Coord>, (_, nodes)| {
            acc.extend(nodes);
            acc
        });

    let part_2 = part_2_antinodes.len();

    // let mut map_lookup: HashMap<Coord, char> = HashMap::new();
    //
    // for (c, locs) in map {
    //     for loc in locs {
    //         map_lookup.insert(loc, c);
    //     }
    // }

    //_print_map(&map_lookup, &HashSet::new(), size);
    //_print_map(&map_lookup, &part_2_antinodes, size);

    (part_1, part_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;
    #[test]
    fn test_sample() {
        let source = utils::read_input("08_sample");
        let result = exec(&source);
        assert_eq!(result, (14, 34));
    }
    #[test]
    fn test_sample_2() {
        let source = utils::read_input("08_sample_2");
        let result = exec(&source);
        assert_eq!(result, (3, 9));
    }

    #[test]
    fn test_single_line_1() {
        let source =
            "a.......\n..a.....\n........\n........\n........\n........\n........\n........";
        let result = exec(&source);
        assert_eq!(result, (1, 2));
    }

    #[test]
    fn test_single_line_2() {
        let source = ".aa.\n....\n....\n....";
        let result = exec(&source);
        assert_eq!(result, (2, 2));
    }

    #[test]
    fn test_single_line_3() {
        let source = ".aA.\n....\n....\n....";
        let result = exec(&source);
        assert_eq!(result, (0, 0));
    }

    #[test]
    fn test_top_left_corner_1() {
        let source = "aa..\n....\n....\n....";
        let result = exec(&source);
        assert_eq!(result, (1, 2));
    }

    #[test]
    fn test_top_left_corner_2() {
        let source = "a...\na...\n....\n....";
        let result = exec(&source);
        assert_eq!(result, (1, 2));
    }

    #[test]
    fn test_top_left_corner_3() {
        let source = "a...\n.a..\n....\n....";
        let result = exec(&source);
        assert_eq!(result, (1, 2));
    }

    #[test]
    fn test_top_left_corner_4() {
        let source = "aa...\n.....\n.....\n.....\n.....";
        let result = exec(&source);
        assert_eq!(result, (1, 3));
    }

    #[test]
    fn test_top_left_corner_5() {
        let source = "a.......\n........\n.a...\n.....\n.....";
        let result = exec(&source);
        assert_eq!(result, (1, 1));
    }

    #[test]
    fn test_top_right_corner_1() {
        let source = "..aa\n....\n....\n....";
        let result = exec(&source);
        assert_eq!(result, (1, 2));
    }

    #[test]
    fn test_top_right_corner_2() {
        let source = "...a\n...a\n....\n....";
        let result = exec(&source);
        assert_eq!(result, (1, 2));
    }
    #[test]
    fn test_top_right_corner_3() {
        let source = "...a\n..a.\n....\n....";
        let result = exec(&source);
        assert_eq!(result, (1, 2));
    }

    #[test]
    fn test_extrapolation_1() {
        let result = calculate_antinodes((0, 0), (2, 1), 10, true).len();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_extrapolation_2() {
        let result = calculate_antinodes((0, 0), (1, 3), 10, true).len();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_extrapolation_3() {
        let result = calculate_antinodes((2, 1), (1, 3), 10, true).len();
        assert_eq!(result, 1);
    }
}
