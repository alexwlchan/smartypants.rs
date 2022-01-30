#[macro_use]
extern crate lazy_static;

use regex::Regex;

mod tokenize;

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

pub fn smartypants(text: &str) -> String {
    let mut result: Vec<String> = vec![];
    let mut in_skipped_tag = false;

    let mut skipped_tag_stack: Vec<String> = vec![];

    for token in tokenize::tokenize(text) {
        match token {
            Token::Tag(contents) => {
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
                    continue;
                }

                let m = skip_match.unwrap();

                let tag_name = m.name("tag_name").unwrap().as_str();
                let is_closing_tag = m.name("closing_slash").is_some();

                // If this isn't a closing tag, it must be an opening tag.
                // Stick it on the stack, and continue.
                if !is_closing_tag {
                    skipped_tag_stack.push(tag_name.to_string());
                    in_skipped_tag = true;
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

                        // If there are no skipped tags left on the stack,
                        // then we're no longer in a skipped tag.
                        //
                        // We should do processing on the next token.
                        if skipped_tag_stack.is_empty() {
                            in_skipped_tag = true;
                        }
                    }
                }
            },
            Token::Text(contents) => (),
        }
    }

    text.to_owned()
}

#[cfg(test)]
mod tests {
    use crate::smartypants;

    #[test]
    fn it_handles_a_simple_string() {
        let result = smartypants("This is a simple string");
        let expected = "This is a simple string";
        assert_eq!(result, expected.to_owned());
    }

    #[test]
    fn it_handles_another_simple_string() {
        let result = smartypants("<p>He said Hello</p>");
        let expected = "<p>He said Hello</p>";
        assert_eq!(1, 0);
    }

    #[test]
    fn it_skips_tags_even_if_they_have_quotes() {
        let result = smartypants("<pre>This isn't text</pre>");
        let expected = "<pre>This isn't text</pre>";
        assert_eq!(result, expected.to_owned());
        assert_eq!(1, 0);
    }
}
