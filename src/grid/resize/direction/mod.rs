#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Top,
    Bottom,
}

impl Direction {
    pub fn toggle(&mut self) {
        *self = match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Top => Self::Bottom,
            Self::Bottom => Self::Top,
        };
    }
}
