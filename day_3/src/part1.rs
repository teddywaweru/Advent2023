use std::{
    fs::File,
    io::{self, BufRead},
};
pub fn execute() {
    let file = File::open("day_3/data1.txt").unwrap();

    let reader = io::BufReader::new(file);
    let mut file_vec: Vec<String> = vec![];

    let start = std::time::Instant::now();
    for (idx, lines) in reader.lines().enumerate() {
        file_vec.insert(idx, lines.unwrap());
    }

    // let mut total: i32 = 0;
    let mut string_total: i32 = 0;
    for (vec_idx, vec_string) in file_vec.iter().enumerate() {
        let mut right_idx: usize;
        let mut left_idx = 0;
        let mut left_is_set = false;
        let mut num: &str;
        // println!("Logging for vec_string: {:#?}", vec_string);
        for (char_idx, character) in vec_string.chars().enumerate() {
            if left_is_set {
                if character.is_numeric() {
                    if char_idx == vec_string.len() - 1 {
                        right_idx = char_idx + 1;
                        num = &vec_string[left_idx..right_idx];
                        if has_special_chars_around_num(&file_vec, vec_idx, right_idx, left_idx) {
                            string_total += num.parse::<i32>().unwrap();
                        }
                        left_is_set = false;
                    }
                } else {
                    right_idx = char_idx;
                    num = &vec_string[left_idx..right_idx];
                    if has_special_chars_around_num(&file_vec, vec_idx, right_idx, left_idx) {
                        string_total += num.parse::<i32>().unwrap();
                    }
                    left_is_set = false;
                }
            } else if character.is_numeric() {
                left_idx = char_idx;
                left_is_set = true;
            }
        }
    }
    // println!("Logging the line count: {}", idx);
    // println!("Logging the total value then: {}", total);
    println!("Logging the String_total value then: {}", string_total);
    // println!("Here's the data: {:#?}", file_vec);
    println!("Elapsed time: {:#?}", start.elapsed());
}

pub fn has_special_chars_around_num(
    file_vec: &[String],
    vec_idx: usize,
    right_idx: usize,
    left_idx: usize,
) -> bool {
    // Check upper vec for special characters
    if vec_idx > 0 {
        // println!("Logging for num: {num}");
        let mut left_upper_idx = left_idx;
        let mut right_upper_idx = right_idx;
        if left_upper_idx == 0 {
            left_upper_idx = 0;
        } else {
            left_upper_idx -= 1;
        }
        if right_upper_idx + 1 > file_vec[vec_idx].len() {
            right_upper_idx = file_vec[vec_idx].len();
        } else {
            right_upper_idx += 1;
        }
        let upper_chars = &file_vec[vec_idx - 1][left_upper_idx..right_upper_idx];
        let mut upper_contains_special_chars: bool = false;
        let _ = upper_chars
            .chars()
            .filter_map(|c| match c {
                '.' => None,
                _ => {
                    // println!("Upper special Chars: {upper_chars}");
                    // println!("Num with upper special chars: {num}");
                    upper_contains_special_chars = true;
                    Some(true)
                }
            })
            .collect::<Vec<bool>>();
        if upper_contains_special_chars {
            return true;
        }
        // match upper_chars.chars().filter_map(|c| Some(())).collect::<Vec<()>>().first().unwrap(){

        // }
        // match file_vec[vec_idx - 1][left_idx - 1..right_idx + 2].cha
        // let upper_chars = file_vec[vec_idx - 1..vec_idx][left_idx - 1..right_idx + 2];
    }
    if vec_idx < file_vec.len() - 1 {
        let mut lower_contains_special_chars: bool = false;
        let left_lower_idx = if left_idx <= 1 { 0 } else { left_idx - 1 };
        let right_lower_idx = if right_idx + 1 > file_vec[vec_idx].len() {
            file_vec[vec_idx].len()
        } else {
            right_idx + 1
        };

        let lower_chars = &file_vec[vec_idx + 1][left_lower_idx..right_lower_idx];
        let _ = lower_chars
            .chars()
            .filter_map(|c| match c {
                '.' => None,
                _ => {
                    // println!("Lower special Chars: {lower_chars}");
                    // println!("Num with lower special chars: {num}");
                    lower_contains_special_chars = true;
                    Some(true)
                }
            })
            .collect::<Vec<bool>>();

        if lower_contains_special_chars {
            return true;
        }
    }
    // Check current against curr
    if right_idx + 1 < file_vec[vec_idx].len()
        && *file_vec[vec_idx][right_idx..right_idx + 1]
            .chars()
            .collect::<Vec<char>>()
            .first()
            .expect("Empty Vec of chars")
            != '.'
    {
        return true;
    }

    if left_idx > 0
        && *file_vec[vec_idx][left_idx - 1..left_idx]
            .chars()
            .collect::<Vec<char>>()
            .first()
            .expect("Empty Vec of Chars")
            != '.'
    {
        return true;
    }
    false

    //
    // Check current against next idx
}
