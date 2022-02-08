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

    backticks: (
        r#"``Isn't this fun?''"#,
        r#"&#8220;Isn't this fun?&#8221;"#,
        SubstitutionConfig::default()
            .with_quote_chars(QuotesSubstitution::DoNothing),
    ),

    backticks_with_backticks_disabled: (
        r#"``Isn't this fun?''"#,
        r#"``Isn't this fun?''"#,
        SubstitutionConfig::default()
            .with_double_backticks(QuotesSubstitution::DoNothing)
            .with_quote_chars(QuotesSubstitution::DoNothing),
    ),

    double_sets_of_quotes: (
        r#"<p>He said, "'Quoted' words in a larger quote."</p>"#,
        r#"<p>He said, &#8220;&#8216;Quoted&#8217; words in a larger quote.&#8221;</p>"#,
        SubstitutionConfig::default()
    ),

    decade_substitutions: (
        r#"It's the '80s"#,
        r#"It&#8217;s the &#8217;80s"#,
        SubstitutionConfig::default()
    ),
}
