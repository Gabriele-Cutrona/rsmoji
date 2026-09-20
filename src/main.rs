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

use clap::Parser;

use crate::commit::commit::{commit_descriptions, commit_message};
use crate::globals::{licenses, OnOff};
use crate::selection::selection::emoji_selection;

#[derive(Parser)]
#[command(
	version,
	about = "✨ Gitmojis, now oxidized! 🦀\nWhen run without arguments it performs a git commit (interactive)"
)]
pub struct CLI {
	/// Enable signing for this specific commit
	#[arg(short = 'S', long, value_enum, default_value = "no", num_args = 0..=1, default_missing_value = "yes")]
	sign: OnOff,

	/// Print final git command before executing it
	#[arg(short, long = "print-command", value_enum, default_value = "yes", num_args = 0..=1, default_missing_value = "yes")]
	print: OnOff,

	/// enable or disable commit descriptions
	#[arg(short, long, value_enum, default_value = "yes", num_args = 0..=1, default_missing_value = "yes")]
	descriptions: OnOff,

	// enable 50/72 git commit guide
	#[arg(short, long, value_enum, default_value = "yes", num_args = 0..=1, default_missing_value = "yes")]
	guide: OnOff,

	// print license information
	#[arg(short, long, value_enum, default_value = "no", num_args = 0..=1, default_missing_value = "yes")]
	licenses: OnOff,
}

fn main() -> io::Result<()> {
	let args = CLI::parse();
	if args.licenses == OnOff::Yes {
		licenses()
	}

	let emojis = return_emojis();

	enable_raw_mode().expect("Failed to enable raw mode");

	let gitmoji = emoji_selection(&emojis);
	let commit_message = commit_message(&args);
	let commit_descriptions: String = match args.descriptions {
		OnOff::Yes => commit_descriptions(&args).join("\n").trim().to_string(),
		OnOff::No => "".to_string(),
	};

	println!();
	cursor_to_start();
	disable_raw_mode().expect("Failed to disable raw mode");

	let final_commit_message = format!("{gitmoji} {commit_message}").trim().to_string();

	let git_args: Vec<&str> = ["commit", "-m", final_commit_message.as_str()]
		.into_iter()
		.chain(if args.sign == OnOff::Yes {
			Some("-S")
		} else {
			None
		})
		.chain(if !commit_descriptions.is_empty() {
			vec!["-m", commit_descriptions.as_str()]
		} else {
			vec![]
		})
		.collect();

	if args.print == OnOff::Yes {
		if args.descriptions == OnOff::Yes {
			println!("git commit -m {final_commit_message} -m\n{commit_descriptions}");
		} else {
			println!("git commit -m {final_commit_message}");
		}
	}

	Command::new("git")
		.args(git_args)
		.status()
		.expect("Failed to run git");

	Ok(())
}
