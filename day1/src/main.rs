use std::fs;
use std::collections::BinaryHeap;

fn main() {
    println!("Advent of Code 🎄🎄 2022 // Day 1");
    let all_elves_in_one_string = fs::read_to_string("src/input.txt")
        .expect("Should have been able to read the file...");
    
    let elves_iterator = all_elves_in_one_string.split("\n\n");

    // part one - get the calorie count of the elf with the most calories in their snack.
    let mut max_calories = 0;
    for elf in elves_iterator {
        let snacks_iterator = elf.split("\n");
        let mut curr_elf_calorie_total = 0;
        for snack in snacks_iterator {
            let snack_calories = snack.parse::<i32>()
                .expect("snack should have been an int (calorie count...)");
            curr_elf_calorie_total += snack_calories
        }
        if curr_elf_calorie_total > max_calories {
            max_calories = curr_elf_calorie_total
        }
    }

    println!("Max is {}", max_calories);

    // part 2 - now need to get the top n elves (n=3). 
    // Simplest solution is reuse the code from before with three "top" variables, but that's
    // ugly and not reuseable for 4.
    // Best DS for this IMO is a binary heap - push all elves into the heap and then pop the 
    // top n. 
    // Also, for fun, rewrote the loop with map-fold
    let elves_iterator = all_elves_in_one_string.split("\n\n");
    let mut heap_of_elves = BinaryHeap::new();

    for elf in elves_iterator {
        let curr_elf_calorie_total = elf.split("\n")
            .map(|snack| snack.parse::<i32>().unwrap())
            .fold(0, |mut a, b| { 
                a += b; 
                a 
            });
        heap_of_elves.push(curr_elf_calorie_total)
    }

    let how_many_top_elves_to_pick = 3;
    let mut top_n_calorie_count_total = 0;
    for i in 0..how_many_top_elves_to_pick {
        let pop_result = heap_of_elves.pop()
            .expect("There should have been enough elves to pop");
        top_n_calorie_count_total += pop_result;
        println!("elf #{} calories: {}, total {}", i, pop_result, top_n_calorie_count_total);
    }
}
