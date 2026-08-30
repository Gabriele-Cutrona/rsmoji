use std::io;

use crossterm::{
	event::{Event, KeyCode, KeyEventKind, KeyModifiers, read},
	execute,
	style::{Attribute, Print, SetAttribute, SetForegroundColor},
};
use unicode_segmentation::UnicodeSegmentation;

use crate::{
	globals::{CATPPUCCIN_ACTIVE, cursor_to_start, die}, selection::{
		draw_menu, handlers::{
			EmojiSelected, handle_backspace, handle_char, handle_enter, handle_keydown, handle_keyup, handle_left, handle_offset_end, handle_offset_start, handle_right,
		},
	}, ui_state::UIState,
};

pub fn emoji_selection(emojis: &[String]) -> String {
	let mut state = UIState {
		offset: 0,
		selection: 2,
		user_input: String::new(),
		filtered_emojis: vec![],
		insert_offset: 0,
		line_count: 0,
	};
	state.filter_emojis(&emojis);
	draw_menu(&state);
	loop {
		let Event::Key(event) = read().expect("Failed to read crossterm event") else {
			continue;
		};

		if event.kind != KeyEventKind::Press {
			continue;
		}

		match event.code {
			KeyCode::Char(c) if !event.modifiers.contains(KeyModifiers::CONTROL) => {
				handle_char(c, &mut state, emojis)
			}
			KeyCode::Down | KeyCode::Char('n') => handle_keydown(&mut state),
			KeyCode::Up | KeyCode::Char('p') => handle_keyup(&mut state),
			KeyCode::Left | KeyCode::Char('b') => handle_left(&mut state),
			KeyCode::Right | KeyCode::Char('f') => handle_right(&mut state),
			KeyCode::Char('a') => handle_offset_start(&mut state),
			KeyCode::Char('e') => handle_offset_end(&mut state),
			KeyCode::Backspace => handle_backspace(&mut state, emojis),
			KeyCode::Enter => match handle_enter(&mut state) {
				EmojiSelected::Yes => break,
				EmojiSelected::No => {}
			},
			KeyCode::Char('c') => die(),
			_ => {}
		}
	}

	let gitmoji: Vec<&str> = state.filtered_emojis[state.offset + state.selection]
		.graphemes(true)
		.collect();
	let gitmoji = gitmoji[0].to_string();
	cursor_to_start();
	execute!(
		io::stdout(),
		SetAttribute(Attribute::Bold),
		SetForegroundColor(CATPPUCCIN_ACTIVE),
		Print("? Gitmoji: ".to_string() + &gitmoji + "!"),
		SetAttribute(Attribute::Reset),
	)
	.expect("failed to print selected gitmoji");
	gitmoji
}
