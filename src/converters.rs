use regex::Regex;

use crate::{Config, DashesBehaviour, QuotesBehaviour};

const SINGLE_STRAIGHT_QUOTE_ENTITY: &str = "&#39";          // '
const DOUBLE_STRAIGHT_QUOTE_ENTITY: &str = "&#34";          // "

const HYPHEN_ENTITY: &str = "&#45;";                        // -
const FULL_STOP_ENTITY: &str = "&#46";                      // .

const SINGLE_BACKSLASH_ENTITY: &str = "&#92;";              // \

const BACKTICK_ENTITY: &str = "&#96;";                      // `

const EN_DASH_ENTITY: &str = "&#8211;";                     // –
const EM_DASH_ENTITY: &str = "&#8212;";                     // —

const EN_DASH_HEX_ENTITY: &str = "&#x2013;";                // –
const EM_DASH_HEX_ENTITY: &str = "&#x2014;";                // —

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
        DashesBehaviour::DoNothing => "---",
        DashesBehaviour::EnDash    => EN_DASH_ENTITY,
        DashesBehaviour::EmDash    => EM_DASH_ENTITY,
    };

    let double_dash_replacement = match config.double_dash {
        DashesBehaviour::DoNothing => "--",
        DashesBehaviour::EnDash    => EN_DASH_ENTITY,
        DashesBehaviour::EmDash    => EM_DASH_ENTITY,
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

/// Converts normal quotes (`"` and `'`) into HTML curly quote entities.
pub fn convert_quotes(text: &str, prev_token_last_char: &Option<char>) -> String {
    match text {
        "\'" =>
            // Special case: single-character ' token
            //
            // If the last character of the previous token was whitespace,
            // then this is an opening quote, otherwise it's a closing quote.
            //
            // e.g. if the previous token was "hello ", then we'd do "hello‘",
            // whereas if the previous token was "isn", then we'd do "isn’".
            //
            if is_whitespace(prev_token_last_char) {
                return OPENING_SINGLE_CURLY_QUOTE_ENTITY.to_string();
            } else {
                return CLOSING_SINGLE_CURLY_QUOTE_ENTITY.to_string();
            },

        "\"" =>
            // Special case: single-character " token.
            //
            // We apply the same logic as the previous case, but we use the
            // double quote entities.
            if is_whitespace(prev_token_last_char) {
                return OPENING_DOUBLE_CURLY_QUOTE_ENTITY.to_string();
            } else {
                return CLOSING_DOUBLE_CURLY_QUOTE_ENTITY.to_string();
            },

        _ => {
            // Special case if the very first character is a quote character
            // followed by punctuation at a non-word break.  Close the
            // quotes by brute force.
            //
            // Note: in the Perl and Python implementations, this uses a
            // lookahead assertion, i.e. `(?=...)`.  They aren't supported
            // in the Regex Rust crate, so instead we capture the punctuation
            // and include it in the replacement.
            lazy_static! {
                static ref FIRST_SINGLE_QUOTE_RE: Regex =
                    Regex::new(r#"^'(?P<punctuation>[[:punct]]]\B)"#).unwrap();

                static ref FIRST_DOUBLE_QUOTE_RE: Regex =
                    Regex::new(r#"^"(?P<punctuation>[[[:punct]]]\B)"#).unwrap();
            }

            let text = (*FIRST_SINGLE_QUOTE_RE).replace(&text, format!("{}$punctuation", CLOSING_SINGLE_CURLY_QUOTE_ENTITY));
            let text = (*FIRST_DOUBLE_QUOTE_RE).replace(&text, format!("{}$punctuation", CLOSING_DOUBLE_CURLY_QUOTE_ENTITY));

            // Special case for double sets of quotes, e.g.:
            //
            //      <p>They said, "'Quoted' words in a larger quote."</p>
            //
            // Note: as above, we have to work around the lack of lookahead
            // assertions in the Rust crate.
            lazy_static! {
                static ref DOUBLE_THEN_SINGLE_QUOTE: Regex =
                    Regex::new(r#""'(?P<word>[[:word:]])"#).unwrap();

                static ref SINGLE_THEN_DOUBLE_QUOTE: Regex =
                    Regex::new(r#"'"(?P<word>[[:word:]])"#).unwrap();
            }

            let text = (*DOUBLE_THEN_SINGLE_QUOTE).replace(&text, format!("{}{}$word", OPENING_DOUBLE_CURLY_QUOTE_ENTITY, OPENING_SINGLE_CURLY_QUOTE_ENTITY));
            let text = (*SINGLE_THEN_DOUBLE_QUOTE).replace(&text, format!("{}{}$word", OPENING_SINGLE_CURLY_QUOTE_ENTITY, OPENING_DOUBLE_CURLY_QUOTE_ENTITY));

            // Special case for decade abbrevations (the '80s)
            lazy_static! {
                static ref DECADE_RE: Regex =
                    Regex::new(r#"\b'(?P<decade>\d{2}s)"#).unwrap();
            }

            let text = (*DECADE_RE).replace(&text, format!("{}$decade", CLOSING_SINGLE_CURLY_QUOTE_ENTITY));

            // Get most single opening quotes.
            lazy_static! {
                // [[:space:]]  a whitespace char, or
                // &nbsp;       a non-breaking space entity, or
                // --           dashes, or
                // &[mn]dash;   named dash entities, or
                //              en/em dash decimal entities, or
                //              en/em dash hex entities
                static ref OPENING_SINGLE_QUOTE_RE: Regex =
                    Regex::new(&format!("(?P<prefix>[[:space:]]|&nbsp;|--|&[mn]dash;|{}|{}|{}|{};)'(?P<word>[[:word:]])", EN_DASH_ENTITY, EM_DASH_ENTITY, EN_DASH_HEX_ENTITY, EM_DASH_HEX_ENTITY)).unwrap();
            }

            let text = (*OPENING_SINGLE_QUOTE_RE).replace(&text, format!("$prefix{}$text", OPENING_SINGLE_CURLY_QUOTE_ENTITY));
            
            // Get most single closing quotes.
            // lazy_static! {
            //     static ref CLOSING_SINGLE_QUOTE)RE: Regex =
            //         Regex::new(r#"(?P<prefix>[^\ \t\r\n\[\{\(\-])'"#)
            // }

            text.to_string()
        }
    }
}

// https://github.com/leohemsted/smartypants.py/blob/c46d26c559d706b6e0aa423190ab2d6edf1fdfcd/smartypants.py#L323-L339

/// Returns true if `c` is whitespace, false otherwise
fn is_whitespace(maybeChar: &Option<char>) -> bool {
    match maybeChar {
        Some(c) => Regex::new(r"\s").unwrap().is_match(&c.to_string()),
        _       => false,
    }
}
