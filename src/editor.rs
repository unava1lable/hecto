use crate::Terminal;
use std::io::{self, Write};
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;

pub struct Editor {
	shoult_quit: bool,
	terminal: Terminal,
}

impl Editor {
	pub fn default() -> Self {
		Self {
			shoult_quit: false,
			terminal: Terminal::default().expect("Failed to initialize terminal")
		}
	}

	pub fn run(&mut self) {
		let _stdout = io::stdout().into_raw_mode().unwrap();
		loop {
			if let Err(e) = self.refresh_screen() {
				die(&e);
			}
			if self.shoult_quit {
				break;
			}
			if let Err(e) = self.process_keypress() {
				die(&e);
			}
		}
	}

	fn process_keypress(&mut self) -> Result<(), io::Error> {
		let pressed_key = read_key()?;
		match pressed_key {
			Key::Ctrl('q') => self.shoult_quit = true,
			_ => ()
		}
		Ok(())
	}

	fn refresh_screen(&self) -> Result<(), io::Error> {
		print!("{}{}", termion::clear::All, termion::cursor::Goto(1,1));
		if self.shoult_quit {
			println!("Goodbye!\r");
		} else {
			self.draw_row();
			print!("{}", termion::cursor::Goto(1,1))
		}
		io::stdout().flush()
	}

	fn draw_row(&self) {
		for _ in 0..self.terminal.size().height {
			println!("~\r");
		}
	}
}

fn read_key() -> Result<Key, io::Error> {
	loop {
		if let Some(key) = io::stdin().lock().keys().next() {
			return key;
		}
	}
}

fn die(e: &io::Error) {
	panic!("{}", e)
}