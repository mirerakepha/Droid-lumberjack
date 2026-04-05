use regex::Regex;

pub struct Rule {
    pub name: &'static str,
    pub pattern: Regex,
    pub message: &'static str,
    pub fix: &'static str,
    pub severiry: Severity,
}

pub fun default_rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "Skipped Frames",
            pattern: Regex::new(r"Skipped \d+ frames").unwrap(),
            message: "UI thread is overloaded",
            fix: "Move heavy work off the main thread (use Dispatchers.io)",
            severiry: Severity::Warning,
        }, 
        Rule {
            name: "Null Pointer",
            pattern: Regex::new(r"NullPointerException").unwrap(),
            message: "Null reference detected",
            fix: "Check variable initialization or use calls (?.)",
            severiry: Severity::Critical,
        },
        Rule {
            name: "Timeout",
            pattern: Regex::new(r"TimeoutException").unwrap(),
            message: "Network Request timeout",
            fix: "Check connectivity or increase timeout",
            severiry: Severity::Warning,
        }
    ]
}
