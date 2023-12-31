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
    padding: usize,                   // Padding between the edge and the 1st border layer
    width: usize,                     // Width of the border
    color: HexColor,                  // Color of the border
    border_type: BorderType,          // Type of the border
    decoration_lines: DecorationLine, // Decoration lines for rendering the border
    border_colors: Vec<HexColor>,     // Store multiple colors for different border layers
}

// Define the decoration lines for rendering the border
pub struct DecorationLine {
    omni_char: char,                     // Character used to render the border
    vertical_char: Vec<char>,            // Vertical character for rendering
    horizontal_char: Vec<char>,          // Horizontal character for rendering
    top_right_corner_char: Vec<char>,    // Character for the top right corner of the border
    top_left_corner_char: Vec<char>,     // Character for the top left corner of the border
    bottom_right_corner_char: Vec<char>, // Character for the bottom right corner of the border
    bottom_left_corner_char: Vec<char>,  // Character for the bottom left corner of the border
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
            border_colors: vec![HexColor::new("#FFFFFF")],
            decoration_lines: DecorationLine {
                omni_char: '\0',
                vertical_char: vec![Self::default_vertical_border_char(BorderType::Solid)],
                horizontal_char: vec![Self::default_horizontal_border_char(BorderType::Solid)],
                top_right_corner_char: vec!['┐'; 1],
                top_left_corner_char: vec!['┌'; 1],
                bottom_right_corner_char: vec!['┘'; 1],
                bottom_left_corner_char: vec!['└'; 1],
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

    fn get_border_color(&self, layer: usize) -> HexColor {
        self.border_colors.get(layer).cloned().unwrap_or_else(|| {
            self.border_colors
                .last()
                .cloned()
                .unwrap_or_else(|| HexColor::new("#FFFFFF"))
        })
    }

    fn default_vertical_border_char(border_type: BorderType) -> char {
        match border_type {
            BorderType::Solid => '│',
            BorderType::Dotted => '┆',
            BorderType::Dashed => '┊',
            BorderType::Double => '║',
        }
    }

    fn default_horizontal_border_char(border_type: BorderType) -> char {
        match border_type {
            BorderType::Solid => '─',
            BorderType::Dotted => '┄',
            BorderType::Dashed => '┈',
            BorderType::Double => '═',
        }
    }

    fn render_vertical_border(
        &self,
        handle: &mut io::StdoutLock,
        y_axis: u16,
        start_x: u16,
        layer: usize,
    ) -> Result<(), io::Error> {
        let border_char = self
            .decoration_lines
            .vertical_char
            .get(layer)
            .copied()
            .unwrap_or_else(|| Self::default_vertical_border_char(self.border_type));

        queue!(handle, cursor::MoveTo(start_x, y_axis))?;
        queue!(
            handle,
            style::SetForegroundColor(self.get_border_color(layer).to_rgb())
        )?;
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
                self.render_vertical_border(handle, y_axis, x_axis, layer)?;
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

            for y_axis in y_start..y_end {
                self.render_vertical_border(handle, y_axis, x_axis, layer)?;
            }
        }
        Ok(())
    }

    pub fn render_vertical_borders(&self, window_size: (usize, usize)) -> Result<(), io::Error> {
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

        self.render_left_vertical_border(&mut handle, window_size)?;
        self.render_right_vertical_border(&mut handle, window_size)?;

        handle.flush()?;
        Ok(())
    }

    fn render_top_horizontal_border(
        &self,
        handle: &mut io::StdoutLock,
        window_size: (usize, usize),
    ) -> Result<(), io::Error> {
        let (width, _) = window_size;

        for layer in 0..self.width {
            let x_start = self.padding as u16 + layer as u16 + 1;
            let x_end = width as u16 - self.padding as u16 - layer as u16 - 1;
            let y_axis = self.padding as u16 + layer as u16;

            if x_start >= x_end {
                break;
            }

            for x_axis in x_start..x_end {
                self.render_horizontal_border(handle, x_axis, y_axis, layer)?;
            }
        }
        Ok(())
    }

    fn render_bottom_horizontal_border(
        &self,
        handle: &mut io::StdoutLock,
        window_size: (usize, usize),
    ) -> Result<(), io::Error> {
        let (width, height) = window_size;

        for layer in 0..self.width {
            let x_start = self.padding as u16 + layer as u16 + 1;
            let x_end = width as u16 - self.padding as u16 - layer as u16 - 1;
            let y_axis = height as u16 - self.padding as u16 - 1 - layer as u16;

            if x_start >= x_end {
                break;
            }

            for x_axis in x_start..x_end {
                self.render_horizontal_border(handle, x_axis, y_axis, layer)?;
            }
        }
        Ok(())
    }

