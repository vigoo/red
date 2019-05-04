use crate::buffer::Buffer;

pub struct BufferView<'a> {
    pub top_line: u32,
    pub buffer: &'a Buffer
}

impl<'a> BufferView<'a> {
    pub fn create(buffer: &Buffer) -> BufferView {
        BufferView {
            top_line: 0,
            buffer
        }
    }
}
