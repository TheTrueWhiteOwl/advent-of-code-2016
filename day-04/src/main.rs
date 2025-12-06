use std::collections::HashMap;

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

fn count(chr: char, string: &str) -> usize {
    string.chars().filter(|c| *c == chr).count()
}

fn part1(input: String) {
    let mut real_room_sector_ids: u32 = 0;
    for room_data in input.lines() {
        let checksum_start = room_data.rfind('[').unwrap();
        let checksum_end = room_data.rfind(']').unwrap();
        let checksum = &room_data[checksum_start+1..checksum_end];

        let sector_id_start = room_data.rfind('-').unwrap();

        let room_name = &room_data[..sector_id_start];

        let mut char_to_count_map = HashMap::new();
        room_name.chars().filter(|&chr| chr != '-').for_each(|chr| { char_to_count_map.entry(chr).or_insert(count(chr, room_name)); });
        let mut char_to_count: Vec<(char, usize)> = char_to_count_map.into_iter().collect();
        char_to_count.sort_by(|&(chr1, count1), &(chr2, count2)| (-(count1 as isize), chr1).cmp(&(-(count2 as isize), chr2)));
        let full_computed_checksum: String = char_to_count.into_iter().collect::<(Vec<char>, Vec<usize>)>().0.iter().collect();
        if full_computed_checksum.starts_with(checksum) {
            let sector_id: u32 = room_data[sector_id_start+1..checksum_start].parse().expect("The sector id is always a number");
            real_room_sector_ids += sector_id;
        }
    }
    println!("{}", real_room_sector_ids);
}

fn part2(input: String) {
    for room_data in input.lines() {
        let checksum_start = room_data.rfind('[').unwrap();
        let checksum_end = room_data.rfind(']').unwrap();
        let checksum = &room_data[checksum_start+1..checksum_end];

        let sector_id_start = room_data.rfind('-').unwrap();

        let room_name = &room_data[..sector_id_start];

        let mut char_to_count_map = HashMap::new();
        room_name.chars().filter(|&chr| chr != '-').for_each(|chr| { char_to_count_map.entry(chr).or_insert(count(chr, room_name)); });
        let mut char_to_count: Vec<(char, usize)> = char_to_count_map.into_iter().collect();
        char_to_count.sort_by(|&(chr1, count1), &(chr2, count2)| (-(count1 as isize), chr1).cmp(&(-(count2 as isize), chr2)));
        let full_computed_checksum: String = char_to_count.into_iter().collect::<(Vec<char>, Vec<usize>)>().0.iter().collect();
        if full_computed_checksum.starts_with(checksum) {
            let sector_id: u32 = room_data[sector_id_start+1..checksum_start].parse().expect("The sector id is always a number");
            let letter_shift = (sector_id % 26) as u8;
            let decoded_room_name: String = room_name.bytes().map(|byte| {
                if byte == '-' as u8 {
                    ' '
                } else {
                    let new_byte = byte + letter_shift;
                    if new_byte > 'z' as u8 {
                        (new_byte - 26) as char
                    } else {
                        new_byte as char
                    }
                }
            }).collect();
            println!("{} {}", decoded_room_name, sector_id);
        }
    }
}
