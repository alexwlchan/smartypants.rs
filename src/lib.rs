#[macro_use]
extern crate lazy_static;

mod tokenize;

lazy_static! {
    static ref TAGS_TO_SKIP_REGEX: Regex = Regex::new(
        "<(/)?(pre|samp|code|tt|kbd|script|style|math)[^>]*>").unwrap();
}

pub fn smartypants(text: &str) -> String {
    let mut result: Vec<&str> = vec![];
    let mut in_pre = false;

    for token in tokenize::tokenize(text) {
        match token {
            tokenize::Tag(contents) => {
                // Don't mess with quotes inside some tags.  This does not
                // handle self <closing/> tags!
                result.push(contents);
            }
        }
        println!("{:?}", token);
    }

    text.to_owned()
}