use chrono::Datelike;

mod processors;

pub use processors::{PROCESSORS, Processor};

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

    pub fn insert(&mut self, blurb: &str) -> bool {
        shorten(blurb)
            .map(|(game, line)| {
                match self.items.iter_mut().find(|(g, _)| *g == game) {
                    Some((_, l)) => *l = line,
                    None => {
                        self.items.push((game, line));
                        self.items.sort_unstable_by_key(|k| k.0);
                    }
                }
                true
            })
            .unwrap_or(false)
    }

    pub fn remove(&mut self, index: usize) {
        if index < self.items.len() {
            self.items.remove(index);
        }
    }

    pub fn get(&self, game: &str) -> Option<&(&'static str, String)> {
        self.items.iter().find(|(g, _)| *g == game)
    }

    pub fn iter(&self) -> impl Iterator<Item = &(&'static str, String)> {
        self.items.iter()
    }
}

/// Returns the names and URLs of all supported daily games.
pub fn supported_games() -> Vec<(&'static str, &'static str)> {
    PROCESSORS.iter().map(|p| (p.name, p.url)).collect()
}

/// Shortens the results string of a daily game, and returns the name of the game alongside the
/// shortened result.
fn shorten(input: &str) -> Result<(&'static str, String), Error> {
    let input = input.trim();
    let mut results = PROCESSORS
        .iter()
        .filter_map(|p| (p.process)(input).map(|r| (p.name, r)));

    match (results.next(), results.next()) {
        (Some(r), None) => Ok(r),
        (_, Some(_)) => Err(Error::MultipleResults),
        (None, None) => Err(Error::NoResults),
    }
}

/// Errors that can occur whilst shortening.
enum Error {
    /// No processor returned a useable result.
    NoResults,
    /// Multiple processors returned useable results.
    MultipleResults,
}
