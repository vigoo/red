use crossterm::{Color, Crossterm, InputEvent, KeyEvent, RawScreen, AlternateScreen};
use rut::*;
use crate::buffer::*;
use crate::buffer_view::BufferView;
use std::time::Duration;

mod buffer;
mod buffer_view;

fn render_status_area(console: &mut Crossterm) -> Result<()> {
    let con_h = console.height();
    let con_w = console.width();
    let mut status_region = console.sub_region(0, console.height()-1, console.width(), 1)?;
    status_region.fill(Color::DarkRed)?;
    status_region.print(0, 0, Color::DarkRed, Color::White, "*** WELCOME TO R.E.D ***")?;

    Ok(())
}

fn render_buffers(console: &mut Crossterm, view: &BufferView) -> Result<()> {
    let mut buffer_region = console.sub_region(0, 0, console.width(), console.height() - 1)?;
    buffer_region.fill(Color::Black)?;
    buffer_region.draw_frame(Color::Black, Color::DarkRed, FrameStyle::Double)?;

    let mut inner_region = buffer_region.inside()?;
    let lines_to_show =
        view.buffer.lines
            .iter()
            .skip(view.top_line as usize)
            .take(inner_region.height() as usize);

    let mut x = 0;
    let mut y = 0;
    for line in lines_to_show {
        for evt in line.iter() {
            match evt {
                BufferRenderEvent::Char(ch) => {
                    inner_region.print_char(x, y, Color::Black, Color::White, ch)?;
                    x = x + 1;
                },
                BufferRenderEvent::SwitchStyle(style_id) => {},
            }
        }
        y = y + 1;
        x = 0;
    }

    Ok(())
}

fn run(console: &mut Crossterm) -> Result<()> {
    let buf = Buffer::from_string("Hello world\nthis is a test!");
    let view = BufferView::create(&buf);

    let mut input = console.input().read_sync();

    let mut test_x = 0;
    let mut test_y = 0;
    let mut n = 0;

    loop {
        render_buffers(console, &view)?;
        render_status_area(console)?;

        let event: Option<InputEvent> = input.next();

        match event {
            Some(InputEvent::Keyboard(KeyEvent::Esc)) =>
                break,
            _ => {}
        }
    }

    Ok(())
}

fn main() {
    let mut console = Crossterm::new();
    let _screen = AlternateScreen::to_alternate(true);
    console.cursor().hide().unwrap();

    if let Err(error) = run(&mut console) {
        eprintln!("Failure: {}", error)
    }

    console.cursor().show().unwrap();
}
