pub struct DailyAkari;

impl super::Processor for DailyAkari {
    fn name(&self) -> &'static str {
        "Daily Akari"
    }

    fn detect(&self, block: &str) -> bool {
        block.starts_with("Daily Akari 😊")
    }

    fn process(&self, block: &str) -> Option<String> {
        let time = block.lines().nth(2)?.trim_end_matches('✅').rsplit_once(' ')?.1;
        Some(format!("Daily Akari 😊 -- ✅ Solved in {time} ✅"))
    }
}

#[cfg(test)]
mod tests {
    crate::processors::file_test!(daily_akari1, super::DailyAkari);
    crate::processors::file_test!(daily_akari2, super::DailyAkari);
}
