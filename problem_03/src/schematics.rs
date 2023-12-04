use std::cmp::max;
use crate::schematics::schematic_range::{SchematicNumberRange, SchematicRange};

pub mod coords;
pub mod schematic_range;
pub mod size;


const NUMBERS:&str = "0123456789";
const IGNORED:&str = "0123456789.\r\n";

const WHITESPACE:&str = "\r ";
const NEWLINE:char = '\n';



pub struct Schematic {
    pub symbols_indices:Vec<usize>,
    pub number_ranges:Vec<SchematicNumberRange>,
    pub line_ranges:Vec<SchematicRange>,
    pub width: u16,
    pub height: u16
}

pub fn make_schematics(file_contents: &String) -> Schematic {


    let mut symbols:Vec<usize> = vec![];
    let mut numbers:Vec<SchematicNumberRange> = vec![];
    let mut lines:Vec<SchematicRange> = vec![];
    let mut width:u16 = 0;
    let mut height:u16 = 0;

    build_symbols(&file_contents, &mut symbols);
    build_numbers(&file_contents, &mut numbers);
    build_lines(&file_contents, &mut lines);
    calculate_size(&file_contents, &mut width, &mut height);

    return Schematic {

        symbols_indices: symbols.clone(),
        number_ranges: numbers.clone(),
        line_ranges: lines.clone(),
        width,
        height
    };
}

fn build_symbols(file_contents: &String, symbols: &mut Vec<usize>) {

    let mut index:usize = 0;
    while index < file_contents.len() {

        index += 1;
        let symbol_result = file_contents.chars().nth(index - 1);
        if symbol_result == None {
            println!("Error trying to get file contents character!");
            continue;
        }
        let symbol = symbol_result.unwrap();
        if IGNORED.contains(symbol) {
            continue;
        }
        symbols.push(index - 1);
    }
}

fn build_numbers(file_contents: &String, numbers: &mut Vec<SchematicNumberRange>) {

    let mut index:usize = 0;
    while index < file_contents.len() {
        let symbol_result = file_contents.chars().nth(index);
        if symbol_result == None {
            index += 1;
            println!("Error trying to get file contents character!");
            continue;
        }
        let symbol = symbol_result.unwrap();
        if !NUMBERS.contains(symbol) {
            index += 1;
            continue;
        }

        let mut end_index: usize = 0;
        walk_number_section(&file_contents, index, &mut end_index);
        let value_string = &file_contents[index..=end_index];

        let value_result = value_string.parse::<i32>();
        if let Err(err) = value_result {
            println!("There was an error trying to parse number\n\tValue: {}", value_string);
            break;
        }
        let value = value_result.unwrap();
        numbers.push(SchematicNumberRange
        {
            value,
            range: SchematicRange
            {
                start: index,
                end: end_index
            }
        });
        index = end_index + 1;
    }
}

fn walk_number_section(file_contents: &String, start_index: usize, end_index: &mut usize) {
    let mut index:usize = start_index + 1;
    *end_index = start_index;
    while index < file_contents.len() {

        let symbol_result = file_contents.chars().nth(index);
        if symbol_result == None {
            println!("Error trying to get file contents character!\n\tIndex: {}", index);
            break;
        }

        let symbol = symbol_result.unwrap();
        if !NUMBERS.contains(symbol) {
            break;
        }


        *end_index = index;
        index +=1;

    }
}

fn build_lines(file_contents: &String, lines: &mut Vec<SchematicRange>) {
    let mut index:usize = 0;
    let mut start_index:usize=0;
    let mut end_index:usize = 0;
    let mut line_end_index:usize = 0;

    while index < file_contents.len() {

        let symbol_result = file_contents.chars().nth(index);
        if symbol_result == None {
            index += 1;
            println!("Error trying to get file contents character!\n\tIndex:{}", index);
            continue;
        }
        let symbol = symbol_result.unwrap();
        if !WHITESPACE.contains(symbol) {
            index += 1;
            continue;
        }


        walk_line_section(&file_contents, start_index, &mut end_index, &mut line_end_index);
        lines.push(SchematicRange { start: start_index, end: line_end_index});
        start_index = end_index + 1;
        index = end_index + 1;
    }

    walk_line_section(&file_contents, start_index, &mut end_index, &mut line_end_index);
    lines.push(SchematicRange { start: start_index, end: line_end_index});
}
fn walk_line_section(file_contents: &String, start_index: usize, end_index: &mut usize, line_end_index: &mut usize) {
    let mut index:usize = start_index + 1;
    *end_index = start_index;
    *line_end_index = start_index;
    while index < file_contents.len() {

        let symbol_result = file_contents.chars().nth(index);
        if symbol_result == None {
            println!("Error trying to get file contents character!\n\tIndex: {}", index);
            break;
        }

        *end_index = index;
        let symbol = symbol_result.unwrap();
        if symbol == NEWLINE  {
            break;
        }

        *line_end_index = index;
        index += 1;
    }

    //clear whitespace
    while WHITESPACE.contains(file_contents.chars().nth(*line_end_index).unwrap()) {
        *line_end_index -= 1;
    }
}
fn calculate_size(file_contents: &String, width: &mut u16, height: &mut u16) {
    let mut max_x:usize = 0;
    let mut x:usize = 0;
    let mut y:usize = 0;
    let mut index:usize = 0;
    while index < file_contents.len() {

        let symbol_result = file_contents.chars().nth(index);
        if symbol_result == None {
            println!("Error trying to get file contents character!\n\tIndex: {}", index);
            break;
        }

        let symbol = symbol_result.unwrap();
        if symbol == NEWLINE  {
            max_x = max(max_x, x);
            x = 0;
            y += 1;
            index += 1;
            continue;
        }

        if !WHITESPACE.contains(symbol) {
            x += 1;
        }


        index += 1;
    }

    *width = max_x as u16;
    *height = y  as u16+ 1;
}