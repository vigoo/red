use nix::*;
use std::io;
use std::mem;
use std::os::unix::io::AsRawFd;
use std::error::Error;
use std::fs::File;

use crate::*;

pub fn guarded_io<S, T, F>(name: S, mut fun: F) -> crate::Result<T>
    where F: FnMut() -> io::Result<T>,
          S: AsRef<str> {

    match fun() {
        Ok(value) => Ok(value),
        Err(io_error) => Err(RutError::IOFailure(name.as_ref().to_string(), io_error))
    }
}

ioctl_read_bad!(tiocgwinsz, nix::libc::TIOCGWINSZ, nix::libc::winsize);

pub fn terminal_size(tty: &File) -> crate::Result<(i16, i16)> {
    unsafe {
        let mut size = mem::zeroed();
        match tiocgwinsz(tty.as_raw_fd(), &mut size) {
            Ok(_) => Ok((size.ws_col as i16, size.ws_row as i16)),
            Err(nix_error) => Err(RutError::SystemCallFailure(
                "tiocgwinsz".to_string(),
                nix_error.as_errno().map(|e| e as u32).unwrap_or(0),
                nix_error.description().to_string()))
        }
    }
}

pub fn fg_color_code(color: Color) -> u8 {
    match color {
        Color::Black => 30,
        Color::Blue => 31,
        Color::Green => 32,
        Color::Cyan => 33,
        Color::Red => 34,
        Color::Magenta => 35,
        Color::Brown => 36,
        Color::LightGray => 37,

        Color::DarkGray => 90,
        Color::LightBlue => 91,
        Color::LightGreen => 92,
        Color::LightCyan => 93,
        Color::LightRed => 94,
        Color::LightMagenta => 95,
        Color::Yellow => 96,
        Color::White => 97
    }
}

pub fn bg_color_code(color: Color) -> u8 {
    match color {
        Color::Black => 40,
        Color::Blue => 41,
        Color::Green => 42,
        Color::Cyan => 43,
        Color::Red => 44,
        Color::Magenta => 45,
        Color::Brown => 46,
        Color::LightGray => 47,

        Color::DarkGray => 100,
        Color::LightBlue => 101,
        Color::LightGreen => 102,
        Color::LightCyan => 103,
        Color::LightRed => 104,
        Color::LightMagenta => 105,
        Color::Yellow => 106,
        Color::White => 107
    }
}