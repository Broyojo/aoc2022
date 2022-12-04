use std::fs;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let text = fs::read_to_string("input.txt").unwrap();

    let mut total_score = 0;

    for line in text.lines() {
        let moves: Vec<&str> = line.split(' ').collect();

        // A | X -> Rock
        // B | Y -> Paper
        // C | Z -> Scissors

        let outcome_score = match (moves[0], moves[1]) {
            ("A", "X") => 3,
            ("A", "Y") => 6,
            ("A", "Z") => 0,
            ("B", "X") => 0,
            ("B", "Y") => 3,
            ("B", "Z") => 6,
            ("C", "X") => 6,
            ("C", "Y") => 0,
            ("C", "Z") => 3,
            _ => panic!("unknown move"),
        };

        let move_score = match moves[1] {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => panic!("unknown move"),
        };

        total_score += outcome_score + move_score;
    }

    println!("{total_score}");
}

fn part_two() {
    let text = fs::read_to_string("input.txt").unwrap();

    let mut total_score = 0;

    for line in text.lines() {
        let moves: Vec<&str> = line.split(' ').collect();

        // A -> Rock
        // B -> Paper
        // C -> Scissors

        // X -> Lose
        // Y -> Draw
        // Z -> Win

        let outcome_score = match moves[1] {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => panic!("unknown outcome"),
        };

        let move_score = match (moves[0], moves[1]) {
            ("A", "X") => 3,
            ("A", "Y") => 1,
            ("A", "Z") => 2,
            ("B", "X") => 1,
            ("B", "Y") => 2,
            ("B", "Z") => 3,
            ("C", "X") => 2,
            ("C", "Y") => 3,
            ("C", "Z") => 1,
            _ => panic!("unknown move"),
        };

        total_score += outcome_score + move_score;
    }

    println!("{total_score}");
}
