use std::collections::VecDeque;
use std::ffi::OsStr;
use std::mem;
use std::os::windows::prelude::*;
use winapi::shared::minwindef::*;
use winapi::um::consoleapi;
use winapi::um::processenv;
use winapi::um::winbase;
use winapi::um::wincon;
use winapi::um::winnt;

mod utils;

use crate::windows::utils::*;
use crate::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct WindowsRegion {
    top: i16,
    left: i16,
    bottom: i16,
    right: i16,

    out_handle: winnt::HANDLE
}

impl WindowsRegion {
    fn from_screen(out_handle: winnt::HANDLE) -> Result<WindowsRegion> {
        let screen_buffer_info = guarded_call("GetConsoleScreenBufferInfo", || unsafe {
            let mut info = mem::zeroed();
            let r = wincon::GetConsoleScreenBufferInfo(out_handle, &mut info);
            (r, info)
        })?;

        Ok(WindowsRegion {
            top: screen_buffer_info.srWindow.Top,
            left: screen_buffer_info.srWindow.Left,
            bottom: screen_buffer_info.srWindow.Bottom,
            right: screen_buffer_info.srWindow.Right,
            out_handle
        })
    }
}

impl Region for WindowsRegion {

    fn height(&self) -> i16 {
        self.bottom - self.top
    }

    fn width(&self) -> i16 {
        self.right - self.left
    }

    fn sub_region(&mut self, rel_x: i16, rel_y: i16, width: i16, height: i16) -> Result<Box<dyn Region>> {
        Ok(Box::new(WindowsRegion {
            top: self.top + rel_y,
            left: self.left + rel_x,
            bottom: self.top + rel_y + height,
            right: self.left + rel_x + width,
            out_handle: self.out_handle
        }))
    }

    fn fill(&mut self, color: Color) -> Result<()> {

        let height = self.height();
        let width = self.width();
        for row in 0..height {
            let start = wincon::COORD {
                X: self.left,
                Y: self.top + row
            };

            let _ = guarded_call("FillConsoleOutputCharacter", || unsafe {
                let mut filled = 0;
                let r = wincon::FillConsoleOutputCharacterA(
                    self.out_handle,
                    ' ' as winnt::CHAR,
                    width as u32,
                    start,
                    &mut filled);
                (r, filled)
            })?;

            let _ = guarded_call("FillConsoleOutputAttribute", || unsafe {
                let mut filled = 0;
                let r = wincon::FillConsoleOutputAttribute(
                    self.out_handle,
                    color_to_background_attributes(color),
                    width as u32,
                    start,
                    &mut filled
                );
                (r, filled)
            })?;
        }

        Ok(())
    }

    fn print(&mut self, rel_x: i16, rel_y: i16, background: Color, foreground: Color, text: &str) -> Result<()> {
        let start = wincon::COORD { X: self.left + rel_x, Y: self.top + rel_y };
        let text_u16: Vec<u16> = OsStr::new(text).encode_wide().collect();
        let attrs = vec![color_to_background_attributes(background) | color_to_foreground_attributes(foreground); text_u16.len()];

        let _ = guarded_call("WriteConsoleOutputCharacter", || unsafe {
            let mut written = 0;
            let r = wincon::WriteConsoleOutputCharacterW(
                self.out_handle,
            text_u16.as_slice().as_ptr(),
                text_u16.len() as u32,
                start,
                &mut written
            );
            (r, written)
        })?;

        let _ = guarded_call("WriteConsoleOutputAttribute", || unsafe {
            let mut written = 0;
            let r = wincon::WriteConsoleOutputAttribute(
                self.out_handle,
                attrs.as_slice().as_ptr(),
                text_u16.len() as u32,
                start,
                &mut written
            );
            (r, written)
        })?;

        Ok(())
    }
}

pub struct WindowsConsole {
    out_handle: winnt::HANDLE,
    in_handle: winnt::HANDLE,
    saved_console_in_mode: DWORD,
    saved_console_out_mode: DWORD,
    event_buffer: VecDeque<Event>,
    full_screen_region: WindowsRegion
}

