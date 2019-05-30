use crossterm::KeyEvent;
use crate::state::State;
use crate::buffer::Buffer;

pub enum GlobalAction {
    Quit,
}

pub fn handle_key_event(key_event: KeyEvent, state: &mut State) -> Option<GlobalAction> {
    // TODO: make this configurable
    match key_event {
        KeyEvent::Esc => Some(GlobalAction::Quit),
        KeyEvent::Char(ch) => { insert_char(ch, state.borrow_buffer(&state.active_view.buffer.name).unwrap()); None }
        _ => None
    }
}

fn insert_char(_ch: char, _view: &mut Buffer) {
    // TODO
}