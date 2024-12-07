use std::cmp::Ordering;
use std::collections::HashSet;

fn item_order(
    l: usize,
    r: usize,
    rules: &HashSet<(usize, usize)>,
    rules_rev: &HashSet<(usize, usize)>,
) -> Ordering {
    for (rule_l, rule_r) in rules {
        if l == *rule_l && r == *rule_r {
            return Ordering::Less;
        }
    }
    for (rule_l, rule_r) in rules_rev {
        if l == *rule_l && r == *rule_r {
            return Ordering::Greater;
        }
    }

    Ordering::Equal
}

fn in_order(
    line: &Vec<usize>,
    rules: &HashSet<(usize, usize)>,
    rules_rev: &HashSet<(usize, usize)>,
) -> bool {
    for i in 0..line.len() {
        let item = &line[i];
        let left = &line[..i];
        let right = &line[i + 1..];

        for left_i in left {
            if item_order(*item, *left_i, rules, rules_rev) == Ordering::Less {
                return false;
            }
        }

        for right_i in right {
            if item_order(*item, *right_i, rules, rules_rev) == Ordering::Greater {
                return false;
            }
        }
    }

    true
}

pub fn exec(source: &str) -> (usize, usize) {
    let (rules_raw, pages_raw) = source.split_once("\n\n").unwrap();

    let rules: HashSet<_> = rules_raw
        .lines()
        .map(|line| {
            line.split_once("|")
                .map(|(l, r)| (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap()))
                .unwrap()
        })
        .collect();

    let rules_rev: HashSet<_> = rules.iter().map(|(k, v)| (*v, *k)).collect();

    let lines: Vec<Vec<usize>> = pages_raw
        .lines()
        .map(|line| line.split(',').map(|i| i.parse().unwrap()).collect())
        .collect();

    let part_1 = &lines
        .iter()
        .filter(|line| in_order(line, &rules, &rules_rev))
        .map(|items| items[items.len() / 2])
        .sum();

    let part_2 = &lines
        .iter()
        .filter(|l| !in_order(l, &rules, &rules_rev))
        .map(|l| {
            let mut sorted: Vec<_> = l.clone();
            sorted.sort_by(|a, b| item_order(*a, *b, &rules, &rules_rev));

            sorted[sorted.len() / 2]
        })
        .sum();

    (*part_1, *part_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_sample() {
        let day_05 = utils::read_input("05_sample");
        let day_05 = exec(&day_05);

        assert_eq!(day_05, (143, 123));
    }
}
