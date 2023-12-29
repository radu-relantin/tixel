pub enum TextAlignment {
    Left,
    Center,
    Right,
}

pub struct FontStyle {
    bold: bool,
    italic: bool,
    underline: bool,
}

impl FontStyle {
    pub fn new(bold: bool, italic: bool, underline: bool) -> Self {
        Self {
            bold,
            italic,
            underline,
        }
    }
}
