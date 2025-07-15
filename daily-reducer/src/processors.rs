use std::fmt::Write as _;

#[rustfmt::skip]
pub const PROCESSORS: &[Processor] = &[
    p("Daily Akari", "https://dailyakari.com/", daily_akari),
    p("Bracket City", "https://www.theatlantic.com/games/bracket-city/", bracket_city),
    p("Duotrigordle", "https://duotrigordle.com/", duotrigordle),
    p("Clues by Sam", "https://cluesbysam.com/", clues_by_sam),
];

#[derive(Copy, Clone)]
pub struct Processor {
    pub name: &'static str,
    pub url: &'static str,
    pub process: fn(&str) -> Option<String>,
}

const fn p(
    name: &'static str,
    url: &'static str,
    process: fn(&str) -> Option<String>,
) -> Processor {
    Processor { name, url, process }
}

// ========================
// ========================

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
    /// Given a test name and a [`Processor`], checks that it correctly [detects] it can process an
    /// input file; and that the output of [processing it] matches the expected output. The two files
    /// describing the input and output are in the `data` directory in the project root; and are named
    /// `{test name}` and `{test name}_result`, respectively.
    ///
    /// [detects]: Processor::detect
    /// [processing it]: Processor::process
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
