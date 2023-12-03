use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Possible {
    color: String,
    amount: i32
}
fn main() {

    let mut result : i32;

    result = solve_first(String::from("input/sample-first.txt"));
    println!("The result of the first sample is: {result}");

    result = solve_first(String::from("input/puzzle.txt"));
    println!("The result of the first puzzle is: {result}");


    result = solve_second(String::from("input/sample-second.txt"));
    println!("The result of the second sample is: {result}");

    result = solve_second(String::from("input/puzzle.txt"));
    println!("The result of the second puzzle is: {result}");
}

fn solve_first(file_path : String) -> i32 {
    let mut result : i32 = 0;
    if let Ok(lines) = read_lines(file_path) {

        for line in lines {
            if let Ok(ip) = line {
                result += parse_game_line(ip);
            }
        }

    }
    return result;
}
fn solve_second(file_path : String) -> i32 {

    let mut result : i32 = 0;
    if let Ok(lines) = read_lines(file_path) {
        for line in lines{
            if let Ok(ip) = line {
                result += parse_game_line_v2(ip);
            }
        }

    }
    return result;
}

fn parse_game_line(line : String) -> i32 {

    let mut line_chars = line.chars();


    let game_section_end_index = line_chars.position(|ch| ch ==  ':').unwrap();
    let game_section_end_index_integer = game_section_end_index as i32;


    let game_id_slice = &line[5..game_section_end_index];
    let game_id = game_id_slice.parse::<i32>().unwrap();

    let possible : [Possible;3] = [
        Possible { color: String::from("red"), amount: 12 },
        Possible { color: String::from("green"), amount: 13 },
        Possible { color: String::from("blue"), amount: 14 },
    ];

    if !check_if_possible(game_section_end_index_integer + 2, line, &possible) {
        return 0;
    }

    return game_id;
}
fn check_if_possible(start_index : i32, line : String, possible: &[Possible]) -> bool {
    let slice = &line[(start_index as usize)..];
    for part in slice.split(|c| c == ';') {
        if !check_set(&part, &possible) {
            return false;
        }
    }
    return true;
}

fn check_set(set : &str, possible: &[Possible]) -> bool {
    let mut map = HashMap::new();
    map.insert(String::from("red"), 0);
    map.insert(String::from("green"), 0);
    map.insert(String::from("blue"), 0);

    for part in set.split(|c| c == ',') {
        let part_entry = part.trim();
        let split_index = part_entry.chars().position(|c| c == ' ').unwrap();
        let amount_slice = &part_entry[0..split_index];
        let color_slice = &part_entry[(split_index + 1)..];
        let amount = amount_slice.parse::<i32>().unwrap();
        if let Some(value) = map.get_mut(color_slice){
            *value += amount;
        }
    }

    for entry in possible {
        if let Some(value) = map.get(&entry.color) {
            if *value > entry.amount {
                return false;
            }
        }
    }


    return true;
}


fn parse_game_line_v2(line : String) -> i32 {
    let mut min_map = HashMap::from([
        ("red", i32::MIN),
        ("blue", i32::MIN),
        ("green", i32::MIN)
    ]);
    let game_section_index_option = line.chars().position(|c| c == ':');
    if game_section_index_option == None {
        return -1;
    }
    let game_section_index = game_section_index_option.unwrap();
    let game_results_slice = &line[(game_section_index + 2)..];
    for part in game_results_slice.split(|c| c == ';') {
        parse_data_section(part, &mut min_map);
    }

    let mut result = 1;
    for (key, value) in min_map {
        result = result * value;
    }

    return result;
    //return 0;
}
fn parse_data_section(section: &str, min_map: &mut HashMap<&str, i32>) {
    let mut local_min_map = HashMap::from([
        ("red", 0),
        ("blue", 0),
        ("green", 0)
    ]);

    for part in section.split(|c| c == ',') {
        let cubes_pulled_section = part.trim();
        let split_index = cubes_pulled_section.chars().position(|c| c == ' ').unwrap();

        let count_section = &cubes_pulled_section[..split_index];
        let color_section = &cubes_pulled_section[(split_index + 1)..];
        if let Some(value) = local_min_map.get_mut(color_section) {
            let count = count_section.parse::<i32>().unwrap();
            *value += count;
        }
    }

    for (key, val) in local_min_map {
        if val == 0 {
            continue;
        }
        let value = min_map.get_mut(key).unwrap();
        if val > *value {
            *value = val;
        }
    }
}



// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}