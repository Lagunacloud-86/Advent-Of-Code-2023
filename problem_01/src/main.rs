use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

static CHAR_DIGITS: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9' ];
static NAMED_DIGITS: &[&str] = &["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

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

fn solve_first(_file_path : String) -> i32 {
    let mut result : i32 = 0;
    if let Ok(lines) = read_lines(_file_path) {

        let mut left_digit : char = ' ';
        let mut right_digit : char = ' ';

        for line in lines {
            if let Ok(_ip) = line {

                for c in _ip.chars() {
                    if c.is_digit(10) {
                        left_digit = c;
                        break;
                    }
                }
                for c in _ip.chars().rev() {
                    if c.is_digit(10) {
                        right_digit = c;
                        break;
                    }
                }

                let result_str : String = format!("{}{}", left_digit, right_digit);
                result += result_str.parse::<i32>().unwrap();
            }
        }

    }
    return result;
}
fn solve_second(_file_path : String) -> i32 {

    let mut result : i32 = 0;
    if let Ok(lines) = read_lines(_file_path) {

        let mut left_digit : char = ' ';
        let mut right_digit : char = ' ';

        for line in lines {
            if let Ok(_ip) = line {
                left_digit = get_left_digit(_ip.clone());
                right_digit = get_right_digit(_ip.clone());

                let result_str : String = format!("{}{}", left_digit, right_digit);
                result += result_str.parse::<i32>().unwrap();
            }
        }
    }
    return result;
}

fn get_left_digit(line : String) -> char {
    for index in 0..line.len() {
        for c in CHAR_DIGITS {
            let char_entry :char = line.chars().nth(index).unwrap();
            if *c == char_entry {
                return char_entry;
            }
        }
        let left = line.len() - index;
        for nd in NAMED_DIGITS {
            if nd.len() > left {
                continue;
            }
            let slice =  &line[index..(index + nd.len())];
            if slice.eq(*nd) {
                let char_index = NAMED_DIGITS.iter().position(|&named_digit| named_digit == *nd).unwrap();
                let char_result = CHAR_DIGITS.iter().nth(char_index);

                return *char_result.unwrap();
            }
        }
    }
    return '0';
}

fn get_right_digit(line : String) -> char {
    for index in (0..line.len()).rev() {
        for c in CHAR_DIGITS {
            let char_entry :char = line.chars().nth(index).unwrap();
            if *c == char_entry {
                return char_entry;
            }
        }
        let left = line.len() - index;
        for nd in NAMED_DIGITS {
            if nd.len() > left {
                continue;
            }
            let slice =  &line[index..(index + nd.len())];
            if slice.eq(*nd) {
                let char_index = NAMED_DIGITS.iter().position(|&named_digit| named_digit == *nd).unwrap();
                let char_result = CHAR_DIGITS.iter().nth(char_index);

                return *char_result.unwrap();
            }
        }
    }
    return '0';
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}