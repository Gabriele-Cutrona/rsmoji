use crossterm::{
	event::{KeyEvent, KeyModifiers},
	terminal::disable_raw_mode,
};
use unicode_segmentation::UnicodeSegmentation;

use crate::{
	commit::{CommitTextType, reload_commit_message},
	globals::cursor_to_start,
	ui_state::UIState,
};

pub fn handle_char_commit(
	c: char,
	event: KeyEvent,
	state: &mut UIState,
	commit_message: &mut String,
	text_type: CommitTextType,
) {
	if event.modifiers.contains(KeyModifiers::CONTROL) {
		match c {
			'c' => {
				cursor_to_start();
				disable_raw_mode().expect("Failed to disable raw mode");
				std::process::exit(0);
			}
			'b' => {
				if state.insert_offset < commit_message.graphemes(true).count() {
					state.insert_offset += 1;
					reload_commit_message(commit_message, state.insert_offset, false, text_type);
				}
			}
			'f' => {
				if state.insert_offset != 0 {
					state.insert_offset -= 1;
					reload_commit_message(commit_message, state.insert_offset, false, text_type);
				}
			}
			_ => {}
		}
		return;
	}

	let mut graphemes: Vec<&str> = commit_message.graphemes(true).collect();
	let cs = c.to_string();
	graphemes.insert(
		commit_message.graphemes(true).count() - state.insert_offset,
		&cs,
	);
	*commit_message = graphemes.concat();
	reload_commit_message(commit_message, state.insert_offset, false, text_type);
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
	reload_commit_message(&commit_message, state.insert_offset, false, text_type);
}

pub fn handle_left_commit(state: &mut UIState, commit_message: &String, text_type: CommitTextType) {
	if state.insert_offset < commit_message.graphemes(true).count() {
		state.insert_offset += 1;
		reload_commit_message(&commit_message, state.insert_offset, false, text_type);
	}
}

pub fn handle_right_commit(
	state: &mut UIState,
	commit_message: &String,
	text_type: CommitTextType,
) {
	if state.insert_offset != 0 {
		state.insert_offset -= 1;
		reload_commit_message(&commit_message, state.insert_offset, false, text_type);
	}
}
