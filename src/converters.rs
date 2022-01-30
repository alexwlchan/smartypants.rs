use crate::{Config, DashesConfig, QuotesBehaviour};

const SINGLE_STRAIGHT_QUOTE_ENTITY: &str = "&#39";          // '
const DOUBLE_STRAIGHT_QUOTE_ENTITY: &str = "&#34";          // "

const HYPHEN_ENTITY: &str = "&#45;";                        // -
const FULL_STOP_ENTITY: &str = "&#46";                      // .

const SINGLE_BACKSLASH_ENTITY: &str = "&#92;";              // \

const BACKTICK_ENTITY: &str = "&#96;";                      // `

const EN_DASH_ENTITY: &str = "&#8211;";                     // –
const EM_DASH_ENTITY: &str = "&#8212;";                     // —

const OPENING_SINGLE_CURLY_QUOTE_ENTITY: &str = "&#8216;";  // ‘
const CLOSING_SINGLE_CURLY_QUOTE_ENTITY: &str = "&#8217;";  // ’

const OPENING_DOUBLE_CURLY_QUOTE_ENTITY: &str = "&#8220;";  // “
const CLOSING_DOUBLE_CURLY_QUOTE_ENTITY: &str = "&#8221;";  // ”

/// Apply a series of backslash escapes in `text`.
///
/// This is useful if you want to force a "dumb" quote or other character
/// to appear.
pub fn process_escapes(text: &str) -> String {
    text
        .replace(r"\\",  SINGLE_BACKSLASH_ENTITY)
        .replace("\\\"", DOUBLE_STRAIGHT_QUOTE_ENTITY)
        .replace(r"\'",  SINGLE_STRAIGHT_QUOTE_ENTITY)
        .replace(r"\.",  FULL_STOP_ENTITY)
        .replace(r"\-",  HYPHEN_ENTITY)
        .replace(r"\`",  BACKTICK_ENTITY)
}

/// Convert `--` and `---` in `text` into HTML entities.
pub fn convert_dashes(text: &str, config: &Config) -> String {
    let triple_dash_replacement = match config.triple_dash {
        DashesConfig::DoNothing => "---",
        DashesConfig::EnDash    => EN_DASH_ENTITY,
        DashesConfig::EmDash    => EM_DASH_ENTITY,
    };

    let double_dash_replacement = match config.double_dash {
        DashesConfig::DoNothing => "--",
        DashesConfig::EnDash    => EN_DASH_ENTITY,
        DashesConfig::EmDash    => EM_DASH_ENTITY,
    };

    // Note: we have to do the triple dash replacement before the
    // double dash replacement, otherwise we'll get weird results.
    //
    // e.g. "a---b" could become "a&#8211;-b" rather than "a&#8212;b".
    //
    text
        .replace("---", triple_dash_replacement)
        .replace("--", double_dash_replacement)
}

/// Converts `...` in `text` into ellipsis HTML entities.
pub fn convert_ellipses(text: &str) -> String {
    text
        .replace("...", "&#8230;")
        .replace(". . .", "&#8230;")
}

/// Converts ```double backticks''`-style quotes in `text` into HTML curly quote entities.
pub fn convert_double_backticks(text: &str) -> String {
    text
        .replace("``", OPENING_DOUBLE_CURLY_QUOTE_ENTITY)
        .replace("''", CLOSING_DOUBLE_CURLY_QUOTE_ENTITY)
}

/// Converts ``single backticks'`-style quotes in `text` into HTML curly quote entities.
pub fn convert_single_backticks(text: &str) -> String {
    text
        .replace("`", OPENING_SINGLE_CURLY_QUOTE_ENTITY)
        .replace("'", CLOSING_SINGLE_CURLY_QUOTE_ENTITY)
}
