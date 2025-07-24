pub mod handlers;

use crate::globals::{CATPPUCCIN_ACTIVE, CATPPUCCIN_INACTIVE, MAX_LIST_LENGTH, cursor_to_start};
use crate::ui_state::UIState;
use crate::{Clear, ClearType};
use crossterm::cursor::{MoveDown, MoveToColumn, MoveUp};
use crossterm::style::{Attribute, Print, SetAttribute, SetForegroundColor};
use std::io;
use unicode_segmentation::UnicodeSegmentation;

use crossterm::execute;

pub fn redraw_menu(state: &UIState) {
	delete_menu(&state);
	draw_menu(state);
}

pub fn draw_menu(state: &UIState) {
	cursor_to_start();
	let user_input: String = state.user_input.clone();
	let user_input: String = user_input + "\n";
	let message = "? Choose a gitmoji! ";
	execute!(
		io::stdout(),
		SetAttribute(Attribute::Bold),
		SetForegroundColor(CATPPUCCIN_ACTIVE),
		Print(message),
		SetAttribute(Attribute::Reset),
		SetForegroundColor(CATPPUCCIN_INACTIVE),
		Print(user_input),
		SetAttribute(Attribute::Reset),
	)
	.expect("Failed to print select text");
	for i in 0..MAX_LIST_LENGTH {
		cursor_to_start();
		if i == state.selection {
			execute!(
				io::stdout(),
				SetForegroundColor(CATPPUCCIN_ACTIVE),
				Print("➜ ".to_string()),
			)
			.expect("Failed to print '➜ '")
		} else {
			execute!(io::stdout(), Print("  ".to_string())).expect("Failed to print '  '")
		}
		if i + state.offset < state.filtered_emojis.len() {
			execute!(
				io::stdout(),
				Print(state.filtered_emojis[i + state.offset]),
				SetAttribute(Attribute::Reset),
				Print("\n".to_string()),
			)
			.expect("Failed to print menu");
		}
	}
	let cols = message.len() + state.user_input.graphemes(true).count() - state.insert_offset;
	execute!(
		io::stdout(),
		MoveToColumn(cols as u16),
		MoveUp(state.emo_clamp() as u16 + 1),
	)
	.expect("Failed to move cursor to writing position");
}

pub fn delete_menu(state: &UIState) {
	execute!(io::stdout(), MoveDown(state.emo_clamp() as u16 + 1),)
		.expect("Failed to move cursor down");
	for _i in 0..state.emo_clamp() + 1 {
		execute!(io::stdout(), MoveUp(1), Clear(ClearType::CurrentLine)).expect("Failed to clear");
	}
}
