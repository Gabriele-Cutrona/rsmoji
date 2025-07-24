use crate::globals::{MAX_LIST_LENGTH, cursor_to_start};
use crate::selection::{delete_menu, draw_menu, redraw_menu};
use crate::ui_state::UIState;
use crossterm::event::{KeyEvent, KeyModifiers};
use crossterm::terminal::disable_raw_mode;
use unicode_segmentation::UnicodeSegmentation;

pub fn handle_keydown(state: &mut UIState) {
	let max_offset = state.filtered_emojis.len().saturating_sub(MAX_LIST_LENGTH);
	let visible_emojis = state.emo_clamp();
	let emojis_not_empty = !state.filtered_emojis.is_empty();

	if state.offset < max_offset {
		state.offset += 1;
	}

	if state.selection >= visible_emojis && emojis_not_empty {
		state.selection = visible_emojis - 1;
	} else if state.selection < visible_emojis.saturating_sub(1) && state.offset >= max_offset {
		state.selection += 1;
	}

	redraw_menu(state);
}

pub fn handle_keyup(state: &mut UIState) {
	if state.offset > 0 {
		state.offset -= 1;
	} else if state.selection >= 1 {
		state.selection -= 1;
	}
	redraw_menu(&state);
}

pub fn handle_left(state: &mut UIState) {
	if state.insert_offset < state.user_input.graphemes(true).count() {
		state.insert_offset += 1;
		redraw_menu(&state);
	}
}

pub fn handle_right(state: &mut UIState) {
	if state.insert_offset != 0 {
		state.insert_offset -= 1;
		redraw_menu(&state);
	}
}

pub fn handle_char(c: char, state: &mut UIState, event: KeyEvent, emojis: &Vec<&'static str>) {
	if event.modifiers.contains(KeyModifiers::CONTROL) {
		match c {
			'c' => {
				cursor_to_start();
				disable_raw_mode().expect("Failed to disable raw mode");
				std::process::exit(0);
			}
			'p' => handle_keyup(state),
			'n' => handle_keydown(state),
			'b' => handle_left(state),
			'f' => handle_right(state),
			_ => {}
		}
		return;
	}

	state.offset = 0;
	state.selection = 0;
	delete_menu(&state);
	let mut graphemes: Vec<&str> = state.user_input.graphemes(true).collect();
	let cs = c.to_string();
	graphemes.insert(
		state.user_input.graphemes(true).count() - state.insert_offset,
		&cs,
	);
	state.user_input = graphemes.concat();
	state.filter_emojis(&emojis);
	draw_menu(state);
}

pub fn handle_enter(state: &UIState) -> bool {
	if !state.filtered_emojis.is_empty() {
		delete_menu(&state);
		return true;
	}
	return false;
}

pub fn handle_backspace(state: &mut UIState, emojis: &Vec<&'static str>) {
	delete_menu(&state);
	let mut graphemes: Vec<&str> = state.user_input.graphemes(true).collect();
	if !state.user_input.is_empty() && graphemes.len() > state.insert_offset {
		graphemes.remove(graphemes.len() - state.insert_offset - 1);
		state.user_input = graphemes.concat();
	}
	state.filter_emojis(&emojis);
	draw_menu(&state);
}
