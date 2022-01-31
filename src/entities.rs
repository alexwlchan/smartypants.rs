// This file just contains HTML entities as named constants.
//
// I pulled them into named entities to make the code that uses them
// easier to read, because I don't have them memorised.  The name
// OPENING_DOUBLE_CURLY_QUOTE_ENTITY means something, but not "&#8220;".

pub const SINGLE_STRAIGHT_QUOTE_ENTITY: &str = "&#39";          // '
pub const DOUBLE_STRAIGHT_QUOTE_ENTITY: &str = "&#34";          // "

pub const HYPHEN_ENTITY: &str = "&#45;";                        // -

pub const FULL_STOP_ENTITY: &str = "&#46";                      // .

pub const SINGLE_BACKSLASH_ENTITY: &str = "&#92;";              // \

pub const BACKTICK_ENTITY: &str = "&#96;";                      // `

pub const EN_DASH_ENTITY: &str = "&#8211;";                     // –
pub const EM_DASH_ENTITY: &str = "&#8212;";                     // —

pub const EN_DASH_HEX_ENTITY: &str = "&#x2013;";                // –
pub const EM_DASH_HEX_ENTITY: &str = "&#x2014;";                // —

pub const OPENING_SINGLE_CURLY_QUOTE_ENTITY: &str = "&#8216;";  // ‘
pub const CLOSING_SINGLE_CURLY_QUOTE_ENTITY: &str = "&#8217;";  // ’

pub const OPENING_DOUBLE_CURLY_QUOTE_ENTITY: &str = "&#8220;";  // “
pub const CLOSING_DOUBLE_CURLY_QUOTE_ENTITY: &str = "&#8221;";  // ”
