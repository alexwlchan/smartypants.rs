#[derive(Debug)]
pub enum DashesBehaviour {
    /// Leave dashes as-is
    DoNothing,

    /// Convert dashes to [en-dashes](https://en.wikipedia.org/wiki/Dash#En_dash)
    EnDash,

    /// Convert dashes to [em-dashes](https://en.wikipedia.org/wiki/Dash#Em_dash)
    EmDash,
}

#[derive(Debug)]
pub enum EllipsesBehaviour {
    /// Leave ellipses as-is
    DoNothing,

    /// Convert ellipses to their HTML entities
    ConvertToEntity,
}

#[derive(Debug)]
pub enum QuotesBehaviour {
    /// Leave quotes/backticks as-is
    DoNothing,

    /// Convert quotes/backticks to their curly equivalents
    ConvertToCurly,
}

#[derive(Debug)]
pub enum EntitiesBehaviour {
    /// Use Unicode characters (e.g. He said “Hello world”)
    UnicodeCharacters,

    /// Use HTML numeric entities (e.g. She said &#8220;Hello world&#8221;)
    HtmlNumericEntities,

    /// Use HTML named entities (e.g. They said &ldquo;Hello world&rdquo;)
    HtmlNamedEntities,

    /// Use ASCII equivalents (e.g. Xe said "Hello world")
    AsciiEquivalents,
}

pub struct Config {
    /// Whether to convert double dashes (`--`) to en/em dashes
    pub double_dash: DashesBehaviour,

    /// Whether to convert triple dashes (`---`) to en/em dashes
    pub triple_dash: DashesBehaviour,

    /// Whether to convert ellipses (`...`) into ellipsis HTML entities
    pub ellipses: EllipsesBehaviour,

    /// Whether to convert double backticks (```backticks''`) to curly quotes
    pub double_backticks: QuotesBehaviour,

    /// Whether to convert single backticks (``single'`) to curly quotes
    pub single_backticks: QuotesBehaviour,

    /// Whether to convert normal quotes (`"` and `'`) to curly quotes
    pub quote_chars: QuotesBehaviour,

    /// What to convert
    pub entities: EntitiesBehaviour,
}
