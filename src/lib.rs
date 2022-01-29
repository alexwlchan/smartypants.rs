#[derive(Debug)]
pub struct Config {
    pub do_quotes: bool,
    pub do_backticks: bool,
    pub do_dashes: bool,
    pub do_ellipses: bool,
    pub do_stupefy: bool,
    pub convert_quote: bool,
}

pub enum Token {
    Tag(String),
    Text(String),
}

pub fn _tokenize(html: &str) -> Vec<Token> {
    let mut pos = 0;
    let len = html.len();
    let mut tokens = vec!();

    let mut depth = 6;


    println!("{:?}", len);

    tokens
}

#[cfg(test)]
mod tests {
    use crate::_tokenize;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn it_doesnt_work() {
        _tokenize("ABC");
        assert_eq!(2, 3);
    }
}
