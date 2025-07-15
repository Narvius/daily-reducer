use std::fmt::Write;

pub struct BracketCity;

impl super::Processor for BracketCity {
    fn name(&self) -> &'static str {
        "Bracket City"
    }

    fn url(&self) -> &'static str {
        "https://www.theatlantic.com/games/bracket-city/"
    }

    fn detect(&self, block: &str) -> bool {
        block.starts_with("[Bracket City]")
    }

    fn process(&self, block: &str) -> Option<String> {
        // After the rank line, there are 1-3 lines with stats, then a blank line, then the score.
        let mut s = "Bracket City".to_owned();
        let mut score = false;

        for line in block.lines().skip(5) {
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
    }
}

#[cfg(test)]
mod tests {
    crate::processors::file_test!(bracket_city1, super::BracketCity);
    crate::processors::file_test!(bracket_city2, super::BracketCity);
    crate::processors::file_test!(bracket_city3, super::BracketCity);
}
