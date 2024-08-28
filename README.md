
# Grip-grab (gg) 🧤

A circumstantially faster, more lightweight, ripgrep-inspired alternative.

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

Usage: gg [OPTIONS] [PATTERN] [PATHS]...

Arguments:
  [PATTERN]   a regex pattern to search for
  [PATHS]...  paths in which to search recursively

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
  -H, --disable-hyperlinks
          disable hyperlinks in output (defaults to false)
  -h, --help
          Print help
  -V, --version
          Print version
```

## Benchmarks
**Warning**: _this is just a couple of comparisons on different sizes of codebases to get an idea of the big numbers but not in any way does this pretend to be a rigorous and scientific benchmarking exercise. The general idea it tries to convey is that while `rg` and `gg` should yield similar performances since they share the same core crates, `gg` still might be (marginally) faster circumstancially._



### The `curl` codebase (approx. half a milion lines)
https://github.com/curl/curl

```sh
hyperfine "rg '[A-Z]+_NOBODY' ." "gg '[A-Z]+_NOBODY'" "grep -rE '[A-Z]+_NOBODY' ."
```
```
Benchmark 1: rg '[A-Z]+_NOBODY' .
  Time (mean ± σ):      38.5 ms ±   2.2 ms    [User: 18.1 ms, System: 207.3 ms]
  Range (min … max):    33.8 ms …  42.8 ms    72 runs

Benchmark 2: gg '[A-Z]+_NOBODY'
  Time (mean ± σ):      21.8 ms ±   0.8 ms    [User: 15.4 ms, System: 53.1 ms]
  Range (min … max):    20.2 ms …  23.8 ms    115 runs

Benchmark 3: ggrep -rE '[A-Z]+_NOBODY' .
  Time (mean ± σ):      73.3 ms ±   0.9 ms    [User: 26.5 ms, System: 45.7 ms]
  Range (min … max):    70.8 ms …  75.6 ms    41 runs

Summary
  gg '[A-Z]+_NOBODY' ran
    1.77 ± 0.12 times faster than rg '[A-Z]+_NOBODY' .
    3.36 ± 0.13 times faster than ggrep -rE '[A-Z]+_NOBODY' .
```

### The `tokio` codebase (approx. 160k lines)
https://github.com/tokio-rs/tokio

```sh
hyperfine "gg 'in<\w, W>'" "rg 'in<\w, W>'" "ggrep -r 'in<[[:alnum:]], W>'"
```
```
Benchmark 1: gg 'in<\w, W>'
  Time (mean ± σ):       9.6 ms ±   0.5 ms    [User: 6.3 ms, System: 9.2 ms]
  Range (min … max):     9.0 ms …  11.5 ms    236 runs

Benchmark 2: rg 'in<\w, W>'
  Time (mean ± σ):      11.1 ms ±   0.7 ms    [User: 7.3 ms, System: 20.6 ms]
  Range (min … max):     9.6 ms …  12.8 ms    216 runs

Benchmark 3: ggrep -r 'in<[[:alnum:]], W>'
  Time (mean ± σ):     442.0 ms ±   3.0 ms    [User: 348.6 ms, System: 92.5 ms]
  Range (min … max):   439.0 ms … 446.1 ms    10 runs

Summary
  gg 'in<\w, W>' ran
    1.15 ± 0.09 times faster than rg 'in<\w, W>'
   46.02 ± 2.49 times faster than ggrep -r 'in<[[:alnum:]], W>'
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
