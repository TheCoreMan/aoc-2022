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
    for (i, rucksack) in rucksacks.enumerate() {
        priorities_total += get_snack_priority(find_common_snack(rucksack));
    }

    println!("Sum of dups priorities: {}", priorities_total)
}
