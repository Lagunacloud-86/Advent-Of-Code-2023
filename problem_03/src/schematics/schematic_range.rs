#[derive(Copy, Clone)]
pub struct SchematicRange {
    pub start: usize,
    pub end: usize
}

#[derive(Copy, Clone)]
pub struct SchematicNumberRange {
    pub range: SchematicRange,
    pub value: i32
}

pub trait ContainsIndex {
    fn contains_index(self: Self, index: usize) -> bool;
}

impl ContainsIndex for SchematicRange {
    fn contains_index(self: Self, index: usize) -> bool {
        self.start <= index && self.end >= index
    }
}
impl Default for SchematicRange {
    fn default() -> Self {
        Self { start: 0, end: 0 }
    }
}
