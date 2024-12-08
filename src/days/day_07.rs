struct TestLine {
    total: usize,
    parts: Vec<usize>,
}

impl TestLine {
    fn evaluate_test(
        &self,
        current_total: usize,
        depth: usize,
        max_depth: usize,
        with_concat: bool,
    ) -> bool {
        let value = if depth == 0 {
            self.parts[0]
        } else {
            current_total
        };

        if depth == max_depth {
            value == self.total
        } else {
            let next_part = self.parts[depth + 1];

            let result = self.evaluate_test(value * next_part, depth + 1, max_depth, with_concat)
                || self.evaluate_test(value + next_part, depth + 1, max_depth, with_concat);

            if with_concat {
                let base = value * 10_usize.pow(next_part.checked_ilog10().unwrap_or(0) + 1);
                result || self.evaluate_test(base + next_part, depth + 1, max_depth, with_concat)
            } else {
                result
            }
        }
    }

    fn is_valid(&self, with_concat: bool) -> bool {
        self.evaluate_test(0, 0, self.parts.len() - 1, with_concat)
    }
}

pub fn exec(input: &str) -> (usize, usize) {
    let lines: Vec<_> = input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(": ").unwrap();

            TestLine {
                total: l.parse().unwrap(),
                parts: r.split(' ').map(|p| p.parse().unwrap()).collect(),
            }
        })
        .collect();

    let part_1 = lines
        .iter()
        .filter_map(|l| {
            if l.is_valid(false) {
                Some(l.total)
            } else {
                None
            }
        })
        .sum();
    let part_2 = lines
        .iter()
        .filter_map(|l| {
            if l.is_valid(true) {
                Some(l.total)
            } else {
                None
            }
        })
        .sum();

    (part_1, part_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;
    #[test]
    fn test_sample() {
        let day_07 = utils::read_input("07_sample");
        let day_07 = exec(&day_07);

        assert_eq!(day_07, (3749, 11387));
    }
}
