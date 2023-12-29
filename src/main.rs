mod border;
mod color;
mod font;

use crossterm::cursor;
use crossterm::execute;
use crossterm::terminal;
use std::io::{self};

use crate::border::{Border, BorderType};
use crate::color::HexColor;
use crate::font::{FontStyle, TextAlignment};

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Unable to disable raw mode");
        execute!(io::stdout(), terminal::Clear(terminal::ClearType::All))
            .expect("Unable to clear the terminal");
        execute!(io::stdout(), cursor::MoveTo(0, 0)).expect("Unable to move the cursor");
    }
}

struct BaseLayer {
    window_size: (usize, usize),
    background_color: HexColor,
    foreground_color: HexColor,
    border: Border,
    title: Option<String>,
    cursor_visibility: bool,
    default_cursor_position: (usize, usize),
    text_alignment: TextAlignment,
    font_style: FontStyle,
}

impl BaseLayer {
    fn new() -> Result<Self, io::Error> {
        let window_size = Self::get_window_size()?;
        Ok(Self {
            window_size,
            background_color: HexColor::new("#000000"),
            foreground_color: HexColor::new("#FFFFFF"),
            border: Border::new()
                .visible(true)
                .padding(1)
                .width(1)
                .color(HexColor::new("#FF0BB0"))
                .border_type(BorderType::Dotted)
                .border_char('X')
                .build(),
            title: None,
            cursor_visibility: true,
            default_cursor_position: (0, 0),
            text_alignment: TextAlignment::Left,
            font_style: FontStyle::new(false, false, false),
        })
    }

    fn get_window_size() -> io::Result<(usize, usize)> {
        terminal::size().map(|(w, h)| (w as usize, h as usize))
    }
}

fn main() {
    let _clean_up = CleanUp; // Assign the CleanUp instance to _clean_up
    terminal::enable_raw_mode().expect("Unable to enable raw mode");
    execute!(io::stdout(), terminal::EnterAlternateScreen)
        .expect("Unable to enter alternate screen");

    // Create a new BaseLayer instance
    match BaseLayer::new() {
        Ok(base_layer) => {
            // Render a border around the window
            base_layer
                .border
                .render(base_layer.window_size, true)
                .expect("Failed to render border");

            std::thread::sleep(std::time::Duration::from_secs(5));
            // Print the window size
            println!(
                "Window size: {} columns, {} rows",
                base_layer.window_size.0, base_layer.window_size.1
            );
        }
        Err(e) => {
            eprintln!("Failed to initialize Base Layer: {}", e);
        }
    }

    execute!(io::stdout(), terminal::LeaveAlternateScreen)
        .expect("Unable to leave alternate screen");
}
