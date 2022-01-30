#[cfg(test)]
mod smartypants_tests {
    use crate::smartypants;
    use crate::{Config, DashesConfig};

    #[test]
    fn it_converts_double_dash_to_en_dash() {
        let config = Config {
            double_dash: DashesConfig::EnDash,
            triple_dash: DashesConfig::DoNothing,
            ellipses: false,
        };

        let result = smartypants("Nothing endures but change. -- Heraclitus", &config);
        assert_eq!(result, "Nothing endures but change. &#8211; Heraclitus");
    }

    #[test]
    fn it_converts_multiple_dashes() {
        let config = Config {
            double_dash: DashesConfig::EnDash,
            triple_dash: DashesConfig::EmDash,
            ellipses: false,
        };

        let result = smartypants("Life itself is the proper binge. --- Julia Child (1912--2004)", &config);
        assert_eq!(result, "Life itself is the proper binge. &#8212; Julia Child (1912&#8211;2004)");
    }

    #[test]
    fn it_converts_inverted_dashes() {
        let config = Config {
            double_dash: DashesConfig::EmDash,
            triple_dash: DashesConfig::EnDash,
            ellipses: false,
        };

        let result = smartypants("Dare to be naïve. -- Buckminster Fuller (1895---1983)", &config);
        assert_eq!(result, "Dare to be naïve. &#8212; Buckminster Fuller (1895&#8211;1983)");
    }

    #[test]
    fn it_converts_ellipses() {
        let config = Config{
            double_dash: DashesConfig::DoNothing,
            triple_dash: DashesConfig::DoNothing,
            ellipses: true,
        };

        let result = smartypants("Huh...?", &config);
        assert_eq!(result, "Huh&#8230;?");

        let result = smartypants("Huh. . .?", &config);
        assert_eq!(result, "Huh&#8230;?");
    }

    #[test]
    fn it_skips_ellipses_if_not_enabled() {
        let config = Config{
            double_dash: DashesConfig::DoNothing,
            triple_dash: DashesConfig::DoNothing,
            ellipses: false,
        };

        let result = smartypants("Huh...?", &config);
        assert_eq!(result, "Huh...?");

        let result = smartypants("Huh. . .?", &config);
        assert_eq!(result, "Huh. . .?");
    }
}
