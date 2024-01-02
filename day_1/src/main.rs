use std::fs;
use std::io::{self, BufRead};

fn main() {
    let data = fs::File::open("day_1/day_1_data.txt").expect("Unable to open day_1_data.txt");
    let reader = io::BufReader::new(data);

    let mut total: i32 = 0;
    let mut num: i32;
    let start = std::time::Instant::now();
    for data in reader.lines() {
        num = get_num(data.unwrap().as_str());
        total += num;
    }
    println!("Total's value {:#?}", total);
    println!("Total Elapsed time: {:#?}", start.elapsed());
}

fn get_num(val: &str) -> i32 {
    let mut right_idx = 1;
    let mut left_idx = val.len() - 1;
    let mut left_char = "".to_string();
    let mut right_char = "".to_string();
    let word_digits: Vec<&str> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    // Check for text or numeric at the start of the string
    // two optional factors to check: numeric and text
    // iteration: for each character, check if this is a numeric character:
    // for the total string, check if there is a digit textj
    while right_idx <= val.len() {
        let last_character = val[0..right_idx].chars().last().unwrap();

        if last_character.is_numeric() {
            left_char = last_character.to_string();
            break;
        }
        for digit in &word_digits {
            let mut index_range = 0;
            if right_idx > 5 {
                index_range = right_idx - 5;
            }
            if val[index_range..right_idx].contains(digit) {
                left_char = get_num_from_word(digit).to_string();
                break;
            }
        }
        if !left_char.is_empty() {
            break;
        }
        right_idx += 1;
    }

    while left_idx >= right_idx {
        let last_character = val[left_idx..val.len()].chars().rev().last().unwrap();
        if last_character.is_numeric() {
            right_char = last_character.to_string();
            break;
        }
        for digit in &word_digits {
            let mut index_range = val.len();
            if left_idx + 5 <= index_range {
                index_range = left_idx + 5;
            }
            if val[left_idx..index_range].contains(digit) {
                right_char = get_num_from_word(digit).to_string();
                break;
            }
        }
        if !right_char.is_empty() {
            break;
        }
        left_idx -= 1;
    }
    if right_char.is_empty() {
        right_char = left_char.clone();
    }

    combine_chars(left_char, &right_char)
}

fn get_num_from_word(word: &str) -> &str {
    return match word {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => panic!("Not a word that can be converted"),
    };
}
fn combine_chars(mut left: String, right: &str) -> i32 {
    left.push_str(right);

    let combined_digit = left
        .parse::<i32>()
        .expect("Unable to create any numer from string");
    combined_digit
}
