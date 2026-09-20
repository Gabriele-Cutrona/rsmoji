use std::io;

use crossterm::{
	cursor::MoveUp,
	event::{KeyEvent, KeyModifiers},
	execute,
	terminal::{Clear, ClearType},
};
use unicode_segmentation::UnicodeSegmentation;

use crate::{
	CLI,
	commit::{
		CommitTextType::{self, Description},
		reload_commit_message,
	},
	ui_state::UIState,
};

pub fn handle_char_commit(
	c: char,
	event: KeyEvent,
	state: &mut UIState,
	commit_message: &mut String,
	text_type: CommitTextType,
	args: &CLI
) {
	if event.modifiers.contains(KeyModifiers::CONTROL) {
		return;
	}

	let mut graphemes: Vec<&str> = commit_message.graphemes(true).collect();
	let cs = c.to_string();
	graphemes.insert(
		commit_message.graphemes(true).count() - state.insert_offset,
		&cs,
	);
	*commit_message = graphemes.concat();
	reload_commit_message(commit_message, state.insert_offset, text_type, args);
}

pub fn handle_backspace_commit(
	state: &mut UIState,
	commit_message: &mut String,
	text_type: CommitTextType,
	args: &CLI
) {
	let mut graphemes: Vec<&str> = commit_message.graphemes(true).collect();
	if !commit_message.is_empty() && graphemes.len() > state.insert_offset {
		graphemes.remove(graphemes.len() - state.insert_offset - 1);
		*commit_message = graphemes
			.concat()
			.parse()
			.expect("Failed to parse commit message");
	}
	reload_commit_message(commit_message, state.insert_offset, text_type, args);
}

pub fn handle_offset_start_commit(
	commit_message: &str,
	text_type: CommitTextType,
	state: &mut UIState,
	args: &CLI
) {
	state.insert_offset = commit_message.graphemes(true).count();
	reload_commit_message(commit_message, state.insert_offset, text_type, args);
}

pub fn handle_offset_end_commit(
	commit_message: &str,
	text_type: CommitTextType,
	state: &mut UIState,
	args: &CLI
) {
	state.insert_offset = 0;
	reload_commit_message(commit_message, state.insert_offset, text_type, args);
}

pub fn handle_left_commit(
	state: &mut UIState,
	commit_message: &str,
	text_type: CommitTextType,
	args: &CLI,
) {
	if state.insert_offset < commit_message.graphemes(true).count() {
		state.insert_offset += 1;
		reload_commit_message(commit_message, state.insert_offset, text_type, args);
	}
}

pub fn handle_right_commit(
	state: &mut UIState,
	commit_message: &str,
	text_type: CommitTextType,
	args: &CLI,
) {
	if state.insert_offset != 0 {
		state.insert_offset -= 1;
		reload_commit_message(commit_message, state.insert_offset, text_type, args);
	}
}

pub fn handle_up_commit(state: &mut UIState, commit_descriptions: &[String], args: &CLI) {
	if state.line_count == 0 {
		return;
	};
	state.insert_offset = 0;
	execute!(
		io::stdout(),
		Clear(ClearType::CurrentLine),
		MoveUp(1),
		Clear(ClearType::CurrentLine)
	)
	.expect("Failed to move cursor up one line");

	state.line_count -= 1;
	reload_commit_message(
		&commit_descriptions[state.line_count],
		state.insert_offset,
		Description(state.line_count),
		args
	);
}

pub fn handle_enter_commit(state: &mut UIState, commit_descriptions: &mut Vec<String>, args: &CLI) {
	state.insert_offset = 0;
	reload_commit_message(
		&commit_descriptions[state.line_count],
		state.insert_offset,
		Description(state.line_count),
		args
	);
	state.line_count += 1;
	commit_descriptions.push(String::new());
	println!();
	reload_commit_message(
		&commit_descriptions[state.line_count],
		state.insert_offset,
		Description(state.line_count),
		args
	);
}
