#![feature(iter_array_chunks)]

use kdam::tqdm;
use std::{collections::HashSet, fs};

fn get_snack_priority(snack: char) -> u32 {
    const LOWERCASE_BASE: u32 = 'a' as u32;
    const UPPERCASE_BASE: u32 = 'A' as u32;
    if snack.is_ascii_lowercase() {
        return snack as u32 - LOWERCASE_BASE + 1;
    } else {
        return snack as u32 - UPPERCASE_BASE + 1 + 26;
    }
}

fn find_common_snack(rucksack: &str) -> char {
    let (comp1, comp2) = rucksack.split_at(rucksack.len() / 2);
    // let compartments: Vec<&str> = vec![comp1, comp2];
    let mut comp1set: HashSet<char> = HashSet::new();
    let mut comp2set: HashSet<char> = HashSet::new();
    for c in comp1.chars() {
        comp1set.insert(c);
    }
    for c in comp2.chars() {
        comp2set.insert(c);
    }
    let intersection = comp1set.intersection(&comp2set);
    let intersection_vec: Vec<&char> = intersection.collect();
    assert!(intersection_vec.len() == 1);
    let result_char = intersection_vec[0].clone();
    return result_char;
    /*
    let mut all_compartments: Vec<HashSet<char>> = Vec::new();
    for compartment in compartments {
        let mut set = HashSet::new();
        for char in compartment.chars() {
            set.insert(char);
        }
        all_compartments.push(set);
    }

    let compartments_iter = all_compartments.clone().iter();
    let mut intersection = compartments_iter
        .next()
        .map(|set| compartments_iter.fold(set, |set1, set2| set1 & set2))
        .unwrap();

    assert!(intersection.len() == 1);
    return intersection.drain().collect::<Vec<char>>()[0];
    */
}

fn find_common_snack_v2(rucksacks: Vec<&str>) -> char {
    let mut all_snacks: HashSet<char> = HashSet::new();
    for c in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
        all_snacks.insert(c);
    }

    let binding = rucksacks
        .iter()
        .map(|rucksack| {
            let mut rucksack_dedup: HashSet<char> = HashSet::new();
            for single_snack in rucksack.chars() {
                rucksack_dedup.insert(single_snack);
            }
            return rucksack_dedup;
        })
        .fold(all_snacks, |a: HashSet<char>, b: HashSet<char>| {
            a.into_iter().filter(|snack| b.contains(snack)).collect()
        });
    let common_snacks_vec: Vec<&char> = binding.iter().collect();

    assert_eq!(common_snacks_vec.len(), 1);

    return *common_snacks_vec[0];
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_snack_priority() {
        assert_eq!(get_snack_priority('a'), 1);
        assert_eq!(get_snack_priority('z'), 26);
        assert_eq!(get_snack_priority('A'), 27);
        assert_eq!(get_snack_priority('Z'), 52);
    }

    #[test]
    fn test_find_common_snack() {
        assert_eq!(find_common_snack("vJrwpWtwJgWrhcsFMMfFFhFp"), 'p');
    }
}

fn main() {
    println!("Advent of Code 🎄🎄 2022 // Day 3");

    let rucksacks_big_string =
        fs::read_to_string("input.txt").expect("Should have been able to read input");

    let rucksacks = rucksacks_big_string.split("\n");

    let mut priorities_total = 0;

    for (_i, rucksack) in tqdm!(rucksacks.enumerate()) {
        priorities_total += get_snack_priority(find_common_snack(rucksack));
    }

    println!("Sum of dups priorities: {}", priorities_total);

    // part 2
    let rucksacks_v2 = rucksacks_big_string.split("\n");
    println!("------ part 2");
    let mut priorities_total_v2 = 0;
    for group in tqdm!(rucksacks_v2.array_chunks::<3>()) {
        priorities_total_v2 += get_snack_priority(find_common_snack_v2(group.to_vec()));
    }

    println!("Sum of dups priorities V2: {}", priorities_total_v2);
}
