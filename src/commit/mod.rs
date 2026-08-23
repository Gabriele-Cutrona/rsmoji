pub mod commit;
pub mod handlers;

use crate::globals::{CATPPUCCIN_ACTIVE, cursor_to_start};
use crossterm::cursor::{MoveDown, MoveUp};
use crossterm::style::{Attribute, Print, SetAttribute, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{execute, terminal};
use std::io;
use unicode_segmentation::UnicodeSegmentation;

const OFFSET: usize = 20;

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
	let mut printme_text = text + commit_message.as_str();

	let (t_cols, _) = terminal::size().expect("no");

	let number_of_lines = printme_text.len() / (t_cols as usize - OFFSET);
	let number_of_lines_prev = match op {
		Operation::Del => (printme_text.len() + 1) / (t_cols as usize - OFFSET),
		Operation::Add => (printme_text.len() - 1) / (t_cols as usize - OFFSET),
		Operation::None => number_of_lines,
	};

	for i in 0..number_of_lines {
		let index = (t_cols as usize - OFFSET) * (i + 1);

		// arbitrary number (5), checks for multi-byte chars like あ
		for j in 0..5 {
			if printme_text.is_char_boundary(index - j) {
				printme_text.insert_str(index - j, "\r\n  ");
				break;
			}
		}
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
		Print(&printme_text),
		// SetAttribute(Attribute::Reset),
		// Print(&commit_message),
		MoveUp(1),
	)
	.expect("Failed to reload title input");
}
