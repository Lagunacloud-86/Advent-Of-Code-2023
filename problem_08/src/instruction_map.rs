pub struct Instruction {
    left: String,
    right: String
}

impl Default for Instruction {
    fn default() -> Self {
        Self {left: String::default(), right: String::default()}
    }
}
impl Instruction {

    pub fn create(left: &str, right: &str) -> Self {
        Self{ left: String::from(left), right: String::from(right) }
    }

    pub fn get_left(self: &Self) -> &str {
        &self.left
    }
    pub fn get_right(self: &Self) -> &str {
        &self.right
    }
}