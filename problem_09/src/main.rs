mod sequence;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::sequence::Sequence;

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
    let sequences: Vec<Sequence> = file_contents
        .lines()
        .map(|line| line.trim().split(' '))
        .map(|entry| entry.map(|ch| ch.trim().parse::<i64>().unwrap()).collect::<Vec<_>>())
        .map(|e| Sequence::create_from_vec(&e))
        .collect();

    let mut sum= 0;
    for mut s in sequences {
        let result = s.calculate_next_value(None);
        sum += result;
    }


    return sum;
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
    let sequences: Vec<Sequence> = file_contents
        .lines()
        .map(|line| line.trim().split(' '))
        .map(|entry| entry.map(|ch| ch.trim().parse::<i64>().unwrap()).collect::<Vec<_>>())
        .map(|e| Sequence::create_from_vec(&e))
        .collect();

    let mut sum= 0;
    for mut s in sequences {
        let result = s.calculate_previous_value(None);
        sum += result;
    }


    return sum;
}