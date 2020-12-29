use std::borrow::Cow;

const REGEX: &str = r"[\p{Emoji}\p{Emoji_Presentation}\p{Emoji_Modifier}\p{Emoji_Modifier_Base}\p{Emoji_Component}--\p{Ascii}\.,!\?。、！？…]";

pub struct RegexPatterns {
    regex: regex::Regex,
}

impl RegexPatterns {
    pub fn new() -> Self {
        let regex = regex::Regex::new(REGEX).unwrap();
        RegexPatterns {
            regex
        }
    }

    pub fn clean_message<'t>(&self, message: &'t str) -> Cow<'t, str> {
        self.regex.replace_all(&message, "")
    }
}

#[allow(unused_imports)]
mod test {
    use super::RegexPatterns;

    #[test]
    pub fn test_message_cleaning() {
        struct TestCase {
            input: String,
            expected: String
        }

        let patterns = RegexPatterns::new();

        let test_cases = vec![
            TestCase {
                input: "❤t❤e❤s❤t❤".to_string(),
                expected: "test".to_string()
            },
            TestCase {
                input: "t🤚e🤚🏻s🤚🏼t🤚🏽i🤚🏾n🤚🏿g".to_string(),
                expected: "testing".to_string()
            },
            TestCase {
                input: "🧔t🧔🏻e🧔🏼s🧔🏽t🧔🏾a🧔🏿b".to_string(),
                expected: "testab".to_string()
            },
            TestCase {
                input: "t🧑‍🍼e🧑🏻‍🍼s🧑🏼‍🍼t🧑🏽‍🍼i🧑🏾‍🍼n🧑🏿‍🍼g".to_string(),
                expected: "testing".to_string()
            },
            TestCase {
                input: "t👭🏽e👩🏽‍🤝‍👩🏿s👩🏽‍🤝‍👩🏻t👩🏿‍🤝‍👩🏾i👫🏼n👩🏾‍🤝‍👨🏼g".to_string(),
                expected: "testing".to_string()
            },
            TestCase {
                input: "👩‍👩‍👧‍👧t👩‍👩‍👧‍👧e👩‍👩‍👧‍👧s👩‍👩‍👧‍👧t👩‍👩‍👧‍👧i👩‍👩‍👧‍👧n👩‍👩‍👧‍👧g123".to_string(),
                expected: "testing123".to_string()
            },
        ];

        for test_case in test_cases {
            let result = patterns.clean_message(&test_case.input).to_string();
            assert_eq!(result, test_case.expected);
        }
    }
}