use crate::HexColor;
use crossterm::{
    cursor, queue,
    style::{self, Stylize},
};
use std::io::{self, Write};

#[derive(Clone, Copy)]
pub enum BorderType {
    Solid,
    Dotted,
    Dashed,
    Double,
}

pub struct Border {
    visible: bool,
    padding: usize,
    width: usize,
    color: HexColor,
    border_type: BorderType,
    border_char: char,
}

impl Default for Border {
    fn default() -> Self {
        Self {
            visible: true,
            padding: 1,
            width: 1,
            color: HexColor::new("#FFFFFF"),
            border_type: BorderType::Solid,
            border_char: '█',
        }
    }
}

impl Border {
    pub fn new() -> BorderBuilder {
        BorderBuilder {
            border: Border::default(),
        }
    }

    pub fn render(&self, window_size: (usize, usize)) -> io::Result<()> {
        if !self.visible {
            return Ok(());
        }

        let (width, height) = window_size;

        for y in self.padding..(height - self.padding) {
            for x in self.padding..(width - self.padding) {
                if x == self.padding
                    || x == width - self.padding - 1
                    || y == self.padding
                    || y == height - self.padding - 1
                {
                    queue!(io::stdout(), cursor::MoveTo(x as u16, y as u16))?;
                    queue!(
                        io::stdout(),
                        style::PrintStyledContent(
                            style::style(self.border_char).with(self.color.to_color())
                        )
                    )?;
                }
            }
        }
        io::stdout().flush()?;
        Ok(())
    }
}

pub struct BorderBuilder {
    border: Border,
}

impl BorderBuilder {
    pub fn visible(mut self, visible: bool) -> Self {
        self.border.visible = visible;
        self
    }

    pub fn padding(mut self, padding: usize) -> Self {
        self.border.padding = padding;
        self
    }

    pub fn width(mut self, width: usize) -> Self {
        self.border.width = width;
        self
    }

    pub fn color(mut self, color: HexColor) -> Self {
        self.border.color = color;
        self
    }

    pub fn border_type(mut self, border_type: BorderType) -> Self {
        self.border.border_type = border_type;
        self
    }

    pub fn border_char(mut self, border_char: char) -> Self {
        self.border.border_char = border_char;
        self
    }

    pub fn build(self) -> Border {
        self.border
    }
}
