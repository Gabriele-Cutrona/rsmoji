use crossterm::{cursor::MoveToColumn, execute, style::Color, terminal::disable_raw_mode};
use std::io;

pub fn cursor_to_start() {
	execute!(io::stdout(), MoveToColumn(0))
		.expect("Failed to move cursor to the start of the line");
}

pub fn die() {
	cursor_to_start();
	disable_raw_mode().expect("Failed to disable raw mode");
	std::process::exit(0);
}

pub const MAX_LIST_LENGTH: usize = 6;

pub const CATPPUCCIN_ACTIVE: Color = Color::Rgb {
	r: 203,
	g: 166,
	b: 247,
};

pub const CATPPUCCIN_INACTIVE: Color = Color::Rgb {
	r: 186,
	g: 194,
	b: 222,
};

pub fn licenses() {
	const LICENSES: &str = include_str!("../licenses/third-party-licenses.html");
	const FILE: &str = "./rsmoji-licenses.html";
	if let Result::Ok(_) = std::fs::write(FILE, LICENSES) {
		println!("wrote to file {FILE}");
	} else {
		println!("Unable to write to file, writing to stdout instead");
		println!("{LICENSES}");
		println!("Unable to write to file, wrote to stdout instead");
	}
	std::process::exit(0);
}
