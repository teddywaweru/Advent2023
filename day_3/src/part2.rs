#![allow(unused_variables)]
#![allow(dead_code)]
use crate::part1::has_special_chars_around_num;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
pub fn execute() {
    let file = File::open("day_3/data2.txt").expect("Unable to read data2.txt");
    let reader = BufReader::new(file);
    let mut file_vec: Vec<String> = vec![];
    let mut num_vec: HashMap<usize, HashMap<String, Vec<usize>>> = HashMap::new();

    let mut string_total = 0;
    for (idx, lines) in reader.lines().enumerate() {
        file_vec.insert(idx, lines.unwrap());
    }

    for (vec_idx, vec_string) in file_vec.iter().enumerate() {
        let mut right_idx: usize;
        let mut left_idx = 0;
        let mut left_is_set: bool = false;
        let mut num;
        let default_hm: HashMap<String, Vec<usize>> = HashMap::from([
            ("right".to_string(), vec![]),
            ("left".to_string(), vec![]),
            ("asterix".to_string(), vec![]),
        ]);

        num_vec.insert(vec_idx, default_hm);
        for (char_idx, character) in vec_string.chars().enumerate() {
            if character == '*' {
                let insert_idx = num_vec[&vec_idx]["asterix"].len();
                num_vec
                    .get_mut(&vec_idx)
                    .unwrap()
                    .get_mut("asterix")
                    .expect("Unable to get key 'asterix' in HashMap")
                    .insert(insert_idx, char_idx);
            }
            if left_is_set {
                if character.is_numeric() {
                    if char_idx == vec_string.len() - 1 {
                        let insert_idx = num_vec[&vec_idx]["right"].len();
                        num_vec
                            .get_mut(&vec_idx)
                            .unwrap()
                            .get_mut("right")
                            .expect("Unable to get key 'right' in HashMap")
                            .insert(insert_idx, char_idx);
                        right_idx = char_idx + 1;
                        num = &vec_string[left_idx..right_idx];
                        if has_special_chars_around_num(&file_vec, vec_idx, right_idx, left_idx) {
                            string_total += num.parse::<i32>().unwrap();
                        }
                        left_is_set = false;
                    }
                } else {
                    right_idx = char_idx;
                    let insert_idx = num_vec[&vec_idx]["right"].len();
                    num_vec
                        .get_mut(&vec_idx)
                        .unwrap()
                        .get_mut("right")
                        .unwrap()
                        .insert(insert_idx, char_idx);
                    num = &vec_string[left_idx..right_idx];
                    if has_special_chars_around_num(&file_vec, vec_idx, right_idx, left_idx) {
                        string_total += num.parse::<i32>().unwrap();
                    }
                    left_is_set = false;
                }
            } else if character.is_numeric() {
                let insert_idx = num_vec[&vec_idx]["left"].len();
                num_vec
                    .get_mut(&vec_idx)
                    .unwrap()
                    .get_mut("left")
                    .expect("Unable to get key 'left' in HashMap")
                    .insert(insert_idx, char_idx);
                left_idx = char_idx;
                left_is_set = true;
            }
        }
    }
    let mut adj_num: Vec<&str> = vec![];
    for (vec_idx, vec_string) in file_vec.iter().enumerate() {
        for asterix_idx in &num_vec[&vec_idx]["asterix"] {
            let left_range = asterix_idx - 1;
            let right_range = asterix_idx + 2;

            // check top for numerics
            if vec_idx > 0 {
                let mut top_num_exists: bool = false;
                let mut top_num_left_idx: usize = 0;
                let mut top_num_right_idx: usize = 0;
                for (char_idx, character) in file_vec[vec_idx - 1][left_range..right_range]
                    .chars()
                    .enumerate()
                {
                    //get start of number
                    if character.is_numeric() && char_idx == 0 {
                        top_num_exists = true;
                        //check if number starts further  than the left range index
                        for (idx, val) in file_vec[vec_idx - 1][0..*asterix_idx]
                            .chars()
                            .rev()
                            .enumerate()
                        {
                            if !val.is_numeric() {
                                top_num_left_idx = idx + 1;
                            }
                        }
                    } else if character.is_numeric() {
                        top_num_exists = true;
                        top_num_left_idx = char_idx;
                    }
                    //get end of number
                    for (idx, val) in file_vec[vec_idx - 1]
                        [top_num_left_idx..file_vec[vec_idx].len()]
                        .chars()
                        .enumerate()
                    {
                        if !val.is_numeric() {
                            top_num_right_idx = idx;
                        }
                    }
                }
                if top_num_exists {
                    println!(
                        "Logging for left_idx: {}, right_idx: {}, asterix_idx: {}, num: ",
                        top_num_left_idx,
                        top_num_right_idx,
                        asterix_idx,
                        // &file_vec[vec_idx - 1][top_num_left_idx..top_num_right_idx]
                    );
                    println!("Logging for vec_idx - 1:\n {:#?}", file_vec[vec_idx - 1]);
                    println!("Logging for vec_idx: \n {:#?}", file_vec[vec_idx]);
                    adj_num.insert(
                        adj_num.len(),
                        &file_vec[vec_idx - 1][top_num_left_idx..top_num_right_idx],
                    );
                }
            }
            // check bottom for numerics
            if vec_idx < file_vec.len() {
                let mut bottom_num_exists: bool = false;
                let mut bottom_num_left_idx: usize = 0;
                let mut bottom_num_right_idx: usize = 0;
                for (char_idx, character) in file_vec[vec_idx - 1][left_range..right_range]
                    .chars()
                    .enumerate()
                {
                    //get start of number
                    if character.is_numeric() && char_idx == 0 {
                        bottom_num_exists = true;
                        //check if number starts further  than the left range index
                        for (idx, val) in file_vec[vec_idx + 1][0..*asterix_idx]
                            .chars()
                            .rev()
                            .enumerate()
                        {
                            if !val.is_numeric() {
                                bottom_num_left_idx = idx + 1;
                            }
                        }
                    } else if character.is_numeric() {
                        bottom_num_exists = true;
                        bottom_num_left_idx = char_idx;
                    }
                    //get end of number
                    for (idx, val) in file_vec[vec_idx + 1][*asterix_idx..file_vec[vec_idx].len()]
                        .chars()
                        .enumerate()
                    {
                        if !val.is_numeric() {
                            bottom_num_right_idx = idx;
                        }
                    }
                }
                if bottom_num_exists {
                    adj_num.insert(
                        adj_num.len(),
                        &file_vec[vec_idx - 1][bottom_num_left_idx..bottom_num_right_idx],
                    );
                }
            }
        }
    }
    println!("Logging for adj_num: {:#?}", adj_num);
}

fn check_special_chars_around_num(
    line: &str,
    line_idx: usize,
    left_idx: usize,
    right_idx: usize,
) -> i32 {
    // check right char
    if left_idx != 0
        && line[left_idx - 1..left_idx].contains(|c| !matches!(c, '.'))
        && line[left_idx - 1..left_idx] == *"*"
    {
        special_operation("left", line, left_idx, right_idx, line_idx);
        todo!()
    }

    todo!()
}
fn special_operation(
    location: &str,
    line: &str,
    left_idx: usize,
    right_idx: usize,
    line_idx: usize,
) -> bool {
    match location {
        "top" => {}
        "bottom" => {}
        "right" => {}
        "left" => {
            if left_idx as i32 - 2 != 0 {
                for left_chars in line.chars().rev() {}
            }
        }
        _ => panic!("Unknown location"),
    }
    todo!()
}
