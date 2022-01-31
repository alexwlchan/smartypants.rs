#[cfg(test)]
mod smartypants_tests {
    use crate::smartypants;
    use crate::{Config, DashesBehaviour, EllipsesBehaviour, EntitiesBehaviour, QuotesBehaviour};

    #[test]
    fn it_converts_double_dash_to_en_dash() {
        let config = Config {
            double_dash: DashesBehaviour::EnDash,
            triple_dash: DashesBehaviour::DoNothing,
            ellipses: EllipsesBehaviour::DoNothing,
            double_backticks: QuotesBehaviour::DoNothing,
            single_backticks: QuotesBehaviour::DoNothing,
            quote_chars: QuotesBehaviour::DoNothing,
            entities: EntitiesBehaviour::HtmlNumericEntities,
        };

        let result = smartypants("Nothing endures but change. -- Heraclitus", &config);
        assert_eq!(result, "Nothing endures but change. &#8211; Heraclitus");
    }

    #[test]
    fn it_converts_multiple_dashes() {
        let config = Config {
            double_dash: DashesBehaviour::EnDash,
            triple_dash: DashesBehaviour::EmDash,
            ellipses: EllipsesBehaviour::DoNothing,
            double_backticks: QuotesBehaviour::DoNothing,
            single_backticks: QuotesBehaviour::DoNothing,
            quote_chars: QuotesBehaviour::DoNothing,
            entities: EntitiesBehaviour::HtmlNumericEntities,
        };

        let result = smartypants("Life itself is the proper binge. --- Julia Child (1912--2004)", &config);
        assert_eq!(result, "Life itself is the proper binge. &#8212; Julia Child (1912&#8211;2004)");
    }

    #[test]
    fn it_converts_inverted_dashes() {
        let config = Config {
            double_dash: DashesBehaviour::EmDash,
            triple_dash: DashesBehaviour::EnDash,
            ellipses: EllipsesBehaviour::DoNothing,
            double_backticks: QuotesBehaviour::DoNothing,
            single_backticks: QuotesBehaviour::DoNothing,
            quote_chars: QuotesBehaviour::DoNothing,
            entities: EntitiesBehaviour::HtmlNumericEntities,
        };

        let result = smartypants("Dare to be naïve. -- Buckminster Fuller (1895---1983)", &config);
        assert_eq!(result, "Dare to be naïve. &#8212; Buckminster Fuller (1895&#8211;1983)");
    }

    #[test]
    fn it_converts_ellipses() {
        let config = Config{
            double_dash: DashesBehaviour::DoNothing,
            triple_dash: DashesBehaviour::DoNothing,
            ellipses: EllipsesBehaviour::ConvertToEntity,
            double_backticks: QuotesBehaviour::DoNothing,
            single_backticks: QuotesBehaviour::DoNothing,
            quote_chars: QuotesBehaviour::DoNothing,
            entities: EntitiesBehaviour::HtmlNumericEntities,
        };

        let result = smartypants("Huh...?", &config);
        assert_eq!(result, "Huh&#8230;?");

        let result = smartypants("Huh. . .?", &config);
        assert_eq!(result, "Huh&#8230;?");
    }

    #[test]
    fn it_skips_ellipses_if_not_enabled() {
        let config = Config{
            double_dash: DashesBehaviour::DoNothing,
            triple_dash: DashesBehaviour::DoNothing,
            ellipses: EllipsesBehaviour::DoNothing,
            double_backticks: QuotesBehaviour::DoNothing,
            single_backticks: QuotesBehaviour::DoNothing,
            quote_chars: QuotesBehaviour::DoNothing,
            entities: EntitiesBehaviour::HtmlNumericEntities,
        };

        let result = smartypants("Huh...?", &config);
        assert_eq!(result, "Huh...?");

        let result = smartypants("Huh. . .?", &config);
        assert_eq!(result, "Huh. . .?");
    }

    #[test]
    fn it_converts_backticks() {
        let config = Config{
            double_dash: DashesBehaviour::DoNothing,
            triple_dash: DashesBehaviour::DoNothing,
            ellipses: EllipsesBehaviour::DoNothing,
            double_backticks: QuotesBehaviour::ConvertToCurly,
            single_backticks: QuotesBehaviour::DoNothing,
            quote_chars: QuotesBehaviour::DoNothing,
            entities: EntitiesBehaviour::HtmlNumericEntities,
        };

        let result = smartypants("``Isn't this fun?''", &config);
        assert_eq!(result, "&#8220;Isn't this fun?&#8221;");

        let config = Config{
            double_dash: DashesBehaviour::DoNothing,
            triple_dash: DashesBehaviour::DoNothing,
            ellipses: EllipsesBehaviour::DoNothing,
            double_backticks: QuotesBehaviour::ConvertToCurly,
            single_backticks: QuotesBehaviour::ConvertToCurly,
            quote_chars: QuotesBehaviour::DoNothing,
            entities: EntitiesBehaviour::HtmlNumericEntities,
        };

        let result = smartypants("``Isn't this fun?''", &config);
        assert_eq!(result, "&#8220;Isn&#8217;t this fun?&#8221;");

        let result = smartypants("`Isn't this fun?'", &config);
        assert_eq!(result, "&#8216;Isn&#8217;t this fun?&#8217;");
    }
}
