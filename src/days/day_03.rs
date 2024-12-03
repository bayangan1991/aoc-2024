use regex::Regex;

pub fn exec(source: &str) -> (usize, usize) {
    let mul = Regex::new(r"(?<inst>(do|don't|mul))\(((?<left>\d+),(?<right>\d+))?\)").unwrap();

    let mut enabled = true;

    let mut part_1 = 0;
    let mut part_2 = 0;
    for result in mul.captures_iter(source) {
        println!("{:?}", result);
        match &result["inst"] {
            "do" => {
                enabled = true;
            }
            "don't" => {
                enabled = false;
            }
            "mul" => {
                let result = &result["left"].parse::<usize>().unwrap()
                    * result["right"].parse::<usize>().unwrap();
                part_1 += result;

                if enabled {
                    part_2 += result;
                }
            }
            _ => {}
        }
    }

    (part_1, part_2)
}
