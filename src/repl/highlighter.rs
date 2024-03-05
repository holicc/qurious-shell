use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use std::{borrow::Cow, collections::HashSet};

pub struct ShellHighlighter<'a> {
    keywords: HashSet<&'a str>,
    bracket_highlighter: MatchingBracketHighlighter,
}

impl<'a> ShellHighlighter<'a> {
    pub fn new() -> Self {
        Self {
            keywords: HashSet::from_iter(vec![
                "select",
                "from"
            ]),
            bracket_highlighter: MatchingBracketHighlighter::new(),
        }
    }
}

impl<'a> Highlighter for ShellHighlighter<'a> {
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        let mut line = self.bracket_highlighter.highlight(line, pos).into_owned();

        for kw in &self.keywords {
            line = line.replace(kw, &format!("\x1b[1;31m{}\x1b[0m", kw));
        }

        Cow::Owned(line)
    }

    fn highlight_char(&self, line: &str, pos: usize, forced: bool) -> bool {
        for kw in &self.keywords {
            if line.contains(kw) {
                return true;
            }
        }
        self.bracket_highlighter.highlight_char(line, pos, forced)
    }
}
