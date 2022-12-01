use std::fs;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let text = fs::read_to_string("input.txt").unwrap();
    let sections: Vec<&str> = text.split("\n\n").collect();
    let max: u64 = sections
        .iter()
        .map(|s| s.split('\n').map(|c| c.parse::<u64>().unwrap()).sum())
        .max()
        .unwrap();
    println!("{}", max);
}

fn part_two() {
    let text = fs::read_to_string("input.txt").unwrap();
    let sections: Vec<&str> = text.split("\n\n").collect();
    let mut totals: Vec<u64> = sections
        .iter()
        .map(|s| s.split('\n').map(|c| c.parse::<u64>().unwrap()).sum())
        .collect();
    totals.sort();
    let top: u64 = totals.iter().rev().take(3).sum();
    println!("{}", top);
}
