#[derive(Clone)]
struct Position {
    x: isize,
    y: isize,
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
                'U' => if keypad_position.y != 0 { keypad_position.y -= 1 },
                'R' => if keypad_position.x != 2 { keypad_position.x += 1 },
                'D' => if keypad_position.y != 2 { keypad_position.y += 1 },
                'L' => if keypad_position.x != 0 { keypad_position.x -= 1 },
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
    let mut keypad_position = Position { x: 1, y: 1 };
    let mut code = String::new();
    for instruction in input.lines() {
        for direction in instruction.chars() { 
            let previous_position = keypad_position.clone();
            match direction {
                'U' => keypad_position.y -= 1,
                'R' => keypad_position.x += 1,
                'D' => keypad_position.y += 1,
                'L' => keypad_position.x -= 1,
                _ => unreachable!("This is wrong :("),
            }
            if keypad_position.x.abs() + keypad_position.y.abs() > 2 {
                keypad_position = previous_position
            }
        }
        let keypad_char = match (keypad_position.x, keypad_position.y) {
            (0, -2) => '1',
            (-1, -1) => '2',
            (0, -1) => '3',
            (1, -1) => '4',
            (-2, 0) => '5',
            (-1, 0) => '6',
            (0, 0) => '7',
            (1, 0) => '8',
            (2, 0) => '9',
            (-1, 1) => 'A',
            (0, 1) => 'B',
            (1, 1) => 'C',
            (0, 2) => 'D',
            _ => unreachable!("The position shouldn't be out of bounds"),
        };
        code.push(keypad_char);
    }
    println!("{}", code);
}
