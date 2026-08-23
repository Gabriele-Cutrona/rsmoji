mod commit;
mod emojis;
mod globals;
mod selection;
mod ui_state;

use crossterm::cursor::MoveDown;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType, DisableLineWrap, disable_raw_mode, enable_raw_mode};
use emojis::return_emojis;
use globals::cursor_to_start;
use std::io::{self};
use std::process::Command;

use clap::{arg, command};

use crate::commit::commit::{commit_descriptions, commit_message};
use crate::selection::selection::emoji_selection;

fn main() -> io::Result<()> {
	let matches = command!()
		.about(
			r#"✨ Gitmojis, now oxidized! 🦀
When run without arguments it performs a git commit (interactive)"#,
		)
		.arg(arg!(-S --sign "enable signing for this specific commit"))
		.get_matches();

	let emojis = return_emojis();

	enable_raw_mode().expect("Failed to enable raw mode");
	execute!(io::stdout(), DisableLineWrap).expect("Failed to Disable Line Wrap");

	let gitmoji = emoji_selection(&emojis);
	let commit_message = commit_message();
	let commit_descriptions = commit_descriptions();

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
