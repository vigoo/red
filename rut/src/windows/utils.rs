use std::ffi::OsString;
use std::os::windows::prelude::*;
use std::ptr;
use winapi::shared::minwindef::{BOOL, DWORD, WORD};
use winapi::um::consoleapi;
use winapi::um::errhandlingapi;
use winapi::um::winbase;
use winapi::um::wincon;
use winapi::um::winnt;

use crate::{Color, ControlKeyStates, Key, Result};

fn get_last_error() -> (DWORD, String) {
    unsafe {
        let code = errhandlingapi::GetLastError();
        let mut buffer = [0 as winnt::WCHAR; 2048];
        let _ = winbase::FormatMessageW(
            winbase::FORMAT_MESSAGE_FROM_SYSTEM | winbase::FORMAT_MESSAGE_IGNORE_INSERTS,
            ptr::null(),
            code,
            winnt::MAKELANGID(winnt::LANG_NEUTRAL as u16, winnt::SUBLANG_DEFAULT as u16) as u32,
            buffer.as_mut_ptr(),
            buffer.len() as DWORD,
            ptr::null_mut()
        );

        let message = OsString::from_wide(&buffer).into_string().unwrap_or("?".to_string());

        (code, message)
    }
}

pub fn guarded_call<S, T, F>(name: S, fun: F) -> crate::Result<T>
    where F: Fn() -> (BOOL, T),
          S: AsRef<str> {

    match fun() {
        (0, _) => {
            let (code, message) = get_last_error();
            Err(crate::RutError::SystemCallFailure(name.as_ref().to_string(), code, message))
        },
        (_, result) => Ok(result)
    }
}

pub fn get_control_key_states(flags: DWORD) -> ControlKeyStates {
    ControlKeyStates {
        capslock: (flags & wincon::CAPSLOCK_ON) != 0,
        numlock: (flags & wincon::NUMLOCK_ON) != 0,
        scrollock: (flags & wincon::SCROLLLOCK_ON) != 0,

        shift: (flags & wincon::SHIFT_PRESSED) != 0,
        left_alt: (flags & wincon::LEFT_ALT_PRESSED) != 0,
        right_alt: (flags & wincon::RIGHT_ALT_PRESSED) != 0,
        left_ctrl: (flags & wincon::LEFT_CTRL_PRESSED) != 0,
        right_ctrl: (flags & wincon::RIGHT_CTRL_PRESSED) != 0,
    }
}

pub fn get_key(event: &wincon::KEY_EVENT_RECORD) -> Option<Key> {
    use winapi::um::winuser::*;

    match event.wVirtualKeyCode as i32 {
        VK_BACK => Some(Key::Backspace),
        VK_TAB => Some(Key::Tab),
        VK_RETURN => Some(Key::Enter),
        VK_SHIFT => Some(Key::Shift),
        VK_CAPITAL => Some(Key::CapsLock),
        VK_NUMLOCK => Some(Key::NumLock),
        VK_ESCAPE => Some(Key::Escape),
        VK_PRIOR => Some(Key::PageUp),
        VK_NEXT => Some(Key::PageDown),
        VK_HOME => Some(Key::Home),
        VK_END => Some(Key::End),
        VK_DELETE => Some(Key::Delete),
        VK_INSERT => Some(Key::Insert),
        VK_LEFT => Some(Key::Left),
        VK_RIGHT => Some(Key::Right),
        VK_UP => Some(Key::Up),
        VK_DOWN => Some(Key::Down),
        VK_SNAPSHOT => Some(Key::PrintScreen),
        VK_F1 => Some(Key::Function(1)),
        VK_F2 => Some(Key::Function(2)),
        VK_F3 => Some(Key::Function(3)),
        VK_F4 => Some(Key::Function(4)),
        VK_F5 => Some(Key::Function(5)),
        VK_F6 => Some(Key::Function(6)),
        VK_F7 => Some(Key::Function(7)),
        VK_F8 => Some(Key::Function(8)),
        VK_F9 => Some(Key::Function(9)),
        VK_F10 => Some(Key::Function(10)),
        VK_F11 => Some(Key::Function(11)),
        VK_F12 => Some(Key::Function(12)),
        VK_F13 => Some(Key::Function(13)),
        VK_F14 => Some(Key::Function(14)),
        VK_F15 => Some(Key::Function(15)),
        VK_F16 => Some(Key::Function(16)),
        VK_F17 => Some(Key::Function(17)),
        VK_F18 => Some(Key::Function(18)),
        VK_F19 => Some(Key::Function(19)),
        VK_F20 => Some(Key::Function(20)),
        VK_F21 => Some(Key::Function(21)),
        VK_F22 => Some(Key::Function(22)),
        VK_F23 => Some(Key::Function(23)),
        VK_F24 => Some(Key::Function(24)),

        _ => {
            let u16_ch: &u16 = unsafe { event.uChar.UnicodeChar() };

            if *u16_ch == 0 {
                None
            } else {
                let ch =
                    if event.dwControlKeyState &
                        (wincon::LEFT_CTRL_PRESSED | wincon::RIGHT_CTRL_PRESSED |
                         wincon::LEFT_ALT_PRESSED | wincon::RIGHT_ALT_PRESSED) != 0 {

                        if (('A' as u16) <= event.wVirtualKeyCode) && (event.wVirtualKeyCode <= ('Z' as u16)) {
                            if (event.dwControlKeyState & wincon::SHIFT_PRESSED) != 0 {
                                event.wVirtualKeyCode as u16
                            } else {
                                event.wVirtualKeyCode as u16 + (('a' as u16) - ('A' as u16))
                            }
                        }  else {
                            *u16_ch
                        }
                    } else {
                        *u16_ch
                    };

                let os_str: OsString = OsString::from_wide(&[ch]);

                os_str
                    .into_string()
                    .ok()
                    .and_then(|s| s.chars().next())
                    .map(|ch| Key::CharKey(ch))
            }
        }
    }
}

