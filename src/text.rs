//! Measuring and wrapping text, and picking the rendering that fits.

use unicode_width::UnicodeWidthChar;

#[must_use]
pub fn char_width(c: char) -> i64 {
    c.width().unwrap_or(1).max(1) as i64
}

pub fn width(s: &str) -> i64 {
    s.chars().map(char_width).sum()
}

/// The width of the widest line: the columns a block of text needs.
pub fn block_width(s: &str) -> i64 {
    s.lines().map(width).max().unwrap_or(0)
}

/// Cut to at most `w` columns, ending in `…` if anything was dropped.
pub fn truncate(s: &str, w: i64) -> String {
    if width(s) <= w {
        return s.to_string();
    }
    let mut used = 0;
    let mut out: String = s
        .chars()
        .take_while(|&c| {
            used += char_width(c);
            used < w
        })
        .collect();
    if w > 0 {
        out.push('…');
    }
    out
}

/// Greedy word wrap at `max` columns. Words longer than `max` are split only
/// if `split` is set.
pub fn wrap(s: &str, max: usize, split: bool) -> Vec<String> {
    let opts = textwrap::Options::new(max.max(1))
        .break_words(split)
        .word_separator(textwrap::WordSeparator::AsciiSpace)
        .word_splitter(textwrap::WordSplitter::NoHyphenation);
    let out: Vec<String> = textwrap::wrap(&s.split_whitespace().collect::<Vec<_>>().join(" "), opts)
        .into_iter()
        .map(String::from)
        .collect();
    if out.is_empty() {
        vec![String::new()]
    } else {
        out
    }
}

/// A rendering and whether it met the column budget.
pub struct Fit {
    pub text: String,
    pub width: usize,
    pub fits: bool,
}

/// `line` without a trailing `%% comment`. A line that starts with `%%` is
/// left alone: it is a whole-line comment or a `%% highlight:` / `%% stack:`
/// directive.
pub(crate) fn strip_comment(line: &str) -> &str {
    let at = line.match_indices("%%").map(|(i, _)| i).find(|&i| i > 0 && line[..i].ends_with(char::is_whitespace));
    at.map_or(line, |i| line[..i].trim_end())
}

/// `s` trimmed, without one pair of matching surrounding quotes (`"…"` or `'…'`).
pub(crate) fn unquote(s: &str) -> String {
    let s = s.trim();
    let quoted = ['"', '\''].iter().any(|&q| s.len() >= 2 && s.starts_with(q) && s.ends_with(q));
    if quoted { &s[1..s.len() - 1] } else { s }.to_string()
}
