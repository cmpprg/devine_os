use super::screen_character::ScreenChar;

pub(super) const BUFFER_HEIGHT: usize = 25;
pub(super) const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
pub(crate) struct Buffer {
    pub(super) chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}
