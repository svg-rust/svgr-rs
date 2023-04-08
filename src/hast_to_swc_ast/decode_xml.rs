pub fn decode_xml(s: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_xml_text() {
        let test_cases = vec![
            ("&amp;amp;", "&amp;"),
            ("&amp;#38;", "&#38;"),
            ("&amp;#x26;", "&#x26;"),
            ("&#38;#38;", "&#38;"),
            ("&#x26;#38;", "&#38;"),
            ("&#x3a;", ":"),
            ("&>", "&>"),
            ("id=770&#anchor", "id=770&#anchor"),
        ];
        test_cases.into_iter().for_each(|(input, expected)| {
            assert_eq!(decode_xml(input), expected);
        });
    }
}
