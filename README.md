# 💖 `rsmoji` 🦀
Hello there! ^w^
This is a simple rust CLI tool to add **emojis** to commits!
It uses [crossterm](https://crates.io/crates/crossterm) and [unicode-segmentation](https://crates.io/crates/unicode-segmentation)

## ✨ Features
- 😃 It adds emojis to git commits
- 🚧 The end for now :( (see roadmap for details)

## Installation - using `cargo` (crates.io)
First, ensure you have [rust](https://doc.rust-lang.org/stable/book/ch01-01-installation.html) installed on your system, and then run:
```zsh
cargo install rsmoji
```

To update, you can use cargo-update, install it with `cargo install cargo-update`, and then you can run:
```zsh
cargo-install-update install-update rsmoji
```

## Usage
Just run `rsmoji` without any additional arguments
If it's not found, you have to add $HOME/.cargo/bin to your $PATH
```sh
# bash
echo "EXPORT PATH=$PATH:$HOME/.cargo/bin" > ~/.bashrc
# zsh
echo "EXPORT PATH=$PATH:$HOME/.cargo/bin" > ~/.zshrc
```

You can use emacs keybindings to move the cursor while typing (Ctrl b, Ctrl f, Ctrl p, Ctrl n)

### Roadmap
- [ ] Support commit description (not only title)
- [ ] Support signed tags message
- [ ] Custom toml configuration (colors...)

### Credits
The default colors are [catppuccin-mocha](https://catppuccin.com)
The emoji list was taken from [gitmoji-cli](https://github.com/carloscuesta/gitmoji-cli)
This project is currently licensed under the MIT license, see `LICENSE` for the license text
