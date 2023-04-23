use Color::*;

pub enum Color {
    Red,
}

impl From<Color> for u32 {
    fn from(color: Color) -> Self {
        match color {
            Red => 0xff6464,
        }
    }
}
