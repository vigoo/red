use crossterm::{Color, Crossterm, AlternateScreen};
use rut::*;
use riker::actors::*;
use riker_patterns::ask::ask;
use std::sync::{Arc, Mutex};

use crate::buffer::*;
use crate::buffer_view::BufferView;
use crate::input::{Input, InputMsg};
use crate::state::State;
use futures::executor::block_on;
use futures::prelude::future::RemoteHandle;

mod buffer;
mod buffer_view;
mod commands;
mod input;
mod state;

fn render_status_area(console: &mut Crossterm) -> Result<()> {
    let mut status_region = console.sub_region(0, console.height()-1, console.width(), 1)?;
    status_region.fill(Color::DarkRed)?;
    status_region.print(0, 0, Color::DarkRed, Color::White, "*** WELCOME TO R.E.D ***")?;

    Ok(())
}

fn render_buffer(console: &mut Crossterm, buffer: &Buffer, view: &BufferView) -> Result<()> {
    let mut buffer_region = console.sub_region(0, 0, console.width(), console.height() - 1)?;
    buffer_region.fill(Color::Black)?;
    buffer_region.draw_frame(Color::Black, Color::DarkRed, FrameStyle::Double)?;

    let mut inner_region = buffer_region.inside()?;
    let lines_to_show =
        buffer.lines
            .iter()
            .skip(view.top_line as usize)
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

fn run(console: &mut Crossterm, system: &ActorSystem) -> Result<()> {
    let state = system.actor_of(State::props(), "state").unwrap();
    let input_reader = console.input().read_async();
    let input = system.actor_of(Input::props(state.clone(), Arc::new(Mutex::new(input_reader))), "input").unwrap();

    let handle: RemoteHandle<bool> = ask(system, &input, InputMsg::AskForQuit);
    let result = block_on(handle);
//
//    loop {
////        render_buffers(console, active_buffer, active_view)?;
//        render_status_area(console)?;
//
//    }

    Ok(())
}

fn main() {
    let system = ActorSystem::new().unwrap();
    let mut console = Crossterm::new();
    let _screen = AlternateScreen::to_alternate(true);
    console.cursor().hide().unwrap();

    if let Err(error) = run(&mut console, &system) {
        eprintln!("Failure: {}", error)
    }

    console.cursor().show().unwrap();
}
