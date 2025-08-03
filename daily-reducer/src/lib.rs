//! Implements the actual string manipulation functionality that powers the string reducer app.
//!
//! Of primary interest are [`GAMES`] (which contains a list of all games for which parsing is
//! implemented), and [`DailyReducer`], which exposes a convenient way of using implemented
//! parsers.

use chrono::Datelike;

mod games;

pub use games::{GAMES, Game};

/// Automatically reduces results blurbs from various daily games.
#[derive(Clone, Default)]
pub struct DailyReducer {
    items: Vec<(&'static str, String)>,
}

impl DailyReducer {
    /// Creates a new [`DailyReducer`].
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    /// Produces a single markdown-formatted block containing all results added to this reducer;
    /// ready to be posted to a forum.
    pub fn to_forum_block(&self) -> String {
        let now = chrono::Utc::now();

        let year = now.year();

        let month = match now.month() {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => unreachable!(),
        };

        let day = now.day();
        let suffix = match day {
            1 | 21 | 31 => "st",
            2 | 22 => "nd",
            3 | 23 => "rd",
            _ => "th",
        };

        let block = self
            .items
            .iter()
            .map(|(_, line)| line.as_str())
            .collect::<Vec<_>>()
            .join("\n");

        format!("{month} {day}{suffix}, {year}\n\n---\n\n{block}\n\n---\n\n")
    }

    /// Parses the provided `blurb` and stores the shortened output; returns true if successful,
    /// false otherwise. Existing shortened output for the same game is overwritten, which still
    /// counts as a success.
    pub fn insert(&mut self, blurb: &str) -> bool {
        match shorten(blurb) {
            Some((game, line)) => {
                match self.items.iter_mut().find(|(g, _)| *g == game) {
                    Some((_, l)) => *l = line,
                    None => {
                        self.items.push((game, line));
                        self.items.sort_unstable_by_key(|k| k.0);
                    }
                }
                true
            }
            None => false,
        }
    }

    /// Removes the `n`th game in the list (sorted alphabetically).
    pub fn remove(&mut self, index: usize) {
        if index < self.items.len() {
            self.items.remove(index);
        }
    }

    /// Iterates over all games that have a shortened line in this [`DailyReducer`].
    pub fn iter(&self) -> impl Iterator<Item = &(&'static str, String)> {
        self.items.iter()
    }
}

/// Shortens the results string of a daily game, and returns the name of the game alongside the
/// shortened result.
fn shorten(input: &str) -> Option<(&'static str, String)> {
    let input = input.trim();
    GAMES
        .iter()
        .filter_map(|g| (g.processor)(input).map(|r| (g.name, r)))
        .next()
}
