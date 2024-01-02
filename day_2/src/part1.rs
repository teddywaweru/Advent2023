use std::{
    fs,
    io::{self, BufRead},
};
pub fn execute() {
    println!("Hello, world!");
    let file = fs::File::open("day_2/day_2_data1.txt").expect("Unable to open file");
    let reader = io::BufReader::new(file);
    let mut total: i32 = 0;

    for line in reader.lines() {
        let split_by_game = line.unwrap();
        let split_by_game = split_by_game.splitn(2, ":").collect::<Vec<&str>>();
        let game_id = split_by_game[0]
            .split(" ")
            .filter_map(|x| x.parse::<i32>().ok())
            .collect::<Vec<i32>>();
        let game_id = *game_id.first().unwrap();
        if check_str(split_by_game[1]) {
            total += game_id;
        }
    }
    println!("Part 1: Total of Games: {}", total);
}
//Parse string to get quantity of red, blue and green and game ID
//Compare quantities with the base value: 12 red cubes, 13 green cubes, and 14 blue cubes
//Only add the game IDs for the ones whose quantities satisfy the base values
fn check_str(line: &str) -> bool {
    let base_g = 13;
    let base_r = 12;
    let base_b = 14;
    let word_parts = line.split(&[',', ';'][..]).collect::<Vec<&str>>();

    for word in &word_parts {
        let val = *word
            .split(" ")
            .filter_map(|x| x.parse::<i32>().ok())
            .collect::<Vec<i32>>()
            .first()
            .unwrap();
        // let val = val.first().unwrap().parse::<i32>().unwrap();
        if word.contains("green") && val > base_g {
            return false;
        }
        if word.contains("blue") && val > base_b {
            return false;
        }
        if word.contains("red") && val > base_r {
            return false;
        }
    }
    true
}
