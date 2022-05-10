use std::io;
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;

pub struct Editor {}

impl Editor {
	pub fn default() -> Self {
		Self {}
	}

	pub fn run(&self) {
		let _stdout = io::stdout().into_raw_mode().unwrap();
		loop {
			if let Err(e) = self.process_keypress() {
				die(&e);
			}
		}
	}

	fn process_keypress(&self) -> Result<(), io::Error> {
		let pressed_key = read_key()?;
		match pressed_key {
			Key::Ctrl('q') => panic!("End"),
			_ => ()
		}
		Ok(())
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