use macroquad::color::Color;

pub const TEXT: PaletteC = PaletteC::Anthracite;
pub const BORDER: PaletteC = PaletteC::AnthraciteLight;
pub const WINDOW_BACKGROUND: PaletteC = PaletteC::DarkGray;
pub const GAME_BACKGROUND: PaletteC = PaletteC::Brown;
pub const CARD_BACKGROUND: PaletteC = PaletteC::Mocha;
pub const BAR_BACKGROUND: PaletteC = PaletteC::WhiteTransparent;
pub const BUTTON_BACKGROUND: PaletteC = PaletteC::WhiteTransparent;
pub const BUTTON_HOVER: PaletteC = PaletteC::Peach;
pub const BUTTON_CLICKED: PaletteC = PaletteC::BlackTransparent;
pub const BUTTON_TEXT: PaletteC = PaletteC::Anthracite;
pub const IMAGE_BACKGROUND: PaletteC = PaletteC::WhiteTransparent;
pub const SKILL_COLOR: PaletteC = PaletteC::Aqua;
pub const JOB_COLOR: PaletteC = PaletteC::WarmYellow;
pub const PILL_COLOR: PaletteC = PaletteC::AnthraciteLight;
pub const PILL_TEXT_COLOR: PaletteC = PaletteC::OffWhite;
pub const PRODUCT_COLOR: PaletteC = PaletteC::WhiteTransparent;
pub const PROGRESS_COLOR: PaletteC = PaletteC::Grass;

pub enum PaletteC {
    Anthracite,
    AnthraciteLight,
    DarkGray,
    Coral,
    Aqua,
    Peach,
    OffWhite,
    Grass,
    Mocha,
    Brown,
    Black,
    White,
    WarmYellow,
    WhiteTransparent,
    GreenTransparent,
    BlackTransparent,
}

impl PaletteC {
    pub fn get_color(&self) -> Color {
        match self {
            PaletteC::Anthracite => Color::from_rgba(22, 27, 30, 255),
            PaletteC::AnthraciteLight => Color::from_rgba(36, 41, 46, 255),
            PaletteC::DarkGray => Color::from_rgba(84, 84, 84, 255),
            PaletteC::Coral => Color::from_rgba(255, 87, 87, 255),
            PaletteC::Aqua => Color::from_rgba(12, 192, 223, 255),
            PaletteC::Peach => Color::from_rgba(254, 197, 114, 255),
            PaletteC::OffWhite => Color::from_rgba(221, 221, 221, 255),
            PaletteC::Grass => Color::from_rgba(126, 217, 87, 255),
            PaletteC::Mocha => Color::from_rgba(195, 157, 117, 255),
            PaletteC::Brown => Color::from_rgba(65, 19, 8, 255),
            PaletteC::Black => Color::from_rgba(0, 0, 0, 255),
            PaletteC::White => Color::from_rgba(255, 255, 255, 255),
            PaletteC::WarmYellow => Color::from_rgba(255, 200, 51, 255),
            PaletteC::WhiteTransparent => Color::from_rgba(255, 255, 255, 100),
            PaletteC::GreenTransparent => Color::from_rgba(126, 217, 87, 200),
            PaletteC::BlackTransparent => Color::from_rgba(0, 0, 0, 40),
        }
    }
}