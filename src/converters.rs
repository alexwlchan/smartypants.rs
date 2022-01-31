use regex::Regex;

use crate::{Config, DashesBehaviour, EntitiesBehaviour, QuotesBehaviour};
use crate::entities::*;
use crate::quotes;

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
        .replace("...", ELLIPSIS_ENTITY)
        .replace(". . .", ELLIPSIS_ENTITY)
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
        "\'" => quotes::handle_single_straight_quote_token(prev_token_last_char),
        "\"" => quotes::handle_double_straight_quote_token(prev_token_last_char),

        _ => {
            let text = quotes::handle_leading_quote_with_punctuation(&text);
            let text = quotes::handle_double_sets_of_quotes(&text);
            let text = quotes::handle_decade_abbreviations(&text);

            let text = quotes::handle_opening_single_quotes(&text);
            let text = quotes::handle_closing_single_quotes(&text);
            let text = quotes::handle_remaining_single_quotes(&text);

            let text = quotes::handle_opening_closing_quotes(&text);
            let text = quotes::handle_closing_double_quotes(&text);
            let text = quotes::handle_remaining_double_quotes(&text);

            text.to_string()
        }
    }
}

/// Converts numeric character references to other entities, if desired.
///
/// It would be more efficient to plumb the EntitiesBehaviour into all
/// the converter functions and substitute the correct choice of entity
/// when we initially add it, but that diverges from the original design
/// of SmartyPants much more substantially.
pub fn convert_entities(text: &str, entities_behaviour: &EntitiesBehaviour) -> String {
    match entities_behaviour {
        EntitiesBehaviour::UnicodeCharacters =>
            text
                .replace(EN_DASH_ENTITY, "–")
                .replace(EM_DASH_ENTITY, "—")
                .replace(OPENING_SINGLE_CURLY_QUOTE_ENTITY, "‘")
                .replace(CLOSING_SINGLE_CURLY_QUOTE_ENTITY, "’")
                .replace(OPENING_DOUBLE_CURLY_QUOTE_ENTITY, "“")
                .replace(OPENING_DOUBLE_CURLY_QUOTE_ENTITY, "”"),

        EntitiesBehaviour::HtmlNumericEntities => text.to_string(),

        EntitiesBehaviour::HtmlNamedEntities =>
            text
                .replace(EN_DASH_ENTITY, "&ndash;")
                .replace(EM_DASH_ENTITY, "&mdash;")
                .replace(OPENING_SINGLE_CURLY_QUOTE_ENTITY, "&lsquo;")
                .replace(CLOSING_SINGLE_CURLY_QUOTE_ENTITY, "&rsquo;")
                .replace(OPENING_DOUBLE_CURLY_QUOTE_ENTITY, "&ldquo;")
                .replace(OPENING_DOUBLE_CURLY_QUOTE_ENTITY, "&rdquo;"),

        EntitiesBehaviour::AsciiEquivalents =>
            text
                .replace(EN_DASH_ENTITY, "-")
                .replace(EM_DASH_ENTITY, "--")
                .replace(OPENING_SINGLE_CURLY_QUOTE_ENTITY, "'")
                .replace(CLOSING_SINGLE_CURLY_QUOTE_ENTITY, "'")
                .replace(OPENING_DOUBLE_CURLY_QUOTE_ENTITY, "\"")
                .replace(OPENING_DOUBLE_CURLY_QUOTE_ENTITY, "\""),
    }
}
