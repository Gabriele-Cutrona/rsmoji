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

use clap::{Parser, ValueEnum};

use crate::commit::commit::{commit_descriptions, commit_message};
use crate::selection::selection::emoji_selection;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Print {
	Yes,
	No,
}

#[derive(Parser)]
#[command(
	version,
	about = "✨ Gitmojis, now oxidized! 🦀\nWhen run without arguments it performs a git commit (interactive)"
)]
struct CLI {
	/// Enable signing for this specific commit
	#[arg(short = 'S', long)]
	sign: bool,

	/// Print final git command before executing it
	#[arg(short, long = "print-command", value_enum, default_value_t = Print::Yes)]
	print: Print,
}

fn main() -> io::Result<()> {
	let args = CLI::parse();

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
		.chain(args.sign.then_some("-S"))
		.chain(vec!["-m"])
		.chain(vec![descriptions_string.as_str()])
		.collect();

	if args.print == Print::Yes {
		println!("git commit -m {final_commit_message} -m\n{descriptions_string}");
	}

	Command::new("git")
		.args(git_args)
		.status()
		.expect("Failed to run git");

	Ok(())
}
