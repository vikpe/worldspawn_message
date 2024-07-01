#[derive(Debug, Default, Clone, PartialEq)]
pub struct Message {
    pub lines: Vec<String>,
    pub authors: Vec<String>,
    pub release_year: Option<String>,
}

impl From<&str> for Message {
    fn from(message: &str) -> Self {
        let lines = to_lines(message);

        Self {
            lines: lines.clone(),
            authors: lines.iter().filter_map(|l| get_author(l)).collect(),
            release_year: lines.iter().filter_map(|l| get_release_year(l)).next(),
        }
    }
}

fn to_lines(message: &str) -> Vec<String> {
    let lines: Vec<String> = message
        .split('\n')
        .map(|line| strip(line).trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();
    lines
}

fn strip(value: &str) -> String {
    let result: String = value
        .chars()
        .map(|c| match c.is_ascii_control() {
            true => ' ',
            false => c,
        })
        .collect();
    result
        .replace("   ", " ")
        .replace("  ", " ")
        .trim()
        .to_string()
}

fn is_delimiter(c: char) -> bool {
    c == '(' || c == '[' || c == ',' || c == '-'
}

fn get_author(line: &str) -> Option<String> {
    let line_ = format!(" {}", line);
    let haystack = line_.to_lowercase();
    let index_from = haystack.rfind(" by ").map(|i| i + 4)?;
    let index_to = haystack[index_from..]
        .find(is_delimiter)
        .map(|i| index_from + i)
        .unwrap_or(line_.len());

    let author = &line_[index_from..index_to].trim();
    Some(author.to_string())
}

fn get_release_year(line: &str) -> Option<String> {
    for year in 1996..=2025 {
        if line.contains(&year.to_string()) {
            return Some(year.to_string());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_to_lines() {
        assert_eq!(
            to_lines("thefoobar\n\n by Bar\n"),
            vec!["the foo bar".to_string(), "by Bar".to_string(),]
        )
    }

    #[test]
    fn test_strip() {
        assert_eq!(strip("the foo bar"), "the foo bar".to_string());
        assert_eq!(strip(" the   foo  bar  "), "the foo bar".to_string());
        assert_eq!(strip("thefoobar"), "the foo bar".to_string());
    }

    #[test]
    fn test_get_author() {
        let test_cases: HashMap<&str, Option<String>> = HashMap::from([
            ("Foo", None),
            ("Foo BY Bar", Some("Bar".to_string())),
            ("Foo by Bar", Some("Bar".to_string())),
            ("Foo - by Bar", Some("Bar".to_string())),
            ("Foo (abc) - by Bar", Some("Bar".to_string())),
            ("Foo by Bar, dmm4 edition", Some("Bar".to_string())),
            ("Foo by Bar - dmm4 edition", Some("Bar".to_string())),
            ("by Bar", Some("Bar".to_string())),
            ("Foo BY Bar (1996)", Some("Bar".to_string())),
            ("Foo BY Bar [1996]", Some("Bar".to_string())),
        ]);

        for (line, expected) in test_cases {
            assert_eq!(get_author(line), expected, "input: {}", line);
        }
    }

    #[test]
    fn test_get_release_date() {
        let test_cases: HashMap<&str, Option<String>> = HashMap::from([
            ("Foo", None),
            ("Foo BY Bar", None),
            ("Foo BY Bar (1996)", Some("1996".to_string())),
            ("Foo BY Bar [1996]", Some("1996".to_string())),
        ]);

        for (line, expected) in test_cases {
            assert_eq!(get_release_year(line), expected, "input: {:?}", line);
        }
    }

    #[test]
    fn test_from() {
        let test_cases: HashMap<&str, Message> = HashMap::from([
            (
                "Foo",
                Message {
                    lines: vec!["Foo".to_string()],
                    authors: vec![],
                    release_year: None,
                },
            ),
            (
                "Foo BY Bar",
                Message {
                    lines: vec!["Foo BY Bar".to_string()],
                    authors: vec!["Bar".to_string()],
                    release_year: None,
                },
            ),
            (
                "Foo BY Bar (1996)",
                Message {
                    lines: vec!["Foo BY Bar (1996)".to_string()],
                    authors: vec!["Bar".to_string()],
                    release_year: Some("1996".to_string()),
                },
            ),
            (
                "(1996)",
                Message {
                    lines: vec!["(1996)".to_string()],
                    authors: vec![],
                    release_year: Some("1996".to_string()),
                },
            ),
        ]);

        for (input, expected) in test_cases {
            assert_eq!(Message::from(input), expected, "input: {}", input);
        }
    }
}
