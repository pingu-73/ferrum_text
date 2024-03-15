use crate::Terminal;
use::std::io;
use::std::io::Write;

use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn run(&mut self) {
        let _stdout = io::stdout().into_raw_mode().unwrap(); //binding raw mode (refer to Ownership rules)

        loop{
            if let Err(error) = self.refresh_screen(){
                die(error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress(){
                die(error)
            }
        }
    }

    pub fn default() -> Self {
        Self{
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initilize terminal"),
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error>{
        // print!("\x1b[2J");
        print!("{} {}", termion::clear::All, termion::cursor::Goto(1,1));
        if self.should_quit {
            println!("Goodbye\r");
        }
        else{
            self.draw_rows();
            print!("{}", termion::cursor::Goto(1, 1));
        }
        io::stdout().flush()
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error>{
        let pressed_key = Self::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }

    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height {
            println!("~\r");
        }
    }

    fn read_key() -> Result<Key, std::io::Error> {
        loop{
            if let Some(key) = io::stdin().lock().keys().next() {
                return  key;
            }
        }
    }
}

fn die(e: std::io::Error){
    print!("{}", termion::clear::All);
    panic!("{}", e);
}