pub fn create_console() -> Result<WindowsConsole> {
    let out_handle = unsafe { processenv::GetStdHandle(winbase::STD_OUTPUT_HANDLE) };
    let in_handle = unsafe { processenv::GetStdHandle(winbase::STD_INPUT_HANDLE) };
    let saved_console_in_mode = guarded_call("GetConsoleMode", || unsafe {
        let mut mode = 0;
        let r = consoleapi::GetConsoleMode(in_handle, &mut mode);
        consoleapi::SetConsoleMode(
            in_handle,
            wincon::ENABLE_WINDOW_INPUT | wincon::ENABLE_MOUSE_INPUT |
                     wincon::ENABLE_EXTENDED_FLAGS
        );
        (r, mode)
    })?;

    let saved_console_out_mode = guarded_call("GetConsoleMode", || unsafe {
        let mut mode = 0;
        let r = consoleapi::GetConsoleMode(out_handle, &mut mode);
        consoleapi::SetConsoleMode(
            in_handle,
            0
        );
        (r, mode)
    })?;

    let full_screen_region = WindowsRegion::from_screen(out_handle)?;

    Ok(WindowsConsole {
        out_handle,
        in_handle,
        saved_console_in_mode,
        saved_console_out_mode,
        event_buffer: VecDeque::new(),
        full_screen_region
    })
}

impl Drop for WindowsConsole {
    fn drop(&mut self) {
        use winapi::um::handleapi;
        unsafe {
            consoleapi::SetConsoleMode(self.in_handle, self.saved_console_in_mode);
            consoleapi::SetConsoleMode(self.out_handle, self.saved_console_out_mode);
            handleapi::CloseHandle(self.in_handle);
            handleapi::CloseHandle(self.out_handle);
        }
    }
}

impl Console for WindowsConsole {
    fn clear(&mut self) -> Result<()> {
        let screen_buffer_info = guarded_call("GetConsoleScreenBufferInfo", || unsafe {
            let mut info = mem::zeroed();
            let r = wincon::GetConsoleScreenBufferInfo(self.out_handle, &mut info);
            (r, info)
        })?;
        let console_size= (screen_buffer_info.dwSize.X) as u32 * (screen_buffer_info.dwSize.Y as u32);
        let coord00 = wincon::COORD { X: 0, Y: 0 };

        let _ = guarded_call("FillConsoleOutputCharacter", || unsafe {
            let mut filled = 0;
            let r = wincon::FillConsoleOutputCharacterA(
                self.out_handle,
                ' ' as winnt::CHAR,
                console_size,
                coord00,
                &mut filled);
            (r, filled)
        })?;

        let _ = guarded_call("FillConsoleOutputAttribute", || unsafe {
            let mut filled = 0;
            let r = wincon::FillConsoleOutputAttribute(
                self.out_handle,
                screen_buffer_info.wAttributes,
                console_size,
                coord00,
                &mut filled
            );
            (r, filled)
        })?;

        let _ = guarded_call("SetConsoleCursorPosition", || unsafe {
            (wincon::SetConsoleCursorPosition(self.out_handle, coord00), ())
        })?;

        Ok(())
    }

