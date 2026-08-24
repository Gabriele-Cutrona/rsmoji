# 💖 `rsmoji` 🦀

Hello there! ^w^

This is a simple rust CLI tool to add **emojis** to commits!
It uses [crossterm](https://crates.io/crates/crossterm), [unicode-segmentation](https://crates.io/crates/unicode-segmentation), [unicode-width](https://crates.io/crates/unicode-width) and [clap](https://crates.io/crates/clap)

<div>
	<img width="100" height="100" src="https://rustacean.net/assets/cuddlyferris.svg" />
	<img width="100" src="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/git/git-original.svg" />
</div>

## ✨ Features

- 😃 It adds emojis to git commits
- 🖊️ You can tell it to manually sign a commit (`-S` or `--sign` flag)
- 💬 It can add multi-line commit descriptions
- 🚧 The end for now :( (see roadmap for details)

## Installation - using `cargo` (crates.io)

First, ensure you have [rust](https://doc.rust-lang.org/stable/book/ch01-01-installation.html) installed on your system, and then run:

```zsh
cargo install rsmoji
```

You also have to add `$HOME/.cargo/bin` to your `$PATH`

```sh
# bash
echo 'export PATH="$PATH:$HOME/.cargo/bin"' >> ~/.bashrc
# zsh
echo 'export PATH="$PATH:$HOME/.cargo/bin"' >> ~/.zshrc
```

To update, you can use `cargo-update`, install it with `cargo install cargo-update`, and then you can run:

```zsh
cargo-install-update install-update rsmoji
```

## Usage

Just run `rsmoji` without any additional arguments (obviously you need `git`)

You can use emacs keybindings to move the cursor while typing (Ctrl b, Ctrl f, Ctrl p, Ctrl n)

## Tips - LazyGit Integration

I use `[lazygit](https://github.com/jesseduffield/lazygit)` regularly, and while you can use
it to add and push and use `rsmoji` to commit, you can also set a keybind to call rsmoji
inside lazygit itself

Inside `~/.config/lazygit/config.yml` (accessible also by pressing `e` inside the status tab in lazygit)
you can put this to execute `rsmoji` when pressing **`R`**, you can set it to whatever

```yaml
customCommands:
  - key: "R"
    context: "global"
    command: "rsmoji"
    output: "terminal"
```

## Why?

I made this for the following reasons:

- I used [gitmoji-cli](https://github.com/carloscuesta/gitmoji), and I liked it a lot, but I wanted something more, such as:
- not needing a JavaScript runtime
- being faster (text flickering, startup time, etc...)
- I wanted to learn rust
- I wanted more features (signed commit, multi-line descriptions, and the other things in the roadmap)

Instead of using some higher-level TUI toolkit, I used only crossterm for the UI, which brought me great pain, but also a great learning experience.
It isn't an efficient or even "good" way, but... recreational programming FTW, I suppose?
Afterall it's just a simple tool that somebody experienced could recreate in very little time with the right toolkit (I mean, if _I_ managed to do it...)

### Roadmap

- [x] Manually sign commits (`-S`)
- [x] Support commit description (not only title)
  - [ ] disable/enable with flag
  - [x] go back up to a previous line to edit
- [ ] Support signed tags message
- [ ] Support merge messages
- [ ] Themes (built in and custom)
- [ ] API to update emoji list dynamically (optional)

### License

Licensed MIT OR Apache-2.0 (./LICENSE-MIT and ./LICENSE-APACHE respectively)

### Credits

The default colors are [catppuccin-mocha](https://catppuccin.com)

The emoji list used by this program is taken from [gitmoji](https://github.com/carloscuesta/gitmoji)
