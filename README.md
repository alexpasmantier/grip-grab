
# Grip-grab 🧤

A faster, more lightweight, ripgrep alternative.

```sh
❯ gg "regex_pattern" .
```

## Installation
### Using Cargo
```bash
❯ cargo install grip-grab
```

**NOTE:** if using zsh with the `git` plugin, you might need to add the following line to your `~/.zshrc` in order for `gg` to work:
```sh
unalias gg
```

## Usage
```sh
❯ gg --help
```

```plaintext
A faster, more lightweight, ripgrep alternative.

Usage: gg [OPTIONS] <PATTERN> <PATH>

Arguments:
  <PATTERN>  a regex pattern to search for
  <PATH>     path in which to search recursively

Options:
  -I, --ignore-paths <IGNORE_PATHS>  paths to ignore when recursively walking target directory
  -G, --respect-gitignore            respect .gitignore when recursively walking directory
  -M, --max-results <MAX_RESULTS>    upper boundary for the number of results to expect (will panic if #results > max_results) [default: 1000]
  -T, --n-threads <N_THREADS>        number of threads to use [default: 4]
  -U, --multiline                    enable multiline matching
      --json                         output in JSON format
  -f, --file-paths-only              output file paths only
  -A, --absolute-paths               output absolute paths (defaults to relative)
  -C, --colored-output               toggle colored output (defaults to ON)
  -h, --help                         Print help
  -V, --version                      Print version

```

## Examples
### Basic usage
```sh
❯ gg "for" .
```
<img width="1382" alt="Screenshot 2024-07-21 at 16 21 25" src="https://github.com/user-attachments/assets/c3e9a74b-7d21-48dd-878f-2fb8e4de3eaf">

### JSON output
```sh
❯ gg "impl" . --json | jq
```
```json
{
  "path": "/somewhere/gg/src/search.rs",
  "results": [
    {
      "line_number": 23,
      "line": "impl fmt::Display for FileResults {\n"
    },
    {
      "line_number": 33,
      "line": "impl FileResults {\n"
    },
    {
      "line_number": 43,
      "line": "impl<'a> IntoIterator for &'a FileResults {\n"
    }
  ]
}
{
  "path": "/somewhere/gg/src/printer.rs",
  "results": [
    {
      "line_number": 17,
      "line": "impl Printer {\n"
    }
  ]
}
```

### Filenames only
```sh
❯ gg "for" . -f
```
<img width="1057" alt="Screenshot 2024-07-21 at 16 29 27" src="https://github.com/user-attachments/assets/388ce171-9ff2-49c6-a7a6-aedc99516978">
