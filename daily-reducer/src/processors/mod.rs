pub mod bracket_city;
pub mod clues_by_sam;
pub mod daily_akari;
pub mod duotrigordle;

pub const ALL_PROCESSORS: &[&dyn Processor] = &[
    &daily_akari::DailyAkari,
    &bracket_city::BracketCity,
    &duotrigordle::Duotrigordle,
    &clues_by_sam::CluesBySam,
];

/// General trait for processing Daily Game result snippets.
pub trait Processor {
    /// The name of the game.
    fn name(&self) -> &'static str;
    /// URL to the game.
    fn url(&self) -> &'static str;
    /// Whether this processor applies to the given piece of text.
    fn detect(&self, block: &str) -> bool;
    /// Summarize the Daily Game snippet into one line.
    fn process(&self, block: &str) -> Option<String>;
}

/// Given a test name and a [`Processor`], checks that it correctly [detects] it can process an
/// input file; and that the output of [processing it] matches the expected output. The two files
/// describing the input and output are in the `data` directory in the project root; and are named
/// `{test name}` and `{test name}_result`, respectively.
///
/// [detects]: Processor::detect
/// [processing it]: Processor::process
#[cfg(test)]
macro_rules! file_test {
    ($test_name:ident, $processor:expr) => {
        #[test]
        fn $test_name() {
            use $crate::processors::Processor;

            const INPUT: &str = include_str!(concat!("../../data/", stringify!($test_name)));
            const OUTPUT: &str =
                include_str!(concat!("../../data/", stringify!($test_name), "_result"));

            assert!($processor.detect(INPUT));
            assert_eq!($processor.process(INPUT).as_deref(), Some(OUTPUT.trim()));
        }
    };
}

#[cfg(test)]
pub(crate) use file_test;
