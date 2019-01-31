use std::clone::Clone;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::env;

mod sequences;
mod utils;

use crate::*;
use crate::unix::sequences::*;
use crate::unix::utils::*;

pub struct UnixRegion<'a> {
    console: &'a mut UnixConsole,
    left: i16,
    top: i16,
    width: i16,
    height: i16,
}

impl<'a> Region for UnixRegion<'a> {
    fn height(&self) -> i16 {
        self.height
    }

    fn width(&self) -> i16 {
        self.width
    }

    fn fill(&mut self, color: Color) -> Result<()> {
        let row: String = vec!['.'; self.width as usize].iter().collect();

        for y in 0..self.height {
            let _ = self.print(0, y, color, self.console.current_foreground, &row)?;
        }

        let s = format!("{} {}", self.width, self.height);
        self.print(3, 5, color, Color::Black, &s);

        Ok(())
    }

    fn print(&mut self, rel_x: i16, rel_y: i16, background: Color, foreground: Color, text: &str) -> Result<()> {
        let _ = self.console.set_attributes(background, foreground)?;

        let mut x = self.left + rel_x;
        let mut y = self.top + rel_y;
        for ch in text.chars() {
            let _ = self.console.write_char(x, y, ch)?;

            if x == (self.left + self.width - 1) {
                x = self.left;
                y = y + 1;
            } else {
                x = x + 1;
            }
        }

        Ok(())
    }

    fn sub_region<'b>(&'b mut self, rel_x: i16, rel_y: i16, width: i16, height: i16) -> Result<Box<dyn Region + 'b>> {
        Ok(Box::new(
            UnixRegion {
                console: self.console,
                left: self.left + rel_x,
                top: self.top + rel_y,
                width,
                height,
            }
        ))
    }
}

#[derive(Debug)]
pub struct UnixConsole {
    tty: File,
    function_sequences: FunctionSequences,
    width: i16,
    height: i16,
    x: i16,
    y: i16,
    current_background: Color,
    current_foreground: Color,
}

impl UnixConsole {
    fn write_char(&mut self, x: i16, y: i16, ch: char) -> Result<()> {
        if (self.x != (x - 1)) || (self.y != y) {
            let _ = self.move_cursor(x, y)?;
        }

        let mut buf = [0; 8];
        ch.encode_utf8(&mut buf);
        let _ = guarded_io("write", || self.tty.write(&buf))?;
        if self.x == (self.width - 1) {
            self.x = 0;
            self.y = self.y + 1;
        } else {
            self.x = self.x + 1;
        }

        Ok(())
    }

    fn move_cursor(&mut self, x: i16, y: i16) -> Result<()> {
        let _ = guarded_io("write_fmt", || self.tty.write_fmt(format_args!("\x1b[{};{}H", y+1, x+1)))?;
        self.x = x;
        self.y = y;
        Ok(())
    }

    fn set_attributes(&mut self, background: Color, foreground: Color) -> Result<()> {
        if (self.current_background != background) || (self.current_foreground != foreground) {
            let _ = guarded_io(
                "write_fmt",
                || self.tty.write_fmt(format_args!("\x1b[{};{}m", fg_color_code(foreground), bg_color_code(background))))?;

            self.current_background = background;
            self.current_foreground = foreground;
        }

        Ok(())
    }
}

pub fn create_console() -> Result<UnixConsole> {
    let tty = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/tty")
        .unwrap();

    let function_sequences = match env::var("TERM") {
        Ok(term_name) => {
            TERM_MAPPING
                .iter()
                .find(|(name, _)| *name == term_name)
                .or(TERM_MAPPING.iter()
                    .find(|(name, _)| term_name.contains(*name)))
                .map(|(_, &funs)| funs)
                .ok_or({
                    let invalid_msg = format!("Unsupported terminal type: {}", term_name);
                    RutError::UnsupportedTerminal(invalid_msg)
                })
        }
        Err(_) => Err(RutError::UnsupportedTerminal("TERM environment variable is not defined".to_string()))
    }?;

    let (width, height) = terminal_size(&tty)?;

    let mut console = UnixConsole {
        tty,
        function_sequences: function_sequences.clone(),
        width,
        height,
        x: 0,
        y: 0,
        current_foreground: Color::Black,
        current_background: Color::White
    };

    let _ = console.move_cursor(0, 0)?;
    let _ = console.set_attributes(Color::Black, Color::LightGray)?;

    Ok(console)
}

#[cfg(not(target_os = "windows"))]
impl Console for UnixConsole {
    fn clear(&mut self) -> Result<()> {
        let _ = guarded_io("write", || self.tty.write(self.function_sequences.clear_screen.as_bytes()))?;
        Ok(())
    }

    fn get_next_event(&mut self) -> Result<Event> {
        // TODO
        Ok(Event::Resize {
            width: self.width,
            height: self.height,
        })
    }

    fn full_screen<'b>(&'b mut self) -> Result<Box<dyn Region + 'b>> {
        let width = self.width;
        let height = self.height;
        Ok(Box::new(UnixRegion {
            console: self,
            left: 0,
            top: 0,
            width,
            height,
        }))
    }
}
