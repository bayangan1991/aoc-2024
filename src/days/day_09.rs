use std::collections::HashMap;

fn parse_diskmap(input: &str) -> Vec<Option<i32>> {
    let mut disk = vec![];

    let mut switch = true;
    let mut id = 0;
    for char in input.chars().map(|c| c.to_digit(10).unwrap() as usize) {
        if switch {
            disk.extend([Some(id)].repeat(char));
            id += 1;
        } else {
            if char > 0 {
                disk.extend([None].repeat(char));
            }
        }

        switch = !switch;
    }

    disk
}

fn simple_defrag(disk: &Vec<Option<i32>>) -> Vec<Option<i32>> {
    let mut defragged = disk.clone();

    let mut rev_map = disk.iter().enumerate().rev().filter(|(_, x)| !x.is_none());

    for index in 0..disk.len() {
        match &disk[index] {
            None => match rev_map.next() {
                None => break,
                Some((swap_index, swap)) => {
                    if swap_index < index {
                        break;
                    }
                    defragged[index] = *swap;
                    defragged[swap_index] = None;
                }
            },
            _ => {}
        }
    }

    defragged
}

fn better_defrag(disk: &Vec<Option<i32>>) -> Vec<Option<i32>> {
    let mut defragged = disk.clone();

    let files = disk.iter().enumerate().filter(|(_, x)| !x.is_none()).fold(
        HashMap::new(),
        |mut acc, (i, x)| {
            acc.entry(x.unwrap())
                .and_modify(|(_, x)| *x += 1)
                .or_insert((i, 1));

            acc
        },
    );
    let max_index = *files.keys().max().unwrap();

    'f: for file_index in (0..=max_index).rev() {
        let (file_start, file_size) = files[&file_index];
        let mut empty_size = 0;
        for i in 0..disk.len() {
            if file_start < i {
                break;
            }
            match &defragged[i] {
                None => {
                    empty_size += 1;

                    if empty_size == file_size {
                        let empty_start = i + 1 - empty_size;
                        for j in 0..empty_size {
                            defragged[empty_start + j] = Some(file_index);
                            defragged[file_start + j] = None
                        }
                        continue 'f;
                    }
                }
                _ => empty_size = 0,
            }
        }
    }

    defragged
}

fn checksum(disk: &Vec<Option<i32>>) -> usize {
    disk.iter()
        .enumerate()
        .map(|(i, index)| match index {
            Some(f) => *f as usize * i,
            None => 0,
        })
        .sum()
}

pub fn exec(input: &str) -> (usize, usize) {
    let disk = parse_diskmap(input);
    let defragged = simple_defrag(&disk);
    let better_defrag = better_defrag(&disk);

    let part_1 = checksum(&defragged);
    let part_2 = checksum(&better_defrag);

    (part_1, part_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_sample() {
        let input = utils::read_input("09_sample");
        let result = exec(&input);

        assert_eq!(result, (1928, 2858));
    }

    #[test]
    fn test_parse_1() {
        let result = parse_diskmap("101");
        assert_eq!(result, vec![Some(0), Some(1)]);
    }
    #[test]
    fn test_parse_2() {
        let result = parse_diskmap("432");
        assert_eq!(
            result,
            vec![
                Some(0),
                Some(0),
                Some(0),
                Some(0),
                None,
                None,
                None,
                Some(1),
                Some(1)
            ]
        );
    }

    #[test]
    fn test_defrag_1() {
        let disk = parse_diskmap("101");
        let result = simple_defrag(&disk);
        assert_eq!(result, vec![Some(0), Some(1)]);
    }

    #[test]
    fn test_defrag_2() {
        let disk = parse_diskmap("432");
        let result = simple_defrag(&disk);
        assert_eq!(
            result,
            vec![
                Some(0),
                Some(0),
                Some(0),
                Some(0),
                Some(1),
                Some(1),
                None,
                None,
                None,
            ]
        );
    }
    #[test]
    fn test_defrag_3() {
        let disk = parse_diskmap("12113");
        let result = simple_defrag(&disk);
        assert_eq!(
            result,
            vec![
                Some(0),
                Some(2),
                Some(2),
                Some(1),
                Some(2),
                None,
                None,
                None,
            ]
        );
    }

    #[test]
    fn test_better_defrag_1() {
        let disk = parse_diskmap("112");
        let result = better_defrag(&disk);
        assert_eq!(result, vec![Some(0), None, Some(1), Some(1)]);
    }

    #[test]
    fn test_better_defrag_2() {
        let disk = parse_diskmap("122");
        let result = better_defrag(&disk);
        assert_eq!(result, vec![Some(0), Some(1), Some(1), None, None]);
    }

    #[test]
    fn test_better_defrag_3() {
        let disk = parse_diskmap("1333102");
        let result = better_defrag(&disk);
        assert_eq!(
            result,
            vec![
                Some(0),
                Some(3),
                Some(3),
                Some(2),
                Some(1),
                Some(1),
                Some(1),
                None,
                None,
                None,
                None,
                None,
                None
            ]
        );
    }
}
