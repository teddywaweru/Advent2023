use std::fs::File;
use std::io::{self, BufRead};
pub fn execute() {
    let file = File::open("day_4/data1.txt").expect("Unable to open day 4 data1.txt");

    let reader = io::BufReader::new(file);

    let mut total = 0;
    for line in reader.lines() {
        // Drop first section that contains words cards
        let line = line.unwrap();
        let (_, card_deck_nums) = line.split_once(":").unwrap();
        let (winning_nums, held_nums) = card_deck_nums.split_once("|").unwrap();
        let winning_vec: Vec<i32> = winning_nums
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        let held_vec: Vec<i32> = held_nums
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        let mut counter: i32 = 0;
        println!("Logging for winning_vec: {:?}", winning_vec);
        println!("Logging for held_vec: {:?}", held_vec);
        for num in winning_vec {
            if held_vec.contains(&num) {
                counter += 1;
            }
        }
        println!("Logging for counter: {:?}", counter);
        if counter == 0 {
        } else if counter == 1 {
            total += 1;
        } else {
            total += i32::pow(2, (counter - 1) as u32);
            println!("Logging for counter value: {:?}", (counter - 1).pow(2));
        }
        println!("Logging for total: {:?}", total);
        // Separate winning deck from held deck
        // Get vec of numbers for the winning dec from chars
    }
    println!("Total value of winning card decks: {}", total);
}
