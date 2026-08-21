mod commit;
mod emojis;
mod globals;
mod selection;
mod ui_state;

use crossterm::cursor::MoveDown;
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers, read};
use crossterm::execute;
use crossterm::style::{Attribute, Print, SetAttribute, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode};
use emojis::return_emojis;
use globals::{CATPPUCCIN_ACTIVE, cursor_to_start};
use selection::draw_menu;
use std::io;
use std::process::Command;
use unicode_segmentation::UnicodeSegmentation;

use clap::{arg, command};

use crate::commit::CommitTextType::{Description, Title};
use crate::commit::handlers::{
	handle_backspace_commit, handle_char_commit, handle_enter_commit, handle_left_commit,
	handle_right_commit, handle_up_commit,
};
use crate::commit::reload_commit_message;
use crate::globals::die;
use crate::selection::handlers::{
	EmojiSelected, handle_backspace, handle_char, handle_enter, handle_keydown, handle_keyup,
	handle_left, handle_right,
};
use crate::ui_state::UIState;

fn main() -> io::Result<()> {
	let matches = command!()
		.about(
			r#"✨ Gitmojis, now oxidized! 🦀
When run without arguments it performs a git commit (interactive)"#,
		)
		.arg(arg!(-S --sign "enable signing for this specific commit"))
		.get_matches();

	let emojis: Vec<&'static str> = return_emojis();

	let mut state = UIState {
		offset: 0,
		selection: 2,
		user_input: String::new(),
		filtered_emojis: vec![""],
		insert_offset: 0,
		line_count: 0,
	};
	state.filter_emojis(&emojis);

	enable_raw_mode().expect("Failed to enable raw mode");

	let gitmoji = emoji_selection(&mut state, &emojis);
	let commit_message = commit_message(&mut state);
	let commit_descriptions = commit_descriptions(&mut state);

	execute!(io::stdout(), MoveDown(1)).expect("Failed to move cursor down by one line");
	cursor_to_start();
	disable_raw_mode().expect("Failed to disable raw mode");

	let final_commit_message = gitmoji + " " + &commit_message;
	let value = commit_descriptions.join("\n");

	let git_args = ["commit", "-m", final_commit_message.as_str()]
		.into_iter()
		.chain(matches.get_flag("sign").then_some("-S"))
		.chain(vec!["-m"])
		.chain(vec![value.as_str()]);

	Command::new("git")
		.args(git_args)
		.status()
		.expect("Failed to run git");

	Ok(())
}

fn emoji_selection(state: &mut UIState, emojis: &Vec<&'static str>) -> String {
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
				handle_char(c, state, emojis)
			}
			KeyCode::Down | KeyCode::Char('n') => handle_keydown(state),
			KeyCode::Up | KeyCode::Char('p') => handle_keyup(state),
			KeyCode::Left | KeyCode::Char('b') => handle_left(state),
			KeyCode::Right | KeyCode::Char('f') => handle_right(state),
			KeyCode::Backspace => handle_backspace(state, emojis),
			KeyCode::Enter => match handle_enter(state) {
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

fn commit_message(state: &mut UIState) -> String {
	execute!(io::stdout(), MoveDown(1)).expect("Failed to move cursor down by one line");
	let mut commit_message: String = String::new();
	reload_commit_message(&commit_message, state.insert_offset, Title);
	state.insert_offset = 0;
	loop {
		let Event::Key(event) = read().expect("Failed to read crossterm event") else {
			continue;
		};

		if event.kind != KeyEventKind::Press {
			continue;
		}

		match event.code {
			KeyCode::Char(c) if !event.modifiers.contains(KeyModifiers::CONTROL) => {
				handle_char_commit(c, event, state, &mut commit_message, Title)
			}
			KeyCode::Backspace => handle_backspace_commit(state, &mut commit_message, Title),
			KeyCode::Left | KeyCode::Char('b') => handle_left_commit(state, &commit_message, Title),
			KeyCode::Right | KeyCode::Char('f') => {
				handle_right_commit(state, &commit_message, Title)
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

fn commit_descriptions(state: &mut UIState) -> Vec<String> {
	let mut commit_descriptions: Vec<String> = vec![];
	state.insert_offset = 0;
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

		match event.code {
			KeyCode::Char(c) if !event.modifiers.contains(KeyModifiers::CONTROL) => {
				handle_char_commit(
					c,
					event,
					state,
					&mut commit_descriptions[state.line_count],
					Description(state.line_count),
				)
			}
			KeyCode::Backspace => handle_backspace_commit(
				state,
				&mut commit_descriptions[state.line_count],
				Description(state.line_count),
			),
			KeyCode::Left | KeyCode::Char('b') => handle_left_commit(
				state,
				&commit_descriptions[state.line_count],
				Description(state.line_count),
			),
			KeyCode::Right | KeyCode::Char('f') => handle_right_commit(
				state,
				&commit_descriptions[state.line_count],
				Description(state.line_count),
			),
			KeyCode::Up | KeyCode::Char('p') => handle_up_commit(state, &commit_descriptions),
			KeyCode::Enter | KeyCode::Down | KeyCode::Char('n') => {
				if commit_descriptions[state.line_count].is_empty() {
					break;
				}
				handle_enter_commit(state, &mut commit_descriptions);
			}
			KeyCode::Char('c') => die(),
			_ => {}
		}
	}
	commit_descriptions
}
