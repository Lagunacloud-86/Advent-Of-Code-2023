mod parsed;

use std::cmp::{max, min};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::parsed::Race;

const PARTITIONS:usize = 5;


fn main() {
    let mut result : i64;

    // result = solve_first(String::from("input/sample-first.txt"));
    // println!("The result of the first sample is: {result}");
    //
    // result = solve_first(String::from("input/puzzle.txt"));
    // println!("The result of the first puzzle is: {result}");


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

// fn solve_first<P>(file_path : P) -> i64
//     where P: AsRef<Path>
// {
//     0
// }

fn solve_second<P>(file_path : P) -> i64
    where P: AsRef<Path>
{
    let file_contents_result = read_file_contents(file_path);
    if file_contents_result == None {
        println!("Unable to read file contents");
        return 0;
    }
    let file_content = file_contents_result.unwrap();
    let mut lines = file_content.split('\n');

    let time_line = lines.nth(0).unwrap().split(':').nth(1).unwrap().trim();
    let dist_line = lines.nth(0).unwrap().split(':').nth(1).unwrap().trim();

    let time = time_line
        .split(' ')
        .filter(|&p|p != "")
        .map(|e| e.chars())
        .flatten()
        .collect::<String>();

    let dist = dist_line
        .split(' ')
        .filter(|&p|p != "")
        .map(|e| e.trim().chars())
        .flatten()
        .collect::<String>();

    let race:Race = Race::create(
        time.parse::<i64>().unwrap(),
        dist.parse::<i64>().unwrap()
    );

    solve_race(&race)
}


fn solve_race(race: &Race) -> i64 {
    let mut current_min = i64::MAX;
    let mut current_max = i64::MIN;
    let adder = race.get_time() / PARTITIONS as i64;
    for i in 1..PARTITIONS-1 {
        let index = adder * i as i64;
        if does_this_beat_record(&race, index) {
            current_min = min(current_min, index);
            current_max = max(current_max, index);
        }
    }

    if current_min == i64::MAX || current_max == i64::MIN {
        return -1;
    }

    current_min = find_min(&race, current_min, 0, current_min);
    current_max = find_max(&race, current_max, current_max, race.get_time());

    current_max + 1 - current_min
}

fn does_this_beat_record(race: &Race, entry: i64) -> bool {
    entry * (race.get_time() - entry) > race.get_distance()
}

fn find_min(race: &Race, current: i64, start: i64, end: i64) -> i64 {

    let mut s = start;
    let mut e = end;
    let mut c = current;
    loop {
        if !does_this_beat_record(&race, c)
            && does_this_beat_record(&race, c + 1) {
            return c + 1;
        }
        if does_this_beat_record(&race, c)
            && !does_this_beat_record(&race, c - 1) {
            return c;
        }

        let range = c - s;
        let range_adder = range / 2;
        if does_this_beat_record(&race, c) {
            e = c;
            c -= range_adder;
            continue;
        }
        s = c;
        c += range_adder;
    }
    c
}
fn find_max(race: &Race, current: i64, start: i64, end: i64) -> i64 {
    let mut s = start;
    let mut e = end;
    let mut c = current;
    loop {
        if !does_this_beat_record(&race, c)
            && does_this_beat_record(&race, c - 1) {
            return c - 1;
        }
        if does_this_beat_record(&race, c)
            && !does_this_beat_record(&race, c + 1) {
            return c;
        }

        let range = e - c;
        let range_adder = range / 2;
        if does_this_beat_record(&race, c) {
            s = c;
            c += range_adder;
            continue;
        }
        e = c;
        c -= range_adder;
    }
    c
}
