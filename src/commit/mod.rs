pub mod commit;
pub mod handlers;

use crate::globals::{CATPPUCCIN_ACTIVE, cursor_to_start};
use crossterm::cursor::{MoveDown, MoveUp};
use crossterm::style::{Attribute, Print, SetAttribute, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{execute, terminal};
use std::io;
use std::sync::Mutex;
use unicode_width::UnicodeWidthStr;

pub enum CommitTextType {
	Description(usize), // line_count
	Title,
}

pub enum Operation {
	UpOrDown,
	Add,
	Del,
	None,
}

static PREVIOUS_NUMBER_OF_LINES: Mutex<usize> = Mutex::new(0);

pub fn reload_commit_message(commit_message: &str, text_type: CommitTextType, op: Operation) {
	let mut previous_number_of_lines = PREVIOUS_NUMBER_OF_LINES
		.lock()
		.expect("unable to lock Mutex PREVIOUS_NUMBER_OF_LINES");
	let type_text = match text_type {
		CommitTextType::Description(line_count) => {
			format!("(line {line_count}) description (empty to confirm)")
		}
		CommitTextType::Title => "title".to_string(),
	};

	let text = format!("? Commit {type_text}: ");
	let commit_message = commit_message.to_owned() + "\n";
	let printme_text = text + commit_message.as_str();

	let (t_cols, _) = terminal::size().unwrap_or((80, 24));

	let mut new_printme_text = String::new();
	let mut added_ago: usize = 0;
	let mut current_number_of_lines: usize = 0;
	for (_i, c) in printme_text.chars().enumerate() {
		let index = t_cols as usize;

		// arbitrary tolerance numbers (5, 35)
		if new_printme_text.width() != 0 && new_printme_text.width() % index <= 5 && added_ago > 35
		{
			new_printme_text.push_str("\r\n  ");
			added_ago = 0;
			current_number_of_lines += 1;
		}
		added_ago += 1;
		new_printme_text.push_str(c.to_string().as_str());
	}
	cursor_to_start();

	for _ in 0..=*previous_number_of_lines {
		execute!(io::stdout(), Clear(ClearType::CurrentLine), MoveUp(1),)
			.expect("Failed to reload title input");
	}
	execute!(
		io::stdout(),
		MoveDown(1),
		SetAttribute(Attribute::Bold),
		// SetForegroundColor(CATPPUCCIN_ACTIVE),
		// Print(&text),
		Print(&new_printme_text),
		// SetAttribute(Attribute::Reset),
		// Print(&commit_message),
		MoveUp(1),
	)
	.expect("Failed to reload title input");
	match op {
		Operation::UpOrDown => *previous_number_of_lines = 0,
		_ => *previous_number_of_lines = current_number_of_lines,
	}
}
