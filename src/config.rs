#[derive(Debug)]
pub enum DashesSubstitution {
    /// Leave dashes as-is
    DoNothing,

    /// Convert dashes to [en-dashes](https://en.wikipedia.org/wiki/Dash#En_dash)
    EnDash,

    /// Convert dashes to [em-dashes](https://en.wikipedia.org/wiki/Dash#Em_dash)
    EmDash,
}

#[derive(Debug)]
pub enum EllipsesSubstitution {
    /// Leave ellipses as-is
    DoNothing,

    /// Convert ellipses to their HTML entities
    ConvertToEntity,
}

#[derive(Debug)]
pub enum QuotesSubstitution {
    /// Leave quotes/backticks as-is
    DoNothing,

    /// Convert quotes/backticks to their curly equivalents
    ConvertToCurly,
}

#[derive(Debug)]
pub enum EntitiesSubstitution {
    /// Use Unicode characters (e.g. He said “Hello world”)
    UnicodeCharacters,

    /// Use HTML numeric entities (e.g. She said &#8220;Hello world&#8221;)
    HtmlNumericEntities,

    /// Use HTML named entities (e.g. They said &ldquo;Hello world&rdquo;)
    HtmlNamedEntities,

    /// Use ASCII equivalents (e.g. Xe said "Hello world")
    AsciiEquivalents,
}

pub struct SubstitutionConfig {
    /// Whether to convert double dashes (`--`) to en/em dashes
    pub double_dash: DashesSubstitution,

    /// Whether to convert triple dashes (`---`) to en/em dashes
    pub triple_dash: DashesSubstitution,

    /// Whether to convert ellipses (`...`) into ellipsis HTML entities
    pub ellipses: EllipsesSubstitution,

    /// Whether to convert double backticks (```backticks''`) to curly quotes
    pub double_backticks: QuotesSubstitution,

    /// Whether to convert single backticks (``single'`) to curly quotes
    pub single_backticks: QuotesSubstitution,

    /// Whether to convert normal quotes (`"` and `'`) to curly quotes
    pub quote_chars: QuotesSubstitution,

    /// What to convert
    pub entities: EntitiesSubstitution,
}
