const MAX_SIZE:usize = 32;

#[derive(Copy, Clone)]
pub struct Sequence {
    entries: [i64;MAX_SIZE],
    length: u16
}

impl Default for Sequence {
    fn default() -> Self {
        Self {
            entries: [i64::default(); MAX_SIZE],
            length: 0
        }
    }
}

impl Sequence {

    pub fn create_from_vec(vec: &Vec<i64>) -> Self {
        let mut entries = [i64::default(); MAX_SIZE];
        for index in 0..vec.len() {
            entries[index] = vec[index];
        }

        Self {
            length: vec.len() as u16,
            entries
        }
    }
    pub fn create(initial_sequence: &[i64]) -> Self {
        let mut entries = [i64::default(); MAX_SIZE];
        for index in 0..initial_sequence.len() {
            entries[index] = initial_sequence[index];
        }
        //entries.copy_from_slice(initial_sequence);

        Self {
            length: initial_sequence.len() as u16,
            entries
        }
    }
    // pub fn get_entry(self: &Self, index: usize) -> i64 {
    //     self.entries[index]
    // }
    // pub fn get_entries(self: &Self) -> &[i64] {
    //     &self.entries[..self.length as usize]
    // }

    pub fn calculate_next_value(self: &mut Self, prev: Option<&mut Self>) -> i64 {
        let mut current_sequence = [i64::default();MAX_SIZE];
        let new_length =self.length - 1;

        for i in 0..new_length {
            let result = self.entries[(i + 1) as usize] - self.entries[i as usize];
            current_sequence[i as usize] = result;
        }

        let mut sequence = Sequence
            ::create(&current_sequence[..new_length as usize]);

        if current_sequence[..new_length as usize].iter().all(|e| *e == 0 ) {
            let prev_sequence = prev.unwrap();

            return prev_sequence.entries[prev_sequence.length as usize - 1]
                + self.entries[self.length as usize - 1];
        }

        let result = sequence.calculate_next_value( Some(self));
        if let Some(p) = prev {
            p.entries[p.length as usize] = p.entries[p.length as usize - 1] + result;
            return p.entries[p.length as usize - 1] + result;
        }
        return result;
    }

    pub fn calculate_previous_value(self: &mut Self, prev: Option<&mut Self>) -> i64 {
        let mut current_sequence = [i64::default();MAX_SIZE];
        let new_length =self.length - 1;

        for i in 0..new_length {
            let result = self.entries[(i + 1) as usize] - self.entries[i as usize];
            current_sequence[i as usize] = result;
        }

        let mut sequence = Sequence
            ::create(&current_sequence[..new_length as usize]);

        if current_sequence[..new_length as usize].iter().all(|e| *e == 0 ) {
            let p = prev.unwrap();
            return p.entries[0] - self.entries[0];
        }

        let result = sequence.calculate_previous_value( Some(self));
        if let Some(p) = prev {
            p.entries[p.length as usize] = p.entries[0] - result;
            return p.entries[0] - result;
        }
        return result;
    }


}

