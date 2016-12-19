use worldgen::context::Pos;

pub enum Direction {
    North, East, South, West
}

impl Direction {
    pub fn add_pos(&self, &(x, y): &Pos) -> Pos {
        match *self {
            Direction::North => (x, y - 1),
            Direction::East => (x + 1, y),
            Direction::South => (x, y + 1),
            Direction::West => (x - 1, y)
        }
    }
}