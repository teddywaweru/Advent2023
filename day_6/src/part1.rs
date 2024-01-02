use std::{
    fs::File,
    io::{self, BufRead},
};
pub fn execute() {
    let file = File::open("day_6/data1.txt").expect("Unable to open Day 6 data 1");
    let reader = io::BufReader::new(file);

    let mut time_vec: Vec<i32> = vec![];
    let mut distance_vec: Vec<i32> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains("Time:") {
            let (_, time_str) = line.split_once(":").unwrap();
            time_vec = time_str
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect();
        } else {
            let (_, distance_str) = line.split_once(":").unwrap();
            distance_vec = distance_str
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect();
        }
    }
    let mut error_margin: i32 = 1;
    for (idx, time) in time_vec.iter().enumerate() {
        let mut count_victories: i32 = 0;
        for opt in 1..=*time {
            if distance_vec[idx] < opt * (time - opt) {
                count_victories += 1;
            }
        }
        error_margin *= count_victories;
    }
    println!("Logging for error_margin: {error_margin}");
}
