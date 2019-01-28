use crate::*;

struct UnixConsole{

}

#[cfg(not(target_os = "windows"))]
impl Console for UnixConsole {
    fn clear(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn get_next_event(&mut self) -> Result<Event> {
        unimplemented!()
    }

    fn full_screen(&mut self) -> Result<Box<Region>> {
        unimplemented!()
    }
}
