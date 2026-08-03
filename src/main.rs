mod commit;
mod emojis;
mod globals;
mod selection;
mod ui_state;

use crossterm::cursor::MoveDown;
use crossterm::event::{Event, KeyCode, KeyEventKind, read};
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
	handle_backspace_commit, handle_char_commit, handle_left_commit, handle_right_commit,
};
use crate::commit::reload_commit_message;
use crate::selection::handlers::{
	handle_backspace, handle_char, handle_enter, handle_keydown, handle_keyup, handle_left,
	handle_right,
};
use crate::ui_state::instantiate_state;

fn main() -> io::Result<()> {
	let matches = command!()
		.about(
			r#"✨ Gitmojis, now oxidized! 🦀
When run without arguments it performs a git commit (interactive)"#,
		)
		.arg(arg!(-S --sign "enable signing for this specific commit"))
		.get_matches();

	let emojis: Vec<&'static str> = return_emojis();

	let mut state = instantiate_state().expect("failed to instantiate state");
	state.filter_emojis(&emojis);
	draw_menu(&state);

	enable_raw_mode().expect("Failed to enable raw mode");
	loop {
		let Event::Key(event) = read()? else {
			continue;
		};

		if event.kind != KeyEventKind::Press {
			continue;
		}

		match event.code {
			KeyCode::Down => handle_keydown(&mut state),
			KeyCode::Up => handle_keyup(&mut state),
			KeyCode::Left => handle_left(&mut state),
			KeyCode::Right => handle_right(&mut state),
			KeyCode::Char(c) => handle_char(c, &mut state, event, &emojis),
			KeyCode::Backspace => handle_backspace(&mut state, &emojis),
			KeyCode::Enter => {
				let result = handle_enter(&state);
				if result {
					break;
				}
			}

			_ => {}
		}
	}

	let gitmoji: Vec<&str> = state.filtered_emojis[state.offset + state.selection]
		.graphemes(true)
		.collect();
	let gitmoji = gitmoji[0].to_string();
	let headline = "? Gitmoji: ".to_string() + &gitmoji + "!";
	cursor_to_start();
	execute!(
		io::stdout(),
		SetAttribute(Attribute::Bold),
		SetForegroundColor(CATPPUCCIN_ACTIVE),
		Print(headline),
		SetAttribute(Attribute::Reset),
	)
	.expect("failed to print selected gitmoji");

	execute!(io::stdout(), MoveDown(1)).expect("Failed to move cursor down by one line");
	let mut commit_message: String = String::new();
	reload_commit_message(&commit_message, state.insert_offset, false, Title);
	state.insert_offset = 0;
	loop {
		let Event::Key(event) = read()? else {
			continue;
		};

		if event.kind != KeyEventKind::Press {
			continue;
		}

		match event.code {
			KeyCode::Char(c) => {
				handle_char_commit(c, event, &mut state, &mut commit_message, Title)
			}
			KeyCode::Backspace => handle_backspace_commit(&mut state, &mut commit_message, Title),
			KeyCode::Left => handle_left_commit(&mut state, &commit_message, Title),
			KeyCode::Right => handle_right_commit(&mut state, &commit_message, Title),
			KeyCode::Enter => {
				reload_commit_message(&commit_message, state.insert_offset, true, Title);
				break;
			}
			_ => {}
		}
	}

	let mut commit_descriptions: Vec<String> = vec![];

	let mut line_count: usize = 0;
	'outer: loop {
		line_count += 1;
		let mut commit_description: String = String::new();
		state.insert_offset = 0;
		execute!(io::stdout(), MoveDown(1)).expect("Failed to move cursor down one line");
		reload_commit_message(
			&commit_description,
			state.insert_offset,
			false,
			Description { line_count },
		);
		loop {
			let Event::Key(event) = read()? else {
				continue;
			};

			if event.kind != KeyEventKind::Press {
				continue;
			}

			match event.code {
				KeyCode::Char(c) => handle_char_commit(
					c,
					event,
					&mut state,
					&mut commit_description,
					Description { line_count },
				),
				KeyCode::Backspace => handle_backspace_commit(
					&mut state,
					&mut commit_description,
					Description { line_count },
				),
				KeyCode::Left => {
					handle_left_commit(&mut state, &commit_description, Description { line_count })
				}
				KeyCode::Right => {
					handle_right_commit(&mut state, &commit_description, Description { line_count })
				}
				KeyCode::Enter => {
					if commit_description.is_empty() {
						break 'outer;
					}
					reload_commit_message(
						&commit_description,
						state.insert_offset,
						true,
						Description { line_count },
					);
					commit_descriptions.push(commit_description);
					break;
				}
				_ => {}
			}
		}
	}

	let final_commit_message = gitmoji + " " + &commit_message;

	execute!(io::stdout(), MoveDown(1)).expect("Failed to move cursor down by one line");
	cursor_to_start();
	disable_raw_mode().expect("Failed to disable raw mode");

	let mut git_args = Vec::new();

	git_args.push("commit");
	git_args.push("-m");
	git_args.push(final_commit_message.as_str());

	if matches.get_flag("sign") {
		git_args.push("-S")
	}

	for descs in &commit_descriptions {
		git_args.push("-m");
		git_args.push(descs);
	}

	Command::new("git")
		.args(git_args)
		.status()
		.expect("Failed to run git");

	return Ok(());
}
