mod instruction_map;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::instruction_map::Instruction;


fn main() {
    let mut result : i64;

    result = solve_first(String::from("input/sample-first.txt"));
    println!("The result of the first sample is: {result}");

    result = solve_first(String::from("input/puzzle.txt"));
    println!("The result of the first puzzle is: {result}");


    result = solve_second(String::from("input/sample-second.txt"));
    println!("The result of the second sample is: {result}");

    result = solve_second(String::from("input/puzzle.txt"));
    println!("The result of the second puzzle is: {result}");
}
fn read_file_contents<P>(file_path : P) -> Option<String>
    where P: AsRef<Path> {
    let file_result = File::open(file_path);
    if let Err(error) = file_result {
        eprintln!("Error: {error}");
        return None;
    }
    let mut file = file_result.unwrap();
    let mut file_contents:String = Default::default();
    let size_result = file.read_to_string(&mut file_contents);
    if let Err(err) = size_result {
        println!("Error occurred reading file to string; {}", err);
        return None;
    }

    return Some(file_contents);
}

fn solve_first<P>(file_path : P) -> i64
    where P: AsRef<Path>
{
    let file_contents_result = read_file_contents(file_path);
    if file_contents_result == None {
        println!("Unable to read file contents");
        return 0;
    }
    let file_contents = file_contents_result.unwrap();
    let mut file_sections = file_contents.split("\r\n\r\n");

    let mut instruction_map:HashMap<String, Instruction> = HashMap::new();

    let instructions = file_sections.nth(0).unwrap().trim();

    let mappings = file_sections.nth(0).unwrap().trim();
    let lines = mappings.split("\r\n");

    for line in lines {
        let mut section_split = line.split('=');
        let name = section_split.nth(0).unwrap().trim();
        let map = section_split.nth(0).unwrap().trim();
        let mut left_right = map.split(',');
        let left = left_right.nth(0).unwrap().trim();
        let right = left_right.nth(0).unwrap().trim();

        instruction_map.insert(
            String::from(name),
            Instruction::create(
                left[1..].trim(),
                right[..right.len()-1].trim())
        );
    }


    let mut count:i64 = 0;
    let mut current = "AAA";
    let end = "ZZZ";
    let mut index:usize = 0;
    while current != end {

        let instruction = instructions.chars().nth(index).unwrap();
        index = (index + 1) % instructions.len();

        current = match instruction {
            'L' => instruction_map[current].get_left(),
            'R' => instruction_map[current].get_right(),
            _ => current
        };

        count += 1;
    }



    return count;
}

fn solve_second<P>(file_path : P) -> i64
    where P: AsRef<Path>
{
    let file_contents_result = read_file_contents(file_path);
    if file_contents_result == None {
        println!("Unable to read file contents");
        return 0;
    }
    let file_contents = file_contents_result.unwrap();
    let mut file_sections = file_contents.split("\r\n\r\n");

    let mut instruction_map:HashMap<String, Instruction> = HashMap::new();

    let instructions = file_sections.nth(0).unwrap().trim();

    let mappings = file_sections.nth(0).unwrap().trim();
    let lines = mappings.split("\r\n");

    for line in lines {
        let mut section_split = line.split('=');
        let name = section_split.nth(0).unwrap().trim();
        let map = section_split.nth(0).unwrap().trim();
        let mut left_right = map.split(',');
        let left = left_right.nth(0).unwrap().trim();
        let right = left_right.nth(0).unwrap().trim();

        instruction_map.insert(
            String::from(name.trim()),
            Instruction::create(
                left[1..].trim(),
                right[..right.len()-1].trim())
        );
    }


    let mut current_map:Vec<&str> = instruction_map
        .iter()
        .filter(|(k,_)| k.ends_with('A'))
        .map(|(k, _)| k.as_str())
        .collect();

    let mut count:i64 = 0;
    let mut index:usize = 0;
    while current_map.iter().any(|e| !e.ends_with('Z')) {

        let instruction = instructions.chars().nth(index).unwrap();
        index = (index + 1) % instructions.len();

        for mi in 0..current_map.len() {
            current_map[mi] = match instruction {
                'L' => instruction_map[current_map[mi]].get_left(),
                'R' => instruction_map[current_map[mi]].get_right(),
                _ => current_map[mi]
            };
        }


        count += 1;
    }



    return count;
}