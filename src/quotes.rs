// This file contains the code for converting normal quotes (`"` and `'`)
// into HTML curly quote entities.
//
// This logic is more complicated than the other converters, and I found
// it useful to break it up into smaller functions to keep the converters
// file relatively simple.

use regex::Regex;
use fancy_regex::{Regex as FancyRegex};

use crate::entities::*;

/// Handle the special case of a single-character ' token.
///
/// If the last character of the previous token was whitespace,
/// then this is an opening quote, otherwise it's a closing quote.
///
/// e.g. if the previous token was "hello ", then we'd do "hello‘",
/// whereas if the previous token was "isn", then we'd do "isn’".
///
pub fn handle_single_straight_quote_token(prev_token_last_char: &Option<char>) -> String {
    if is_whitespace(prev_token_last_char) {
        OPENING_SINGLE_CURLY_QUOTE_ENTITY.to_string()
    } else {
        CLOSING_SINGLE_CURLY_QUOTE_ENTITY.to_string()
    }
}

/// Handle the special case of a single-character " token.
///
/// We apply the same logic as `handle_single_straight_quote_token`, but
/// with the HTML entities for double quotes.
pub fn handle_double_straight_quote_token(prev_token_last_char: &Option<char>) -> String {
    if is_whitespace(prev_token_last_char) {
        OPENING_DOUBLE_CURLY_QUOTE_ENTITY.to_string()
    } else {
        CLOSING_DOUBLE_CURLY_QUOTE_ENTITY.to_string()
    }
}

