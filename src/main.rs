mod days;
mod utils;

fn main() {
    let day_01 = utils::read_input("01");
    let day_01 = days::day_01::exec(&day_01);
    println!("Day 1, Part 1: {}", day_01.0);
    println!("Day 1, Part 2: {}\n", day_01.1);

    let day_02 = utils::read_input("02");
    let day_02 = days::day_02::exec(&day_02);
    println!("Day 2, Part 1: {}", day_02.0);
    println!("Day 2, Part 2: {}\n", day_02.1);

    let day_03 = utils::read_input("03");
    let day_03 = days::day_03::exec(&day_03);
    println!("Day 3, Part 1: {}", day_03.0);
    println!("Day 3, Part 2: {}\n", day_03.1);

    let day_04 = utils::read_input("04");
    let day_04 = days::day_04::exec(&day_04);
    println!("Day 4, Part 1: {}", day_04.0);
    println!("Day 4, Part 2: {}\n", day_04.1);
}
