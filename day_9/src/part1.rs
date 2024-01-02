use std::{
    fs::File,
    io::{self, BufRead},
};
pub fn execute() {
    let file = File::open("day_9/data1.txt").unwrap();
    let reader = io::BufReader::new(file);

    let totals: i32 = 0;
    let mut history: Vec<i32> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        history = line
            .split_whitespace()
            .filter_map(|val| val.parse::<i32>().ok())
            .collect();
        let mut hist2: Vec<i32> = vec![];
        for idx in 0..history.len() {
            hist2.push(history[idx + 1] - history[idx])
        }
    }
    todo!()
}
