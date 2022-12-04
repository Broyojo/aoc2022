use std::fs;

fn main() {
    part_one();
    part_one_functional();
    part_two();
    part_two_functional();
}

fn part_one() {
    let text = fs::read_to_string("input.txt").unwrap();
    let sections: Vec<&str> = text.split("\n\n").collect();

    let mut max = 0;
    for section in sections {
        let mut total = 0;
        for line in section.split('\n') {
            total += line.parse::<u64>().unwrap();
        }
        if total > max {
            max = total;
        }
    }

    println!("{max}");
}

fn part_one_functional() {
    let max: u64 = fs::read_to_string("input.txt")
        .unwrap()
        .split("\n\n")
        .map(|s| s.split('\n').map(|l| l.parse::<u64>().unwrap()).sum())
        .max()
        .unwrap();
    println!("{max}");
}

fn part_two() {
    let text = fs::read_to_string("input.txt").unwrap();
    let sections: Vec<&str> = text.split("\n\n").collect();

    let mut totals = vec![];
    for section in sections {
        let mut total = 0;
        for line in section.split('\n') {
            total += line.parse::<u64>().unwrap();
        }
        totals.push(total);
    }

    totals.sort();

    let top: u64 = totals.iter().rev().take(3).sum();

    println!("{top}");
}

fn part_two_functional() {
    let mut totals: Vec<u64> = fs::read_to_string("input.txt")
        .unwrap()
        .split("\n\n")
        .map(|s| s.split('\n').map(|c| c.parse::<u64>().unwrap()).sum())
        .collect();
    totals.sort();
    let top: u64 = totals.iter().rev().take(3).sum();
    println!("{top}");
}
