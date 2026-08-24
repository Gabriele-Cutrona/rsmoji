pub mod commit;
pub mod handlers;

use crate::globals::{CATPPUCCIN_ACTIVE, cursor_to_start};
use crossterm::cursor::{MoveDown, MoveUp};
use crossterm::style::{Attribute, Print, SetAttribute, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{execute, terminal};
use std::io;
use unicode_segmentation::UnicodeSegmentation;

const OFFSET: usize = 10;

pub enum CommitTextType {
	Description(usize), // line_count
	Title,
}

pub enum Operation {
	Add,
	Del,
	None,
}

pub fn reload_commit_message(commit_message: &str, text_type: CommitTextType, op: Operation) {
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

	let number_of_lines_prev = match op {
		// TODO: this can absolutely be done better to account for
		// other tings such as window resizing, but for now it's
		// fine
		Operation::Del => (printme_text.graphemes(true).count() + 0) / (t_cols as usize - OFFSET),
		Operation::Add => (printme_text.graphemes(true).count() - 2) / (t_cols as usize - OFFSET),
		Operation::None => printme_text.graphemes(true).count() / (t_cols as usize - OFFSET),
	};

	let mut new_printme_text = String::new();
	for (i, c) in printme_text.chars().enumerate() {
		let index = t_cols as usize - OFFSET;
		if i != 0 && i % index == 0 {
			new_printme_text.push_str("\r\n  ");
		}
		new_printme_text.push_str(c.to_string().as_str());
	}
	cursor_to_start();

	for _ in 0..number_of_lines_prev + 1 {
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
}
