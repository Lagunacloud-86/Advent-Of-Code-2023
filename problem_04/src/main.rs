use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

mod scratch_card;
use scratch_card::create_scratch_cards;

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


fn solve_first<P>(file_path : P) -> i32
    where P: AsRef<Path>
{
    let file_contents_result = read_file_contents(file_path);
    if file_contents_result == None {
        println!("Unable to read file contents");
        return -1;
    }
    let file_contents = file_contents_result.unwrap();
    let scratch_cards = create_scratch_cards(&file_contents);

    let mut result = 0;
    let mut adder = 1;
    for card in scratch_cards {
        //println!("Processing Card '{}'", card.get_card_number());

        for _number in card.get_matching_numbers() {
            adder += adder;
        }

        if adder > 1 {
            result += adder / 2;
        }

        adder = 1;
    }


    return result;
}

fn solve_second<P>(file_path : P) -> i32
    where P: AsRef<Path>
{
    let file_contents_result = read_file_contents(file_path);
    if file_contents_result == None {
        println!("Unable to read file contents");
        return -1;
    }
    let file_contents = file_contents_result.unwrap();
    let scratch_cards = create_scratch_cards(&file_contents);
    let mut score_cards_map = HashMap::new();
    let mut index:usize = 0;
    while index < scratch_cards.len() {
        score_cards_map.insert(index, 1);
        index += 1;
    }
    index = 0;
    while index < scratch_cards.len() {
        let card = &scratch_cards[index];
        let card_count = *score_cards_map.get(&index).unwrap();

        for adder in 1..=card.get_matching_numbers_count() {
            let card_index = index + adder as usize;
            let count = score_cards_map.entry(card_index).or_insert(0);
            *count += 1 * card_count;
        }
        index += 1;
    }

    return score_cards_map.iter().map(|(_, v)| v).sum();
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
