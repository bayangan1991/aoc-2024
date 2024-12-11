mod days;
mod utils;

fn main() {
    let functions = [
        days::day_01::exec,
        days::day_02::exec,
        days::day_03::exec,
        days::day_04::exec,
        days::day_05::exec,
        days::day_06::exec,
        days::day_07::exec,
        days::day_08::exec,
        days::day_09::exec,
        days::day_10::exec,
    ];

    for (day, exec) in functions.iter().enumerate() {
        let input = utils::read_input(format!("{:02}", day + 1).as_str());
        let result = exec(&input);
        println!("Day {}, Part 1: {}", day + 1, result.0);
        println!("Day {}, Part 2: {}\n", day + 1, result.1);
    }
}
