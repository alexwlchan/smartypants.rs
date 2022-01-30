#[derive(Debug)]
pub enum DashesConfig {
    /// Leave dashes as-is
    DoNothing,

    /// Convert dashes to [en-dashes](https://en.wikipedia.org/wiki/Dash#En_dash)
    EnDash,

    /// Convert dashes to [em-dashes](https://en.wikipedia.org/wiki/Dash#Em_dash)
    EmDash,
}

pub struct Config {
    /// How to convert double dashes (`--`)
    pub double_dash: DashesConfig,

    /// How to convert triple dashes (`---`)
    pub triple_dash: DashesConfig,

    /// Whether to convert ellipses (`...`) into ellipsis HTML entities
    pub ellipses: bool,
}
