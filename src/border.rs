use std::io::{self, Write};

use crossterm::{
    cursor, queue,
    style::{self},
};

use crate::HexColor;

// Define an enumeration of different border types
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BorderType {
    Solid,
    Dotted,
    Dashed,
    Double,
}

// Define the properties and structure of a border
pub struct Border {
    visible: bool,                    // Indicates if the border is visible
    padding: usize,                   // Padding around the border
    width: usize,                     // Width of the border
    color: HexColor,                  // Color of the border
    border_type: BorderType,          // Type of the border
    decoration_lines: DecorationLine, // Decoration lines for rendering the border
}

// Define the decoration lines for rendering the border
pub struct DecorationLine {
    omni_char: char,                // Character used to render the border
    vertical_char: Vec<char>,       // Vertical character for rendering
    horizontal_char: char,          // Horizontal character for rendering
    top_right_corner_char: char,    // Character for the top right corner of the border
    top_left_corner_char: char,     // Character for the top left corner of the border
    bottom_right_corner_char: char, // Character for the bottom right corner of the border
    bottom_left_corner_char: char,  // Character for the bottom left corner of the border
}

// Implement the default values for the Border structure
impl Default for Border {
    fn default() -> Self {
        Self {
            visible: true,
            padding: 0,
            width: 1,
            color: HexColor::new("#FFFFFF"), // Default color set to white
            border_type: BorderType::Solid,  // Default border type set to solid
            decoration_lines: DecorationLine {
                omni_char: '\0',
                vertical_char: vec!['│'; 1],
                horizontal_char: '─',
                top_right_corner_char: '┐',
                top_left_corner_char: '┌',
                bottom_right_corner_char: '┘',
                bottom_left_corner_char: '└',
            },
        }
    }
}

impl Border {
    pub fn new() -> BorderBuilder {
        BorderBuilder {
            border: Border::default(),
        }
    }
    fn render_vertical_border(
        &self,
        handle: &mut io::StdoutLock,
        y_axis: u16,
        start_x: u16,
        border_char: char,
    ) -> Result<(), io::Error> {
        queue!(handle, cursor::MoveTo(start_x, y_axis))?;
        queue!(handle, style::SetForegroundColor(self.color.to_rgb()))?;
        queue!(handle, style::Print(border_char))?;
        queue!(handle, style::SetForegroundColor(style::Color::Reset))?;
        Ok(())
    }

    fn render_left_vertical_border(
        &self,
        handle: &mut io::StdoutLock,
        window_size: (usize, usize),
    ) -> Result<(), io::Error> {
        let (_, height) = window_size;

        for layer in 0..self.width {
            let y_start = self.padding as u16 + layer as u16 + 1;
            let y_end = height as u16 - self.padding as u16 - layer as u16 - 1;
            let x_axis = self.padding as u16 + layer as u16;

            if y_start >= y_end {
                break;
            }

            for y_axis in y_start..y_end {
                let border_char = self
                    .decoration_lines
                    .vertical_char
                    .get(layer)
                    .copied()
                    .unwrap_or('│');
                self.render_vertical_border(handle, y_axis, x_axis, border_char)?;
            }
        }
        Ok(())
    }

    fn render_right_vertical_border(
        &self,
        handle: &mut io::StdoutLock,
        window_size: (usize, usize),
    ) -> Result<(), io::Error> {
        let (width, height) = window_size;

        for layer in 0..self.width {
            let y_start = self.padding as u16 + layer as u16 + 1;
            let y_end = height as u16 - self.padding as u16 - layer as u16 - 1;
            let x_axis = width as u16 - self.padding as u16 - 1 - layer as u16;

            if y_start >= y_end {
                break;
            }

            let border_char = match self.decoration_lines.vertical_char.get(layer) {
                Some(&c) => c,
                None => self.decoration_lines.vertical_char[0],
            };

            for y_axis in y_start..y_end {
                self.render_vertical_border(handle, y_axis, x_axis, border_char)?;
            }
        }
        Ok(())
    }

    fn render_vertical_borders(
        &self,
        handle: &mut io::StdoutLock,
        window_size: (usize, usize),
    ) -> Result<(), io::Error> {
        self.render_left_vertical_border(handle, window_size)?;
        self.render_right_vertical_border(handle, window_size)?;
        Ok(())
    }

