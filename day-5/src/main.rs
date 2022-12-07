use std::fs;

fn main() {
    part_one();
    part_two();
    part_one_better();
}

fn part_one_better() {
    let text = fs::read_to_string("input.txt").unwrap();

    let mut stacks: Vec<Vec<&str>>;

    let mut index = 0;

    'outer: for (l, line) in text.lines().enumerate() {
        for (i, c) in line.char_indices() {
            if c.is_numeric() {
                println!("character: {c}, line: {l}, index: {i}");
                index = i;
                break 'outer;
            }
        }
    }

    println!("{index}");
}

fn part_one() {
    let text = fs::read_to_string("input.txt").unwrap();

    let mut stacks = vec![
        vec!["P", "F", "M", "Q", "W", "G", "R", "T"],
        vec!["H", "F", "R"],
        vec!["P", "Z", "R", "V", "G", "H", "S", "D"],
        vec!["Q", "H", "P", "B", "F", "W", "G"],
        vec!["P", "S", "M", "J", "H"],
        vec!["M", "Z", "T", "H", "S", "R", "P", "L"],
        vec!["P", "T", "H", "N", "M", "L"],
        vec!["F", "D", "Q", "R"],
        vec!["D", "S", "C", "N", "L", "P", "H"],
    ];

    for line in text.lines().skip(10) {
        let parts: Vec<&str> = line.split(' ').collect();

        let number: i32 = parts[1].parse().unwrap();
        let source_stack: i32 = parts[3].parse().unwrap();
        let destination_stack: i32 = parts[5].parse().unwrap();

        for _ in 0..number {
            let popped = stacks[source_stack as usize - 1].pop().unwrap();
            stacks[destination_stack as usize - 1].push(popped);
        }
    }
    for stack in stacks {
        print!("{}", stack.iter().rev().next().unwrap());
    }
    println!();
}

fn part_two() {
    let text = fs::read_to_string("input.txt").unwrap();

    let mut stacks = vec![
        vec!["P", "F", "M", "Q", "W", "G", "R", "T"],
        vec!["H", "F", "R"],
        vec!["P", "Z", "R", "V", "G", "H", "S", "D"],
        vec!["Q", "H", "P", "B", "F", "W", "G"],
        vec!["P", "S", "M", "J", "H"],
        vec!["M", "Z", "T", "H", "S", "R", "P", "L"],
        vec!["P", "T", "H", "N", "M", "L"],
        vec!["F", "D", "Q", "R"],
        vec!["D", "S", "C", "N", "L", "P", "H"],
    ];

    for line in text.lines().skip(10) {
        let parts: Vec<&str> = line.split(' ').collect();

        let number: i32 = parts[1].parse().unwrap();
        let source_stack: i32 = parts[3].parse().unwrap();
        let destination_stack: i32 = parts[5].parse().unwrap();

        let mut popped_vec = vec![];

        for _ in 0..number {
            let popped = stacks[source_stack as usize - 1].pop().unwrap();
            popped_vec.push(popped);
        }

        popped_vec.reverse();

        stacks[destination_stack as usize - 1].extend(popped_vec.iter());
    }

    for stack in stacks {
        print!("{}", stack.iter().rev().next().unwrap());
    }
    println!();
}
