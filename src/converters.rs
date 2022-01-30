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
/// # Examples
///
/// ```
/// # use smartypants::converters::process_escapes;
/// let result = process_escapes(r"\\");
/// // "&#92;"
/// # assert_eq!(result, String::from("&#92;"));
///
/// # use smartypants::smartypants;
/// let result = smartypants("\"smarty\" \\\"pants\\\"");
/// // "&#8220;smarty&#8221; &#34;pants&#34;"
/// # assert_eq!(result, String::from("&#8220;smarty&#8221; &#34;pants&#34;"));
/// ```
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

/// Convert `--` and `---` into HTML entities.
pub fn convert_dashes(text: &str, config: &Config) -> String {
    let enDash = "&#8211;";
    let emDash = "&#8212;";

    let triple_dash_replacement = match config.tripleDash {
        DashesConfig::DoNothing => "---",
        DashesConfig::EnDash    => enDash,
        DashesConfig::EmDash    => emDash,
    };

    let double_dash_replacement = match config.doubleDash {
        DashesConfig::DoNothing => "--",
        DashesConfig::EnDash    => enDash,
        DashesConfig::EmDash    => emDash,
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
