struct Position {
    x: usize,
    y: usize,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_filepath = args.get(1).expect("Must pass at least one parameter for the filepath");

    let input: String = std::fs::read_to_string(input_filepath).expect("The filepath given was not valid");

    let part2_flag: Option<&str> = args.get(2).map(String::as_str);

    match part2_flag {
        Some("--part2") => part2(input),
        Some(_) => unreachable!("wtf are you trying to do?"),
        None => part1(input),
    }
}

fn part1(input: String) {
    let mut keypad_position = Position { x: 1, y: 1 };
    let mut code: usize = 0;
    for instruction in input.lines() {
        for direction in instruction.chars() { 
            match direction {
                'U' => keypad_position.y = keypad_position.y.saturating_sub(1),
                'R' => if keypad_position.x != 2 { keypad_position.x = keypad_position.x + 1; },
                'D' => if keypad_position.y != 2 { keypad_position.y = keypad_position.y + 1; },
                'L' => keypad_position.x = keypad_position.x.saturating_sub(1),
                _ => unreachable!("This is wrong :("),
            }
        }
        let keypad_num = match (keypad_position.x, keypad_position.y) {
            (0, 0) => 1,
            (1, 0) => 2,
            (2, 0) => 3,
            (0, 1) => 4,
            (1, 1) => 5,
            (2, 1) => 6,
            (0, 2) => 7,
            (1, 2) => 8,
            (2, 2) => 9,
            _ => unreachable!("The position shouldn't be out of bounds"),
        };
        code = code * 10 + keypad_num;
    }
    println!("{}", code);
}

fn part2(input: String) {
    todo!()
}
