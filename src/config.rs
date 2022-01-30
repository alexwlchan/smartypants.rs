#[derive(Debug)]
pub enum DashesConfig {
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

pub struct Config {
    /// How to convert double dashes (`--`)
    pub double_dash: DashesConfig,

    /// How to convert triple dashes (`---`)
    pub triple_dash: DashesConfig,

    /// Whether to convert ellipses (`...`) into ellipsis HTML entities
    pub ellipses: EllipsesBehaviour,

    /// Whether to convert double backticks (```backticks''`) to curly quotes
    pub double_backticks: QuotesBehaviour,

    /// Whether to convert single backticks (``single'`) to curly quotes
    pub single_backticks: QuotesBehaviour,
}
