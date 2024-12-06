use crate::utils::pairwise;

fn parse_line(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .filter_map(|w| w.parse::<u32>().ok())
        .collect()
}

fn check_line(line: &Vec<u32>) -> bool {
    let sorted = line.iter().is_sorted() || line.iter().rev().is_sorted();

    if !sorted {
        return false;
    }

    let diff_match: Vec<_> = pairwise(line)
        .filter(|(l, r)| match (l, r) {
            (Some(l), r) => {
                let diff = l.abs_diff(**r);
                diff > 3 || diff == 0
            }
            (None, _) => false,
        })
        .collect();

    diff_match.len() == 0
}

fn check_unsafe_line(line: &Vec<u32>) -> bool {
    let length = line.len();

    for i in 0..length {
        let mut copy = line.clone();
        copy.remove(i);
        if check_line(&copy) {
            return true;
        }
    }

    false
}

pub fn exec(source: &str) -> (usize, usize) {
    let lines: Vec<_> = source.lines().map(|line| parse_line(line)).collect();
    let safe_lines = lines.iter().filter(|line| check_line(line)).count();

    let unsafe_lines = lines.iter().filter(|line| !check_line(line));

    let probably_safe_lines = unsafe_lines.filter(|line| check_unsafe_line(line)).count();

    (safe_lines, safe_lines + probably_safe_lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_line() {
        let line = "7 6 4 2 1";

        let result = check_line(&parse_line(line));
        assert!(result);
    }
    #[test]
    fn test_incorrect_line() {
        let line = "12 6 4 2 1";

        assert!(!check_line(&parse_line(line)));
    }
    #[test]
    fn test_unsorted_line() {
        let line = "12 6 18 4 1";

        assert!(!check_line(&parse_line(line)));
    }

    #[test]
    fn test_sample() {
        let sample = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let result = exec(sample);
        assert_eq!(result, (2, 4));
    }
}
