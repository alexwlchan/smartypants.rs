use crate::{Config, DashesConfig, QuotesBehaviour};

const EN_DASH_ENTITY: &str = "&#8211;";
const EM_DASH_ENTITY: &str = "&#8212;";

const SINGLE_OPENING_QUOTE_ENTITY: &str = "&#8216;";
const SINGLE_CLOSING_QUOTE_ENTITY: &str = "&#8217;";

const DOUBLE_OPENING_QUOTE_ENTITY: &str = "&#8220;";
const DOUBLE_CLOSING_QUOTE_ENTITY: &str = "&#8221;";

/// Process the following backslash escape sequences in `text`.
///
/// This is useful if you want to force a "dumb" quote or other character
/// to appear.
///
/// +--------+-----------+-----------+
/// | Escape | Value     | Character |
/// +========+===========+===========+
/// | ``\\`` | ``&#92;`` | ``\``     |
/// +--------+-----------+-----------+
/// | ``\"`` | ``&#34;`` | ``"``     |
/// +--------+-----------+-----------+
/// | ``\'`` | ``&#39;`` | ``'``     |
/// +--------+-----------+-----------+
/// | ``\.`` | ``&#46;`` | ``.``     |
/// +--------+-----------+-----------+
/// | ``\-`` | ``&#45;`` | ``-``     |
/// +--------+-----------+-----------+
/// | ``\``` | ``&#96;`` | ``\```    |
/// +--------+-----------+-----------+
///
pub fn process_escapes(text: &str) -> String {
    text
        .replace(r"\\", "&#92;")
        .replace("\\\"", "&#34;")
        .replace(r"\'", "&#39;")
        .replace(r"\.", "&#46;")
        .replace(r"\-", "&#45;")
        .replace(r"\`", "&#96;")
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
        .replace("``", DOUBLE_OPENING_QUOTE_ENTITY)
        .replace("''", DOUBLE_CLOSING_QUOTE_ENTITY)
}

/// Converts ``single backticks'`-style quotes in `text` into HTML curly quote entities.
pub fn convert_single_backticks(text: &str) -> String {
    text
        .replace("`", SINGLE_OPENING_QUOTE_ENTITY)
        .replace("'", SINGLE_CLOSING_QUOTE_ENTITY)
}
