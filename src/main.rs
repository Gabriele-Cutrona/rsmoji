mod commit;
mod emojis;
mod globals;
mod selection;
mod ui_state;

use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode};
use emojis::return_emojis;
use globals::cursor_to_start;
use std::io;
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
		.arg(arg!(-S --sign "Enable signing for this specific commit"))
		.arg(
			arg!(-p --"print-command" <choice> "Print final git command before executing it")
			.value_parser(["yes", "no"])
			.default_value("yes")
		)
		.get_matches();

	let emojis = return_emojis();

	enable_raw_mode().expect("Failed to enable raw mode");

	let gitmoji = emoji_selection(&emojis);
	let commit_message = commit_message();
	let commit_descriptions = commit_descriptions();

	print!("\n");
	cursor_to_start();
	disable_raw_mode().expect("Failed to disable raw mode");

	let final_commit_message = format!("{gitmoji} {commit_message}").trim().to_string();
	let descriptions_string = commit_descriptions.join("\n").trim().to_string();

	let git_args: Vec<&str> = ["commit", "-m", final_commit_message.as_str()]
		.into_iter()
		.chain(matches.get_flag("sign").then_some("-S"))
		.chain(vec!["-m"])
		.chain(vec![descriptions_string.as_str()])
		.collect();

	let print_command: &String = matches
		.get_one("print-command")
		.expect("failed to read print-command");
	if print_command == "yes" {
		println!("git commit -m {final_commit_message} -m\n{descriptions_string}");
	};

	Command::new("git")
		.args(git_args)
		.status()
		.expect("Failed to run git");

	Ok(())
}
