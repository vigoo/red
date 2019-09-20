use crate::buffer::BufferName;

pub struct BufferView {
    pub top_line: u32,
    pub buffer: BufferName
}

impl BufferView {
    pub fn create(buffer: BufferName) -> BufferView {
        BufferView {
            top_line: 0,
            buffer
        }
    }
}
