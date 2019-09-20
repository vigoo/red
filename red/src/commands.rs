use crossterm::KeyEvent;
use crate::state::StateMessage;
use riker::actors::{ActorRef, Tell};

pub enum GlobalAction {
    Quit,
}

pub fn handle_key_event(key_event: KeyEvent, state: &ActorRef<StateMessage>) -> Option<GlobalAction> {
    // TODO: make this configurable
    match key_event {
        KeyEvent::Esc => Some(GlobalAction::Quit),
        KeyEvent::Char(ch) => {
            state.tell(StateMessage::InsertChar(ch), None);
            None
        }
        _ => None
    }
}