pub fn color_to_background_attributes(color: Color) -> WORD {
    match color {
        Color::Black => 0,
        Color::Blue => wincon::BACKGROUND_BLUE,
        Color::Green => wincon::BACKGROUND_GREEN,
        Color::Cyan => wincon::BACKGROUND_GREEN | wincon::BACKGROUND_BLUE,
        Color::Red => wincon::BACKGROUND_RED,
        Color::Magenta => wincon::BACKGROUND_RED | wincon::BACKGROUND_BLUE,
        Color::Brown => wincon::BACKGROUND_RED | wincon::BACKGROUND_GREEN,
        Color::LightGray => wincon::BACKGROUND_RED | wincon::BACKGROUND_GREEN | wincon::BACKGROUND_BLUE,

        Color::DarkGray => wincon::BACKGROUND_INTENSITY,
        Color::LightBlue => wincon::BACKGROUND_BLUE | wincon::BACKGROUND_INTENSITY,
        Color::LightGreen => wincon::BACKGROUND_GREEN | wincon::BACKGROUND_INTENSITY,
        Color::LightCyan => wincon::BACKGROUND_GREEN | wincon::BACKGROUND_BLUE | wincon::BACKGROUND_INTENSITY,
        Color::LightRed => wincon::BACKGROUND_RED | wincon::BACKGROUND_BLUE | wincon::BACKGROUND_INTENSITY,
        Color::LightMagenta => wincon::BACKGROUND_RED | wincon::BACKGROUND_GREEN | wincon::BACKGROUND_INTENSITY,
        Color::Yellow => wincon::BACKGROUND_RED | wincon::BACKGROUND_GREEN | wincon::BACKGROUND_INTENSITY,
        Color::White => wincon::BACKGROUND_RED | wincon::BACKGROUND_GREEN | wincon::BACKGROUND_BLUE | wincon::BACKGROUND_INTENSITY
    }
}

pub fn color_to_foreground_attributes(color: Color) -> WORD {
    match color {
        Color::Black => 0,
        Color::Blue => wincon::FOREGROUND_BLUE,
        Color::Green => wincon::FOREGROUND_GREEN,
        Color::Cyan => wincon::FOREGROUND_GREEN | wincon::FOREGROUND_BLUE,
        Color::Red => wincon::FOREGROUND_RED,
        Color::Magenta => wincon::FOREGROUND_RED | wincon::FOREGROUND_BLUE,
        Color::Brown => wincon::FOREGROUND_RED | wincon::FOREGROUND_GREEN,
        Color::LightGray => wincon::FOREGROUND_RED | wincon::FOREGROUND_GREEN | wincon::FOREGROUND_BLUE,

        Color::DarkGray => wincon::FOREGROUND_INTENSITY,
        Color::LightBlue => wincon::FOREGROUND_BLUE | wincon::FOREGROUND_INTENSITY,
        Color::LightGreen => wincon::FOREGROUND_GREEN | wincon::FOREGROUND_INTENSITY,
        Color::LightCyan => wincon::FOREGROUND_GREEN | wincon::FOREGROUND_BLUE | wincon::FOREGROUND_INTENSITY,
        Color::LightRed => wincon::FOREGROUND_RED | wincon::FOREGROUND_BLUE | wincon::FOREGROUND_INTENSITY,
        Color::LightMagenta => wincon::FOREGROUND_RED | wincon::FOREGROUND_GREEN | wincon::FOREGROUND_INTENSITY,
        Color::Yellow => wincon::FOREGROUND_RED | wincon::FOREGROUND_GREEN | wincon::FOREGROUND_INTENSITY,
        Color::White => wincon::FOREGROUND_RED | wincon::FOREGROUND_GREEN | wincon::FOREGROUND_BLUE | wincon::FOREGROUND_INTENSITY
    }
}

pub fn has_console_event(in_handle: winnt::HANDLE) -> Result<bool> {
    guarded_call("GetNumberOfConsoleInputEvents", || unsafe {
        let mut count= 0;
        let r = consoleapi::GetNumberOfConsoleInputEvents(in_handle, &mut count);
        (r, count)
    }).map(|n| n > 0)
}