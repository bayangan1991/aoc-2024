mod utils;
mod days;

fn main() {
    let day_01 = utils::read_input("01");
    let day_01 = days::day_01::exec(&day_01);
    println!("Day 1, Part 1: {}", day_01.0);
    println!("Day 1, Part 2: {}\n", day_01.1);

    let day_02 = utils::read_input("02");
    let day_02 = days::day_02::exec(&day_02);
    println!("Day 2, Part 1: {}", day_02.0);
    println!("Day 2, Part 2: {}\n", day_02.1);
}
