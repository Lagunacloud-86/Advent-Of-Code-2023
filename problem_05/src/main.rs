mod almanac;

use std::fs::File;
use std::io::Read;
use std::path::Path;



fn main() {
    let mut result : i128;

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

// fn solve_first<P>(file_path : P) -> i128
//     where P: AsRef<Path>
// {
//     let file_contents_result = read_file_contents(file_path);
//     if file_contents_result == None {
//         println!("Unable to read file contents");
//         return 0;
//     }
//     let file_contents = file_contents_result.unwrap();
//     let almanac:Almanac = create_almanac(&file_contents);
//     let mut result:i128 = i128::MAX;
//
//
//     // for seed in &almanac.seeds {
//     //     result = i128::min(result, find_seed_location(*seed, &almanac));
//     // }
//
//
//     return result;
// }

fn solve_second<P>(file_path : P) -> i128
    where P: AsRef<Path>
{
    let file_contents_result = read_file_contents(file_path);
    if file_contents_result == None {
        println!("Unable to read file contents");
        return 0;
    }

    let file_contents = file_contents_result.unwrap();

    let parsed = file_contents
        .split("\n\n");

    for p in parsed {
        println!("{}", p);
    }

    // let almanac:Almanac = create_almanac(&file_contents);
    // let mut result:i128 = i128::MAX;
    //
    //
    // let mut index:usize = 0;
    // while index < almanac.seeds.len() {
    //     let start_index = &almanac.seeds[index];
    //     let length = &almanac.seeds[index + 1];
    //     //let seed_range = Range::create(*start_index, *length);
    //     let mut seed:usize = *start_index as usize;
    //     while seed < (start_index + length) as usize {
    //         result = i128::min(result, find_seed_location(seed as i128, &almanac));
    //         seed += 1;
    //     }
    //     //result = i128::min(result, find_seed_range_lowest_location(&seed_range, &almanac));
    //     //index += 2;
    //     index += 2;
    // }
    // for seed in &almanac.seeds {
    //     result = i128::min(result, find_seed_location(*seed, &almanac));
    // }


    return 0;
}

// fn find_seed_range_lowest_location(seed_range: &Range, almanac: &Almanac) -> i128 {
//     let mut result:i128 = 0;
//     let mut result_ranges:Vec<Range> = vec![];
//     for entry in almanac.seed_to_soil.iter().filter(|m|m.get_source_range().overlaps(&seed_range)) {
//         result_ranges.push(*entry.get_destination_range());
//     }
//     return result;
// }
// fn find_range_lowest_location(seed_range: &Range, almanac: &Almanac) -> i128 {
//     let mut result:i128 = 0;
//     let mut ranges:Vec<Range> = vec![];
//     let mut index:usize = 0;
//     ranges.push(*seed_range);
//
//     while index < almanac.iter_almanac_ranges().len() {
//         let entry = &almanac.iter_almanac_ranges()[index];
//
//         let mut result_ranges:Vec<Range> = vec![];
//         for lookup_range in &ranges {
//             for range in entry.iter().filter(|r| r.get_source_range().overlaps(&lookup_range)) {
//                 result_ranges.push(*range.get_destination_range());
//             }
//         }
//         ranges.clear();
//
//         for result_range in result_ranges {
//             ranges.push(result_range);
//         }
//
//         index += 1;
//     }
//
//
//
//     return result;
// }
//
// fn find_seed_location(seed: i128, almanac: &Almanac) -> i128 {
//     let mut result:i128 = seed;
//
//     result = find_value(result, &almanac.seed_to_soil);
//     result = find_value(result, &almanac.soil_to_fertilizer);
//     result = find_value(result, &almanac.fertilizer_to_water);
//     result = find_value(result, &almanac.water_to_light);
//     result = find_value(result, &almanac.light_to_temperature);
//     result = find_value(result, &almanac.temperature_to_humidity);
//     result = find_value(result, &almanac.humidity_to_location);
//
//     return result;
// }
//
//
// fn find_value(lookup: i128, ranges: &Vec<AlmanacRange>) -> i128 {
//     for range in ranges {
//         if range.get_source_range().contains(lookup) {
//             return range.calculate_destination(lookup);
//         }
//     }
//     return lookup;
// }
