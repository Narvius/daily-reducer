pub struct CluesBySam;

impl super::Processor for CluesBySam {
    fn name(&self) -> &'static str {
        "Clues by Sam"
    }

    fn detect(&self, block: &str) -> bool {
        block.starts_with("I solved the daily Clues by Sam") || block.starts_with("Clues by Sam")
    }

    fn process(&self, block: &str) -> Option<String> {
        let mut lines = block.lines();
        let (time, checks) = if block.starts_with("Clues by Sam") {
            lines.next();
            (lines.next()?, lines.next()?)
        } else {
            (lines.next()?.split_once(") in ")?.1, lines.next()?)
        };

        Some(format!("Clues by Sam -- {checks} in {time}"))
    }
}

#[cfg(test)]
mod tests {
    crate::processors::file_test!(clues_by_sam1, super::CluesBySam);
    crate::processors::file_test!(clues_by_sam2, super::CluesBySam);
    crate::processors::file_test!(clues_by_sam3, super::CluesBySam);
}
