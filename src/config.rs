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

    /// Whether to convert backticks (```backticks''`) to curly quotes
    pub backticks: QuotesSubstitution,

    /// Whether to convert normal quotes (`"` and `'`) to curly quotes
    pub quote_chars: QuotesSubstitution,

    /// What to convert
    pub entities: EntitiesSubstitution,
}

impl Default for SubstitutionConfig {
    fn default() -> Self {
        SubstitutionConfig {
            double_dash: DashesSubstitution::EnDash,
            triple_dash: DashesSubstitution::EmDash,
            ellipses: EllipsesSubstitution::ConvertToEntity,
            backticks: QuotesSubstitution::ConvertToCurly,
            quote_chars: QuotesSubstitution::ConvertToCurly,
            entities: EntitiesSubstitution::HtmlNumericEntities,
        }
    }
}

trait SubstitutionConfigHelpers {
    fn with_double_dash(self, substitution: DashesSubstitution) -> Self;
    fn with_triple_dash(self, substitution: DashesSubstitution) -> Self;
    fn with_ellipses(self, substitution: EllipsesSubstitution) -> Self;
    fn with_backticks(self, substitution: QuotesSubstitution) -> Self;
    fn with_quote_chars(self, substitution: QuotesSubstitution) -> Self;
    fn with_entities(self, substitution: EntitiesSubstitution) -> Self;
}

impl SubstitutionConfigHelpers for SubstitutionConfig {
    fn with_double_dash(self, substitution: DashesSubstitution) -> Self {
        SubstitutionConfig {
            double_dash: substitution,
            ..self
        }
    }

    fn with_triple_dash(self, substitution: DashesSubstitution) -> Self {
        SubstitutionConfig {
            triple_dash: substitution,
            ..self
        }
    }

    fn with_ellipses(self, substitution: EllipsesSubstitution) -> Self {
        SubstitutionConfig {
            ellipses: substitution,
            ..self
        }
    }

    fn with_backticks(self, substitution: QuotesSubstitution) -> Self {
        SubstitutionConfig {
            backticks: substitution,
            ..self
        }
    }

    fn with_quote_chars(self, substitution: QuotesSubstitution) -> Self {
        SubstitutionConfig {
            quote_chars: substitution,
            ..self
        }
    }

    fn with_entities(self, substitution: EntitiesSubstitution) -> Self {
        SubstitutionConfig {
            entities: substitution,
            ..self
        }
    }
}
