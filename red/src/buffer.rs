use crate::buffer::BufferRenderEvent::SwitchStyle;
use std::str::Chars;

#[derive(Debug, Clone)]
pub struct StyleId(String);

pub struct EnrichedSegment {
    characters: String,
    style: StyleId,
}

impl EnrichedSegment {
    fn iter<'a>(&'a self) -> EnrichedSegmentIter<'a> {
        EnrichedSegmentIter {
            segment: &self,
            first: true,
            chars: self.characters.chars()
        }
    }
}

pub struct EnrichedSegmentIter<'a> {
    segment: &'a EnrichedSegment,
    first: bool,
    chars: Chars<'a>
}

impl<'a> Iterator for EnrichedSegmentIter<'a> {
    type Item = BufferRenderEvent;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            Some(SwitchStyle(self.segment.style.clone()))
        }
        else {
            self.chars.next().map(BufferRenderEvent::Char)
        }
    }
}

#[derive(Debug)]
pub enum BufferRenderEvent {
    Char(char),
    SwitchStyle(StyleId)
}

pub struct Line {
    segments: Vec<EnrichedSegment> // TODO: better data structure for this
}

impl Line {
    fn process(source: &str) -> Line {
        let parts = source
            .split(' ')
            .map(|part| EnrichedSegment {
                characters: part.to_string() + " ",
                style: StyleId("default".to_string()) // TODO: predefined style id
            })
            .collect();

        Line {
            segments: parts
        }
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = BufferRenderEvent> + 'a {
        self.segments.iter().flat_map(|segment| segment.iter())
    }

    pub fn find_col(&self, col: u32) -> Option<(usize, usize)> {
        let mut total = 0;
        let mut result = None;
        let mut segment_idx = 0;
        while segment_idx < self.segments.len() && result.is_none() {
            let segment = &self.segments[segment_idx];
            let segment_len = segment.characters.len();
            let next_total = total + segment_len;
            println!("total {}, next_total {}", total, next_total);
            if next_total > col as usize {
                result = Some((segment_idx, col as usize - total));
            } else {
                total = next_total;
            }
            segment_idx = segment_idx + 1;
        }

        println!("result is {:?}", result);
        result
    }

    pub fn get(&self, col: u32) -> Option<char> {
        self.find_col(col).and_then(|res| {
            let (segment_idx, char_idx) = res;
            let chars: Vec<char> = self.segments[segment_idx].characters.chars().collect();
            chars.get(char_idx).map(|&ch| ch)
        })
    }
}

pub struct Cursor {
    row: u32,
    col: u32
}

pub struct Buffer {
    pub lines: Vec<Line>,
    pub cursors: Vec<Cursor>
}

impl Buffer {
    pub fn from_string(source: &str) -> Buffer {
        let lines = source
            .split('\n')
            .map(Line::process)
            .collect();
        Buffer {
            lines,
            cursors: vec![]
        }
    }
}