use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};
pub fn execute() {
    let file = File::open("day_8/data1.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut navigation = "".to_string();
    let mut node_network: HashMap<String, Vec<String>> = HashMap::new();
    let mut curr = "".to_string();
    for (idx, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if idx == 0 {
            navigation = line;
        } else if line == "" {
            continue;
        } else {
            let (key, values) = line.split_once("=").unwrap();
            let (mut left_val, mut right_val) = values.split_once(",").unwrap();
            left_val = left_val.trim_start().strip_prefix("(").unwrap();
            right_val = right_val.trim_start().strip_suffix(")").unwrap();
            println!(
                "Logging for idx: {idx}, key: {key}, left_val: {left_val}, right_val: {right_val}"
            );
            if node_network.is_empty() {
                // curr = key.trim_end().to_string();
                curr = "AAA".to_string();
            }
            node_network.insert(
                key.trim_end().to_string(),
                vec![left_val.to_string(), right_val.to_string()],
            );
        }
    }
    // let mut curr = node_network.;
    let mut steps = 0;
    // println!("Logging for node_network: {:#?}", node_network);
    while curr != "ZZZ" {
        for (nav_idx, nav) in navigation.chars().enumerate() {
            steps += 1;
            if nav_idx == navigation.len() - 1 {
                print!("Logging end of nav");
            }
            if nav == 'L' {
                // println!("Logging for curr: {curr}");
                curr = (*node_network.get(&curr).unwrap()[0]).to_string();
                if curr == "ZZZ" {
                    print!("Shet");
                    break;
                }
            } else if nav == 'R' {
                // println!("Logging for curr: {curr}");
                curr = (*node_network.get(&curr).unwrap()[1]).to_string();
                if curr == "ZZZ" {
                    print!("Shet");
                    break;
                }
            }
        }
    }
    println!("Count of Steps: {steps}");
}
