use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    println!("Advent of Code 🎄🎄 2022 // Day 6");

    // We should use streams etc. - but for now, just read the whole file into a string
    let input = fs::read_to_string("input.txt").expect("Should have been able to read input");

    let mut window_start: usize = 0;

    //const WINDOW_SIZE: usize = 4; <-- This was for part one.
    const WINDOW_SIZE: usize = 14;
    loop {
        // scan the next window.
        let letters_in_window = &input[window_start..window_start + WINDOW_SIZE];
        let mut letters_set: HashSet<char> = HashSet::new();
        for (_i, letter) in letters_in_window.chars().enumerate() {
            letters_set.insert(letter);
        }
        if letters_set.len() == WINDOW_SIZE {
            break;
        }
        window_start += 1;

        // We could add check for boundary here, but the input is guaranteed to
        // have the marker, so... 😴
    }
    println!(
        "packet found at: {}, scanned {} letters, window: {}",
        window_start,
        window_start + WINDOW_SIZE,
        &input[window_start..window_start + WINDOW_SIZE],
    )
}
