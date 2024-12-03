use std::fs;
use std::iter;

pub fn read_input(day: &str) -> String {
    let filepath = format!("data/{}", day);
    let err = format!("Unable to find '{filepath}'");
    fs::read_to_string(filepath).expect(&err)
}

pub fn pairwise<I>(right: I) -> impl Iterator<Item = (Option<I::Item>, I::Item)>
where
    I: IntoIterator + Clone,
{
    let left = iter::once(None).chain(right.clone().into_iter().map(Some));
    left.zip(right)
}
