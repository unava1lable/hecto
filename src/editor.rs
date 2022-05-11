use crate::Terminal;
use termion::event::Key;

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

	fn process_keypress(&mut self) -> Result<(), std::io::Error> {
		let pressed_key = Terminal::read_key()?;
		match pressed_key {
			Key::Ctrl('q') => self.shoult_quit = true,
			_ => ()
		}
		Ok(())
	}

	fn refresh_screen(&self) -> Result<(), std::io::Error> {
		Terminal::clear_screen();
		Terminal::cursor_position(0, 0);
		if self.shoult_quit {
			println!("Goodbye!\r");
		} else {
			self.draw_row();
			Terminal::cursor_position(0, 0);
		}
		Terminal::flush()
	}

	fn draw_row(&self) {
		for _ in 0..self.terminal.size().height {
			println!("~\r");
		}
	}
}

fn die(e: &std::io::Error) {
	Terminal::clear_screen();
	panic!("{}", e)
}