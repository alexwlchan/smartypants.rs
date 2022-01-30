use regex::Regex;

#[derive(Debug)]
enum Token {
    Tag(String),
    Text(String),
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Tag(tagSelf), Token::Tag(tagOther))     => tagSelf == tagOther,
            (Token::Text(textSelf), Token::Text(textOther)) => textSelf == textOther,
            _ => false,
        }
    }
}

/// Returns an array of tokens comprising the input string.
///
/// Each token is either a tag (possibly with nested tags contained therein,
/// such as <a href="<MTFoo>">), or a run of text between tags.
///
/// Based on the _tokenize() subroutine from Brad Choate's MTRegex plugin.
/// <http://www.bradchoate.com/past/mtregex.php>
///
/// Based on the _tokenize() function from Leo Hemsted's Smartypants
/// Python library.
/// <https://github.com/leohemsted/smartypants.py/blob/c46d26c559d706b6e0aa423190ab2d6edf1fdfcd/smartypants.py#L556-L608>
///
fn tokenize(html: &str) -> Vec<Token> {
    let mut tokens = vec!();

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
    let tag_soup = Regex::new("(?P<text>[^<]*)(?P<tag><!--.*?--\\s*>|<[^>]*>)?").unwrap();

    for cap in tag_soup.captures_iter(html) {
        if cap["text"].len() > 0 {
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
                if tag.starts_with("<!--") {
                    // remove --[white space]> from the end of tag
                    let tag_contents = tag.trim_start_matches("<!--").trim_end_matches(">").trim_end().trim_end_matches("-");

                    if tag_contents.contains("--") {
                        tokens.push(Token::Text(cap["tag"].to_owned()));
                        continue;
                    }
                }

                tokens.push(Token::Tag(cap["tag"].to_owned()));
            },
            _ => (),
        };
    }

    tokens
}

#[cfg(test)]
mod tests {
    use crate::tokenize::{tokenize, Token};

    #[test]
    fn it_handles_a_simple_string() {
        let result = tokenize("This is a vanilla string");
        assert_eq!(result, vec![Token::Text(String::from("This is a vanilla string"))]);
    }

    #[test]
    fn it_handles_a_single_tag() {
        let result = tokenize("<p>This is a paragraph</p>");
        assert_eq!(result, vec![Token::Tag(String::from("<p>")), Token::Text(String::from("This is a paragraph")), Token::Tag(String::from("</p>"))]);
    }

    #[test]
    fn it_doesnt_work() {
        println!("{:?}", tokenize("This isn’t very good <a> <img> Hello I’m Lexie"));
        println!("{:?}", tokenize("<p>"));
        assert_eq!(2, 3);
    }
}
