use std::fs;

#[derive(Debug, Clone, PartialEq)]
struct MoveOperation {
    amount: u32,
    from_stack: u32,
    to_stack: u32,
}

// All move operations should look like this:
// "move 3 from 6 to 2"
// And the operations should be seperated by \n.
fn parse_move_operations(move_operations_raw: &str) -> Vec<MoveOperation> {
    let mut parsed_move_operations: Vec<MoveOperation> = Vec::new();
    let unparsed_move_operations = move_operations_raw.split("\n");

    //let re = Regex::new(r#"move (\d+) from (\d+) to (\d+)"#).expect("Regex ought to be valid.");
    for unparsed_move_operation in unparsed_move_operations {
        /*let mut numbers_in_operation: Vec<&str> = re
        .captures_iter(&unparsed_move_operation)
        .collect()
        .map(|cap: <regex::Regex as Trait>::Capture| cap); */

        // Text & indexes
        // "move 3 from 64 to 13"
        //  |    | |    |   | |
        //  0    1 2    3  4  5
        let split_unparsed: Vec<&str> = unparsed_move_operation.split(" ").collect();
        let parsed_move_operation = MoveOperation {
            amount: split_unparsed[1]
                .parse::<u32>()
                .expect("amount ought to be a number"),
            from_stack: split_unparsed[3]
                .parse::<u32>()
                .expect("from ought to be a number"),
            to_stack: split_unparsed[5]
                .parse::<u32>()
                .expect("to ought to be a number"),
        };
        parsed_move_operations.push(parsed_move_operation.clone());
    }
    return parsed_move_operations;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_move_operations_sanity() {
        let mo1 = MoveOperation {
            amount: 1,
            from_stack: 2,
            to_stack: 3,
        };
        let mo2 = MoveOperation {
            amount: 15,
            from_stack: 9,
            to_stack: 1,
        };
        let test_case_1: Vec<MoveOperation> = vec![mo1.clone()].into_iter().collect();
        assert_eq!(parse_move_operations("move 1 from 2 to 3"), test_case_1);

        let test_case_2: Vec<MoveOperation> = vec![mo2.clone()].into_iter().collect();
        assert_eq!(parse_move_operations("move 15 from 9 to 1"), test_case_2);

        let test_case_3: Vec<MoveOperation> = vec![mo1.clone(), mo2.clone(), mo1.clone()]
            .into_iter()
            .collect();
        assert_eq!(
            parse_move_operations("move 1 from 2 to 3\nmove 15 from 9 to 1\nmove 1 from 2 to 3"),
            test_case_3
        );
    }
}

fn main() {
    println!("Advent of Code 🎄🎄 2022 // Day 5");

    let crane_operation_instructions =
        fs::read_to_string("input.txt").expect("Should have been able to read input");

    // This is a pretty janky format.
    let crane_operation_seperated: Vec<&str> = crane_operation_instructions.split("\n\n").collect();
    let _crane_initial_state_raw: &str = crane_operation_seperated[0];
    let crane_move_operations_raw: &str = crane_operation_seperated[1];

    let move_operations = parse_move_operations(crane_move_operations_raw);
    dbg!(
        move_operations[0].clone(),
        move_operations[1].clone(),
        move_operations[move_operations.len() - 1].clone()
    );
}
