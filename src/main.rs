use crossterm::cursor::{MoveToColumn, MoveUp};
use crossterm::event::{Event, KeyCode, read};
use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io;

const MAX_SELECTION_LENGTH: usize = 6;

fn main() -> io::Result<()> {
    let emojis: Vec<&'static str> = return_emojis();
    let mut selection: usize = 2;
    let mut offset: usize = 0;
    let mut user_input: String = String::new();

    let mut filtered_emojis: Vec<&&str> = emojis
        .iter()
        .filter(|&emoji| emoji.contains(&user_input))
        .collect();
    draw_menu(&filtered_emojis, offset, selection, &user_input);

    enable_raw_mode().expect("Failed to enable raw mode");
    loop {
        match read()? {
            Event::Key(event) => match event.code {
                KeyCode::Down => {
                    let offset_ = offset as isize;
                    let selection_ = selection as isize;
                    let max_selection_length_ = MAX_SELECTION_LENGTH as isize;

                    if offset_ < filtered_emojis.len() as isize - max_selection_length_ {
                        offset += 1;
                    }

                    if selection >= return_length(&filtered_emojis) && filtered_emojis.len() != 0 {
                        selection = return_length(&filtered_emojis) - 1;
                    } else if selection_ < return_length(&filtered_emojis) as isize - 1
                        && offset_ >= filtered_emojis.len() as isize - max_selection_length_
                    {
                        selection += 1;
                    }

                    redraw_menu(&filtered_emojis, offset, selection, &user_input);
                }
                KeyCode::Up => {
                    if offset > 0 {
                        offset -= 1;
                    } else if selection >= 1 {
                        selection -= 1;
                    }
                    redraw_menu(&filtered_emojis, offset, selection, &user_input);
                }
                KeyCode::Enter => {
                    break;
                }
                KeyCode::Char(c) => {
                    offset = 0;
                    selection = 0;
                    filtered_emojis = emojis
                        .iter()
                        .filter(|&emoji| emoji.contains(&user_input))
                        .collect();
                    delete_menu(&filtered_emojis);
                    user_input += &c.to_string();
                    filtered_emojis = emojis
                        .iter()
                        .filter(|&emoji| emoji.contains(&user_input))
                        .collect();
                    draw_menu(&filtered_emojis, offset, selection, &user_input);
                }
                KeyCode::Backspace => {
                    filtered_emojis = emojis
                        .iter()
                        .filter(|&emoji| emoji.contains(&user_input))
                        .collect();
                    delete_menu(&filtered_emojis);
                    user_input.pop();
                    filtered_emojis = emojis
                        .iter()
                        .filter(|&emoji| emoji.contains(&user_input))
                        .collect();
                    draw_menu(&filtered_emojis, offset, selection, &user_input);
                }
                _ => {}
            },
            _ => {}
        }
    }
    disable_raw_mode().expect("Failed to disable raw mode");

    Ok(())
}

fn redraw_menu(emojis: &Vec<&&str>, offset: usize, selection: usize, user_input: &String) {
    delete_menu(emojis);
    draw_menu(emojis, offset, selection, user_input);
}

fn draw_menu(emojis: &Vec<&&str>, offset: usize, selection: usize, user_input: &String) {
    cursor_to_start();
    let headline = "Choose a gitmoji! ".to_string() + user_input + "\n";
    execute!(io::stdout(), Print(headline)).expect("Failed to print select text");
    for i in 0..MAX_SELECTION_LENGTH {
        cursor_to_start();
        if i == selection {
            execute!(io::stdout(), Print("> ".to_string())).expect("Failed to print '> '")
        } else {
            execute!(io::stdout(), Print("  ".to_string())).expect("Failed to print '  '")
        }
        if i + offset < emojis.len() {
            execute!(
                io::stdout(),
                Print(emojis[i + offset]),
                Print("\n".to_string()),
            )
            .expect("Failed to print menu");
        }
    }
}

fn delete_menu(emojis: &Vec<&&str>) {
    for _i in 0..return_length(emojis) + 1 {
        execute!(io::stdout(), MoveUp(1), Clear(ClearType::CurrentLine)).expect("Failed to clear");
    }
}

fn return_length(emojis: &Vec<&&str>) -> usize {
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
