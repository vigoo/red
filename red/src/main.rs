use crossterm::{Color, Crossterm, InputEvent, AlternateScreen};
use rut::*;
use crate::buffer::*;
use crate::buffer_view::BufferView;
use crate::state::State;
use crate::commands::GlobalAction;

mod buffer;
mod buffer_view;
mod commands;
mod state;

fn render_status_area(console: &mut Crossterm) -> Result<()> {
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
                BufferRenderEvent::SwitchStyle(_style_id) => {},
            }
        }
        y = y + 1;
        x = 0;
    }

    Ok(())
}

fn run(console: &mut Crossterm) -> Result<()> {
//    let mut buf = Buffer::from_string("test","Hello world\nthis is a test!");
//    let mut view = BufferView::create(&buf);
    let mut state = State::initial();

    let mut input = console.input().read_sync();

    loop {
        render_buffers(console, state.active_view)?;
        render_status_area(console)?;

        let event: Option<InputEvent> = input.next();

        match event {
            Some(InputEvent::Keyboard(key_event)) =>
                match commands::handle_key_event(key_event, &mut state) {
                    Some(GlobalAction::Quit) =>
                        break,
                    None =>
                        {}
                }
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
