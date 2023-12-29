pub struct HexColor {
    code: String,
}

impl HexColor {
    pub fn new(code: &str) -> Self {
        Self {
            code: String::from(code),
        }
    }

    // Convert HexColor to crossterm's Color
    pub fn to_color(&self) -> crossterm::style::Color {
        // Assuming the HexColor is in the format '#RRGGBB'
        let r = u8::from_str_radix(&self.code[1..3], 16).unwrap_or(0);
        let g = u8::from_str_radix(&self.code[3..5], 16).unwrap_or(0);
        let b = u8::from_str_radix(&self.code[5..7], 16).unwrap_or(0);
        crossterm::style::Color::Rgb { r, g, b }
    }
}
