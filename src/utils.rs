use fancy_regex::{Regex as FancyRegex};

pub fn create_re(s: &str) -> FancyRegex {
    FancyRegex::new(s).unwrap()
}
