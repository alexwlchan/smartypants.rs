// This file implements a tokenizer for HTML strings.
//
// In particular, it splits a string into tag tokens and text tokens.
// This is important because we want to treat the two differently when
// applying the Smartypants conversions:
//
//  1.  Text tokens may have conversions applied, but tag tokens
//      should never have conversions applied.
//
//      e.g. we'd convert Text("isn't this nice") to "isn’t this nice",
//      but we'd leave Tag(<a href="example.com">) as-is – applying any
//      conversions there would potentially break the document.
//
//  2.  We only want to apply conversions inside some tags, e.g. we don't
//      want to change the contents of <pre> tags.
//
//      By extracting the tags separately, we can inspect them later
//      to see whether we're inside a tag where we don't want to do any
//      text conversions.
//
// The tests at the bottom of this file give some examples of how HTML strings
// get tokenized.

use regex::Regex;

#[derive(Debug)]
pub enum Token {
    Tag(String),
    Text(String),
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Tag(tag_self), Token::Tag(tag_other))     => tag_self == tag_other,
            (Token::Text(text_self), Token::Text(text_other)) => text_self == text_other,
            _ => false,
        }
    }
}

// Note: this regex is slightly different to the one used by
// Smartypants.pl or Leo Hemsted's library.
//
// In particular:
//
//    - it uses named capture groups for clarity in the code that follows
//    - the 'tag' group is optional -- that allows us to get any text
//      that comes after the final tag as the final Capture, rather
//      than slicing into the string
//
lazy_static! {
    static ref TAG_SOUP: Regex = Regex::new("(?P<text>[^<]*)(?P<tag><!--.*?--\\s*>|<[^>]*>)?").unwrap();
}

/// Returns an array of tokens comprising the input string.
///
/// Each token is either a tag (possibly with nested tags contained therein,
/// such as <a href="<MTFoo>">), or a run of text between tags.
///
/// Based on the following previous implementations:
///
///   * The _tokenize() function from Leo Hemsted's Smartypants Python
///     library.
///     https://github.com/leohemsted/smartypants.py/blob/c46d26c559d706b6e0aa423190ab2d6edf1fdfcd/smartypants.py#L556-L608
///
///   * The _tokenize subroutine in John Gruber's SmartyPants Perl script.
///     https://daringfireball.net/projects/smartypants/
///
///   * The _tokenize subroutine from Brad Choate's MTRegex plugin.
///     https://web.archive.org/web/20041215155739/http://bradchoate.com/weblog/2002/07/27/mtregex
///
pub fn tokenize(text: &str) -> Vec<Token> {
    let mut tokens = vec!();

    for cap in (*TAG_SOUP).captures_iter(text) {
        if !cap["text"].is_empty() {
            tokens.push(Token::Text(cap["text"].to_owned()));
        }

        // if -- in text part of comment, then it's not a comment, therefore it
        // should be converted.
        //
        // In HTML4 [1]:
        //   [...] Authors should avoid putting two or more adjacent hyphens
        //   inside comments.
        //
        // In HTML5 [2]:
        //   [...] the comment may have text, with the additional restriction
        //   that the text must not [...], nor contain two consecutive U+002D
        //   HYPHEN-MINUS characters (--)
        //
        // [1]: http://www.w3.org/TR/REC-html40/intro/sgmltut.html#h-3.2.4
        // [2]: http://www.w3.org/TR/html5/syntax.html#comments
        //
        match cap.name("tag") {
            Some(tag_match) => {
                let tag = tag_match.as_str();

                if is_comment(tag) && comment_text(tag).contains("--") {
                    tokens.push(Token::Text(cap["tag"].to_owned()));
                    continue;
                }

                tokens.push(Token::Tag(cap["tag"].to_owned()));
            },
            _ => (),
        };
    }

    tokens
}

/// Returns true if `tag` is an HTML comment, false otherwise.
fn is_comment(tag: &str) -> bool {
    tag.starts_with("<!--")
}

/// Returns the contents of an HTML comment
fn comment_text(comment: &str) -> &str {
    // remove --[white space]> from the end of tag
    comment
        .trim_start_matches("<!--")
        .trim_end_matches('>')
        .trim_end()
        .trim_end_matches('-')
}

#[cfg(test)]
mod tests {
    use crate::tokenize::{tokenize, Token};

    #[test]
    fn it_handles_a_simple_string() {
        let result = tokenize("This is a vanilla string");
        let expected = vec![Token::Text(String::from("This is a vanilla string"))];
        assert_eq!(result, expected);
    }

    #[test]
    fn it_handles_a_single_tag() {
        let result = tokenize("<p>This is a paragraph</p>");
        let expected = vec![
            Token::Tag(String::from("<p>")),
            Token::Text(String::from("This is a paragraph")),
            Token::Tag(String::from("</p>")),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn it_treats_a_comment_as_a_tag() {
        let result = tokenize("<!-- This is a comment -->");
        let expected = vec![
            Token::Tag(String::from("<!-- This is a comment -->")),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn it_treats_a_comment_with_two_dashes_as_text() {
        let result = tokenize("<!-- This is a comment with -- two dashes -->");
        let expected = vec![
            Token::Text(String::from("<!-- This is a comment with -- two dashes -->")),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn it_handles_a_complex_example() {
        let result = tokenize("Some text <em>with emphasis</em> and <span class=\"big\">inline attributes</span>.");
        let expected = vec![
            Token::Text(String::from("Some text ")),
            Token::Tag(String::from("<em>")),
            Token::Text(String::from("with emphasis")),
            Token::Tag(String::from("</em>")),
            Token::Text(String::from(" and ")),
            Token::Tag(String::from("<span class=\"big\">")),
            Token::Text(String::from("inline attributes")),
            Token::Tag(String::from("</span>")),
            Token::Text(String::from(".")),
        ];
        assert_eq!(result, expected);
    }
}