    pub fn render_horizontal_borders(&self, window_size: (usize, usize)) -> Result<(), io::Error> {
        if self.width == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Border width cannot be 0",
            ));
        }

        if self.decoration_lines.omni_char != '\0'
            || self.decoration_lines.horizontal_char.is_empty()
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Appropriate horizontal character not provided",
            ));
        }

        let stdout = io::stdout();
        let mut handle = stdout.lock();

        self.render_top_horizontal_border(&mut handle, window_size)?;
        self.render_bottom_horizontal_border(&mut handle, window_size)?;

        handle.flush()?;
        Ok(())
    }

    fn render_corner(
        &self,
        handle: &mut io::StdoutLock,
        x_axis: u16,
        y_axis: u16,
        corner_char: char,
        layer: usize,
    ) -> Result<(), io::Error> {
        queue!(handle, cursor::MoveTo(x_axis, y_axis))?;
        queue!(
            handle,
            style::SetForegroundColor(self.get_border_color(layer).to_rgb())
        )?;
        queue!(handle, style::Print(corner_char))?;
        queue!(handle, style::SetForegroundColor(style::Color::Reset))?;
        Ok(())
    }

    fn render_corners(&self, window_size: (usize, usize)) -> Result<(), io::Error> {
        let (width, height) = window_size;

        let stdout = io::stdout();
        let mut handle = stdout.lock();

        for layer in 0..self.width {
            let top_y = self.padding as u16 + layer as u16;
            let bottom_y = height as u16 - self.padding as u16 - 1 - layer as u16;
            let left_x = self.padding as u16 + layer as u16;
            let right_x = width as u16 - self.padding as u16 - 1 - layer as u16;

            if top_y >= height as u16
                || bottom_y >= height as u16
                || left_x >= width as u16
                || right_x >= width as u16
            {
                break;
            }

            let top_left_char = self
                .decoration_lines
                .top_left_corner_char
                .get(layer)
                .copied()
                .unwrap_or('┌');
            let top_right_char = self
                .decoration_lines
                .top_right_corner_char
                .get(layer)
                .copied()
                .unwrap_or('┐');
            let bottom_left_char = self
                .decoration_lines
                .bottom_left_corner_char
                .get(layer)
                .copied()
                .unwrap_or('└');
            let bottom_right_char = self
                .decoration_lines
                .bottom_right_corner_char
                .get(layer)
                .copied()
                .unwrap_or('┘');

            self.render_corner(&mut handle, left_x, top_y, top_left_char, layer)?;
            self.render_corner(&mut handle, right_x, top_y, top_right_char, layer)?;
            self.render_corner(&mut handle, left_x, bottom_y, bottom_left_char, layer)?;
            self.render_corner(&mut handle, right_x, bottom_y, bottom_right_char, layer)?;
        }

        handle.flush()?;
        Ok(())
    }

    pub fn render_box(&self, window_size: (usize, usize)) -> Result<(), io::Error> {
        self.render_vertical_borders(window_size)?;
        self.render_horizontal_borders(window_size)?;
        self.render_corners(window_size)?;
        Ok(())
    }

    fn render_horizontal_border(
        &self,
        handle: &mut io::StdoutLock,
        x_axis: u16,
        start_y: u16,
        layer: usize,
    ) -> Result<(), io::Error> {
        let border_char = self
            .decoration_lines
            .horizontal_char
            .get(layer)
            .copied()
            .unwrap_or_else(|| Self::default_horizontal_border_char(self.border_type));

        queue!(handle, cursor::MoveTo(x_axis, start_y))?;
        queue!(
            handle,
            style::SetForegroundColor(self.get_border_color(layer).to_rgb())
        )?;
        queue!(handle, style::Print(border_char))?;
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
        self.border.decoration_lines.vertical_char =
            vec![Border::default_vertical_border_char(border_type); self.border.width];
        self.border.decoration_lines.horizontal_char =
            vec![Border::default_horizontal_border_char(border_type); self.border.width];
        self
    }

    pub fn with_color(mut self, color: HexColor) -> Self {
        self.border.border_colors = vec![color];
        self
    }

    pub fn with_colors(mut self, colors: Vec<HexColor>) -> Self {
        if colors.len() > self.border.width {
            panic!("Number of colors provided exceeds border width");
        }
        self.border.border_colors = colors;
        self
    }

    pub fn border_char(mut self, border_char: char) -> Self {
        self.border.decoration_lines.vertical_char = vec![border_char];
        self
    }

    pub fn vertical_border_char(mut self, chars: Vec<char>) -> Self {
        // NOTE: Must be equal to the border width
        self.border.decoration_lines.vertical_char = chars;
        self
    }

    pub fn horizontal_border_char(mut self, chars: Vec<char>) -> Self {
        // NOTE: Must be equal to the border width
        self.border.decoration_lines.horizontal_char = chars;
        self
    }

    pub fn build(self) -> Border {
        self.border
    }
}