    fn get_next_event(&mut self) -> Result<Event> {
        let mut result: Option<Result<Event>> = None;

        while result.is_none() {
            match self.event_buffer.pop_front() {
                Some(event) =>
                    result = Some(Ok(event)),
                None => {
                    // TODO: wait if empty with timeout

                    if has_console_event(self.in_handle).unwrap_or(false) {
                        let input_record = guarded_call("ReadConsoleInput", || unsafe {
                            let mut record = mem::zeroed();
                            let mut read = 0;
                            let r = consoleapi::ReadConsoleInputW(self.in_handle, &mut record, 1, &mut read);
                            (r, record)
                        })?;

                        match input_record.EventType {
                            wincon::KEY_EVENT => {
                                let key_event = unsafe { input_record.Event.KeyEvent() };

                                result = match get_key(&key_event) {
                                    Some(key) => {
                                        let control_key_state = get_control_key_states(key_event.dwControlKeyState);

                                        let event =
                                            if key_event.bKeyDown != 0 {
                                                Event::KeyPressed {
                                                    key,
                                                    control_key_state
                                                }
                                            } else {
                                                Event::KeyReleased {
                                                    key,
                                                    control_key_state
                                                }
                                            };

                                        if key_event.wRepeatCount > 1 {
                                            for _ in 1..key_event.wRepeatCount {
                                                self.event_buffer.push_back(event.clone());
                                            }
                                        }

                                        Some(Ok(event))
                                    }
                                    None => None
                                }
                            },
                            wincon::MOUSE_EVENT => {
                                let mouse_event = unsafe { input_record.Event.MouseEvent() };
                                let control_key_state = get_control_key_states(mouse_event.dwControlKeyState);

                                result = match mouse_event.dwEventFlags {
                                    0 => Some(Ok(Event::MouseButtonStateChange {
                                        x: mouse_event.dwMousePosition.X,
                                        y: mouse_event.dwMousePosition.Y,
                                        left_button: (mouse_event.dwButtonState & wincon::FROM_LEFT_1ST_BUTTON_PRESSED) != 0,
                                        right_button: (mouse_event.dwButtonState & wincon::RIGHTMOST_BUTTON_PRESSED) != 0,
                                        control_key_state
                                    })),
                                    wincon::DOUBLE_CLICK => Some(Ok(Event::MouseDoubleClick {
                                        x: mouse_event.dwMousePosition.X,
                                        y: mouse_event.dwMousePosition.Y,
                                        left_button: (mouse_event.dwButtonState & wincon::FROM_LEFT_1ST_BUTTON_PRESSED) != 0,
                                        right_button: (mouse_event.dwButtonState & wincon::RIGHTMOST_BUTTON_PRESSED) != 0,
                                        control_key_state
                                    })),
                                    wincon::MOUSE_HWHEELED => Some(Ok(Event::MouseHorizontalWheel {
                                        x: mouse_event.dwMousePosition.X,
                                        y: mouse_event.dwMousePosition.Y,
                                        amount: (mouse_event.dwButtonState >> 16) as i16,
                                        control_key_state
                                    })),
                                    wincon::MOUSE_WHEELED => Some(Ok(Event::MouseWheel {
                                        x: mouse_event.dwMousePosition.X,
                                        y: mouse_event.dwMousePosition.Y,
                                        amount: (mouse_event.dwButtonState >> 16) as i16,
                                        control_key_state
                                    })),
                                    wincon::MOUSE_MOVED => Some(Ok(Event::MouseMove {
                                        x: mouse_event.dwMousePosition.X,
                                        y: mouse_event.dwMousePosition.Y,
                                        control_key_state
                                    })),
                                    _ => None
                                }
                            },
                            wincon::WINDOW_BUFFER_SIZE_EVENT => {
                                let _resize_event = unsafe { input_record.Event.WindowBufferSizeEvent() };
                                self.full_screen_region = WindowsRegion::from_screen(self.out_handle)?;

                                result = Some(Ok(Event::Resize {
                                    width: self.full_screen_region.width(),
                                    height: self.full_screen_region.height()
                                }))
                            },
                            _ => {}
                        }
                    } else {
                        // No console event available, we get the console size (as it does not generate any events)
                        // and fire a resize event if it has been changed
                        let full_screen_region = WindowsRegion::from_screen(self.out_handle)?;
                        if full_screen_region != self.full_screen_region {
                            self.full_screen_region = full_screen_region;

                            result = Some(Ok(Event::Resize {
                                width: self.full_screen_region.width(),
                                height: self.full_screen_region.height()
                            }))
                        }
                    }
                }
            }
        }

        result.unwrap()
    }

    fn full_screen(&mut self) -> Result<Box<dyn Region>> {
        Ok(Box::new(self.full_screen_region.clone()))
    }
}
