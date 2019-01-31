use quick_error::*;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(not(target_os = "windows"))]
mod unix;

quick_error! {
    #[derive(Debug)]
    pub enum RutError {
        SystemCallFailure(call: String, code: u32, message: String) {
            display("System call {} failed with code: [{}] {}", call, code, message)
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

#[derive(Debug, Copy, Clone)]
pub enum Key {
    CharKey(char),
    Backspace,
    Tab,
    Enter,
    Shift,
    CapsLock,
    NumLock,
    Escape,
    PageUp,
    PageDown,
    Home,
    End,
    Delete,
    Insert,
    Left,
    Right,
    Up,
    Down,
    PrintScreen,
    Function(u8),
}

#[derive(Debug, Copy, Clone)]
pub struct ControlKeyStates {
    capslock: bool,
    numlock: bool,
    scrollock: bool,

    shift: bool,
    left_alt: bool,
    right_alt: bool,
    left_ctrl: bool,
    right_ctrl: bool,
}

impl ControlKeyStates {
    pub fn alt(self) -> bool { self.left_alt || self.right_alt }
    pub fn ctrl(self) -> bool { self.left_ctrl || self.right_ctrl }
}

#[derive(Debug, Copy, Clone)]
pub enum Event {
    KeyPressed { key: Key, control_key_state: ControlKeyStates },
    KeyReleased { key: Key, control_key_state: ControlKeyStates },
    MouseButtonStateChange { x: i16, y: i16, left_button: bool, right_button: bool, control_key_state: ControlKeyStates },
    MouseDoubleClick { x: i16, y: i16, left_button: bool, right_button: bool, control_key_state: ControlKeyStates },
    MouseHorizontalWheel { x: i16, y: i16, amount: i16, control_key_state: ControlKeyStates },
    MouseWheel { x: i16, y: i16, amount: i16, control_key_state: ControlKeyStates },
    MouseMove { x: i16, y: i16, control_key_state: ControlKeyStates },
    Resize { width: i16, height: i16 },
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum Color {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGray,

    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    LightMagenta,
    Yellow,
    White,
}

pub trait Region {
    fn height(&self) -> i16;
    fn width(&self) -> i16;

    fn clear(&mut self) -> Result<()> {
        self.fill(Color::Black)
    }

    fn fill(&mut self, color: Color) -> Result<()>;
    fn print(&mut self, rel_x: i16, rel_y: i16, background: Color, foreground: Color, text: &str) -> Result<()>;

    fn print_char(&mut self, rel_x: i16, rel_y: i16, background: Color, foreground: Color, char: char) -> Result<()> {
        let s = format!("{}", char);
        self.print(rel_x, rel_y, background, foreground, &s)
    }

    fn sub_region<'b>(&'b mut self, rel_x: i16, rel_y: i16, width: i16, height: i16) -> Result<Box<dyn Region + 'b>>;
}

pub trait Console {
    fn clear(&mut self) -> Result<()>;
    fn get_next_event(&mut self) -> Result<Event>;

    fn full_screen<'b>(&'b mut self) -> Result<Box<dyn Region + 'b>>;
}

#[cfg(target_os = "windows")]
pub use windows::create_console;

#[cfg(not(target_os = "windows"))]
pub use unix::create_console;

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
}
