
const MAX_NUMBERS_SIZE:usize = 32;
const MAX_WINNING_NUMBERS_SIZE:usize = 16;
pub struct ScratchCard {
    numbers:[i8;MAX_NUMBERS_SIZE],
    winning_numbers:[i8;MAX_WINNING_NUMBERS_SIZE],
    matching_numbers:[i8;MAX_WINNING_NUMBERS_SIZE],
    card_index:i16,
    numbers_count:i8,
    winning_numbers_count:i8,
    matching_numbers_count:i8
}
impl Default for ScratchCard {
    fn default() -> Self {
        ScratchCard
        {
            numbers: [i8::default(); MAX_NUMBERS_SIZE],
            winning_numbers: [i8::default(); MAX_WINNING_NUMBERS_SIZE],
            matching_numbers: [i8::default(); MAX_WINNING_NUMBERS_SIZE],
            card_index: -1,
            numbers_count: 0,
            winning_numbers_count: 0,
            matching_numbers_count: 0
        }
    }
}


 impl ScratchCard {
     // pub fn get_card_number(self: &Self) -> i16 {
     //     return self.card_index;
     // }
     // pub fn get_numbers(self: &Self) -> &[i8] {
     //    return &self.numbers[..self.numbers_count as usize];
     // }
     // pub fn get_winning_numbers(self: &Self) -> &[i8] {
     //     return &self.winning_numbers[..self.winning_numbers_count as usize];
     // }
     pub fn get_matching_numbers(self: &Self) -> &[i8] {
         return &self.matching_numbers[..self.matching_numbers_count as usize];
     }
     pub fn get_matching_numbers_count(self: &Self) -> i8 {
         return self.matching_numbers_count;
     }
 }

pub fn create_scratch_cards(file_contents: &String) -> Vec<ScratchCard> {

    let mut scratch_cards:Vec<ScratchCard> = vec![];
    for line in file_contents.split('\n') {
        scratch_cards.push(create_scratch_card(&line));
    }


    return scratch_cards;
}

fn create_scratch_card(line: &str) -> ScratchCard {
    let mut scratch_card:ScratchCard = ScratchCard::default();

    let data_section = &line[4..];
    let index_of_card_index = data_section.chars().position(|c| c == ':').unwrap();

    let card_index_section = &data_section[..index_of_card_index].trim();
    scratch_card.card_index = card_index_section.parse::<i16>().unwrap();

    let numbers_sections = &data_section[index_of_card_index+1..];

    let index_of_bar = numbers_sections.chars().position(|c| c == '|').unwrap();
    let winning_numbers_section = &numbers_sections[0..index_of_bar];
    let numbers_section = &numbers_sections[(index_of_bar + 1)..];

    let mut count:usize = 0;
    for winning_number in winning_numbers_section.split(' ') {
        let entry = winning_number.trim();
        if entry == "" {
            continue;
        }
        scratch_card.winning_numbers[count] = entry.parse::<i8>().unwrap();
        count += 1;
    } scratch_card.winning_numbers_count = count as i8;
    count = 0;

    for number in numbers_section.split(' ') {
        let entry = number.trim();
        if entry == "" {
            continue;
        }
        scratch_card.numbers[count] = entry.parse::<i8>().unwrap();
        count += 1;
    } scratch_card.numbers_count = count as i8;

    count = 0;
    let numbers = &scratch_card.numbers[..scratch_card.numbers_count as usize];
    let winning_numbers = &scratch_card.winning_numbers[..scratch_card.winning_numbers_count as usize];
    for number in numbers {
        if winning_numbers.contains(number) {
            scratch_card.matching_numbers[count] = *number;
            count += 1;
        }
    } scratch_card.matching_numbers_count = count as i8;



    return scratch_card;
}