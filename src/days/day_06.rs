use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Thing {
    Air,
    Obstruction,
    Start,
}

type MapKey = (isize, isize);
type Map = HashMap<MapKey, Thing>;

fn _print_map(map: &Map, current_position: MapKey, current_dir: usize, visited: &HashSet<MapKey>) {
    let size = map.iter().map(|(k, _)| k.0).max().unwrap() + 1;

    for x in 0..size {
        for y in 0..size {
            if (x, y) == current_position {
                print!("{}", ['^', '>', 'v', '<'][current_dir]);
            } else {
                print!(
                    "{}",
                    match &map.get(&(x, y)).unwrap() {
                        Thing::Air | Thing::Start =>
                            if visited.contains(&(x, y)) {
                                'X'
                            } else {
                                '.'
                            },
                        Thing::Obstruction => '#',
                    }
                )
            }
        }
        println!();
    }
    println!();
}

fn evaluate_path(
    map: &Map,
    start: MapKey,
    direction: isize,
) -> (HashSet<(MapKey, isize)>, bool, HashMap<MapKey, isize>) {
    let mut direction = direction;
    let mut touched_places = HashSet::new();
    let mut first_visits = HashMap::new();
    let mut current_pos = start;

    touched_places.insert((start, direction));

    loop {
        let next_step = match direction {
            0 => (current_pos.0 - 1, current_pos.1),
            1 => (current_pos.0, current_pos.1 + 1),
            2 => (current_pos.0 + 1, current_pos.1),
            3 => (current_pos.0, current_pos.1 - 1),
            _ => panic!("Unknown direction: {}", direction),
        };

        if touched_places.contains(&(next_step, direction)) {
            return (touched_places, true, first_visits);
        }

        match map.get(&next_step) {
            Some(&Thing::Air | &Thing::Start) => {
                touched_places.insert((next_step, direction));
                current_pos = next_step;
                if !first_visits.contains_key(&current_pos) {
                    first_visits.insert(current_pos, direction);
                }
            }
            Some(&Thing::Obstruction) => {
                direction = (direction + 1) % 4;
            }
            None => break,
        }
        //_print_map(&map, current_pos, direction, &touched_places);
    }

    (touched_places, false, first_visits)
}

pub fn exec(input: &str) -> (usize, usize) {
    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| match c {
                '.' => ((y as isize, x as isize), Thing::Air),
                '#' => ((y as isize, x as isize), Thing::Obstruction),
                '^' => ((y as isize, x as isize), Thing::Start),
                _ => panic!("Unknown thing"),
            })
        })
        .fold(HashMap::new(), |mut acc, b| {
            b.for_each(|item| {
                let foo = item.0;
                let bar = item.1;
                acc.insert(foo, bar);
            });

            acc
        });

    let inverted = &map
        .iter()
        .map(|(pos, thing)| (thing, pos))
        .collect::<HashMap<_, _>>();
    let start = **inverted.get(&Thing::Start).unwrap();

    let part_1_path = evaluate_path(&map, start, 0);

    let part_1 = part_1_path
        .0
        .iter()
        .map(|(pos, _)| pos)
        .collect::<HashSet<_>>()
        .len();

    let part_2 = part_1_path
        .2
        .iter()
        .filter_map(|(obst, direction)| {
            let new_map = &mut map.clone();
            new_map.insert(*obst, Thing::Obstruction);

            let prev = match direction {
                0 => (obst.0 + 1, obst.1),
                1 => (obst.0, obst.1 - 1),
                2 => (obst.0 - 1, obst.1),
                3 => (obst.0, obst.1 + 1),
                _ => panic!("Unknown direction: {}", direction),
            };

            if evaluate_path(&new_map, prev, *direction).1 {
                Some(obst)
            } else {
                None
            }
        })
        .collect::<HashSet<_>>()
        .len();

    (part_1, part_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_sample() {
        let day_06 = utils::read_input("06_sample");
        let day_06 = exec(&day_06);

        assert_eq!(day_06, (41, 6));
    }
}
