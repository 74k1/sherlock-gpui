use crate::utils::intent::{Intent, translation::Language};

pub struct TranslationParser;
impl TranslationParser {
    pub fn parse_intent(input: &str) -> Option<Intent> {
        // Search from the right so "translate X to Y to Z" picks the last "to Z"
        let connectors = [" to ", " in "];
        let (idx, conn) = connectors
            .iter()
            .filter_map(|&c| input.rfind(c).map(|i| (i, c)))
            .max_by_key(|&(i, _)| i)?;

        let text = input[..idx].trim();
        let lang_str = input[idx + conn.len()..].trim();
        if text.is_empty() || lang_str.is_empty() {
            return None;
        }

        let target_lang = Language::from_str(lang_str)?;
        Some(Intent::Translation {
            text: text.to_string().into(),
            target_lang,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::intent::{Intent, translation::Language};

    #[test]
    fn test_translation_parsing() {
        let cases = vec![
            ("something to german", "something", Language::German),
            (
                "what is something to german",
                "what is something",
                Language::German,
            ),
            (
                "translate hello to spanish",
                "translate hello",
                Language::Spanish,
            ),
        ];

        for (input, expected_text, expected_lang) in cases {
            let result = TranslationParser::parse_intent(input);

            match result {
                Some(Intent::Translation { text, target_lang }) => {
                    assert_eq!(
                        text, expected_text,
                        "Failed text extraction on: '{}'",
                        input
                    );
                    assert_eq!(
                        target_lang, expected_lang,
                        "Failed language detection on: '{}'",
                        input
                    );
                }
                _ => panic!(
                    "Expected Translation intent, but got None for input: '{}'",
                    input
                ),
            }
        }
    }

    #[test]
    fn test_translation_parsing_failures() {
        let failures = vec!["something", "to german", "something to", ""];

        for input in failures {
            let result = TranslationParser::parse_intent(input);
            assert!(
                result.is_none(),
                "Expected None for input: '{}', but got {:?}",
                input,
                result
            );
        }
    }
}
