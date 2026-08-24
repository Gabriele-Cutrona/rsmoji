pub mod commit;
pub mod handlers;

use crate::globals::{CATPPUCCIN_ACTIVE, cursor_to_start};
use crossterm::cursor::{MoveToColumn, MoveUp};
use crossterm::style::{Attribute, Print, SetAttribute, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{execute, terminal};
use std::io;
use std::sync::Mutex;
use unicode_width::UnicodeWidthStr;

static PREVIOUS_NUMBER_OF_LINES: Mutex<u16> = Mutex::new(0);
// anti-pattern in single threaded apps, yes I know, I don't care (for now at least)

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
	let commit_message = commit_message.to_owned();

	let total = format!("{text}{commit_message}");
	let (t_cols, _) = terminal::size().unwrap_or((80, 24));

	let mut guard = PREVIOUS_NUMBER_OF_LINES
		.lock()
		.expect("failed to lock mutex");
	if *guard != 0 {
		// for some reason MoveUp(0) is the same as MoveUp(1)
		execute!(io::stdout(), MoveUp(*guard)).expect("Failed to move up");
	}
	*guard = if total.width() as u16 % t_cols == 0 {
		(total.width() as u16 / t_cols) - 1
	} else {
		total.width() as u16 / t_cols
	};

	cursor_to_start();
	execute!(
		io::stdout(),
		Clear(ClearType::FromCursorDown),
		SetAttribute(Attribute::Bold),
		SetForegroundColor(CATPPUCCIN_ACTIVE),
		Print(&text),
		SetAttribute(Attribute::Reset),
		Print(&commit_message),
	)
	.expect("Failed to reload title input");

	let cols = ((total.width() - *guard as usize * t_cols as usize) as isize
		- insert_offset as isize)
		.rem_euclid(t_cols as isize);

	if *guard > 0 {
		let rows = *guard as usize - ((total.width() - 1 - insert_offset) / t_cols as usize);
		if rows > 0 {
			execute!(io::stdout(), MoveToColumn(cols as u16), MoveUp(rows as u16))
				.expect("Failed to move cursor to writing position");
		} else {
			execute!(io::stdout(), MoveToColumn(cols as u16))
				.expect("Failed to move cursor to writing position");
		}
		*guard = *guard - rows as u16;
	} else {
		execute!(io::stdout(), MoveToColumn(cols as u16))
			.expect("Failed to move cursor to writing position");
	}
}
