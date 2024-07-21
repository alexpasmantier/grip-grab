use std::fmt;
use std::{path::PathBuf, slice::Iter};

use grep::{
    matcher::Matcher,
    regex::{RegexMatcher, RegexMatcherBuilder},
    searcher::{sinks::UTF8, Searcher, SearcherBuilder},
};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct SearchResult {
    pub line_number: u64,
    pub line: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct FileResults {
    pub path: PathBuf,
    pub results: Vec<SearchResult>,
}

impl fmt::Display for FileResults {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n", self.path.to_string_lossy()).and_then(|_| {
            self.results
                .iter()
                .try_for_each(|r| write!(f, "{}: {}", r.line_number, r.line))
        })
    }
}

impl FileResults {
    pub fn is_empty(&self) -> bool {
        self.results.is_empty()
    }

    pub fn len(&self) -> usize {
        self.results.len()
    }
}

impl<'a> IntoIterator for &'a FileResults {
    type Item = &'a SearchResult;
    type IntoIter = Iter<'a, SearchResult>;

    fn into_iter(self) -> Self::IntoIter {
        self.results.iter()
    }
}

pub fn search_file<'a>(
    path: PathBuf,
    matcher: &RegexMatcher,
    multiline: bool,
) -> anyhow::Result<FileResults> {
    let mut matches: Vec<SearchResult> = Vec::new();

    let mut searcher = build_searcher(multiline);
    searcher.search_path(
        &matcher,
        &path,
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

    Ok(FileResults {
        path,
        results: matches,
    })
}

pub fn build_matcher(pattern: &str) -> anyhow::Result<RegexMatcher> {
    let builder = RegexMatcherBuilder::new();
    Ok(builder.build(pattern)?)
}

pub fn build_searcher(multiline: bool) -> Searcher {
    let mut builder = SearcherBuilder::new();
    builder.multi_line(multiline);
    builder.build()
}
