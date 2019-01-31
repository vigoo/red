use rut::*;

fn redraw<C: Console>(console: &mut C) -> Result<()> {
    let mut region = console.full_screen()?;
    region.fill(Color::Green)?;

    region.print(1, 1, Color::Brown, Color::LightRed, "Welcome to RED")?;

    {
        let mut subregion = region.sub_region(10, 10, 10, 10)?;
        subregion.print(0, 0, Color::Black, Color::White, "hello?")?;
        subregion.draw_frame(Color::Green, Color::Yellow, FrameStyle::Single)?;
    }

    {
        let mut subregion = region.sub_region(20, 10, 10, 10)?;
        subregion.draw_frame(Color::Green, Color::Yellow, FrameStyle::SingleRounded)?;
    }

    {
        let mut subregion = region.sub_region(10, 20, 10, 10)?;
        subregion.draw_frame(Color::Green, Color::Yellow, FrameStyle::Dashed)?;
    }

    {
        let mut subregion = region.sub_region(20, 20, 10, 10)?;
        subregion.draw_frame(Color::Green, Color::Yellow, FrameStyle::Double)?;
    }

    Ok(())
}

fn playground<C: Console>(console: &mut C) -> Result<()> {
    console.clear()?;
    redraw(console)?;

    loop {
        let event = console.get_next_event()?;
        let s = format!("{:?}", event);
        {
            let mut region = console.full_screen()?;
            region.print(1, region.height() - 1, Color::Black, Color::LightGray, &s)?;
        }

        match event {
            Event::KeyPressed { key: Key::Escape, .. } => break,
            Event::Resize { .. } => {
                redraw(console)?;
            }
            _ => {}
        }
    }

    Ok(())
}

fn main() {
    let mut console = create_console().unwrap();
    if let Err(error) = playground(&mut console) {
        eprintln!("Failed! {}", error)
    }
}
