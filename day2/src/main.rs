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

fn calc_round_score(opponent_shape: Shape, my_shape: Shape) -> i32 {
    let shape_score = match my_shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    };

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

fn main() {
    println!("Advent of Code 🎄🎄 2022 // Day 2");

    let strategy_guide =
        fs::read_to_string("input.txt").expect("Should have been able to read input");

    let rounds = strategy_guide.split("\n");

    let mut score_total = 0;
    for round in rounds {
        let round_shapes: Vec<&str> = round.split(" ").collect();
        let my_shape: Shape = new_shape(round_shapes[1]);
        let opponent_shape: Shape = new_shape(round_shapes[0]);
        score_total += calc_round_score(opponent_shape, my_shape);
    }

    println!("Score total: {}", score_total)
}
