use std::{collections::HashSet, fs};

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let text = fs::read_to_string("input.txt").unwrap();

    let mut priority_sum: u64 = 0;

    for line in text.lines() {
        let (first_half, second_half) = line.split_at(line.len() / 2);

        let mut first_half_set = HashSet::new();

        for c in first_half.chars() {
            first_half_set.insert(c);
        }

        let mut second_half_set = HashSet::new();

        for c in second_half.chars() {
            second_half_set.insert(c);
        }

        for c in first_half_set.intersection(&second_half_set) {
            priority_sum += *c as u64 - if c.is_lowercase() { 96 } else { 38 };
        }
    }

    println!("{priority_sum}");
}

fn part_two() {
    let text = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = text.lines().collect();

    let mut priority_sum: u64 = 0;

    for group in lines.chunks(3) {
        let mut first_set = HashSet::new();
        let mut second_set = HashSet::new();
        let mut third_set = HashSet::new();

        for c in group[0].chars() {
            first_set.insert(c);
        }

        for c in group[1].chars() {
            second_set.insert(c);
        }

        for c in group[2].chars() {
            third_set.insert(c);
        }

        let c = first_set
            .iter()
            .filter(|k| second_set.contains(k))
            .find(|k| third_set.contains(k))
            .unwrap();

        priority_sum += *c as u64 - if c.is_lowercase() { 96 } else { 38 };
    }

    println!("{priority_sum}")
}
