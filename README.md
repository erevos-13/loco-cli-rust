# Localise.biz CLI Tool

This CLI tool is designed to interact with the Localise.biz API for managing
translations. It allows users to import JSON files, export translations, and
manage locales.

## Features

- Import JSON files to Localise.biz
- Export translations from Localise.biz
- Support for multiple locales
- Progress bar for operation feedback
- Filter translations by tags
- Extract all available locales at once

## Installation

To install this CLI tool, you can use Cargo, Rust's package manager:

---

```bash
cargo install localise-cli
```

---

## Prerequisites

Before using the tool, you need to:

1. Have a Localise.biz account and API token
2. Create a `.env` file with your token:

```bash
LOCALISE_API_TOKEN=your_api_token
```

## Usage

### Basic Command Structure

```bash
localise-cli get --locale en --tags "tag1,tag2" --source "source_file.json" --export-path "output_file.json"
```

### Required Arguments

- `--token, -t`: Your Localise.biz API token
- `--path, -p`: Path to the JSON file for import or the source file
- `--export-path, -e`: Path where exported translations will be saved

### Optional Arguments

- `--locale, -l`: Specify the locale (default: "en")
- `--filters, -f`: Add filters for translation export (can specify multiple)
- `--post`: Enable/disable import functionality (default: true)
- `--get`: Enable/disable export functionality (default: true)
- `--source, -s`: Specify source locale for translations
- `--extract-all, -x`: Extract all available locales (default: false)

### Examples

1. Import translations:

```bash
localise-cli --token your_token --path ./translations.json --export-path ./output --locale fr --post
```

2. Export translations:

```bash
localise-cli --token your_token --path ./source.json --export-path ./output --locale de --get
```

3. Extract all available locales:

```bash
localise-cli --token your_token --path ./source.json --export-path ./output --locale es --filters tag1 tag2
```

4. Extract all available locales:

```bash
localise-cli --token your_token --path ./source.json --export-path ./output --extract-all
```

## Output Format

The tool exports translations in JSON format. Each exported file will be saved
with a `.json` extension in the specified export path.

## Error Handling

The tool provides clear error messages for common issues:

- Invalid API token
- File access problems
- Network connectivity issues
- API response errors

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major
changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under either of:

- MIT License
- Apache License, Version 2.0

## Author

Orfeas Voutsaridis <erevos13@gmail.com>

## Repository

[GitHub Repository](https://github.com/erevos-13/loco-cli-rust)
