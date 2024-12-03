fn parse_line(line: &str) -> (i32, i32) {
    let mut items = line
        .split_whitespace()
        .filter_map(|w| w.parse::<i32>().ok());
    let first = items.next().unwrap();
    let second = items.next().unwrap();

    (first, second)
}

fn parse_list(source: &str) -> Vec<(i32, i32)> {
    source.split('\n').map(parse_line).collect()
}

pub fn exec(source: &str) -> (u32, usize) {
    let result = parse_list(source);

    let (mut left, mut right): (Vec<_>, Vec<_>) = result.iter().cloned().unzip();

    left.sort();
    right.sort();

    let part_one = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum();

    let part_two = left
        .iter()
        .map(|l| right.iter().filter(|r| *r == l).count() * *l as usize)
        .sum();

    (part_one, part_two)
}
