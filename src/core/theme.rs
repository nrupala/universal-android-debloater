#[derive(Default, Debug, PartialEq, Eq, Copy, Clone)]
pub enum Theme {
    #[default]
    Lupin,
    Dark,
    Light,
}

#[cfg(feature = "gui")]
mod colors {
    #[cfg(not(target_os = "android"))]
    pub use iced::{color, Color};

    #[cfg(target_os = "android")]
    #[derive(Debug, Clone, Copy)]
    pub struct Color {
        pub r: f32,
        pub g: f32,
        pub b: f32,
        pub a: f32,
    }

    #[cfg(target_os = "android")]
    impl Color {
        pub const fn from_hex(hex: u32) -> Self {
            Self {
                r: ((hex >> 16) & 0xFF) as f32 / 255.0,
                g: ((hex >> 8) & 0xFF) as f32 / 255.0,
                b: (hex & 0xFF) as f32 / 255.0,
                a: 1.0,
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct BaseColors {
        pub background: Color,
        pub foreground: Color,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct NormalColors {
        pub primary: Color,
        pub secondary: Color,
        pub surface: Color,
        pub error: Color,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct BrightColors {
        pub primary: Color,
        pub secondary: Color,
        pub surface: Color,
        pub error: Color,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct ColorPalette {
        pub base: BaseColors,
        pub normal: NormalColors,
        pub bright: BrightColors,
    }

    use super::Theme;

    impl Theme {
        pub fn palette(self) -> ColorPalette {
            match self {
                Self::Dark => ColorPalette {
                    base: BaseColors {
                        background: hex_color!(0x111111),
                        foreground: hex_color!(0x1C1C1C),
                    },
                    normal: NormalColors {
                        primary: hex_color!(0x5E4266),
                        secondary: hex_color!(0x386e50),
                        surface: hex_color!(0x828282),
                        error: hex_color!(0x992B2B),
                    },
                    bright: BrightColors {
                        primary: hex_color!(0xBA84FC),
                        secondary: hex_color!(0x49eb7a),
                        surface: hex_color!(0xE0E0E0),
                        error: hex_color!(0xC13047),
                    },
                },
                Self::Light => ColorPalette {
                    base: BaseColors {
                        background: hex_color!(0xEEEEEE),
                        foreground: hex_color!(0xE0E0E0),
                    },
                    normal: NormalColors {
                        primary: hex_color!(0x230F08),
                        secondary: hex_color!(0xF9D659),
                        surface: hex_color!(0x818181),
                        error: hex_color!(0x992B2B),
                    },
                    bright: BrightColors {
                        primary: hex_color!(0x673AB7),
                        secondary: hex_color!(0x3797A4),
                        surface: hex_color!(0x000000),
                        error: hex_color!(0xC13047),
                    },
                },
                Self::Lupin => ColorPalette {
                    base: BaseColors {
                        background: hex_color!(0x282a36),
                        foreground: hex_color!(0x353746),
                    },
                    normal: NormalColors {
                        primary: hex_color!(0x58406F),
                        secondary: hex_color!(0x386e50),
                        surface: hex_color!(0xa2a4a3),
                        error: hex_color!(0xA13034),
                    },
                    bright: BrightColors {
                        primary: hex_color!(0xbd94f9),
                        secondary: hex_color!(0x49eb7a),
                        surface: hex_color!(0xf4f8f3),
                        error: hex_color!(0xE63E6D),
                    },
                },
            }
        }
    }

    #[cfg(not(target_os = "android"))]
    macro_rules! hex_color {
        ($hex:expr) => {
            color!($hex)
        };
    }

    #[cfg(target_os = "android")]
    macro_rules! hex_color {
        ($hex:expr) => {
            Color::from_hex($hex)
        };
    }

    pub(crate) use hex_color;
}

#[cfg(feature = "gui")]
pub use colors::*;

impl Theme {
    pub const ALL: [Self; 3] = [Self::Lupin, Self::Dark, Self::Light];
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Dark => "Dark",
                Self::Light => "Light",
                Self::Lupin => "Lupin",
            }
        )
    }
}
