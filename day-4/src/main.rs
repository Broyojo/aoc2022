use std::fs;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let text = fs::read_to_string("input.txt").unwrap();

    let mut one_inside_the_other = 0;

    for line in text.lines() {
        let ranges: Vec<&str> = line.split(',').collect();

        let first_range: Vec<i32> = ranges[0].split('-').map(|x| x.parse().unwrap()).collect();
        let second_range: Vec<i32> = ranges[1].split('-').map(|x| x.parse().unwrap()).collect();

        if second_range[0] <= first_range[0] && first_range[1] <= second_range[1]
            || first_range[0] <= second_range[0] && second_range[1] <= first_range[1]
        {
            one_inside_the_other += 1;
        }
    }
    println!("{one_inside_the_other}");
}

fn part_two() {
    let text = fs::read_to_string("input.txt").unwrap();

    let mut overlap = 0;

    for line in text.lines() {
        let ranges: Vec<&str> = line.split(',').collect();

        let first_range: Vec<i32> = ranges[0].split('-').map(|x| x.parse().unwrap()).collect();
        let second_range: Vec<i32> = ranges[1].split('-').map(|x| x.parse().unwrap()).collect();

        if second_range[0] <= first_range[0] && first_range[0] <= second_range[1]
            || first_range[0] <= second_range[0] && second_range[0] <= first_range[1]
            || second_range[0] <= first_range[1] && first_range[1] <= second_range[1]
            || first_range[0] <= second_range[1] && second_range[1] <= first_range[1]
        {
            overlap += 1;
        }
    }
    println!("{overlap}");
}
