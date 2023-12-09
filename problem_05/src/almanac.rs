// #[derive(Copy, Clone)]
// pub struct Range {
//     start: i64,
//     length: i64
// }
// impl Default for Range {
//     fn default() -> Self {
//         Range {start: 0, length: 0 }
//     }
// }
// // impl Range {
// //     pub fn create(start: i128, length: i128) -> Range {
// //         Range {start, length}
// //     }
// // }
//
// impl Range {
//     pub fn contains(self: &Self, value: i64) -> bool {
//         return value >= self.start && value <= self.start + self.length
//     }
//     pub fn overlaps(self: &Self, oth: &Self) -> bool {
//         let total_length = self.length + oth.length;
//         if self.start > oth.start {
//             return self.start - oth.start < total_length;
//         }
//         return oth.start - self.start < total_length;
//     }
// }
// #[derive(Copy, Clone)]
// pub struct AlmanacRange {
//     destination_range: Range,
//     source_range: Range,
//     mapped: bool
// }
//
// impl AlmanacRange {
//
//     // pub fn get_destination_range(self: &Self) -> &Range {
//     //     &self.destination_range
//     // }
//     pub fn get_source_range(self: &Self) -> &Range {
//         &self.source_range
//     }
//     pub fn calculate_destination(self: &Self, value: i64) -> i64 {
//         let offset = self.source_range.start + self.source_range.length - value;
//         return self.destination_range.start + self.destination_range.length - offset;
//     }
//
//     fn create(line: &str) -> AlmanacRange {
//
//         let mut line_entries = [i64::default();3];
//         let mut count:usize = 0;
//
//         for entry in line.split(' ') {
//             let trimmed_entry = entry.trim();
//             if trimmed_entry == "" {
//                 continue;
//             }
//
//             let parse_result = trimmed_entry.parse::<i64>();
//             if let Err(err) = parse_result {
//                 println!("Failed to parse map entry, Error: {}", err);
//                 continue;
//             }
//
//             line_entries[count] = parse_result.unwrap();
//             count += 1;
//             if count == 3 {
//                 break;
//             }
//         }
//         return AlmanacRange {
//             destination_range: Range{ start: line_entries[0], length: line_entries[2] },
//             source_range: Range{ start: line_entries[1], length: line_entries[2] },
//             mapped: false
//         };
//     }
// }
//
// pub struct AlmanacMap {
//
//     mapped_ranges:Vec<AlmanacRange>,
//     unmapped_ranges:Vec<AlmanacRange>,
//     upper_bound: i128
// }
//
// impl Default for AlmanacMap {
//     fn default() -> Self {
//         Self { mapped_ranges: vec![], unmapped_ranges: vec![], upper_bound: 0 }
//     }
// }
//
// impl AlmanacMap {
//
//     fn set_map(self: &mut Self, section: &str) {
//         for line in section.lines() {
//             let trimmed_line = line.trim();
//             if trimmed_line == "" {
//                 break;
//             }
//             self.mapped_ranges.push(AlmanacRange::create(line));
//         }
//         self.mapped_ranges.sort_by(|l,r| r.source_range.start.cmp(&l.source_range.start));
//
//         let mut index:i64 = 0;
//         for entry in self.mapped_ranges {
//
//         }
//
//     }
// }
//
// pub struct Almanac {
//     pub seeds: Vec<i128>,
//     seed_to_soil_map: AlmanacMap,
//     soil_to_fertilizer_map: AlmanacMap,
//     fertilizer_to_water_map: AlmanacMap,
//     water_to_light_map: AlmanacMap,
//     light_to_temperature_map: AlmanacMap,
//     temperature_to_humidity_map: AlmanacMap,
//     humidity_to_location_map: AlmanacMap
// }
// impl Default for Almanac {
//     fn default() -> Self {
//         Almanac {
//             seeds: vec![],
//             seed_to_soil_map: AlmanacMap::default(),
//             soil_to_fertilizer_map: AlmanacMap::default(),
//             fertilizer_to_water_map: AlmanacMap::default(),
//             water_to_light_map: AlmanacMap::default(),
//             light_to_temperature_map: AlmanacMap::default(),
//             temperature_to_humidity_map: AlmanacMap::default(),
//             humidity_to_location_map: AlmanacMap::default()
//         }
//     }
// }
// impl Almanac {
//     pub fn get_seeds(self: &Self) -> &Vec<i128> {
//         &self.seeds
//     }
//     // pub fn iter_almanac_ranges(self: &Self) -> [&Vec<AlmanacRange>; 7] {
//     //     [
//     //         &self.seed_to_soil,
//     //         &self.soil_to_fertilizer,
//     //         &self.fertilizer_to_water,
//     //         &self.water_to_light,
//     //         &self.light_to_temperature,
//     //         &self.temperature_to_humidity,
//     //         &self.humidity_to_location
//     //     ]
//     // }
// }
//
//
//
// pub fn create_almanac(file_contents: &String) -> Almanac {
//     let mut almanac:Almanac = Almanac::default();
//
//     parse_seeds(&file_contents, &mut almanac);
//     parse_map(&file_contents, "seed-to-soil map:", &mut almanac.seed_to_soil_map);
//     parse_map(&file_contents, "soil-to-fertilizer map:", &mut almanac.soil_to_fertilizer_map);
//     parse_map(&file_contents, "fertilizer-to-water map:", &mut almanac.fertilizer_to_water_map);
//     parse_map(&file_contents, "water-to-light map:", &mut almanac.water_to_light_map);
//     parse_map(&file_contents, "light-to-temperature map:", &mut almanac.light_to_temperature_map);
//     parse_map(&file_contents, "temperature-to-humidity map:", &mut almanac.temperature_to_humidity_map);
//     parse_map(&file_contents, "humidity-to-location map:", &mut almanac.humidity_to_location_map);
//     return almanac;
// }
// fn parse_seeds(file_contents: &String, almanac: &mut Almanac) {
//     let seeds_str = "seeds: ";
//     let index_result = file_contents.find(seeds_str);
//     if index_result == None {
//         println!("Failed to find seeds section");
//         return;
//     }
//     let index = index_result.unwrap() + seeds_str.len();
//     let end_line_index_result = file_contents[index..].find('\n');
//     if end_line_index_result == None {
//         println!("Failed to find seeds section");
//         return;
//     }
//
//     let end_line_index = end_line_index_result.unwrap();
//     let line = &file_contents[index..][..end_line_index];
//     for entry in line.split(' ') {
//         let trimmed_entry = entry.trim();
//         if trimmed_entry == "" {
//             continue;
//         }
//         let seed_result = trimmed_entry.parse::<i128>();
//         if let Err(err) = seed_result {
//             println!("Failed to parse integer from seed entry. Error: {}", err);
//             continue;
//         }
//         almanac.seeds.push(seed_result.unwrap());
//     }
//
//
// }
// fn parse_map(file_contents: &String, map: &str, almanac_map: &mut AlmanacMap)
// {
//     let index_result = file_contents.find(map);
//     if index_result == None {
//         println!("Failed to find seeds section");
//         return;
//     }
//     let index = index_result.unwrap() + map.len();
//     let mut slice = &file_contents[index..];
//     slice = &slice[slice.find('\n') .unwrap() + 1..];
//
//     almanac_map.set_map(slice);
// }

//pub fn create_maps()