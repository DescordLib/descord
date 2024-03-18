use nanoserde::{DeJson, SerJson};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Color {
    Rgb(u8, u8, u8),

    #[default]
    Black,

    Red,
    Green,
    Blue,
    Yellow,
    Orange,
    Purple,
    Cyan,
    Magenta,
    Pink,
    Teal,
    Brown,
    Navy,
    Maroon,
    Olive,
    Silver,
    Gold,
    White,
}

impl Into<u32> for Color {
    fn into(self) -> u32 {
        match self {
            Self::Rgb(r, g, b) => (r as u32) << 16 | (g as u32) << 8 | b as u32,
            Self::Red => 0xFF0000,
            Self::Green => 0x00FF00,
            Self::Blue => 0x0000FF,
            Self::Yellow => 0xFFFF00,
            Self::Orange => 0xFFA500,
            Self::Purple => 0x800080,
            Self::Cyan => 0x00FFFF,
            Self::Magenta => 0xFF00FF,
            Self::Pink => 0xFFC0CB,
            Self::Teal => 0x008080,
            Self::Brown => 0xA52A2A,
            Self::Navy => 0x000080,
            Self::Maroon => 0x800000,
            Self::Olive => 0x808000,
            Self::Silver => 0xC0C0C0,
            Self::Gold => 0xFFD700,
            Self::White => 0xFFFFFF,
            Self::Black => 0x000000,
        }
    }
}
