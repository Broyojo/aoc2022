use std::{collections::HashSet, fs};

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let text = fs::read_to_string("input.txt").unwrap();
    let chars: Vec<char> = text.chars().collect();

    for (i, packet) in chars.windows(4).enumerate() {
        let set: HashSet<char> = packet.iter().copied().collect();
        if set.len() == 4 {
            println!("chars: {:?}, index: {}", set, i + 4);
            break;
        }
    }
}

fn part_two() {
    let text = fs::read_to_string("input.txt").unwrap();
    let chars: Vec<char> = text.chars().collect();

    for (i, packet) in chars.windows(14).enumerate() {
        let set: HashSet<char> = packet.iter().copied().collect();
        if set.len() == 14 {
            println!("chars: {:?}, index: {}", set, i + 14);
            break;
        }
    }
}
