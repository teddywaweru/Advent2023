use std::{
    fs::File,
    io::{self, BufRead},
};
pub fn execute() {
    let file = File::open("day_5/data1.txt").expect("Unable to open day5 data 1");
    let file_len = io::BufReader::new(file).lines().count();
    let file = File::open("day_5/data1.txt").expect("Unable to open day5 data 1");
    let reader = io::BufReader::new(file);

    let mut maps_vec: Vec<Vec<Vec<i64>>> = vec![];
    let mut count = 0;
    let mut line_vec: Vec<i64> = vec![];
    let mut map_vec: Vec<Vec<i64>> = vec![];
    let mut seeds_vec: Vec<i64> = vec![];
    let start = std::time::Instant::now();
    for (idx, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if seeds_vec.is_empty() && line.starts_with("seeds:") {
            let (_, seeds_string) = line.split_once(":").unwrap();
            seeds_vec = seeds_string
                .split_whitespace()
                .filter_map(|s| s.parse::<i64>().ok())
                .collect();
        }
        if line != String::from("") && line.chars().nth(0).unwrap().is_numeric() {
            line_vec = line
                .split_whitespace()
                .filter_map(|s| s.parse::<i64>().ok())
                .collect();
            map_vec.push(line_vec.clone());
        }
        if line == String::from("") && !map_vec.is_empty() || idx == file_len - 1 {
            maps_vec.insert(count, map_vec.clone());
            map_vec.clear();
            line_vec.clear();
            count += 1;
        }
    }
    // println!("Logging for file_vec: {:#?}", maps_vec);

    let mut mapping_vec: Vec<i64> = vec![];
    for seed in seeds_vec {
        // println!("Seed: {seed}");
        let mut mapping = seed;
        for mapping_vec in &maps_vec {
            // println!("Mapping Vec {:#?}", mapping_vec);
            for range in mapping_vec {
                // println!("Range: {:#?}", range);
                // println!("Logging for mapping: {}", mapping);
                if (range[1]..=range[1] + range[2]).contains(&mapping) {
                    mapping = range[0] + mapping - range[1];
                    // println!("Logging for range: {:#?}", range);
                    // println!("New Mapping: {mapping}");
                    break;
                }
            }
        }
        mapping_vec.push(mapping);
        mapping_vec.sort();
    }
    println!("Logging for closest: {}", mapping_vec.first().unwrap());

    println!("Runtime: {:#?}", start.elapsed());
}
