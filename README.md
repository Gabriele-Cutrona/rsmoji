# rsmoji
Hello there! This is a simple rust CLI tool to add emojis to commits!
It uses [crossterm](https://crates.io/crates/crossterm) and [unicode-segmentation](https://crates.io/crates/unicode-segmentation)
## Installation - using `cargo`
First, ensure you have [rust](https://doc.rust-lang.org/stable/book/ch01-01-installation.html) installed on your system, and then run:
```zsh
cargo install rsmoji
```
To update, you can use cargo-update, install it with `cargo install cargo-update`, and then you can run:
```zsh
cargo-install-update install-update --all
```

## Usage
Just run `rsmoji` without any additional arguments

You can use emacs keybindings for moving the cursor while typing (C-b, C-f, C-p, C-n)

### Credits
The colors are [catppuccin-mocha](https://catppuccin.com)
The emoji list was taken from [gitmoji-cli](https://github.com/carloscuesta/gitmoji-cli)

### License
This project is currently licensed under the MIT license
