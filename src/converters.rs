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
