use std::io;

use crossterm::{
	cursor::{MoveDown, MoveUp},
	event::{KeyEvent, KeyModifiers},
	execute,
	terminal::{self, Clear, ClearType},
};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::{
	commit::{
		CommitTextType::{self, Description},
		Operation, reload_commit_message,
	},
	ui_state::UIState,
};

pub fn handle_char_commit(
	c: char,
	state: &mut UIState,
	commit_message: &mut String,
	text_type: CommitTextType,
) {
	let mut graphemes: Vec<&str> = commit_message.graphemes(true).collect();
	let cs = c.to_string();
	graphemes.insert(
		commit_message.graphemes(true).count() - state.insert_offset,
		&cs,
	);
	*commit_message = graphemes.concat();
	reload_commit_message(commit_message, text_type, Operation::Add);
}

pub fn handle_backspace_commit(
	state: &mut UIState,
	commit_message: &mut String,
	text_type: CommitTextType,
) {
	let mut graphemes: Vec<&str> = commit_message.graphemes(true).collect();
	if !commit_message.is_empty() && graphemes.len() > state.insert_offset {
		graphemes.remove(graphemes.len() - state.insert_offset - 1);
		*commit_message = graphemes
			.concat()
			.parse()
			.expect("Failed to parse commit message");
	}
	reload_commit_message(&commit_message, text_type, Operation::Del);
}

pub fn handle_left_commit(state: &mut UIState, commit_message: &str, text_type: CommitTextType) {
	if state.insert_offset < commit_message.graphemes(true).count() {
		state.insert_offset += 1;
		reload_commit_message(&commit_message, text_type, Operation::None);
	}
}

pub fn handle_right_commit(
	state: &mut UIState,
	commit_message: &String,
	text_type: CommitTextType,
) {
	if state.insert_offset != 0 {
		state.insert_offset -= 1;
		reload_commit_message(&commit_message, text_type, Operation::None);
	}
}

pub fn handle_up_commit(state: &mut UIState, commit_descriptions: &Vec<String>) {
	if state.line_count == 0 {
		return;
	};
	state.insert_offset = 0;

	let (t_cols, _) = terminal::size().unwrap_or((80, 24));

	let commit_description =
		format!("? Commit Description: ") + commit_descriptions[state.line_count - 1].as_str();

	let current_number_of_lines: usize = commit_description.width() / t_cols as usize;

	state.line_count -= 1;
	reload_commit_message(
		&commit_descriptions[state.line_count],
		Description(state.line_count),
		Operation::Up(current_number_of_lines),
	);
}

pub fn handle_enter_commit(state: &mut UIState, commit_descriptions: &mut Vec<String>) {
	state.insert_offset = 0;
	reload_commit_message(
		&commit_descriptions[state.line_count],
		Description(state.line_count),
		Operation::Down,
	);
	state.line_count += 1;
	commit_descriptions.push(String::new());
	execute!(io::stdout(), MoveDown(1)).expect("Failed to move cursor down one line");
	reload_commit_message(
		&commit_descriptions[state.line_count],
		Description(state.line_count),
		Operation::None,
	);
}
