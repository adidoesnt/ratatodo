# ratatodo

A terminal todo list app built with [Ratatui], started from the [Hello World template] as a way to learn ratatui.

## Features

- Add, complete/uncomplete, delete, and navigate todos entirely from the keyboard
- Vim-inspired modal input: `Normal` mode for navigating/acting on todos, `Insert` mode for typing a new todo's title

## Usage

```
cargo run
```

### Keybindings

**Normal mode**

| Key         | Action                          |
| ----------- | -------------------------------- |
| `↑` / `↓`   | Move selection up/down           |
| `Space`     | Toggle selected todo's completed state |
| `d`         | Delete selected todo             |
| `i`         | Enter Insert mode to add a todo  |
| `q`         | Quit                             |

**Insert mode**

| Key         | Action                          |
| ----------- | -------------------------------- |
| Any character | Append to the new todo's title |
| `Backspace` | Remove the last character        |
| `Enter`     | Submit the new todo and return to Normal mode |
| `Esc`       | Cancel and return to Normal mode |

## License

Copyright (c) Aditya Banerjee <aditya_banerjee@icloud.com>

This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[LICENSE]: ./LICENSE
