use kdam::tqdm;
use std::{collections::HashSet, fs};

fn parse_sections(sections_unparsed: &str) -> HashSet<u32> {
    let sections: Vec<&str> = sections_unparsed.split("-").collect();
    assert_eq!(sections.len(), 2);
    let start = sections[0]
        .parse::<u32>()
        .expect("section should be a number.");
    let end = sections[1]
        .parse::<u32>()
        .expect("section should be a number.");

    assert!(
        start <= end,
        "expected start to be smaller then end, got start={}, end={}, sections unparsed={}",
        start,
        end,
        sections_unparsed,
    );

    let mut result = HashSet::new();
    for i in start..=end {
        result.insert(i);
    }
    return result;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_sections_sanity() {
        let test_case_1: HashSet<u32> = vec![2u32, 3, 4].into_iter().collect();
        assert_eq!(parse_sections("2-4"), test_case_1);

        let test_case_2: HashSet<u32> = vec![9u32, 10, 11, 12, 13].into_iter().collect();
        assert_eq!(parse_sections("9-13"), test_case_2);
    }
    #[test]
    #[should_panic]
    fn test_parse_sections_start_end_mismatch() {
        _ = parse_sections("8-4");
    }
    #[test]
    #[should_panic]
    fn test_parse_sections_invalid() {
        _ = parse_sections("8");
    }
}

fn main() {
    println!("Advent of Code 🎄🎄 2022 // Day 4");

    let section_assignments_big_string =
        fs::read_to_string("input.txt").expect("Should have been able to read input");

    let section_assignments = section_assignments_big_string.split("\n");

    let mut how_many_assignment_pairs_fully_contain = 0;
    // addition for part 2
    let mut how_many_assignment_pairs_overlap_at_all = 0;
    for section_assignment_pair in tqdm!(section_assignments) {
        let section_assigment_pair_as_vec: Vec<&str> = section_assignment_pair.split(",").collect();
        let section_assignment_1 = parse_sections(section_assigment_pair_as_vec[0]);
        let section_assignment_2 = parse_sections(section_assigment_pair_as_vec[1]);
        if section_assignment_1.is_subset(&section_assignment_2)
            || section_assignment_2.is_subset(&section_assignment_1)
        {
            how_many_assignment_pairs_fully_contain += 1;
        }
        // addition for part 2
        if !section_assignment_1.is_disjoint(&section_assignment_2) {
            how_many_assignment_pairs_overlap_at_all += 1;
        }
    }

    println!(
        "Fully contain count: {}",
        how_many_assignment_pairs_fully_contain
    );

    // addition for part 2
    println!(
        "Overlap at all count: {}",
        how_many_assignment_pairs_overlap_at_all
    );
}
