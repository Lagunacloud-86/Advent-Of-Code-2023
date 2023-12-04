use std::ops::Add;

#[derive(Copy, Clone)]
pub struct Coords {
    pub x: i16,
    pub y: i16
}

impl Default for Coords {
    fn default() -> Self {
        Coords { x: 0, y: 0 }
    }
}

impl Add for Coords {
    type Output = Coords;

    fn add(self, rhs: Self) -> Self {
        Coords {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl PartialEq for Coords {
    fn eq(&self, other: &Coords) -> bool {
        self.x == other.x && self.y == other.y
    }
}
