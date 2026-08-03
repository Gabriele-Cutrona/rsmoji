pub mod handlers;

use crate::globals::{CATPPUCCIN_ACTIVE, cursor_to_start};
use crossterm::cursor::{MoveToColumn, MoveUp};
use crossterm::execute;
use crossterm::style::{Attribute, Print, SetAttribute, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use std::io;
use unicode_segmentation::UnicodeSegmentation;

pub enum CommitTextType {
	Description { line_count: usize },
	Title,
}

pub fn reload_commit_message(
	commit_message: &String,
	insert_offset: usize,
	end: bool,
	text_type: CommitTextType,
) {
	let type_text = match text_type {
		CommitTextType::Description { line_count } => {
			format!("(line number {line_count}) description (enter empty to confirm)")
		}
		CommitTextType::Title => "title".to_string(),
	};

	let message = format!("? Enter commit {type_text}: ");

	let text = if end {
		format!("? Commit {type_text}: ")
	} else {
		message.clone()
	};
	let commit_message = commit_message.to_owned() + "\n";
	cursor_to_start();
	execute!(
		io::stdout(),
		Clear(ClearType::CurrentLine),
		SetAttribute(Attribute::Bold),
		SetForegroundColor(CATPPUCCIN_ACTIVE),
		Print(text),
		SetAttribute(Attribute::Reset),
		Print(&commit_message),
		MoveUp(1)
	)
	.expect("Failed to reload title input");

	let cols = message.len() + commit_message.graphemes(true).count() - insert_offset;
	execute!(io::stdout(), MoveToColumn(cols as u16 - 1),)
		.expect("Failed to move cursor to writing position");
}