/// Handle the case where the first character is a quote, followed by
/// punctuation at a non-word break.
///
/// Close the quotes by brute force.
pub fn handle_leading_quote_with_punctuation(text: &str) -> String {

    // Note: in the Perl and Python implementations of SmartyPants,
    // these regexes use a lookahead assertion, i.e. `(?=...)`.
    lazy_static! {
        static ref FIRST_SINGLE_QUOTE_RE: FancyRegex =
            FancyRegex::new(r#"^'(?=[[:punct:]]]\B)"#).unwrap();

        static ref FIRST_DOUBLE_QUOTE_RE: FancyRegex =
            FancyRegex::new(r#"^"(?=[[[:punct:]]]\B)"#).unwrap();
    }

    let text = (*FIRST_SINGLE_QUOTE_RE).replace(&text, CLOSING_SINGLE_CURLY_QUOTE_ENTITY);
    let text = (*FIRST_DOUBLE_QUOTE_RE).replace(&text, CLOSING_DOUBLE_CURLY_QUOTE_ENTITY);

    text.to_string()
}

/// Handle the special case for double sets of quotes, e.g.:
///
/// <p>They said, "'Quoted' words in a larger quote."</p>
///
pub fn handle_double_sets_of_quotes(text: &str) -> String {

    // Note: as above, the regex in the Perl/Python implementations uses
    // lookahead assertions, but we can't use them in Rust.
    lazy_static! {
        static ref DOUBLE_THEN_SINGLE_QUOTE: FancyRegex =
            FancyRegex::new(r#""'(?=[[:word:]])"#).unwrap();

        static ref SINGLE_THEN_DOUBLE_QUOTE: FancyRegex =
            FancyRegex::new(r#"'"(?=[[:word:]])"#).unwrap();
    }

    let text = (*DOUBLE_THEN_SINGLE_QUOTE).replace(
        &text,
        format!("{}{}", OPENING_DOUBLE_CURLY_QUOTE_ENTITY, OPENING_SINGLE_CURLY_QUOTE_ENTITY)
    );
    let text = (*SINGLE_THEN_DOUBLE_QUOTE).replace(
        &text,
        format!("{}{}", OPENING_SINGLE_CURLY_QUOTE_ENTITY, OPENING_DOUBLE_CURLY_QUOTE_ENTITY)
    );

    text.to_string()
}

/// Handle decade abbreviations, e.g. "the '80s"
pub fn handle_decade_abbreviations(text: &str) -> String {
    lazy_static! {
        static ref DECADE_RE: FancyRegex =
            FancyRegex::new(r#"\b'(?=\d{2}s)"#).unwrap();
    }

    let text = (*DECADE_RE).replace(&text, format!("{}$decade", CLOSING_SINGLE_CURLY_QUOTE_ENTITY));

    text.to_string()
}

pub fn handle_opening_single_quotes(text: &str) -> String {

    lazy_static! {

        // [[:space:]]  a whitespace char, or
        // &nbsp;       a non-breaking space entity, or
        // --           dashes, or
        // &[mn]dash;   named dash entities, or
        //              en/em dash decimal entities, or
        //              en/em dash hex entities
        //
        // '            the quote
        //
        // [[:word:]]   followed by a word character\
        //
        static ref OPENING_SINGLE_QUOTE_RE: FancyRegex =
            FancyRegex::new(&format!(
                r#"(?P<prefix>[[:space:]]|&nbsp;|--|&[mn]dash;|{}|{}|{}|{};)'(?=[[:word:]])"#,
                EN_DASH_ENTITY,
                EM_DASH_ENTITY,
                EN_DASH_HEX_ENTITY,
                EM_DASH_HEX_ENTITY
            )).unwrap();
    }

    let text = (*OPENING_SINGLE_QUOTE_RE).replace(&text, format!("$prefix{}", OPENING_SINGLE_CURLY_QUOTE_ENTITY));

    text.to_string()
}

pub fn handle_closing_single_quotes(text: &str) -> String {

    lazy_static! {
        // This includes lookaheads for a whitespace char or an 's'
        // at a word ending position.
        //
        // This is a special case to handle something like
        // "<i>Custer</i>'s Last Stand.".
        static ref CLOSING_SINGLE_QUOTE_RE: FancyRegex =
            FancyRegex::new(&format!(r#"(?P<close_class>{})?'((?P=close_class)|(?=\s|\s\b))"#, r#"[^ \t\r\n\[\{\(\-]"#)).unwrap();
    }

    let text = (*CLOSING_SINGLE_QUOTE_RE).replace(&text, format!("$close_class{}", CLOSING_SINGLE_CURLY_QUOTE_ENTITY));

    text.to_string()
}

/// Handle any single quotes left after the previous two handlers have
/// been called.
///
/// At this point, any remaining single quotes should be opening ones.
pub fn handle_remaining_single_quotes(text: &str) -> String {
    text.replace("'", OPENING_SINGLE_CURLY_QUOTE_ENTITY)
}

pub fn handle_opening_double_quotes(text: &str) -> String {
    lazy_static! {

        // [[:space:]]  a whitespace char, or
        // &nbsp;       a non-breaking space entity, or
        // --           dashes, or
        // &[mn]dash;   named dash entities, or
        //              en/em dash decimal entities, or
        //              en/em dash hex entities
        //
        // '            the quote
        //
        // [[:word:]]   followed by a word character
        //
        static ref OPENING_DOUBLE_QUOTE_RE: FancyRegex =
            FancyRegex::new(&format!(
                r#"(?P<prefix>[[:space:]]|&nbsp;|--|&[mn]dash;|{}|{}|{}|{})"(?=[[:word:]])"#,
                EN_DASH_ENTITY,
                EM_DASH_ENTITY,
                EN_DASH_HEX_ENTITY,
                EM_DASH_HEX_ENTITY
            )).unwrap();
    }

    let text = (*OPENING_DOUBLE_QUOTE_RE).replace(
        &text, format!("$prefix{}", OPENING_DOUBLE_CURLY_QUOTE_ENTITY)
    );

    text.to_string()
}

pub fn handle_closing_double_quotes(text: &str) -> String {
    lazy_static! {
        static ref CLOSING_DOUBLE_QUOTE_RE: FancyRegex =
            FancyRegex::new(&format!(r#"(?P<close_class>{})?"((?P=close_class)|(?=[[:space:]])|$)"#, r#"[^\ \t\r\n\[\{\(\-]"#)).unwrap();
    }

    let text = (*CLOSING_DOUBLE_QUOTE_RE).replace(&text, format!("$close_class{}", CLOSING_DOUBLE_CURLY_QUOTE_ENTITY));

    text.to_string()
}

/// Handle any double quotes left after the previous two handlers have
/// been called.
///
/// At this point, any remaining double quotes should be opening ones.
pub fn handle_remaining_double_quotes(text: &str) -> String {
    text.replace("\"", OPENING_DOUBLE_CURLY_QUOTE_ENTITY)
}

/// Returns true if `c` is whitespace, false otherwise
fn is_whitespace(maybe_char: &Option<char>) -> bool {
    match maybe_char {
        Some(c) => Regex::new(r"\s").unwrap().is_match(&c.to_string()),
        _       => false,
    }
}
