use crate::buffer_view::BufferView;
use std::collections::HashMap;
use crate::buffer::Buffer;

pub struct State<'a> {
    pub active_view: &'a mut BufferView<'a>,
    buffers: HashMap<String, Buffer>,
    views: HashMap<String, BufferView<'a>>
}

impl<'a> State<'a> {
    pub fn initial() -> State<'a> {
        let default_name = "default";
        let default_buffer = Buffer::from_string(default_name, "");
        let mut default_buffer_view = BufferView::create(&default_buffer);

        let mut initial_buffers: HashMap<String, Buffer> = HashMap::new();
        initial_buffers.insert(default_name.to_string(), default_buffer);

        let mut initial_views: HashMap<String, BufferView<'a>> = HashMap::new();
        initial_views.insert(default_name.to_string(), default_buffer_view);

        State {
            active_view: &mut default_buffer_view,
            buffers: initial_buffers,
            views: initial_views
        }
    }

    pub fn add_buffer(&mut self, new_buffer: Buffer) {
        self.buffers.insert(new_buffer.name.clone(), new_buffer);
    }

    pub fn borrow_buffer(&mut self, name: &str) -> Option<&mut Buffer> {
        self.buffers.get_mut(name)
    }
}