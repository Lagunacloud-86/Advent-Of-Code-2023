use std::cmp::max;

pub const MAX_WIDTH:i32 = 150;
pub const MAX_HEIGHT:i32 = 150;

#[derive(Copy, Clone)]
pub enum Directions {
    Move,
    NoMove,
    NotImplemented
}

#[derive(Copy, Clone)]
pub struct Mapping {

    from_char: char,
    to_char: char,
    current_char: char,

    from_row: i16,
    from_col: i16,
    to_row: i16,
    to_col: i16,


    direction: Directions
}

impl Default for Mapping {
    fn default() -> Self {
        Self {
            from_row: -1,
            from_col: -1,
            to_row: -1,
            to_col: -1,
            from_char: '.',
            to_char: '.',
            current_char: '.',
            direction: Directions::NotImplemented
        }
    }
}

impl Mapping {
    fn make_reverse(self: &Self) -> Self {
        Self {
            from_char: self.to_char,
            to_char: self.from_char,
            current_char: self.current_char,
            from_row: self.to_row,
            to_row: self.from_row,
            from_col: self.to_col,
            to_col: self.from_col,
            direction: self.direction
        }
    }

    fn get_from_char(self: &Self) -> char {
        self.from_char
    }
    fn get_to_char(self: &Self) -> char {
        self.to_char
    }
    fn get_current_char(self: &Self) -> char {
        self.current_char
    }


    fn get_from_row(self: &Self) -> i16 {
        self.from_row
    }
    fn get_to_row(self: &Self) -> i16 {
        self.to_row
    }

    fn get_from_col(self: &Self) -> i16 {
        self.from_col
    }
    fn get_to_col(self: &Self) -> i16 {
        self.to_col
    }
    fn get_direction(self: &Self) -> Directions {
        self.direction
    }
}

pub struct PipeMap {

    mappings: [[Mapping; MAX_WIDTH as usize]; MAX_HEIGHT as usize],
    reverse_mappings: [[Mapping; MAX_WIDTH as usize]; MAX_HEIGHT as usize],
    width: i32,
    height: i32,
    start_row: usize,
    start_col: usize
}

impl PipeMap {



    pub fn create(file_contents: &str) -> PipeMap {
        let mut data = [[char::default(); MAX_WIDTH as usize]; MAX_HEIGHT as usize];
        let mut mappings = [[Mapping::default(); MAX_WIDTH as usize]; MAX_HEIGHT as usize];
        let mut reverse_mappings = [[Mapping::default(); MAX_WIDTH as usize]; MAX_HEIGHT as usize];
        let mut col_count:i32 = 0;

        let mut row:usize = 0;
        for line in file_contents.lines() {
            col_count = max(col_count, line.trim().len() as i32);

            let mut col = 0;
            for ch in line.trim().chars() {
                data[row][col] = ch;
                col += 1;
            }
            row += 1;
        }

        build_mappings(&data, &mut mappings, col_count as usize, row);
        build_reverse_mappings(&mappings, &mut reverse_mappings, col_count as usize, row);

        Self {
            width: col_count,
            height: row as i32,
            mappings,
            reverse_mappings

        }
    }

    pub fn get_entry(self: &Self, row: usize, col: usize) -> char {
        self.mappings[row][col].current_char
    }
    pub fn get_width(self: &Self) -> i32 {
        self.width
    }
    pub fn get_height(self: &Self) -> i32 {
        self.height
    }

}


fn build_mappings(data: &[[char; MAX_WIDTH as usize]; MAX_HEIGHT as usize],
                  mappings: &mut [[Mapping;MAX_WIDTH as usize]; MAX_HEIGHT as usize],
                  width: usize,
                  height: usize) {

    for r in 0..height {
        for c in 0..width {
            if mappings[r][c].direction != Directions::NotImplemented {
                continue;
            }
            if data[r][c] == '.' {
                mappings[r][c].direction = Directions::NoMove;
                continue;
            }
            let mut current_row = r;
            let mut current_col = c;
            loop {


                if current_row == r && current_col == c {
                    break;
                }
            }

        }
    }
}
fn build_reverse_mappings(mappings: &[[Mapping;MAX_WIDTH as usize]; MAX_HEIGHT as usize],
                          reverse_mappings: &mut [[Mapping;MAX_WIDTH as usize]; MAX_HEIGHT as usize],
                          width: usize,
                          height: usize) {
    for r in 0..height {
        for c in 0..width {
            reverse_mappings[r][c] = mappings[r][c].make_reverse();
        }
    }

}

fn char_to_direction(input: char) -> Directions {
    match input {
        '|'|'-'|'J'|'L'|'7'|'F'|'S' => Directions::Move,
        '.' => Directions::NoMove,
        _ => Directions::NotImplemented
    }
}