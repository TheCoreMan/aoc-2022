use std::fs;

#[derive(Debug, Clone, PartialEq)]
struct MoveOperation {
    amount: u32,
    from_stack: u32,
    to_stack: u32,
}

struct LoadingDockState {
    stacks: Vec<Vec<char>>,
}

impl LoadingDockState {
    pub fn new_loading_dock(initial_state_raw: &str) -> LoadingDockState {
        let mut stack_rows: Vec<&str> = initial_state_raw.split("\n").collect();
        let number_row = stack_rows
            .pop()
            .expect("There should be more than 0 rows in the input");
        let how_many_stacks_row: Vec<&str> = number_row
            .split(" ")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty()) // There are some annoying spaces in the string. After trimming they become empty.
            .collect();
        let how_many_stacks: usize = how_many_stacks_row[how_many_stacks_row.len() - 1]
            .parse()
            .expect("should be a number");

        let mut stacks: Vec<Vec<char>> = Vec::with_capacity(how_many_stacks);
        for i in 0..how_many_stacks {
            stacks.insert(i, Vec::new());
        }

        stack_rows.reverse();
        for row in stack_rows.iter() {
            let row_as_vec: Vec<char> = row.chars().collect();
            for which_char_to_read in 0..how_many_stacks {
                // Start with one - row either starts with " " or "["
                let container_name = row_as_vec[1 + (which_char_to_read * 4)];
                match container_name {
                    ' ' => (),
                    _ => stacks[which_char_to_read].push(container_name.clone()),
                }
            }
        }

        return LoadingDockState {
            stacks: stacks.clone(),
        };
    }

    pub fn peek_top_crates_from_each_stack(&self) -> Vec<char> {
        let mut tops: Vec<char> = Vec::with_capacity(self.stacks.capacity());
        for stack in self.stacks.iter() {
            let mut ruin_me = stack.clone();
            // If there's anything on the top of the stack
            if let Some(top) = ruin_me.pop() {
                tops.push(top);
            }
        }
        return tops;
    }

    pub fn render_loading_dock(&self) {
        let mut ruinable_stacks = self.stacks.clone();
        for (index, ruinable_stack) in ruinable_stacks.iter_mut().enumerate() {
            print!("{}: ", index);
            ruinable_stack.reverse();
            while let Some(c) = ruinable_stack.pop() {
                print!("{}", c);
            }
            print!("\n");
        }
    }

    pub fn perform_move_operation(&mut self, op: MoveOperation) {
        for _ in 0..(op.amount) {
            let container = self.stacks[usize::try_from(op.from_stack - 1).unwrap()]
                .pop()
                .expect("there are containers to move from this stack");
            self.stacks[usize::try_from(op.to_stack - 1).unwrap()].push(container);
        }
    }

    pub fn perform_move_operation_cratemover_9001(&mut self, op: MoveOperation) {
        let mut temp: Vec<char> = Vec::with_capacity(usize::try_from(op.amount).unwrap());
        for _ in 0..op.amount {
            let container = self.stacks[usize::try_from(op.from_stack - 1).unwrap()]
                .pop()
                .expect("there are containers to move from this stack");
            temp.push(container);
        }
        while let Some(container) = temp.pop() {
            self.stacks[usize::try_from(op.to_stack - 1).unwrap()].push(container);
        }
    }
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

    #[test]
    fn test_new_and_peek_loading_dock_simple_case() {
        let initial_state_1 = "[a]     \n[A] [B] \n 1  2  ";
        let expected_result_1 = vec!['a', 'B'];
        let loading_dock_1 = LoadingDockState::new_loading_dock(initial_state_1);
        assert_eq!(
            loading_dock_1.peek_top_crates_from_each_stack(),
            expected_result_1
        );
    }
    #[test]
    fn test_new_and_peek_loading_dock_complex_case() {
        let initial_state_2 = "    [G]     \n[A] [B]     \n[Q] [W] [E] \n 1   2   3  ";
        println!("{}", initial_state_2);
        let expected_result_2 = vec!['A', 'G', 'E'];
        let loading_dock_2 = LoadingDockState::new_loading_dock(initial_state_2);
        assert_eq!(
            loading_dock_2.peek_top_crates_from_each_stack(),
            expected_result_2
        );
    }

    #[test]
    fn test_move_operations_on_loading_dock_complex_case() {
        let initial_state = "    [G]     \n[A] [B]     \n[Q] [W] [E] \n 1   2   3  ";
        let expected_result_before_move = vec!['A', 'G', 'E'];
        let expected_result_after_first_move = vec!['A', 'W', 'B'];
        let move_operation_1 = MoveOperation {
            amount: 2,
            from_stack: 2,
            to_stack: 3,
        };
        let expected_result_after_second_move = vec!['A', 'B', 'G'];
        let move_operation_2 = MoveOperation {
            amount: 1,
            from_stack: 3,
            to_stack: 2,
        };
        let mut loading_dock = LoadingDockState::new_loading_dock(initial_state);
        println!(" -- before move");
        loading_dock.render_loading_dock();
        assert_eq!(
            loading_dock.peek_top_crates_from_each_stack(),
            expected_result_before_move
        );

        loading_dock.perform_move_operation(move_operation_1);
        println!(" -- after 1st move");
        loading_dock.render_loading_dock();
        assert_eq!(
            loading_dock.peek_top_crates_from_each_stack(),
            expected_result_after_first_move
        );

        loading_dock.perform_move_operation(move_operation_2);
        println!(" -- after 2nd move");
        loading_dock.render_loading_dock();
        assert_eq!(
            loading_dock.peek_top_crates_from_each_stack(),
            expected_result_after_second_move
        );
    }
}

fn main() {
    println!("Advent of Code 🎄🎄 2022 // Day 5");

    let crane_operation_instructions =
        fs::read_to_string("input.txt").expect("Should have been able to read input");

    // This is a pretty janky format.
    let crane_operation_seperated: Vec<&str> = crane_operation_instructions.split("\n\n").collect();
    let crane_initial_state_raw: &str = crane_operation_seperated[0];
    let crane_move_operations_raw: &str = crane_operation_seperated[1];

    let move_operations = parse_move_operations(crane_move_operations_raw);
    let mut loading_dock = LoadingDockState::new_loading_dock(crane_initial_state_raw);

    println!("before moves");
    loading_dock.render_loading_dock();

    for move_operation in move_operations {
        // V1
        //loading_dock.perform_move_operation(move_operation);

        // V2
        loading_dock.perform_move_operation_cratemover_9001(move_operation);
    }

    println!("after moves");
    loading_dock.render_loading_dock();

    let result: String = loading_dock
        .peek_top_crates_from_each_stack()
        .into_iter()
        .collect();
    println!("final state: {}", result);
}
