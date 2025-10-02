use bevy::prelude::Color;

macro_rules! hex_colors {
    ($($hex:expr),* $(,)?) => {
        [$(Color::srgb_u8((($hex >> 16) & 0xff) as u8, (($hex >> 8) & 0xff) as u8, ($hex & 0xff) as u8)),*]
    }
}

pub struct Nord;

impl Nord {
    pub const AURORA: [Color; 5] =
        [Self::RED, Self::ORANGE, Self::YELLOW, Self::GREEN, Self::PURPLE];
    pub const FROST: [Color; 4] =
        [Self::PALETTE[7], Self::PALETTE[8], Self::PALETTE[9], Self::PALETTE[10]];
    pub const GREEN: Color = Self::PALETTE[14];
    pub const NIGHT: [Color; 4] =
        [Self::PALETTE[0], Self::PALETTE[1], Self::PALETTE[2], Self::PALETTE[3]];
    pub const ORANGE: Color = Self::PALETTE[12];
    #[expect(clippy::unreadable_literal)]
    pub const PALETTE: [Color; 16] = hex_colors!(
        0x2e3440, 0x3b4252, 0x434c5e, 0x4c566a, 0xd8dee9, 0xe5e9f0, 0xeceff4, 0xeceff4, 0x88c0d0,
        0x88c0d0, 0x5e81ac, 0xbf616a, 0xd08770, 0xebcb8b, 0xa3be8c, 0xb48ead,
    );
    pub const PURPLE: Color = Self::PALETTE[15];
    pub const RED: Color = Self::PALETTE[11];
    pub const SNOW: [Color; 3] = [Self::PALETTE[4], Self::PALETTE[5], Self::PALETTE[6]];
    pub const YELLOW: Color = Self::PALETTE[13];
}
