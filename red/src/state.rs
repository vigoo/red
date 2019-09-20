use crate::buffer::{Buffer, BufferName, BufferViewName};
use crate::buffer_view::BufferView;
use std::collections::HashMap;
use riker::actors::*;

pub struct State {
    active_view: BufferViewName,
    buffers: HashMap<BufferName, Buffer>,
    views: HashMap<BufferViewName, BufferView>
}

#[derive(Clone, Debug)]
pub enum StateMessage {
    InsertChar(char)
}

impl State {
    pub fn props() -> BoxActorProd<State> {
        Props::new(State::initial)
    }

    fn initial() -> State {
        let default_name = BufferName::new("default");
        let default_view_name = BufferViewName::new("default");
        let default_buffer = Buffer::from_string(&default_name, "");
        let default_buffer_view = BufferView::create(default_name.clone());

        let mut initial_buffers: HashMap<BufferName, Buffer> = HashMap::new();
        initial_buffers.insert(default_name.clone(), default_buffer);

        let mut initial_views: HashMap<BufferViewName, BufferView> = HashMap::new();
        initial_views.insert(default_view_name.clone(), default_buffer_view);

        State {
            active_view: default_view_name,
            buffers: initial_buffers,
            views: initial_views
        }
    }

    fn add_buffer(&mut self, new_buffer: Buffer) {
        self.buffers.insert(new_buffer.name.clone(), new_buffer);
    }

    fn borrow_active_buffer(&mut self) -> Option<&mut Buffer> {
        let active_buffer_name: Option<BufferName> = self.views.get(&self.active_view).map(|view| view.buffer.clone());
        match active_buffer_name {
            Some(name) => self.buffers.get_mut(&name),
            None => None
        }
    }

    fn insert_char_to_active_buffer(&mut self, ch: char) {
        self.borrow_active_buffer().map(|buffer|
            buffer.insert(ch)
        );
    }
//
//    pub fn borrow_buffer(&mut self, name: &BufferName) -> Option<&mut Rc<Buffer>> {
//        self.buffers.get_mut(name)
//    }
//
//    pub fn borrow_view(&mut self, name: &BufferName) -> Option<&mut Rc<BufferView>> {
//        self.views.get_mut(name)
//    }
}

impl Actor for State {
    type Msg = StateMessage;

    fn recv(&mut self, _ctx: &Context<StateMessage>, msg: StateMessage, _sender: Sender) {
        match msg {
            StateMessage::InsertChar(ch) => self.insert_char_to_active_buffer(ch)
        }
    }
}
