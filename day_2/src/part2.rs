use std::{
    fs,
    io::{self, BufRead},
};

pub fn execute() {
    let file = fs::File::open("day_2/day2_data2.txt").expect("Unable to open the file");
    let reader = io::BufReader::new(file);
    let mut total = 0;

    let start = std::time::Instant::now();
    for lines in reader.lines() {
        let draws = lines.unwrap();
        let draws = draws.split(":").collect::<Vec<&str>>();
        let draws = *draws.last().unwrap();

        total += calculate_powers(draws);
    }
    println!("Finish: {:#?}", start.elapsed());
    println!("Part 2: Total Values: {}", total);
}

fn calculate_powers(draws: &str) -> i32 {
    let mut max_r = 0;
    let mut max_g = 0;
    let mut max_b = 0;
    let split_draws = draws.split(";").collect::<Vec<&str>>();
    for draw in split_draws {
        // // Closure soution version, which is faster than multiple if statements
        // let _ = draw
        //     .split(",")
        //     .map(|cube| match cube.contains("green") {
        //         true => {
        //             let val_g = get_cube_count(cube);
        //             max_g = get_larger(val_g, max_g);
        //         }
        //         false => match cube.contains("red") {
        //             true => {
        //                 let val_r = get_cube_count(cube);
        //                 max_r = get_larger(val_r, max_g)
        //             }
        //             false => {
        //                 let val_b = get_cube_count(cube);
        //                 max_b = get_larger(val_b, max_b);
        //             }
        //         },
        //     })
        //     .collect::<()>();

        let cubes = draw.split(",").collect::<Vec<&str>>();
        for cube in cubes {
            if cube.contains("green") {
                let count_green = get_cube_count(cube);
                max_g = get_larger(count_green, max_g);
            } else if cube.contains("red") {
                let count_red = get_cube_count(cube);
                max_r = get_larger(count_red, max_r);
            } else {
                // if cube.contains("blue") {
                let count_blue = get_cube_count(cube);
                max_b = get_larger(count_blue, max_b);
            }
        }
    }
    max_b * max_g * max_r
}

fn get_cube_count(cube: &str) -> i32 {
    *cube
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>()
        .first()
        .unwrap()
}
fn get_larger(new: i32, curr: i32) -> i32 {
    if new > curr {
        return new;
    }
    curr
}
