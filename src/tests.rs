#![allow(warnings)]

use crate::smartypants;
use crate::config::{SubstitutionConfig, DashesSubstitution, EllipsesSubstitution, EntitiesSubstitution, QuotesSubstitution, SubstitutionConfigHelpers};

macro_rules! smartypants_tests {
    ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected, config) = $value;
                assert_eq!(expected, smartypants(input, &config));
            }
        )*
    }
}

smartypants_tests! {
    double_dash_to_en_dash: (
        "Nothing endures but change. -- Heraclitus",
        "Nothing endures but change. &#8211; Heraclitus",
        SubstitutionConfig::default()
    ),

    multiple_dashes: (
        "Life itself is the proper binge. --- Julia Child (1912--2004)",
        "Life itself is the proper binge. &#8212; Julia Child (1912&#8211;2004)",
        SubstitutionConfig::default()
    ),

    can_skip_dashes_substitution: (
        "Life itself is the proper binge. --- Julia Child (1912--2004)",
        "Life itself is the proper binge. --- Julia Child (1912--2004)",
        SubstitutionConfig::default()
            .with_double_dash(DashesSubstitution::DoNothing)
            .with_triple_dash(DashesSubstitution::DoNothing),
    ),

    dashes_and_quotes: (
        r#""foo" -- bar"#,
        r#"&#8220;foo&#8221; &#8211; bar"#,
        SubstitutionConfig::default()
    ),

    dashes_and_quotes_with_quotes_disabled: (
        r#""foo" -- bar"#,
        r#""foo" &#8211; bar"#,
        SubstitutionConfig::default()
            .with_quote_chars(QuotesSubstitution::DoNothing)
    ),

    mixed_quotes: (
        r#""Isn't this fun?""#,
        r#"&#8220;Isn&#8217;t this fun?&#8221;"#,
        SubstitutionConfig::default()
    ),

    mixed_quotes_with_quotes_disabled: (
        r#""Isn't this fun?""#,
        r#""Isn't this fun?""#,
        SubstitutionConfig::default()
            .with_quote_chars(QuotesSubstitution::DoNothing)
    ),
    //
    // backticks: (
    //     r#"``Isn't this fun?''"#,
    //     r#"&#8220;Isn't this fun?&#8221;"#,
    //     SubstitutionConfig::default(),
    // ),
    //
    // backticks_with_backticks_disabled: (
    //     r#"``Isn't this fun?''"#,
    //     r#"``Isn't this fun?''"#,
    //     SubstitutionConfig::default()
    //         .with_backticks(QuotesSubstitution::DoNothing),
    // ),
}


//     }
//
//     #[test]
//     fn it_converts_inverted_dashes() {
//         let config = SubstitutionConfig {
//             double_dash: DashesSubstitution::EmDash,
//             triple_dash: DashesSubstitution::EnDash,
//             ellipses: EllipsesSubstitution::DoNothing,
//             double_backticks: QuotesSubstitution::DoNothing,
//             single_backticks: QuotesSubstitution::DoNothing,
//             quote_chars: QuotesSubstitution::DoNothing,
//             entities: EntitiesSubstitution::HtmlNumericEntities,
//         };
//
//         let result = smartypants("Dare to be naïve. -- Buckminster Fuller (1895---1983)", &config);
//         assert_eq!(result, "Dare to be naïve. &#8212; Buckminster Fuller (1895&#8211;1983)");
//     }
//
//     #[test]
//     fn it_converts_ellipses() {
//         let config = SubstitutionConfig {
//             double_dash: DashesSubstitution::DoNothing,
//             triple_dash: DashesSubstitution::DoNothing,
//             ellipses: EllipsesSubstitution::ConvertToEntity,
//             double_backticks: QuotesSubstitution::DoNothing,
//             single_backticks: QuotesSubstitution::DoNothing,
//             quote_chars: QuotesSubstitution::DoNothing,
//             entities: EntitiesSubstitution::HtmlNumericEntities,
//         };
//
//         let result = smartypants("Huh...?", &config);
//         assert_eq!(result, "Huh&#8230;?");
//
//         let result = smartypants("Huh. . .?", &config);
//         assert_eq!(result, "Huh&#8230;?");
//     }
//
//     #[test]
//     fn it_skips_ellipses_if_not_enabled() {
//         let config = SubstitutionConfig {
//             double_dash: DashesSubstitution::DoNothing,
//             triple_dash: DashesSubstitution::DoNothing,
//             ellipses: EllipsesSubstitution::DoNothing,
//             double_backticks: QuotesSubstitution::DoNothing,
//             single_backticks: QuotesSubstitution::DoNothing,
//             quote_chars: QuotesSubstitution::DoNothing,
//             entities: EntitiesSubstitution::HtmlNumericEntities,
//         };
//
//         let result = smartypants("Huh...?", &config);
//         assert_eq!(result, "Huh...?");
//
//         let result = smartypants("Huh. . .?", &config);
//         assert_eq!(result, "Huh. . .?");
//     }
//
//     #[test]
//     fn it_converts_backticks() {
//         let config = SubstitutionConfig {
//             double_dash: DashesSubstitution::DoNothing,
//             triple_dash: DashesSubstitution::DoNothing,
//             ellipses: EllipsesSubstitution::DoNothing,
//             double_backticks: QuotesSubstitution::ConvertToCurly,
//             single_backticks: QuotesSubstitution::DoNothing,
//             quote_chars: QuotesSubstitution::DoNothing,
//             entities: EntitiesSubstitution::HtmlNumericEntities,
//         };
//
//         let result = smartypants("``Isn't this fun?''", &config);
//         assert_eq!(result, "&#8220;Isn't this fun?&#8221;");
//
//         let config = SubstitutionConfig {
//             double_dash: DashesSubstitution::DoNothing,
//             triple_dash: DashesSubstitution::DoNothing,
//             ellipses: EllipsesSubstitution::DoNothing,
//             double_backticks: QuotesSubstitution::ConvertToCurly,
//             single_backticks: QuotesSubstitution::ConvertToCurly,
//             quote_chars: QuotesSubstitution::DoNothing,
//             entities: EntitiesSubstitution::HtmlNumericEntities,
//         };
//
//         let result = smartypants("``Isn't this fun?''", &config);
//         assert_eq!(result, "&#8220;Isn&#8217;t this fun?&#8221;");
//
//         let result = smartypants("`Isn't this fun?'", &config);
//         assert_eq!(result, "&#8216;Isn&#8217;t this fun?&#8217;");
//     }
// }
