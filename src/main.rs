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

    let day_05 = utils::read_input("05");
    let day_05 = days::day_05::exec(&day_05);
    println!("Day 5, Part 1: {}", day_05.0);
    println!("Day 5, Part 2: {}\n", day_05.1);

    let day_06 = utils::read_input("06");
    let day_06 = days::day_06::exec(&day_06);
    println!("Day 6, Part 1: {}", day_06.0);
    println!("Day 6, Part 2: {}\n", day_06.1);

    let day_07 = utils::read_input("07");
    let day_07 = days::day_07::exec(&day_07);
    println!("Day 7, Part 1: {}", day_07.0);
    println!("Day 7, Part 2: {}\n", day_07.1);

    let day_08 = utils::read_input("08");
    let day_08 = days::day_08::exec(&day_08);
    println!("Day 8, Part 1: {}", day_08.0);
    println!("Day 8, Part 2: {}\n", day_08.1);
}
