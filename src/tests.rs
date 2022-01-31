#[cfg(test)]
mod smartypants_tests {
    use crate::smartypants;
    use crate::{SubstitutionConfig, DashesSubstitution, EllipsesSubstitution, EntitiesSubstitution, QuotesSubstitution};

    #[test]
    fn it_converts_double_dash_to_en_dash() {
        let config = SubstitutionConfig {
            double_dash: DashesSubstitution::EnDash,
            triple_dash: DashesSubstitution::DoNothing,
            ellipses: EllipsesSubstitution::DoNothing,
            double_backticks: QuotesSubstitution::DoNothing,
            single_backticks: QuotesSubstitution::DoNothing,
            quote_chars: QuotesSubstitution::DoNothing,
            entities: EntitiesSubstitution::HtmlNumericEntities,
        };

        let result = smartypants("Nothing endures but change. -- Heraclitus", &config);
        assert_eq!(result, "Nothing endures but change. &#8211; Heraclitus");
    }

    #[test]
    fn it_converts_multiple_dashes() {
        let config = SubstitutionConfig {
            double_dash: DashesSubstitution::EnDash,
            triple_dash: DashesSubstitution::EmDash,
            ellipses: EllipsesSubstitution::DoNothing,
            double_backticks: QuotesSubstitution::DoNothing,
            single_backticks: QuotesSubstitution::DoNothing,
            quote_chars: QuotesSubstitution::DoNothing,
            entities: EntitiesSubstitution::HtmlNumericEntities,
        };

        let result = smartypants("Life itself is the proper binge. --- Julia Child (1912--2004)", &config);
        assert_eq!(result, "Life itself is the proper binge. &#8212; Julia Child (1912&#8211;2004)");
    }

    #[test]
    fn it_converts_inverted_dashes() {
        let config = SubstitutionConfig {
            double_dash: DashesSubstitution::EmDash,
            triple_dash: DashesSubstitution::EnDash,
            ellipses: EllipsesSubstitution::DoNothing,
            double_backticks: QuotesSubstitution::DoNothing,
            single_backticks: QuotesSubstitution::DoNothing,
            quote_chars: QuotesSubstitution::DoNothing,
            entities: EntitiesSubstitution::HtmlNumericEntities,
        };

        let result = smartypants("Dare to be naïve. -- Buckminster Fuller (1895---1983)", &config);
        assert_eq!(result, "Dare to be naïve. &#8212; Buckminster Fuller (1895&#8211;1983)");
    }

    #[test]
    fn it_converts_ellipses() {
        let config = SubstitutionConfig {
            double_dash: DashesSubstitution::DoNothing,
            triple_dash: DashesSubstitution::DoNothing,
            ellipses: EllipsesSubstitution::ConvertToEntity,
            double_backticks: QuotesSubstitution::DoNothing,
            single_backticks: QuotesSubstitution::DoNothing,
            quote_chars: QuotesSubstitution::DoNothing,
            entities: EntitiesSubstitution::HtmlNumericEntities,
        };

        let result = smartypants("Huh...?", &config);
        assert_eq!(result, "Huh&#8230;?");

        let result = smartypants("Huh. . .?", &config);
        assert_eq!(result, "Huh&#8230;?");
    }

    #[test]
    fn it_skips_ellipses_if_not_enabled() {
        let config = SubstitutionConfig {
            double_dash: DashesSubstitution::DoNothing,
            triple_dash: DashesSubstitution::DoNothing,
            ellipses: EllipsesSubstitution::DoNothing,
            double_backticks: QuotesSubstitution::DoNothing,
            single_backticks: QuotesSubstitution::DoNothing,
            quote_chars: QuotesSubstitution::DoNothing,
            entities: EntitiesSubstitution::HtmlNumericEntities,
        };

        let result = smartypants("Huh...?", &config);
        assert_eq!(result, "Huh...?");

        let result = smartypants("Huh. . .?", &config);
        assert_eq!(result, "Huh. . .?");
    }

    #[test]
    fn it_converts_backticks() {
        let config = SubstitutionConfig {
            double_dash: DashesSubstitution::DoNothing,
            triple_dash: DashesSubstitution::DoNothing,
            ellipses: EllipsesSubstitution::DoNothing,
            double_backticks: QuotesSubstitution::ConvertToCurly,
            single_backticks: QuotesSubstitution::DoNothing,
            quote_chars: QuotesSubstitution::DoNothing,
            entities: EntitiesSubstitution::HtmlNumericEntities,
        };

        let result = smartypants("``Isn't this fun?''", &config);
        assert_eq!(result, "&#8220;Isn't this fun?&#8221;");

        let config = SubstitutionConfig {
            double_dash: DashesSubstitution::DoNothing,
            triple_dash: DashesSubstitution::DoNothing,
            ellipses: EllipsesSubstitution::DoNothing,
            double_backticks: QuotesSubstitution::ConvertToCurly,
            single_backticks: QuotesSubstitution::ConvertToCurly,
            quote_chars: QuotesSubstitution::DoNothing,
            entities: EntitiesSubstitution::HtmlNumericEntities,
        };

        let result = smartypants("``Isn't this fun?''", &config);
        assert_eq!(result, "&#8220;Isn&#8217;t this fun?&#8221;");

        let result = smartypants("`Isn't this fun?'", &config);
        assert_eq!(result, "&#8216;Isn&#8217;t this fun?&#8217;");
    }
}