    pub fn build_vertical_borders(&self, window_size: (usize, usize)) -> Result<(), io::Error> {
        if self.width == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Border width cannot be 0",
            ));
        }

        if self.decoration_lines.omni_char != '\0' || self.decoration_lines.vertical_char.is_empty()
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Appropriate vertical character not provided",
            ));
        }

        let stdout = io::stdout();
        let mut handle = stdout.lock();

        self.render_vertical_borders(&mut handle, window_size)?;

        handle.flush()?;
        Ok(())
    }

    pub fn build_horizontal_borders(&self, window_size: (usize, usize)) -> Result<(), io::Error> {
        if self.decoration_lines.omni_char != '\0' {
            return Ok(());
        } else if self.decoration_lines.horizontal_char == '\0' {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Horizontal character not provided",
            ));
        }

        let (width, height) = (window_size.0, window_size.1);
        let stdout = io::stdout();
        let mut handle = stdout.lock();

        for x_axis in self.padding + 1..width - self.padding - 1 {
            self.render_horizontal_border(
                &mut handle,
                x_axis as u16,
                self.padding as u16,
                height - self.padding - 1,
            )?;
        }
        handle.flush()?;
        Ok(())
    }

    pub fn build_omni_char_border(&self, window_size: (usize, usize)) -> Result<(), io::Error> {
        if self.decoration_lines.omni_char == '\0' {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Omni character not provided",
            ));
        }

        let (width, height) = (window_size.0, window_size.1);
        let stdout = io::stdout();
        let mut handle = stdout.lock();

        for x_axis in 0..width {
            queue!(handle, cursor::MoveTo(x_axis as u16, 0))?;
            queue!(handle, style::Print(self.decoration_lines.omni_char))?;
            queue!(handle, cursor::MoveTo(x_axis as u16, (height - 1) as u16))?;
            queue!(handle, style::Print(self.decoration_lines.omni_char))?;
        }

        for y_axis in 1..(height - 1) {
            queue!(handle, cursor::MoveTo(0, y_axis as u16))?;
            queue!(handle, style::Print(self.decoration_lines.omni_char))?;
            queue!(handle, cursor::MoveTo((width - 1) as u16, y_axis as u16))?;
            queue!(handle, style::Print(self.decoration_lines.omni_char))?;
        }

        handle.flush()?;
        Ok(())
    }

    pub fn build_corner_borders(&self, window_size: (usize, usize)) -> Result<(), io::Error> {
        let (width, height) = (window_size.0, window_size.1);
        let stdout = io::stdout();
        let mut handle = stdout.lock();

        self.render_corner_char(
            &mut handle,
            self.padding,
            self.padding,
            self.decoration_lines.top_left_corner_char,
        )?;
        self.render_corner_char(
            &mut handle,
            width - self.padding - 1,
            self.padding,
            self.decoration_lines.top_right_corner_char,
        )?;
        self.render_corner_char(
            &mut handle,
            self.padding,
            height - self.padding - 1,
            self.decoration_lines.bottom_left_corner_char,
        )?;
        self.render_corner_char(
            &mut handle,
            width - self.padding - 1,
            height - self.padding - 1,
            self.decoration_lines.bottom_right_corner_char,
        )?;

        handle.flush()?;
        Ok(())
    }

    pub fn build_border(&self, window_size: (usize, usize)) -> Result<(), io::Error> {
        self.build_vertical_borders(window_size)?;
        self.build_horizontal_borders(window_size)?;
        self.build_corner_borders(window_size)?;
        Ok(())
    }

    fn render_horizontal_border(
        &self,
        handle: &mut io::StdoutLock,
        x_axis: u16,
        y_axis: u16,
        bottom_y: usize,
    ) -> Result<(), io::Error> {
        queue!(handle, cursor::MoveTo(x_axis, y_axis))?;
        queue!(handle, style::SetForegroundColor(self.color.to_rgb()))?;
        queue!(handle, style::Print(self.decoration_lines.horizontal_char))?;
        queue!(handle, style::SetForegroundColor(style::Color::Reset))?;
        queue!(handle, cursor::MoveTo(x_axis, bottom_y as u16))?;
        queue!(handle, style::SetForegroundColor(self.color.to_rgb()))?;
        queue!(handle, style::Print(self.decoration_lines.horizontal_char))?;
        queue!(handle, style::SetForegroundColor(style::Color::Reset))?;
        Ok(())
    }

    fn render_corner_char(
        &self,
        handle: &mut io::StdoutLock,
        x_axis: usize,
        y_axis: usize,
        char: char,
    ) -> Result<(), io::Error> {
        queue!(handle, cursor::MoveTo(x_axis as u16, y_axis as u16))?;
        queue!(handle, style::SetForegroundColor(self.color.to_rgb()))?;
        queue!(handle, style::Print(char))?;
        queue!(handle, style::SetForegroundColor(style::Color::Reset))?;
        Ok(())
    }

    fn check_current_position_is_padding(&self, x_axis: usize, y_axis: usize) -> bool {
        (x_axis < self.padding)
            || (y_axis < self.padding)
            || (x_axis >= self.width - self.padding)
            || (y_axis >= self.width - self.padding)
    }

    fn should_render_border(&self, x_axis: usize, y_axis: usize) -> bool {
        !self.check_current_position_is_padding(x_axis, y_axis) && self.visible
    }

    fn check_border_type(&self) -> BorderType {
        self.border_type
    }
}

pub struct BorderBuilder {
    border: Border,
}

impl BorderBuilder {
    pub fn new() -> Self {
        BorderBuilder {
            border: Border::default(),
        }
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

    pub fn build(self) -> Border {
        self.border
    }
}
