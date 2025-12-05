#![feature(iter_array_chunks)]

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
    let mut possible_triangle_count = 0;
    for side_lengths_str in input.lines() {
        let side_lengths: Vec<usize> = side_lengths_str.split_whitespace().map(str::parse).map(Result::unwrap).collect();
        let largest_side = side_lengths.iter().max().unwrap();
        if side_lengths.iter().sum::<usize>() - largest_side > *largest_side {
            possible_triangle_count += 1;
        }
    }
    println!("{}", possible_triangle_count);
}

fn part2(input: String) {
    let mut possible_triangle_count = 0;
    let columns: [Vec<usize>; 3] = input.lines()
        .map(|line| {
            let len_vec: Vec<usize> = line.split_whitespace().map(str::parse).map(Result::unwrap).collect();
            (len_vec[0], len_vec[1], len_vec[2])
        })
        .collect::<(Vec<usize>, Vec<usize>, Vec<usize>)>()
        .into();
    for side_lengths in columns.into_iter().flatten().array_chunks::<3>() {
        let largest_side = side_lengths.iter().max().unwrap();
        if side_lengths.into_iter().sum::<usize>() - largest_side > *largest_side {
            possible_triangle_count += 1;
        }
    }
    println!("{}", possible_triangle_count);
}
