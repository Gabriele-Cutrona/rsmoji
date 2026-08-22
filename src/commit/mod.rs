pub mod commit;
pub mod handlers;

use crate::globals::{CATPPUCCIN_ACTIVE, cursor_to_start};
use crossterm::cursor::{MoveToColumn, MoveUp};
use crossterm::execute;
use crossterm::style::{Attribute, Print, SetAttribute, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use std::io;
use unicode_segmentation::UnicodeSegmentation;

pub enum CommitTextType {
	Description(usize), // line_count
	Title,
}

pub fn reload_commit_message(
	commit_message: &str,
	insert_offset: usize,
	text_type: CommitTextType,
) {
	let type_text = match text_type {
		CommitTextType::Description(line_count) => {
			format!("(line {line_count}) description (empty to confirm)")
		}
		CommitTextType::Title => "title".to_string(),
	};

	let text = format!("? Commit {type_text}: ");
	let commit_message = commit_message.to_owned() + "\n";
	cursor_to_start();
	execute!(
		io::stdout(),
		Clear(ClearType::CurrentLine),
		SetAttribute(Attribute::Bold),
		SetForegroundColor(CATPPUCCIN_ACTIVE),
		Print(&text),
		SetAttribute(Attribute::Reset),
		Print(&commit_message),
		MoveUp(1)
	)
	.expect("Failed to reload title input");

	let cols = text.len() + commit_message.graphemes(true).count() - insert_offset;
	execute!(io::stdout(), MoveToColumn(cols as u16 - 1),)
		.expect("Failed to move cursor to writing position");
}
