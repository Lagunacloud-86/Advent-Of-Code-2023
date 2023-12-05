mod schematics;

use std::fs::{File};
use std::io::Read;
use std::ops::Add;
use std::path::Path;
use schematics::Schematic;
use crate::schematics::schematic_range::{ContainsIndex, SchematicRange};
use crate::schematics::coords::Coords;

const GEAR_SYMBOL:char = '*';

fn main()
{

    println!("Beginning problem 03");

    let mut result;
    // result = solve_first(String::from("input/sample-first.txt"));
    // println!("The result of the first sample is: {}", result);
    //
    // result = solve_first(String::from("input/puzzle.txt"));
    // println!("The result of the first puzzle is: {}", result);


    result = solve_second(String::from("input/sample-second.txt"));
    println!("The result of the second sample is: {result}");

    result = solve_second(String::from("input/puzzle.txt"));
    println!("The result of the second puzzle is: {result}");
}

#[tailcall]
fn solve_second<P>(file_path : P) -> i32
    where P: AsRef<Path>
{
    let file_result = File::open(file_path);
    if let Err(error) = file_result {
        eprintln!("Error: {error}");
        return -1;
    }
    let mut file = file_result.unwrap();
    let mut file_contents:String = Default::default();
    let size_result = file.read_to_string(&mut file_contents);
    if let Err(err) = size_result {
        println!("Error occurred reading file to string; {}", err);
        return -1;
    }

    let schematic = schematics::make_schematics(&file_contents);

    let mut result = 0;
    for symbol_index in schematic.symbols_indices.iter() {
        let symbol_result = file_contents.chars().nth(*symbol_index);
        if symbol_result == None {
            println!("Failed to retrieve symbol");
            continue;
        }
        let symbol = symbol_result.unwrap();
        if symbol != GEAR_SYMBOL {
            continue;
        }


        let mut ranges_adjacent = [usize::default();2];
        let adjacent_count = calculate_adjacent_ranges(*symbol_index, &schematic, &mut ranges_adjacent);
        if adjacent_count != 2 {
            continue;
        }



        let left_range = &schematic.number_ranges[ranges_adjacent[0]];
        let right_range = &schematic.number_ranges[ranges_adjacent[1]];

        result += left_range.value * right_range.value

    }

    return result;
}

