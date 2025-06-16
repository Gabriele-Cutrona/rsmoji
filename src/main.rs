use crossterm::cursor::{self, MoveDown, MoveToColumn, MoveUp};
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers, read};
use crossterm::execute;
use crossterm::style::{Attribute, Color::Rgb, Print, SetAttribute, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode};
use std::io;
use std::process::Command;
use unicode_segmentation::UnicodeSegmentation;

const MAX_SELECTION_LENGTH: usize = 6;

struct UIState<'a> {
    offset: usize,
    selection: usize,
    user_input: String,
    filtered_emojis: Vec<&'a str>,
    insert_offset: usize,
}

fn main() -> io::Result<()> {
    let emojis: Vec<&'static str> = return_emojis();

    let mut state = UIState {
        offset: 0,
        selection: 2,
        user_input: String::new(),
        filtered_emojis: vec![""],
        insert_offset: 0,
    };
    state.filtered_emojis = emojis
        .iter()
        .copied()
        .filter(|&emoji| {
            emoji
                .to_lowercase()
                .contains(&state.user_input.to_lowercase())
        })
        .collect();
    draw_menu(&state);

    enable_raw_mode().expect("Failed to enable raw mode");
    loop {
        let Event::Key(event) = read()? else {
            continue;
        };

        if event.kind != KeyEventKind::Press {
            continue;
        }

        let handle_keydown = |state: &mut UIState| {
            let offset_ = state.offset as isize;
            let selection_ = state.selection as isize;
            let max_selection_length_ = MAX_SELECTION_LENGTH as isize;

            if offset_ < state.filtered_emojis.len() as isize - max_selection_length_ {
                state.offset += 1;
            }

            if state.selection >= return_length(&state.filtered_emojis)
                && !state.filtered_emojis.is_empty()
            {
                state.selection = return_length(&state.filtered_emojis) - 1;
            } else if selection_ < return_length(&state.filtered_emojis) as isize - 1
                && offset_ >= state.filtered_emojis.len() as isize - max_selection_length_
            {
                state.selection += 1;
            }

            redraw_menu(state);
        };

        let handle_keyup = |state: &mut UIState| {
            if state.offset > 0 {
                state.offset -= 1;
            } else if state.selection >= 1 {
                state.selection -= 1;
            }
            redraw_menu(&state);
        };

        let handle_char = |c: char, state: &mut UIState| {
            if c == 'c' && event.modifiers.contains(KeyModifiers::CONTROL) {
                cursor_to_start();
                disable_raw_mode().expect("Failed to disable raw mode");
                std::process::exit(0);
            }

            if c == 'p' && event.modifiers.contains(KeyModifiers::CONTROL) {
                handle_keyup(state);
                return;
            }

            if c == 'n' && event.modifiers.contains(KeyModifiers::CONTROL) {
                handle_keydown(state);
                return;
            }

            state.offset = 0;
            state.selection = 0;
            delete_menu(&state.filtered_emojis);
            let mut graphemes: Vec<&str> = state.user_input.graphemes(true).collect();
            let cs = c.to_string();
            graphemes.insert(
                state.user_input.graphemes(true).count() - state.insert_offset,
                &cs,
            );
            state.user_input = graphemes.concat();
            state.filtered_emojis = emojis
                .iter()
                .copied()
                .filter(|&emoji| {
                    emoji
                        .to_lowercase()
                        .contains(&state.user_input.to_lowercase())
                })
                .collect();
            draw_menu(state);
        };

        match event.code {
            KeyCode::Down => handle_keydown(&mut state),

            KeyCode::Up => handle_keyup(&mut state),

            KeyCode::Left => {
                if state.insert_offset < state.user_input.graphemes(true).count() {
                    state.insert_offset += 1;
                    redraw_menu(&state);
                }
            }

            KeyCode::Right => {
                if state.insert_offset != 0 {
                    state.insert_offset -= 1;
                    redraw_menu(&state);
                }
            }

            KeyCode::Enter => {
                if !state.filtered_emojis.is_empty() {
                    delete_menu(&state.filtered_emojis);
                    break;
                }
            }

            KeyCode::Char(c) => handle_char(c, &mut state),

            KeyCode::Backspace => {
                delete_menu(&state.filtered_emojis);
                let mut graphemes: Vec<&str> = state.user_input.graphemes(true).collect();
                if !state.user_input.is_empty() && graphemes.len() > state.insert_offset {
                    graphemes.remove(graphemes.len() - state.insert_offset - 1);
                    state.user_input = graphemes.concat();
                }
                state.filtered_emojis = emojis
                    .iter()
                    .copied()
                    .filter(|&emoji| {
                        emoji
                            .to_lowercase()
                            .contains(&state.user_input.to_lowercase())
                    })
                    .collect();
                draw_menu(&state);
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
        SetForegroundColor(Rgb {
            r: 180,
            g: 190,
            b: 254,
        }),
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

        let mut handle_char = |c: char| {
            if c == 'c' && event.modifiers.contains(KeyModifiers::CONTROL) {
                cursor_to_start();
                disable_raw_mode().expect("Failed to disable raw mode");
                std::process::exit(0);
            }
            let mut graphemes: Vec<&str> = commit_message.graphemes(true).collect();
            let cs = c.to_string();
            graphemes.insert(
                commit_message.graphemes(true).count() - state.insert_offset,
                &cs,
            );
            commit_message = graphemes.concat();
            reload_commit_message(&commit_message, state.insert_offset, false);
        };

        match event.code {
            KeyCode::Char(c) => handle_char(c),

            KeyCode::Backspace => {
                let mut graphemes: Vec<&str> = commit_message.graphemes(true).collect();
                if !commit_message.is_empty() && graphemes.len() > state.insert_offset {
                    graphemes.remove(graphemes.len() - state.insert_offset - 1);
                    commit_message = graphemes.concat();
                }
                reload_commit_message(&commit_message, state.insert_offset, false);
            }

            KeyCode::Left => {
                if state.insert_offset < commit_message.graphemes(true).count() {
                    state.insert_offset += 1;
                    reload_commit_message(&commit_message, state.insert_offset, false);
                }
            }

            KeyCode::Right => {
                if state.insert_offset != 0 {
                    state.insert_offset -= 1;
                    reload_commit_message(&commit_message, state.insert_offset, false);
                }
            }

            KeyCode::Enter => {
                reload_commit_message(&commit_message, state.insert_offset, true);
                break;
            }
            _ => {}
        }
    }

    let final_commit_message = gitmoji + " " + &commit_message;

    execute!(io::stdout(), MoveDown(1)).expect("failed to move cursor down by one line");
    cursor_to_start();
    disable_raw_mode().expect("Failed to disable raw mode");
    Command::new("git")
        .args(["commit", "-m", final_commit_message.as_str()])
        .status()
        .expect("Failed to run git");

    Ok(())
}

fn reload_commit_message(commit_message: &String, insert_offset: usize, end: bool) {
    let message = "? Enter commit title: ";

    let text = if end { "? Commit title: " } else { message };
    let commit_message = commit_message.to_owned() + "\n";
    cursor_to_start();
    execute!(
        io::stdout(),
        Clear(ClearType::CurrentLine),
        SetAttribute(Attribute::Bold),
        SetForegroundColor(Rgb {
            r: 180,
            g: 190,
            b: 254,
        }),
        Print(text),
        SetAttribute(Attribute::Reset),
        Print(&commit_message),
        MoveUp(1)
    )
    .expect("Failed to reload title input");

    let cols: u16 =
        message.len() as u16 + commit_message.graphemes(true).count() as u16 - insert_offset as u16;
    execute!(io::stdout(), cursor::MoveToColumn(cols - 1),)
        .expect("Failed to move cursor to writing position");
}

fn redraw_menu(state: &UIState) {
    delete_menu(&state.filtered_emojis);
    draw_menu(state);
}

fn draw_menu(state: &UIState) {
    cursor_to_start();
    let user_input: String = state.user_input.clone();
    let user_input: String = user_input + "\n";
    let message = "? Choose a gitmoji! ";
    execute!(
        io::stdout(),
        SetAttribute(Attribute::Bold),
        SetForegroundColor(Rgb {
            r: 180,
            g: 190,
            b: 254,
        }),
        Print(message),
        SetAttribute(Attribute::Reset),
        SetForegroundColor(Rgb {
            r: 186,
            g: 194,
            b: 222,
        }),
        Print(user_input),
        SetAttribute(Attribute::Reset),
    )
    .expect("Failed to print select text");
    for i in 0..MAX_SELECTION_LENGTH {
        cursor_to_start();
        if i == state.selection {
            execute!(
                io::stdout(),
                SetForegroundColor(Rgb {
                    r: 180,
                    g: 190,
                    b: 254,
                }),
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
    let cols: u16 = message.len() as u16 + state.user_input.graphemes(true).count() as u16
        - state.insert_offset as u16;
    execute!(
        io::stdout(),
        cursor::MoveToColumn(cols),
        cursor::MoveUp(state.filtered_emojis.len().clamp(0, MAX_SELECTION_LENGTH) as u16 + 1),
    )
    .expect("Failed to move cursor to writing position");
}

fn delete_menu(emojis: &Vec<&str>) {
    execute!(
        io::stdout(),
        cursor::MoveDown(emojis.len().clamp(0, MAX_SELECTION_LENGTH) as u16 + 1),
    )
    .expect("Failed to move cursor down");
    for _i in 0..return_length(emojis) + 1 {
        execute!(io::stdout(), MoveUp(1), Clear(ClearType::CurrentLine)).expect("Failed to clear");
    }
}

fn return_length(emojis: &Vec<&str>) -> usize {
    if emojis.len() > MAX_SELECTION_LENGTH {
        MAX_SELECTION_LENGTH
    } else {
        emojis.len()
    }
}

fn cursor_to_start() {
    execute!(io::stdout(), MoveToColumn(0))
        .expect("Failed to move cursor to the start of the line");
}

fn return_emojis() -> Vec<&'static str> {
    vec![
        "🎨 - Improve structure / format of the code",
        "⚡️ - Improve performance",
        "🔥 - Remove code or files",
        "🐛 - Fix a bug",
        "🚑️ - Critical hotfix",
        "✨ - Introduce new features",
        "📝 - Add or update documentation",
        "🚀 - Deploy stuff",
        "💄 - Add or update the UI and style files",
        "🎉 - Begin a project",
        "✅ - Add, update, or pass tests",
        "🔒️ - Fix security or privacy issues",
        "🔐 - Add or update secrets",
        "🔖 - Release / Version tags",
        "🚨 - Fix compiler / linter warnings",
        "🚧 - Work in progress",
        "💚 - Fix CI Build",
        "⬇️ - Downgrade dependencies",
        "⬆️ - Upgrade dependencies",
        "📌 - Pin dependencies to specific versions",
        "👷 - Add or update CI build system",
        "📈 - Add or update analytics or track code",
        "♻️ - Refactor code",
        "➕ - Add a dependency",
        "➖ - Remove a dependency",
        "🔧 - Add or update configuration files",
        "🔨 - Add or update development scripts",
        "🌐 - Internationalization and localization",
        "✏️ - Fix typos",
        "💩 - Write bad code that needs to be improved",
        "⏪️ - Revert changes",
        "🔀 - Merge branches",
        "📦️ - Add or update compiled files or packages",
        "👽️ - Update code due to external API changes",
        "🚚 - Move or rename resources (e.g.: files, paths, routes)",
        "📄 - Add or update license",
        "💥 - Introduce breaking changes",
        "🍱 - Add or update assets",
        "♿️ - Improve accessibility",
        "💡 - Add or update comments in source code",
        "🍻 - Write code drunkenly",
        "💬 - Add or update text and literals",
        "🗃️ - Perform database related changes",
        "🔊 - Add or update logs",
        "🔇 - Remove logs",
        "👥 - Add or update contributor(s)",
        "🚸 - Improve user experience / usability",
        "🏗️ - Make architectural changes",
        "📱 - Work on responsive design",
        "🤡 - Mock things",
        "🥚 - Add or update an easter egg",
        "🙈 - Add or update a .gitignore file",
        "📸 - Add or update snapshots",
        "⚗️ - Perform experiments",
        "🔍️ - Improve SEO",
        "🏷️ - Add or update types",
        "🌱 - Add or update seed files",
        "🚩 - Add, update, or remove feature flags",
        "🥅 - Catch errors",
        "💫 - Add or update animations and transitions",
        "🗑️ - Deprecate code that needs to be cleaned up",
        "🛂 - Work on code related to authorization, roles and permissions",
        "🩹 - Simple fix for a non-critical issue",
        "🧐 - Data exploration/inspection",
        "⚰️ - Remove dead code",
        "🧪 - Add a failing test",
        "👔 - Add or update business logic",
        "🩺 - Add or update healthcheck",
        "🧱 - Infrastructure related changes",
        "🧑‍💻 Improve developer experience",
        "💸 - Add sponsorships or money related infrastructure",
        "🧵 - Add or update code related to multithreading or concurrency",
        "🦺 - Add or update code related to validation",
        "✈️ - Improve offline support",
    ]
}
