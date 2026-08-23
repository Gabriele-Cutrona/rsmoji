pub mod commit;
pub mod handlers;

use crate::globals::{CATPPUCCIN_ACTIVE, cursor_to_start};
use crossterm::cursor::{MoveDown, MoveUp};
use crossterm::style::{Attribute, Print, SetAttribute, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{execute, terminal};
use std::io;
use unicode_segmentation::UnicodeSegmentation;

pub enum CommitTextType {
	Description(usize), // line_count
	Title,
}

pub fn reload_commit_message(commit_message: &str, text_type: CommitTextType) {
	let type_text = match text_type {
		CommitTextType::Description(line_count) => {
			format!("(line {line_count}) description (empty to confirm)")
		}
		CommitTextType::Title => "title".to_string(),
	};

	let offset = 5;

	let text = format!("? Commit {type_text}: ");
	let commit_message = commit_message.to_owned() + "\n";
	let mut printme_text = text + commit_message.as_str();

	let (t_cols, _) = terminal::size().expect("no");

	for i in 0..(printme_text.graphemes(true).count() / (t_cols as usize - offset)) {
		printme_text.insert_str((t_cols as usize - offset) * (i + 1), "\r\n  ");
	}

	let number_of_lines = {
		let count = printme_text.graphemes(true).count();
		let cols = t_cols as usize - offset;
		if count == cols + 3 {
			// why + 3 you may ask? ...it works.
			count / cols
		} else {
			count / cols + 1
		}
	};

	cursor_to_start();

	for _ in 0..number_of_lines {
		execute!(io::stdout(), Clear(ClearType::CurrentLine), MoveUp(1),)
			.expect("Failed to reload title input");
	}
	execute!(
		io::stdout(),
		MoveDown(1),
		SetAttribute(Attribute::Bold),
		// SetForegroundColor(CATPPUCCIN_ACTIVE),
		// Print(&text),
		Print(&printme_text),
		// SetAttribute(Attribute::Reset),
		// Print(&commit_message),
		MoveUp(1),
	)
	.expect("Failed to reload title input");
}
