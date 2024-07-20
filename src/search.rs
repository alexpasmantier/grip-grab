use std::{path::Path, slice::Iter};

use grep::{
    matcher::Matcher,
    regex::{RegexMatcher, RegexMatcherBuilder},
    searcher::{sinks::UTF8, Searcher},
};

#[derive(Debug)]
pub struct SearchResult {
    pub line_number: u64,
    pub line: String,
}

#[derive(Debug)]
pub struct FileResults(Vec<SearchResult>);

impl FileResults {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a> IntoIterator for &'a FileResults {
    type Item = &'a SearchResult;
    type IntoIter = Iter<'a, SearchResult>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

/// PERF: maybe stop at first match ?
pub fn search_file(path: &Path, matcher: &RegexMatcher) -> anyhow::Result<FileResults> {
    let mut matches: Vec<SearchResult> = Vec::new();

    /// PERF: ```rust
    //grep_searcher::searcher::Searcher
    //```
    //
    //```rust
    //pub fn new() -> Searcher
    //```
    //───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
    //Create a new searcher with a default configuration.
    //
    //To configure the searcher (e.g., invert matching, enable memory maps,
    //enable contexts, etc.), use the [`SearcherBuilder`](https://docs.rs/grep-searcher/0.1.13/grep_searcher/searcher/struct.SearcherBuilder.html).
    Searcher::new().search_path(
        &matcher,
        path,
        UTF8(|lnum, line| match matcher.find(line.as_bytes()) {
            Ok(Some(_)) => {
                matches.push(SearchResult {
                    line_number: lnum,
                    line: line.to_string(),
                });
                Ok(true)
            }
            Ok(None) => Ok(false),
            Err(err) => Err(err.into()),
        }),
    )?;

    Ok(FileResults(matches))
}

pub fn build_matcher(pattern: &str) -> anyhow::Result<RegexMatcher> {
    let builder = RegexMatcherBuilder::new();
    Ok(builder.build(pattern)?)
}
