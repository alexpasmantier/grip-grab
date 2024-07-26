
# Grip-grab 🧤

A circumstancially faster, more lightweight, ripgrep-inspired alternative.

```sh
❯ gg "\b(Read|Write)Half[^<]" tokio/src
```



https://github.com/user-attachments/assets/2c5ed221-77f4-4a75-9f1a-b96341f3313b



## Installation
### Using Cargo
```bash
cargo install grip-grab
```

**NOTE:** if using zsh with the `git` plugin, you might need to unalias `gg` in order for grip-grab's `gg` to work:
```sh
echo 'unalias gg' >> ~/.zshrc
source ~/.zshrc
```

## Usage
```sh
❯ gg --help
```

```plaintext
A somewhat faster, more lightweight, ripgrep-inspired alternative.

Usage: gg [OPTIONS] [PATTERN] [PATH]

Arguments:
  [PATTERN]  a regex pattern to search for
  [PATH]     path in which to search recursively

Options:
  -e, --patterns <PATTERNS>
          you can specify multiple patterns using -e "pattern1" -e "pattern2" etc
  -I, --ignore-paths <IGNORE_PATHS>
          paths to ignore when recursively walking target directory
  -G, --disregard-gitignore
          disregard .gitignore rules when recursively walking directory (defaults to false)
  -M, --max-results <MAX_RESULTS>
          upper boundary for the number of results to expect (will panic if #results > max_results) [default: 1000]
  -T, --n-threads <N_THREADS>
          number of threads to use [default: 4]
  -U, --multiline
          enable multiline matching
      --json
          output in JSON format
  -f, --file-paths-only
          output file paths only
  -A, --absolute-paths
          output absolute paths (defaults to relative)
  -C, --disable-colored-output
          disable colored output (colored by default)
  -t, --filter-filetypes <FILTER_FILETYPES>
          filter on filetype (defaults to all filetypes)
  -h, --help
          Print help
  -V, --version
          Print version
```

## Examples
### Basic usage
```sh
❯ gg "\b(Read|Write)Half[^<]" tokio/src
```
<img width="1973" alt="Screenshot 2024-07-26 at 14 00 31" src="https://github.com/user-attachments/assets/78d408a2-9f00-4c6d-95c0-6af6211ab40d">


### JSON output
```sh
❯ gg --json unsplit tokio/src | jq
```
<img width="1696" alt="Screenshot 2024-07-24 at 13 25 29" src="https://github.com/user-attachments/assets/67d4e90a-9bd1-4808-a260-226007339a55">



### Filenames only
```sh
❯ gg -f "\b(Read|Write)Half[^<]" tokio/src
```
<img width="1713" alt="Screenshot 2024-07-24 at 13 29 52" src="https://github.com/user-attachments/assets/9e5f5cee-218e-4213-bfeb-25df3d5a2a9e">

## Notes
This lightweight utility is largely based on a couple of crates from the extraordinary [ripgrep](https://github.com/BurntSushi/ripgrep) tool.
Its aim is to provide a minimal and lightweight version that can be easily integrated in other programs for search-related purproses.
