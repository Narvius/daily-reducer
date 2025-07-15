use std::fmt::Write as _;

/// A list of all available games.
#[rustfmt::skip]
pub const GAMES: &[Game] = &[
    Game { name: "Daily Akari", url: "https://dailyakari.com/", processor: daily_akari },
    Game { name: "Bracket City", url: "https://www.theatlantic.com/games/bracket-city/", processor: bracket_city },
    Game { name: "Duotrigordle", url: "https://duotrigordle.com/", processor: duotrigordle },
    Game { name: "Clues by Sam", url: "https://cluesbysam.com/", processor: clues_by_sam },
];

/// A single Daily Game for which blurb shortening is implemented.
#[derive(Copy, Clone)]
pub struct Game {
    /// The name of the game.
    pub name: &'static str,
    /// Website URL the game can be found at.
    pub url: &'static str,
    /// Function that shortens the results blurb of the game.
    pub processor: fn(&str) -> Option<String>,
}

fn daily_akari(input: &str) -> Option<String> {
    input.starts_with("Daily Akari 😊").then(|| {
        let time = input
            .lines()
            .nth(2)?
            .trim_end_matches('✅')
            .rsplit_once(' ')?
            .1;
        Some(format!("Daily Akari 😊 -- ✅ Solved in {time} ✅"))
    })?
}

fn bracket_city(input: &str) -> Option<String> {
    input.starts_with("[Bracket City]").then(|| {
        // After the rank line, there are 1-3 lines with stats, then a blank line, then the score.
        let mut s = "Bracket City".to_owned();
        let mut score = false;

        for line in input.lines().skip(5) {
            if score {
                match line.strip_prefix("Total Score: ") {
                    Some(score) => write!(s, " -- {score}").ok()?,
                    None => write!(s, " {line}").ok()?,
                }
            } else if let Some(rank) = line.strip_prefix("Rank:") {
                s.push_str(rank);
            } else if line.starts_with(|c: char| !c.is_alphanumeric()) {
                let (emoji, rest) = line.split_once(' ')?;
                let (_, number) = rest.rsplit_once(' ')?;
                write!(s, " {emoji} {number}").ok()?;
            } else {
                score = true;
            }
        }
        Some(s)
    })?
}

fn duotrigordle(input: &str) -> Option<String> {
    input.starts_with("Daily Duotrigordle").then(|| {
        let mut lines = input.lines().take(2);
        Some(format!(
            "{} -- {}",
            lines.next()?,
            lines.next()?.split_once(' ')?.1
        ))
    })?
}

fn clues_by_sam(input: &str) -> Option<String> {
    (input.starts_with("I solved the daily Clues by Sam") || input.starts_with("Clues by Sam"))
        .then(|| {
            let mut lines = input.lines();
            let (time, checks) = if input.starts_with("Clues by Sam") {
                lines.next();
                (lines.next()?, lines.next()?)
            } else {
                (lines.next()?.split_once(") in ")?.1, lines.next()?)
            };

            Some(format!("Clues by Sam -- {checks} in {time}"))
        })?
}

#[cfg(test)]
mod tests {
    /// Given the name of a `fn(&str) -> Option<String>` that is in scope in `super`; and a list of
    /// numbers corresponding to existing test cases; creates a test with the same name, wherein it
    /// gives the contents of "../data/{name}{test_case_nr}" to the function, and compares the
    /// output with the contents of "../data/{name}{test_case_nr}_result".
    #[cfg(test)]
    macro_rules! file_test {
    ($processor:ident, $($id:literal),+) => {
        #[test]
        fn $processor() {
            $({
                const INPUT: &str = include_str!(concat!(
                    "../data/",
                    stringify!($processor),
                    stringify!($id)
                ));
                const OUTPUT: &str = include_str!(concat!(
                    "../data/",
                    stringify!($processor),
                    stringify!($id),
                    "_result"
                ));

                assert_eq!(super::$processor(INPUT).as_deref(), Some(OUTPUT.trim()));
            })+

        }
    };
}
    file_test!(daily_akari, 1, 2);
    file_test!(bracket_city, 1, 2, 3);
    file_test!(duotrigordle, 1, 2);
    file_test!(clues_by_sam, 1, 2, 3);
}
