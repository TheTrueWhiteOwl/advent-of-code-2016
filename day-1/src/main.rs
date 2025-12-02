enum Direction {
    North,
    East,
    South,
    West,
}

const STARTING_DIRECTION: Direction = Direction::North;

impl Direction {
    const fn rotate_clockwise(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    const fn rotate_anticlockwise(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

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
        Some(_) => panic!("wtf are you trying to do?"),
        None => part1(input),
    }
}

fn part1(input: String) {
    let input = input.trim();
    let mut direction = STARTING_DIRECTION;
    let mut position = Position { x: 0, y: 0 };
    for instruction in input.split(", ") {
        match &instruction[..1] {
            "L" => direction = direction.rotate_anticlockwise(),
            "R" => direction = direction.rotate_clockwise(),
            _ => panic!("This should never be the case"),
        }
        let steps: isize = instruction[1..].parse().expect("This will always be a valid number");
        match direction {
            Direction::North => position.y += steps,
            Direction::East => position.x += steps,
            Direction::South => position.y -= steps,
            Direction::West => position.x -= steps,
        }
    }

    println!("{}", position.x.abs() + position.y.abs());
}

fn part2(input: String) {
    todo!()
}