fn calculate_adjacent_ranges(index: usize, schematic: &Schematic, ranges_adjacent: &mut [usize]) -> i32 {

    let mut current_range_index:usize = usize::MAX;
    let mut range_index:usize = 0;
    let add_coords = [
        Coords {x: 1, y: 0},
        Coords {x: 1, y: 1},
        Coords {x: 0, y: 1},
        Coords {x: -1, y: 1},
        Coords {x: -1, y: 0},
        Coords {x: -1, y: -1},
        Coords {x: 0, y: -1},
        Coords {x: 1, y: -1},
    ];

    while range_index < schematic.line_ranges.len() {

        if schematic.line_ranges[range_index].contains_index(index) {
            current_range_index = range_index;
            break;
        }

        range_index += 1;
    }

    if current_range_index == usize::MAX {
        return -1;
    }
    let current_line_start =schematic.line_ranges[current_range_index].start;
    let local_coord = Coords{
        x: (index - current_line_start) as i16,
        y: current_range_index as i16
    };


    let mut adjacent_indexes:Vec<usize> = vec![];

    for add in add_coords {
        let lookup_coord = local_coord.add(add);
        if lookup_coord.x < 0 ||
            lookup_coord.x >= schematic.width as i16 ||
            lookup_coord.y < 0 ||
            lookup_coord.y >= schematic.height as i16 {
            continue;
        }

        let line_index = lookup_coord.y as usize;
        let line_start_index = schematic.line_ranges[line_index].start;
        let content_index = line_start_index + lookup_coord.x as usize;

        let mut number_range_index:usize = 0;

        while number_range_index < schematic.number_ranges.len() {
            let contains_index =  schematic.number_ranges[number_range_index].range.contains_index(content_index);

            if contains_index && adjacent_indexes.iter().all(|x| *x != number_range_index) {
                adjacent_indexes.push(number_range_index);
            }
            number_range_index+= 1;
        }
    }



    if adjacent_indexes.len() == 2 {
        ranges_adjacent[0] = adjacent_indexes[0];
        ranges_adjacent[1] = adjacent_indexes[1];
    }


    return adjacent_indexes.len() as i32;
}
//
// fn calculate_schematic_size(file_contents: &String) -> SchematicSize
// {
//     let mut x:u8 = 0;
//     let mut max_x:u8 = 0;
//     let mut y:u8 = 0;
//
//     for ch in file_contents.chars() {
//         if ch == '\r' {
//             continue;
//         }
//         if ch != '\n' {
//             x += 1;
//         }
//         else{
//             y+=1;
//             max_x = max(max_x, x);
//             x = 0;
//         }
//     }
//     return SchematicSize {
//         width: max_x,
//         height: y + 1
//     };
// }
// fn build_schematics(file: File, schematic: &mut [Schematic]) -> usize
// {
//     let mut x:i16;
//     let mut y:i16 = 0;
//     let mut count: usize = 0;
//     let buffer_reader = BufReader::new(file);
//     for line_result in buffer_reader.lines() {
//         x = 0;
//         let Ok(line) = line_result else { return 0 };
//
//         for c in line.chars() {
//             schematic[count].coords.x = x;
//             schematic[count].coords.y = y;
//             schematic[count].symbol = c;
//             x = x + 1;
//             count = count + 1;
//         }
//         y = y + 1;
//     }
//     return count;
// }
// fn build_number_ranges(schematic: &[Schematic], ranges: &mut [SchematicRange]) -> usize
// {
//     let mut index:usize = 0;
//     let mut count:usize = 0;
//     while index < schematic.len() {
//         if !NUMBERS.contains(schematic[index].symbol) {
//             index += 1;
//             continue;
//         }
//         let end_index = find_number_range(&schematic, index);
//         ranges[count].start = index as i32;
//         ranges[count].end = end_index as i32;
//         count += 1;
//         index += 1;
//     }
//     return count;
// }
// fn build_number_ranges_v2(file_contents: &String, ranges: &mut [SchematicRange]) -> usize
// {
//     let mut index:usize = 0;
//     let mut count:usize = 0;
//     while index < file_contents.len() {
//         let char_result:Option<char> = file_contents.chars().nth(index);
//         if char_result == None {
//             println!("Error parsing file contents character.");
//             index+=1;
//             continue;
//         }
//         let current_char = char_result.unwrap();
//         if !NUMBERS.contains( current_char) {
//             index += 1;
//             continue;
//         }
//         let end_index = find_number_range_v2(&file_contents, index);
//         ranges[count].start = index as i32;
//         ranges[count].end = end_index as i32;
//         count += 1;
//         index += 1;
//     }
//     return count;
// }
// fn find_number_range(schematic: &[Schematic], start_index:usize) -> usize
// {
//     let mut index:usize = start_index;
//     let mut end_index:usize = start_index;
//     while NUMBERS.contains(schematic[index].symbol) {
//         end_index = index;
//         index = index + 1;
//     }
//     return end_index;
// }
//
// fn find_number_range_v2(file_contents: &String, start_index:usize) -> usize
// {
//     let mut index:usize = start_index;
//     let mut end_index:usize = start_index;
//
//
//
//     while NUMBERS.contains(file_contents.chars().nth(index).unwrap()) {
//         end_index = index;
//         index = index + 1;
//     }
//     return end_index;
// }
// fn build_symbols(schematic: &[Schematic], symbol_coords: &mut [usize]) -> usize
// {
//     let ignored = "0123456789.";
//     let mut count: usize = 0;
//     let mut index: usize = 0;
//     while index < schematic.len() {
//         if ignored.contains(schematic[index].symbol) {
//             index += 1;
//             continue;
//         }
//         symbol_coords[count] = index;
//         count = count + 1;
//         index += 1;
//     }
//     return count;
// }
//
// fn build_gears(file_contents: &String, gear_character: char, gear_indexes: &mut [usize]) -> usize {
//     let mut index:usize = 0;
//     let mut count:usize = 0;
//     while index < file_contents.len() {
//         let char_result = file_contents.chars().nth(index);
//
//         if char_result == None {
//             index +=1;
//             continue;
//         }
//         if char_result.unwrap() == gear_character {
//             gear_indexes[count] = index;
//             count += 1;
//         }
//
//         index += 1;
//     }
//     return count;
// }
//
//
// fn calculate_schematics_result(
//     schematic: &[Schematic],
//     ranges: &[SchematicRange],
//     symbol_coords: &[usize]) -> i32
// {
//     let mut result = 0;
//     let add_coords = [
//         Coords {x: 1, y: 0},
//         Coords {x: 1, y: 1},
//         Coords {x: 0, y: 1},
//         Coords {x: -1, y: 1},
//         Coords {x: -1, y: 0},
//         Coords {x: -1, y: -1},
//         Coords {x: 0, y: -1},
//         Coords {x: 1, y: -1},
//     ];
//     let mut used_ranges = [usize::default(); RANGES_SIZE];
//     let mut used_ranges_count:usize = 0;
//
//
//     for coord in symbol_coords {
//         let symbol_coord = schematic[*coord].coords;
//
//         for add_coord in add_coords {
//             let coord = symbol_coord.add(add_coord);
//             if !check_numbers_hit(&schematic, coord) {
//                 continue;
//                 //result += find_number(&schematic, coord);
//             }
//             let mut out_range_index:usize = 0;
//             if !find_range(&schematic, &ranges, coord, &mut out_range_index) {
//                 continue;
//             }
//
//             if used_ranges[..used_ranges_count].contains(&out_range_index) {
//                 continue;
//             }
//
//             used_ranges[used_ranges_count] = out_range_index;
//             used_ranges_count += 1;
//
//             let range_start = ranges[out_range_index].start as usize;
//             let range_end = ranges[out_range_index].end as usize;
//             let schematic_range = &schematic[range_start..=range_end];
//             let number_string = schematic_range
//                 .iter()
//                 .map(|s|s.symbol)
//                 .collect::<String>();
//             let parse_result = number_string.parse::<i32>();
//             if let Ok(number) = parse_result {
//                 result += number;
//             }
//         }
//     }
//
//     return result;
// }
// fn check_numbers_hit(schematic: &[Schematic], coord: Coords) -> bool
// {
//     let entry_option = schematic
//         .iter()
//         .find(|e| e.coords.eq(&coord));
//     let entry = entry_option.unwrap().symbol;
//
//     return NUMBERS.contains(entry);
// }
// fn find_range(
//     schematic: &[Schematic],
//     ranges: &[SchematicRange],
//     coord: Coords,
//     out_range_index: &mut usize)
//     -> bool
// {
//     let index:usize = schematic.iter()
//         .position(|e| e.coords.eq(&coord))
//         .unwrap();
//
//     let mut range_index:usize = 0;
//     while range_index < ranges.len() {
//
//         if index >= ranges[range_index].start as usize &&
//             index <= ranges[range_index].end as usize {
//
//             *out_range_index = range_index;
//             return true;
//         }
//         range_index += 1;
//     }
//
//     return false;
// }
//
//
// fn calculate_gears_result (
//     file_contents: &String,
//     schematic_size: SchematicSize,
//     ranges: &[SchematicRange],
//     gear_indices: &[usize])
//     -> i32
// {
//     let mut result = 0;
//     for gear_index in gear_indices {
//         let mut adjacent_ranges = [usize::default();2];
//         let adjacent_count = calculate_adjacent_number_count(
//             schematic_size,
//             *gear_index,
//             ranges,
//             &mut adjacent_ranges);
//         if adjacent_count != 2 {
//             continue;
//         }
//         let left_range_start = ranges[adjacent_ranges[0]].start as usize;
//         let left_range_end = ranges[adjacent_ranges[0]].end as usize;
//         let right_range_start = ranges[adjacent_ranges[1]].start as usize;
//         let right_range_end = ranges[adjacent_ranges[1]].end as usize;
//         let left_number_string_result = &file_contents[left_range_start..=left_range_end];
//         let right_number_string_result = &file_contents[right_range_start..=right_range_end];
//
//         let left_result = left_number_string_result.parse::<i32>();
//         let right_result = right_number_string_result.parse::<i32>();
//
//         if let Err(err) = left_result {
//             println!("Failed to parse left number from range. {}", err);
//             continue;
//         }
//         if let Err(err) = right_result {
//             println!("Failed to parse right number from range. {}", err);
//             continue;
//         }
//
//         result += left_result.unwrap() * right_result.unwrap();
//     }
//     return result;
// }
//
// fn calculate_adjacent_number_count(
//     schematic_size: SchematicSize,
//     index: usize,
//     ranges: &[SchematicRange],
//     range_indices: &mut [usize])
//     -> i32
// {
//     let current_coord:Coords = Coords {
//         x: (index % schematic_size.width as usize) as i16,
//         y: (index / schematic_size.width as usize) as i16
//     };
//     let add_coords = [
//         Coords {x: 1, y: 0},
//         Coords {x: 1, y: 1},
//         Coords {x: 0, y: 1},
//         Coords {x: -1, y: 1},
//         Coords {x: -1, y: 0},
//         Coords {x: -1, y: -1},
//         Coords {x: 0, y: -1},
//         Coords {x: 1, y: -1},
//     ];
//     let mut result:i32 = 0;
//
//     for add_coord in add_coords {
//         let result_coord = current_coord.add(add_coord);
//         if result_coord.x < 0 ||
//             result_coord.x >= schematic_size.width as i16 ||
//             result_coord.y < 0 ||
//             result_coord.y >= schematic_size.height as i16 {
//             continue;
//         }
//
//         let index:i32 = (result_coord.y * schematic_size.width as i16 + result_coord.x) as i32;
//         let mut range_index:usize = 0;
//         while range_index < ranges.len() {
//             if index < ranges[range_index].start || index > ranges[range_index].end {
//                 range_index+=1;
//                 continue;
//             }
//             result += 1;
//             if result <= 2 {
//                 range_indices[result as usize - 1] = range_index;
//             }
//             range_index += 1;
//         }
//     }
//
//
//     return result;
// }
