//! Minimalistic version of PISA written in Rust.

#![warn(
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use itertools::Itertools;
use scraper::Html;

/// Returns clean text from markup.
pub fn clean_text(source: &str) -> String {
    let doc = Html::parse_fragment(source);
    doc.root_element()
        .text()
        .filter_map(|t| match t.trim() {
            "" => None,
            txt => Some(txt),
        })
        .join(" ")
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"
<HT> HT </HT>

<HEADER>
<H2>H2</H2>
<DATE1>DATE1</DATE1>
Random Text
<H3> <TI>      MORE TEXT </TI></H3>

</HEADER>


<TEXT>
actual text
</TEXT>
    "#;

    #[test]
    fn test_clean_text() {
        assert_eq!(
            clean_text(INPUT),
            String::from("HT H2 DATE1 Random Text MORE TEXT actual text")
        );
    }
}
