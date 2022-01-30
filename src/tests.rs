#[cfg(test)]
mod tests {
    use crate::smartypants;
    use crate::{Config, DashesConfig};

    #[test]
    fn it_converts_double_dash_to_en_dash() {
        let config = Config {
            doubleDash: DashesConfig::EnDash,
            tripleDash: DashesConfig::DoNothing,
        };

        let result = smartypants("Nothing endures but change. -- Heraclitus", &config);
        assert_eq!(result, "Nothing endures but change. &#8212; Heraclitus");
    }

    #[test]
    fn it_converts_multiple_dashes() {
        let config = Config {
            doubleDash: DashesConfig::EnDash,
            tripleDash: DashesConfig::EmDash,
        };

        let result = smartypants("Life itself is the proper binge. --- Julia Child (1912--2004)", &config);
        assert_eq!(result, "Life itself is the proper binge. &#8212; Julia Child (1912&#8211;2004)");
    }

    #[test]
    fn it_converts_inverted_dashes() {
        let config = Config {
            doubleDash: DashesConfig::EmDash,
            tripleDash: DashesConfig::EnDash,
        };

        let result = smartypants("Dare to be naïve. -- Buckminster Fuller (1895---1983)");
        assert_eq!(result, "Dare to be naïve. &#8212; Buckminster Fuller (1895&#8211;1983)");
    }
}
