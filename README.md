# ask

ask is a simple command-line tool for prompting the user for input with a graphical user interface (GUI).

## Features

- Prompt the user for input with a GUI window
- Customizable input field with CSS themes
- Seamless integration with other command-line tools

## Usage

```sh
# Basic usage
ask # Prompt for input and print it to stdout

# Integration with other commands
echo "Hello, $(ask)" # Prompt for input and include it in the output
```

## Installation

### From Source

```sh
git clone https://github.com/oleksiiluchnikov/ask.git
cd ask
cargo tauri build
# Move the binary to a directory in your PATH
mv target/release/ask ~/.local/bin/
```

## Configuration

The configuration file is located at `~/.config/ask/ask.toml`.

### Options

You can customize the appearance of the input field by specifying a CSS theme in the configuration file:

```toml
theme = "lcd" # Use the 'lcd' theme (located at ~/.config/ask/themes/lcd.css)
```

### Themes

Custom CSS themes for the input field can be placed in the `~/.config/ask/themes/` directory.

## License

[MIT](https://choosealicense.com/licenses/mit/)
