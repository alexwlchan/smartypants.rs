#[macro_use]
extern crate lazy_static;

use regex::Regex;

pub mod config;
pub mod converters;
mod tokenize;
mod tests;

use config::{Config, DashesConfig, QuotesBehaviour};
use tokenize::Token;

// This is used to match tags where we don't want to do any corrections.
//
// This list of tags is taken from the Python implementation, adding a few
// newer HTML tags that aren't skipped in the latest version of Markdown.pl.
//
lazy_static! {
    static ref TAGS_TO_SKIP_REGEX: Regex = Regex::new(
        "<(?P<closing_slash>/)?(?P<tag_name>pre|samp|code|tt|kbd|script|style|math)[^>]*>").unwrap();
}

fn handle_tag_token(contents: String, result: &mut Vec<String>, skipped_tag_stack: &mut Vec<String>) -> () {
    result.push(contents.to_owned());

    // Don't mess with quotes inside some tags, e.g. we don't
    // want to change the contents of a <pre>.
    //
    // If we detect we're looking at a tag we want to skip, we
    // update `in_skipped_tag`.  This will allow us to skip
    // doing any processing when we get a Text token.
    //
    // Note: this doesn't handle self </closing> tags.
    //
    let skip_match = (*TAGS_TO_SKIP_REGEX).captures(&contents);
    if skip_match.is_none() {
        return;
    }

    let m = skip_match.unwrap();

    let tag_name = m.name("tag_name").unwrap().as_str();
    let is_closing_tag = m.name("closing_slash").is_some();

    // If this isn't a closing tag, it must be an opening tag.
    // Stick it on the stack, and continue.
    if !is_closing_tag {
        skipped_tag_stack.push(tag_name.to_string());
    }
    // This is a closing tag, and there's already something
    // on the skipped tag stack.  If this is the last tag on
    // the stack, pop from the stack.
    //
    // Note: there's a comment in the Python implementation
    // that says:
    //
    //      This close doesn't match the open.  This isn't
    //      XHTML.  We should barf here.
    //
    // But in practice, it lets that error pass silently --
    // and so if this closing tag doesn't match the last
    // open on the stack, we also ignore it here.
    //
    else if !skipped_tag_stack.is_empty() {
        if skipped_tag_stack.last().unwrap() == tag_name {
            skipped_tag_stack.pop();
        }
    }
}

fn handle_text_token(text: String, config: &Config, prev_token_last_char: &mut Option<char>, result: &mut Vec<String>, in_skipped_tag: bool) -> () {

    // Remember the last character of this token before processing.
    //
    // We know that text-tokens are non-empty, so the unwrap() is safe here.
    let last_char = text.chars().last().unwrap();

    let processed_text = if in_skipped_tag {
        text
    } else {
        let text = converters::process_escapes(&text);
        let text = converters::convert_dashes(&text, config);

        let text = if config.ellipses {
            converters::convert_ellipses(&text)
        } else {
            text
        };

        // Note: backticks need to be processed before quotes, and double
        // backticks need to be processed before single backticks.
        let text = match config.double_backticks {
            QuotesBehaviour::ConvertToCurly => converters::convert_double_backticks(&text),
            QuotesBehaviour::DoNothing      => text,
        };

        let text = match config.single_backticks {
            QuotesBehaviour::ConvertToCurly => converters::convert_single_backticks(&text),
            QuotesBehaviour::DoNothing      => text,
        };

        text
    };

    *prev_token_last_char = Some(last_char);
    result.push(processed_text);
}

pub fn smartypants(text: &str, config: &Config) -> String {
    let mut result: Vec<String> = vec![];

    // Records whether we're in any skipped tags where we don't
    // want to do text processing.
    let mut skipped_tag_stack: Vec<String> = vec![];

    // This is a cheat, used to get some context for one-character tokens
    // that consist of a single quote character.  We remember the last
    // character of the previous text token, to use as context to curl
    // single-character quote tokens correctly.
    let mut prev_token_last_char: Option<char> = None;

    for token in tokenize::tokenize(text) {
        match token {
            Token::Tag(contents) => handle_tag_token(contents, &mut result, &mut skipped_tag_stack),
            Token::Text(contents) => {
                let in_skipped_tag = !skipped_tag_stack.is_empty();
                handle_text_token(contents, &config, &mut prev_token_last_char, &mut result, in_skipped_tag);
            },
        }
    }

    result.join("")
}
