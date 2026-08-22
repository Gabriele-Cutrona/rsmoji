use std::io;

use crossterm::{
	cursor::MoveDown,
	event::{Event, KeyCode, KeyEventKind, KeyModifiers, read},
	execute,
};

use crate::{
	commit::{
		CommitTextType::{Description, Title},
		handlers::{
			handle_backspace_commit, handle_char_commit, handle_enter_commit, handle_left_commit,
			handle_right_commit, handle_up_commit,
		},
		reload_commit_message,
	},
	globals::die,
	ui_state::UIState,
};

pub fn commit_message() -> String {
	let mut state = UIState {
		offset: 0,
		selection: 2,
		user_input: String::new(),
		filtered_emojis: vec![],
		insert_offset: 0,
		line_count: 0,
	};
	execute!(io::stdout(), MoveDown(1)).expect("Failed to move cursor down by one line");
	let mut commit_message: String = String::new();
	reload_commit_message(&commit_message, state.insert_offset, Title);
	loop {
		let Event::Key(event) = read().expect("Failed to read crossterm event") else {
			continue;
		};

		if event.kind != KeyEventKind::Press {
			continue;
		}

		match event.code {
			KeyCode::Char(c) if !event.modifiers.contains(KeyModifiers::CONTROL) => {
				handle_char_commit(c, event, &mut state, &mut commit_message, Title)
			}
			KeyCode::Backspace => handle_backspace_commit(&mut state, &mut commit_message, Title),
			KeyCode::Left | KeyCode::Char('b') => {
				handle_left_commit(&mut state, &commit_message, Title)
			}
			KeyCode::Right | KeyCode::Char('f') => {
				handle_right_commit(&mut state, &commit_message, Title)
			}
			KeyCode::Enter => {
				reload_commit_message(&commit_message, state.insert_offset, Title);
				break;
			}
			KeyCode::Char('c') => die(),
			_ => {}
		}
	}
	commit_message
}

pub fn commit_descriptions() -> Vec<String> {
	let mut state = UIState {
		offset: 0,
		selection: 2,
		user_input: String::new(),
		filtered_emojis: vec![],
		insert_offset: 0,
		line_count: 0,
	};
	let mut commit_descriptions: Vec<String> = vec![];
	execute!(io::stdout(), MoveDown(1)).expect("Failed to move cursor down one line");
	reload_commit_message(&"", state.insert_offset, Description(state.line_count));
	loop {
		let Event::Key(event) = read().expect("Failed to read crossterm event") else {
			continue;
		};

		if event.kind != KeyEventKind::Press {
			continue;
		}

		if state.line_count >= commit_descriptions.len() {
			commit_descriptions.push(String::new());
		}

		let desc = Description(state.line_count);
		let single_desc = &mut commit_descriptions[state.line_count];

		match event.code {
			KeyCode::Char(c) if !event.modifiers.contains(KeyModifiers::CONTROL) => {
				handle_char_commit(c, event, &mut state, single_desc, desc)
			}
			KeyCode::Backspace => handle_backspace_commit(&mut state, single_desc, desc),
			KeyCode::Left | KeyCode::Char('b') => handle_left_commit(&mut state, single_desc, desc),
			KeyCode::Right | KeyCode::Char('f') => {
				handle_right_commit(&mut state, single_desc, desc)
			}
			KeyCode::Up | KeyCode::Char('p') => handle_up_commit(&mut state, &commit_descriptions),
			KeyCode::Enter | KeyCode::Down | KeyCode::Char('n') => {
				if commit_descriptions[state.line_count].is_empty() {
					break;
				}
				handle_enter_commit(&mut state, &mut commit_descriptions);
			}
			KeyCode::Char('c') => die(),
			_ => {}
		}
	}
	commit_descriptions
}
