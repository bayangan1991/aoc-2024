use std::collections::HashMap;

fn parse_line(line: &str) -> HashMap<usize, char> {
    HashMap::from_iter(line.chars().enumerate())
}

pub fn exec(input: &str) -> (usize, usize) {
    let grid: HashMap<usize, _> = HashMap::from_iter(
        input
            .lines()
            .enumerate()
            .map(|(y, line)| (y, parse_line(line)))
            .into_iter(),
    );

    let part_1 = calc_part_1(&grid);
    let part_2 = calc_part_2(&grid);

    (part_1, part_2)
}

fn calc_part_1(grid: &HashMap<usize, HashMap<usize, char>>) -> usize {
    let size = grid.len();

    let mut result = 0;

    for y in 0..size {
        for x in 0..size {
            if &grid[&y][&x] != &'X' {
                continue;
            }
            let right_inside = x + 3 < size;
            let left_inside = x >= 3;
            let down_inside = y + 3 < size;
            let up_inside = y >= 3;

            let right = if right_inside {
                Some([
                    grid[&y][&(x + 0)],
                    grid[&y][&(x + 1)],
                    grid[&y][&(x + 2)],
                    grid[&y][&(x + 3)],
                ])
            } else {
                None
            };
            let left = if left_inside {
                Some([
                    grid[&y][&(x - 0)],
                    grid[&y][&(x - 1)],
                    grid[&y][&(x - 2)],
                    grid[&y][&(x - 3)],
                ])
            } else {
                None
            };
            let down = if down_inside {
                Some([
                    grid[&(y + 0)][&x],
                    grid[&(y + 1)][&x],
                    grid[&(y + 2)][&x],
                    grid[&(y + 3)][&x],
                ])
            } else {
                None
            };
            let up = if up_inside {
                Some([
                    grid[&(y - 0)][&x],
                    grid[&(y - 1)][&x],
                    grid[&(y - 2)][&x],
                    grid[&(y - 3)][&x],
                ])
            } else {
                None
            };
            let down_right = if down_inside && right_inside {
                Some([
                    grid[&(y + 0)][&(x + 0)],
                    grid[&(y + 1)][&(x + 1)],
                    grid[&(y + 2)][&(x + 2)],
                    grid[&(y + 3)][&(x + 3)],
                ])
            } else {
                None
            };
            let down_left = if down_inside && left_inside {
                Some([
                    grid[&(y + 0)][&(x - 0)],
                    grid[&(y + 1)][&(x - 1)],
                    grid[&(y + 2)][&(x - 2)],
                    grid[&(y + 3)][&(x - 3)],
                ])
            } else {
                None
            };
            let up_right = if up_inside && right_inside {
                Some([
                    grid[&(y - 0)][&(x + 0)],
                    grid[&(y - 1)][&(x + 1)],
                    grid[&(y - 2)][&(x + 2)],
                    grid[&(y - 3)][&(x + 3)],
                ])
            } else {
                None
            };
            let up_left = if up_inside && left_inside {
                Some([
                    grid[&(y - 0)][&(x - 0)],
                    grid[&(y - 1)][&(x - 1)],
                    grid[&(y - 2)][&(x - 2)],
                    grid[&(y - 3)][&(x - 3)],
                ])
            } else {
                None
            };

            result += [
                left, right, down, up, down_right, down_left, up_right, up_left,
            ]
            .map(|d| match d {
                Some(['X', 'M', 'A', 'S']) => 1,
                _ => 0,
            })
            .iter()
            .sum::<usize>()
        }
    }
    result
}

fn calc_part_2(grid: &HashMap<usize, HashMap<usize, char>>) -> usize {
    let size = grid.len();

    let mut result = 0;

    for y in 1..size - 1 {
        for x in 1..size - 1 {
            if &grid[&y][&x] != &'A' {
                continue;
            }

            let patch = [
                grid[&(y - 1)][&(x - 1)],
                grid[&(y - 1)][&(x + 1)],
                grid[&(y + 1)][&(x + 1)],
                grid[&(y + 1)][&(x - 1)],
            ];

            let mut patch_count = 0;

            patch_count += match patch {
                ['M', _, 'S', _] => 1,
                _ => 0,
            };

            patch_count += match patch {
                ['S', _, 'M', _] => 1,
                _ => 0,
            };

            patch_count += match patch {
                [_, 'S', _, 'M'] => 1,
                _ => 0,
            };

            patch_count += match patch {
                [_, 'M', _, 'S'] => 1,
                _ => 0,
            };

            result += if patch_count == 2 { 1 } else { 0 };
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample_input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        assert_eq!(exec(sample_input), (18, 9));
    }

    #[test]
    fn test_right() {
        let sample_input = "XMAS\n....\n....\n....";
        assert_eq!(exec(sample_input), (1, 0));
    }
    #[test]
    fn test_left() {
        let sample_input = "SAMX\n....\n....\n....";
        assert_eq!(exec(sample_input), (1, 0));
    }
    #[test]
    fn test_down() {
        let sample_input = "X...\nM...\nA...\nS...";
        assert_eq!(exec(sample_input), (1, 0));
    }
    #[test]
    fn test_up() {
        let sample_input = "S...\nA...\nM...\nX...";
        assert_eq!(exec(sample_input), (1, 0));
    }
    #[test]
    fn test_simple_2() {
        let sample_input = "SAMX\n.AM.\n.AM.\nS..X";
        assert_eq!(exec(sample_input), (3, 0));
    }
}
