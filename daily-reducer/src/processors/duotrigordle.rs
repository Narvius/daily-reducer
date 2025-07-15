pub struct Duotrigordle;

impl super::Processor for Duotrigordle {
    fn name(&self) -> &'static str {
        "Duotrigordle"
    }

    fn url(&self) -> &'static str {
        "https://duotrigordle.com/"
    }

    fn detect(&self, block: &str) -> bool {
        block.starts_with("Daily Duotrigordle")
    }

    fn process(&self, block: &str) -> Option<String> {
        let mut lines = block.lines().take(2);
        Some(format!(
            "{} -- {}",
            lines.next()?,
            lines.next()?.split_once(' ')?.1
        ))
    }
}

#[cfg(test)]
mod tests {
    crate::processors::file_test!(duotrigordle1, super::Duotrigordle);
    crate::processors::file_test!(duotrigordle2, super::Duotrigordle);
}
