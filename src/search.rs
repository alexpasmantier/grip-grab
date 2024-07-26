use std::fmt;
use std::{path::PathBuf, slice::Iter};

use grep::{
    matcher::{Match, Matcher},
    regex::{RegexMatcher, RegexMatcherBuilder},
    searcher::{sinks::UTF8, Searcher, SearcherBuilder},
};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct SearchResult {
    pub line_number: u64,
    pub line: String,
    pub line_start: u64,
    pub line_end: u64,
    pub matches: Vec<MatchRange>,
}

#[derive(Serialize, Debug, Clone)]
pub struct MatchRange {
    pub start: usize,
    pub end: usize,
}

impl MatchRange {
    /// Create a new match.
    ///
    /// # Panics
    ///
    /// This function panics if `start > end`.
    #[inline]
    pub fn new(start: usize, end: usize) -> MatchRange {
        assert!(start <= end);
        MatchRange { start, end }
    }

    #[inline]
    pub fn from_match(m: Match) -> MatchRange {
        MatchRange::new(m.start(), m.end())
    }

    /// Creates a zero width match at the given offset.
    #[inline]
    pub fn zero(offset: usize) -> MatchRange {
        MatchRange {
            start: offset,
            end: offset,
        }
    }

    /// Return the start offset of this match.
    #[inline]
    pub fn start(&self) -> usize {
        self.start
    }

    /// Return the end offset of this match.
    #[inline]
    pub fn end(&self) -> usize {
        self.end
    }

    /// Return a new match with the start offset replaced with the given
    /// value.
    ///
    /// # Panics
    ///
    /// This method panics if `start > self.end`.
    #[inline]
    pub fn with_start(&self, start: usize) -> MatchRange {
        assert!(start <= self.end, "{} is not <= {}", start, self.end);
        MatchRange { start, ..*self }
    }

    /// Return a new match with the end offset replaced with the given
    /// value.
    ///
    /// # Panics
    ///
    /// This method panics if `self.start > end`.
    #[inline]
    pub fn with_end(&self, end: usize) -> MatchRange {
        assert!(self.start <= end, "{} is not <= {}", self.start, end);
        MatchRange { end, ..*self }
    }

    /// Offset this match by the given amount and return a new match.
    ///
    /// This adds the given offset to the start and end of this match, and
    /// returns the resulting match.
    ///
    /// # Panics
    ///
    /// This panics if adding the given amount to either the start or end
    /// offset would result in an overflow.
    #[inline]
    pub fn offset(&self, amount: usize) -> MatchRange {
        MatchRange {
            start: self.start.checked_add(amount).unwrap(),
            end: self.end.checked_add(amount).unwrap(),
        }
    }

    /// Returns the number of bytes in this match.
    #[inline]
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Returns true if and only if this match is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl std::ops::Index<MatchRange> for [u8] {
    type Output = [u8];

    #[inline]
    fn index(&self, index: MatchRange) -> &[u8] {
        &self[index.start..index.end]
    }
}

impl std::ops::IndexMut<MatchRange> for [u8] {
    #[inline]
    fn index_mut(&mut self, index: MatchRange) -> &mut [u8] {
        &mut self[index.start..index.end]
    }
}

impl std::ops::Index<MatchRange> for str {
    type Output = str;

    #[inline]
    fn index(&self, index: MatchRange) -> &str {
        &self[index.start..index.end]
    }
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

struct PartialSearchResult {
    pub line_number: u64,
    pub line: String,
    pub m: MatchRange,
}

pub fn search_file<'a>(
    path: PathBuf,
    matcher: &RegexMatcher,
    searcher: &mut Searcher,
) -> anyhow::Result<FileResults> {
    let mut partial_results: Vec<PartialSearchResult> = Vec::new();

    // PERF: we could use search_file instead and handle IO ourselves
    // this would allow us to:
    // - search the file in parallel (chunking)
    // - pre-allocate the results vector based on file size / number of lines
    searcher.search_path(
        &matcher,
        &path,
        // TODO: use find_iter instead of find to find multiple matches per line
        UTF8(|lnum, line| {
            matcher.find_iter(line.as_bytes(), |m| {
                partial_results.push(PartialSearchResult {
                    line_number: lnum,
                    line: line.to_string(),
                    m: MatchRange::from_match(m),
                });
                true
            })?;
            Ok(true)
        }),
    )?;

    if partial_results.is_empty() {
        return Ok(FileResults {
            path,
            results: Vec::new(),
        });
    }
    let mut results = vec![SearchResult {
        line_number: partial_results[0].line_number,
        line: partial_results[0].line.clone(),
        line_start: partial_results[0].line_number,
        line_end: partial_results[0].line_number,
        matches: vec![partial_results[0].m.clone()],
    }];
    for partial_result in partial_results[1..].iter() {
        let last_result = results.last_mut().unwrap();
        if last_result.line_number != partial_result.line_number {
            results.push(SearchResult {
                line_number: partial_result.line_number,
                line: partial_result.line.clone(),
                line_start: partial_result.line_number,
                line_end: partial_result.line_number,
                matches: vec![partial_result.m.clone()],
            });
        } else {
            last_result.matches.push(partial_result.m.clone());
            last_result.line_end = partial_result.line_number;
        }
    }

    Ok(FileResults { path, results })
}

pub fn build_matcher(patterns: &Vec<String>) -> anyhow::Result<RegexMatcher> {
    let builder = RegexMatcherBuilder::new();
    Ok(builder.build_many(patterns)?)
}

pub fn build_searcher(multiline: bool) -> Searcher {
    let mut builder = SearcherBuilder::new();
    builder.multi_line(multiline);
    builder.build()
}
