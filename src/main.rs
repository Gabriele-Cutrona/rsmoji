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

use crate::commit::handlers::{
    handle_backspace_commit, handle_char_commit, handle_left_commit, handle_right_commit,
};
use crate::commit::reload_commit_message;
use crate::selection::handlers::{
    handle_backspace, handle_char, handle_enter, handle_keydown, handle_keyup, handle_left,
    handle_right,
};
use crate::ui_state::UIState;

fn main() -> io::Result<()> {
    let emojis: Vec<&'static str> = return_emojis();

    let mut state = UIState {
        offset: 0,
        selection: 2,
        user_input: String::new(),
        filtered_emojis: vec![""],
        insert_offset: 0,
    };
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
    reload_commit_message(&commit_message, state.insert_offset, false);
    state.insert_offset = 0;
    loop {
        let Event::Key(event) = read()? else {
            continue;
        };

        if event.kind != KeyEventKind::Press {
            continue;
        }

        match event.code {
            KeyCode::Char(c) => handle_char_commit(c, event, &mut state, &mut commit_message),
            KeyCode::Backspace => handle_backspace_commit(&mut state, &mut commit_message),
            KeyCode::Left => handle_left_commit(&mut state, &commit_message),
            KeyCode::Right => handle_right_commit(&mut state, &commit_message),
            KeyCode::Enter => {
                reload_commit_message(&commit_message, state.insert_offset, true);
                break;
            }
            _ => {}
        }
    }

    let final_commit_message = gitmoji + " " + &commit_message;

    execute!(io::stdout(), MoveDown(1)).expect("Failed to move cursor down by one line");
    cursor_to_start();
    disable_raw_mode().expect("Failed to disable raw mode");
    Command::new("git")
        .args(["commit", "-m", final_commit_message.as_str()])
        .status()
        .expect("Failed to run git");

    Ok(())
}
