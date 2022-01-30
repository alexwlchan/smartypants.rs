use crate::{Config, DashesConfig};

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
    let en_dash = "&#8211;";
    let em_dash = "&#8212;";

    let triple_dash_replacement = match config.triple_dash {
        DashesConfig::DoNothing => "---",
        DashesConfig::EnDash    => en_dash,
        DashesConfig::EmDash    => em_dash,
    };

    let double_dash_replacement = match config.double_dash {
        DashesConfig::DoNothing => "--",
        DashesConfig::EnDash    => en_dash,
        DashesConfig::EmDash    => em_dash,
    };

    println!("@@AWLC double_dash_replacement = {}", double_dash_replacement);

    println!("@@AWLC out = {}", text
        .replace("---", triple_dash_replacement)
        .replace("--", double_dash_replacement));

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
