use macroquad::color::Color;

pub const TEXT: Palette = Palette::Anthracite;
pub const BORDER: Palette = Palette::Anthracite;
pub const GAME_BACKGROUND: Palette = Palette::Mocha;
pub const CARD_BACKGROUND: Palette = Palette::White;
pub const BAR_BACKGROUND: Palette = Palette::White;
pub const BUTTON_BACKGROUND: Palette = Palette::OffWhite;
pub const IMAGE_BACKGROUND: Palette = Palette::OffWhite;
pub const SKILL_COLOR: Palette = Palette::Aqua;
pub const PILL_COLOR: Palette = Palette::AnthraciteLight;
pub const PILL_TEXT_COLOR: Palette = Palette::White;
pub const PRODUCT_COLOR: Palette = Palette::Peach;
pub const PROGRESS_COLOR: Palette = Palette::Grass;

pub enum Palette {
    Anthracite,
    AnthraciteLight,
    DarkGray,
    Coral,
    Aqua,
    Peach,
    OffWhite,
    Grass,
    Mocha,
    Black,
    White,
}

impl Palette {
    pub fn get_color(&self) -> Color {
        match self {
            Palette::Anthracite => Color::from_rgba(22, 27, 30, 255),
            Palette::AnthraciteLight => Color::from_rgba(36, 41, 46, 255),
            Palette::DarkGray => Color::from_rgba(84, 84, 84, 255),
            Palette::Coral => Color::from_rgba(255, 87, 87, 255),
            Palette::Aqua => Color::from_rgba(12, 192, 223, 255),
            Palette::Peach => Color::from_rgba(254, 197, 114, 255),
            Palette::OffWhite => Color::from_rgba(221, 221, 221, 255),
            Palette::Grass => Color::from_rgba(126, 217, 87, 255),
            Palette::Mocha => Color::from_rgba(195, 157, 117, 255),
            Palette::Black => Color::from_rgba(0, 0, 0, 255),
            Palette::White => Color::from_rgba(255, 255, 255, 255),
        }
    }
}