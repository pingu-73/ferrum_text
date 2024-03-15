use::std::io;
use::std::io::Write;

use termion::raw::{IntoRawMode, RawTerminal};
use termion::input::TermRead;
use termion::event::Key;

pub struct Size{
    pub width: u16,
    pub height: u16,
}

pub struct Terminal{
    size: Size,     // using size fn to return size to avoid changing of size form outside.
    _stdout: RawTerminal<io::Stdout>,
}

impl Terminal{
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size { width: size.0, height: size.1 },
            _stdout: io::stdout().into_raw_mode()?,
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn cursor_position(x: u16, y: u16) {
        let x = x.saturating_add(1); //if overflowing returns max value
        let y = y.saturating_add(1);

        print!("{}", termion::cursor::Goto(x, y)); 
    }

    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    pub fn read_key() -> Result<Key, std::io::Error>{
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide)
    }
    pub fn cursor_show() {
        print!("{}", termion::cursor::Show)
    }
    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine)
    }
}