mod utils;
mod days;

fn main() {
    let day_01 = utils::read_input("01");
    let day_01 = days::day_01::exec(&day_01);
    println!("Day 1, Part 1: {}", day_01.0);
    println!("Day 1, Part 2: {}\n", day_01.1);
}
