use crossterm::{ClearType, Crossterm, Color};
use quick_error::*;

quick_error! {
    #[derive(Debug)]
    pub enum RutError {
        SystemCallFailure(call: String, code: u32, message: String) {
            display("System call {} failed with code: [{}] {}", call, code, message)
        }
        CrosstermFailure(cause: crossterm::ErrorKind) {
            cause(cause)
            display("Crossterm failure: {}", cause)
        }
        IOFailure(call: String, io_error: std::io::Error) {
            cause(io_error)
            display("I/O error when calling {}: {}", call, io_error)
        }
        UnsupportedTerminal(details: String) {
            display("Unsupported terminal: {}", details)
        }
    }
}

pub type Result<T> = std::result::Result<T, RutError>;

pub trait Region {
    fn height(&self) -> u16;
    fn width(&self) -> u16;

    fn clear(&mut self) -> Result<()> {
        self.fill(Color::Black)
    }

    fn fill(&mut self, color: Color) -> Result<()>;
    fn print(&mut self, rel_x: u16, rel_y: u16, background: Color, foreground: Color, text: &str) -> Result<()>;

    fn print_char(&mut self, rel_x: u16, rel_y: u16, background: Color, foreground: Color, char: char) -> Result<()> {
        let s = format!("{}", char);
        self.print(rel_x, rel_y, background, foreground, &s)
    }

    fn sub_region<'b>(&'b mut self, rel_x: u16, rel_y: u16, width: u16, height: u16) -> Result<Box<dyn Region + 'b>>;
}

impl Region for Crossterm {
    fn height(&self) -> u16 {
        let (_, h) = self.terminal().terminal_size();
        h
    }

    fn width(&self) -> u16 {
        let (w, _) = self.terminal().terminal_size();
        w
    }

    fn fill(&mut self, color: Color) -> Result<()> {
        self.color()
            .set_bg(color)
            .map_err(RutError::CrosstermFailure)?;
        self.terminal()
            .clear(ClearType::All)
            .map_err(RutError::CrosstermFailure)
    }

    fn print(&mut self, rel_x: u16, rel_y: u16, background: Color, foreground: Color, text: &str) -> Result<()> {
        self.color()
            .set_bg(background)
            .map_err(RutError::CrosstermFailure)?;
        self.color()
            .set_fg(foreground)
            .map_err(RutError::CrosstermFailure)?;
        self.cursor()
            .goto(rel_x, rel_y)
            .map_err(RutError::CrosstermFailure)?;
        self.terminal()
            .write(text)
            .map(|_| ())
            .map_err(RutError::CrosstermFailure)
    }

    fn sub_region<'b>(&'b mut self, rel_x: u16, rel_y: u16, width: u16, height: u16) -> Result<Box<dyn Region + 'b>> {
        Ok(Box::new(Subregion {
            console: self,
            x: rel_x,
            y: rel_y,
            width,
            height
        }))
    }
}

struct Subregion<'a> {
    console: &'a mut Crossterm,
    x: u16,
    y: u16,
    width: u16,
    height: u16
}

impl<'a> Region for Subregion<'a> {
    fn height(&self) -> u16 {
        self.height
    }

    fn width(&self) -> u16 {
        self.width
    }

    fn fill(&mut self, color: Color) -> Result<()> {
        let horizontal: String = vec![' '; self.width as usize].iter().collect();
        for y in 0..self.height {
            self.print(0, y, color, Color::White, &horizontal)?
        }
        Ok(())
    }

    fn print(&mut self, rel_x: u16, rel_y: u16, background: Color, foreground: Color, text: &str) -> Result<()> {
        self.console.print(self.x + rel_x, self.y + rel_y, background, foreground, text)
    }

    fn sub_region<'b>(&'b mut self, rel_x: u16, rel_y: u16, width: u16, height: u16) -> Result<Box<dyn Region + 'b>> {
        Ok(Box::new(Subregion {
            console: self.console,
            x: self.x + rel_x,
            y: self.y + rel_y,
            width,
            height
        }))
    }
}

pub enum FrameStyle {
    Single,
    SingleRounded,
    Dashed,
    Double,
}

enum FrameCharacter {
    Horizontal = 0,
    Vertical = 1,
    TopLeft = 2,
    TopRight = 3,
    BottomLeft = 4,
    BottomRight = 5,
}

pub fn frame_characters(style: FrameStyle) -> Vec<char> {
    match style {
        FrameStyle::Single => vec!['\u{2500}', '\u{2502}', '\u{250c}', '\u{2510}', '\u{2514}', '\u{2518}'],
        FrameStyle::SingleRounded => vec!['\u{2500}', '\u{2502}', '\u{256d}', '\u{256e}', '\u{2570}', '\u{256f}'],
        FrameStyle::Dashed => vec!['\u{2504}', '\u{250a}', '\u{250c}', '\u{2510}', '\u{2514}', '\u{2518}'],
        FrameStyle::Double => vec!['\u{2550}', '\u{2551}', '\u{2554}', '\u{2557}', '\u{255a}', '\u{255d}'],
    }
}

pub trait Frame {
    fn draw_frame(&mut self, background: Color, foreground: Color, style: FrameStyle) -> Result<()>;
    fn inside<'b>(&'b mut self) -> Result<Box<dyn Region + 'b>>;
}

impl<'a> Frame for Box<Region + 'a> {
    fn draw_frame(&mut self, background: Color, foreground: Color, style: FrameStyle) -> Result<()> {
        let chars = frame_characters(style);
        let width = self.width();
        let height = self.height();
        let horizontal: String = vec![chars[FrameCharacter::Horizontal as usize]; width as usize].iter().collect();
        self.print(0, 0, background, foreground, &horizontal)?;
        self.print(0, height - 1, background, foreground, &horizontal)?;

        for y in 1..(height - 1) {
            self.print_char(0, y, background, foreground, chars[FrameCharacter::Vertical as usize])?;
            self.print_char(width - 1, y, background, foreground, chars[FrameCharacter::Vertical as usize])?;
        }

        self.print_char(0, 0, background, foreground, chars[FrameCharacter::TopLeft as usize])?;
        self.print_char(width - 1, 0, background, foreground, chars[FrameCharacter::TopRight as usize])?;
        self.print_char(0, height - 1, background, foreground, chars[FrameCharacter::BottomLeft as usize])?;
        self.print_char(width - 1, height - 1, background, foreground, chars[FrameCharacter::BottomRight as usize])?;

        Ok(())
    }

    fn inside<'b>(&'b mut self) -> Result<Box<dyn Region + 'b>> {
        self.sub_region(1, 1, self.width()-2, self.height()-2)
    }
}
