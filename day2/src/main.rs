use std::fs;

enum Shape {
    Rock,
    Paper,
    Scissors,
}

fn new_shape(encoded_shape: &str) -> Shape {
    match encoded_shape {
        "A" | "X" => Shape::Rock,
        "B" | "Y" => Shape::Paper,
        "C" | "Z" => Shape::Scissors,
        _ => panic!("Unknown shape {}", encoded_shape),
    }
}

fn get_shape_score(s: &Shape) -> i32 {
    return match s {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    };
}

fn calc_round_score(opponent_shape: Shape, my_shape: Shape) -> i32 {
    let shape_score = get_shape_score(&my_shape);

    // 0 for loss
    // 3 for draw
    // 6 for win
    let outcome_score = match my_shape {
        Shape::Rock => match opponent_shape {
            Shape::Rock => 3,
            Shape::Paper => 0,
            Shape::Scissors => 6,
        },
        Shape::Paper => match opponent_shape {
            Shape::Rock => 6,
            Shape::Paper => 3,
            Shape::Scissors => 0,
        },
        Shape::Scissors => match opponent_shape {
            Shape::Rock => 0,
            Shape::Paper => 6,
            Shape::Scissors => 3,
        },
    };

    return shape_score + outcome_score;
}

enum Outcome {
    Win,
    Draw,
    Lose,
}

fn new_outcome(encoded_outcome: &str) -> Outcome {
    match encoded_outcome {
        "Z" => Outcome::Win,
        "Y" => Outcome::Draw,
        "X" => Outcome::Lose,
        _ => panic!("Unknown outcome encoding {}", encoded_outcome),
    }
}

// It was at this point that I realized that there's probably a nice data
// structure to represent the rules of Rock Paper Scissors, but that in the end
// we're all going to die so match on enums in Good Enough ™️.

fn calc_round_score_v2(opponent_shape: Shape, needed_outcome: Outcome) -> i32 {
    let outcome_score = match needed_outcome {
        Outcome::Win => 6,
        Outcome::Draw => 3,
        Outcome::Lose => 0,
    };

    let my_shape = match needed_outcome {
        Outcome::Win => match opponent_shape {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        },
        Outcome::Draw => match opponent_shape {
            Shape::Rock => Shape::Rock,
            Shape::Paper => Shape::Paper,
            Shape::Scissors => Shape::Scissors,
        },
        Outcome::Lose => match opponent_shape {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        },
    };

    let shape_score = get_shape_score(&my_shape);

    return shape_score + outcome_score;
}

fn main() {
    println!("Advent of Code 🎄🎄 2022 // Day 2");

    let strategy_guide =
        fs::read_to_string("input.txt").expect("Should have been able to read input");

    let rounds = strategy_guide.split("\n");

    let mut score_total = 0;
    for round in rounds {
        let round_shapes: Vec<&str> = round.split(" ").collect();
        let opponent_shape: Shape = new_shape(round_shapes[0]);
        let my_shape: Shape = new_shape(round_shapes[1]);
        score_total += calc_round_score(opponent_shape, my_shape);
    }

    println!("Score total: {}", score_total);

    let rounds_again = strategy_guide.split("\n");

    let mut score_total = 0;
    for round in rounds_again {
        let round_shapes: Vec<&str> = round.split(" ").collect();
        let opponent_shape: Shape = new_shape(round_shapes[0]);
        let needed_outcome: Outcome = new_outcome(round_shapes[1]);
        score_total += calc_round_score_v2(opponent_shape, needed_outcome);
    }

    println!("Score total: {}", score_total)
}
