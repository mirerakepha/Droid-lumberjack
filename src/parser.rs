use crate::rules::default_rules;
use crate::rules::Rule;

/*
 * Loop through log lines
 * match the regex
 * return a match === asin a Detection
 *
 */

#[derive(Debug, Clone)]
pub enum Severity {
    Critical, 
    Warning,
    Info,
}

#[derive(Debug, Clone)]
pub struct Detection {
    pub name: String,
    pub message: String,
    pub fix: String,
    pub severity: Severity,
}


pub fn parse_line(line: &str, rule: &[Rule]) -> Option<Detection> {
    for rule in default_rules() {
        if rule.pattern.is_match(line) {
            return Some( Detection { //Some means an issue is found 
                name: rule.name.to_string(),
                message: rule.message.to_string(),
                fix: rule.fix.to_string(),
                severity: rule.severity.clone(),
            });
        } 
    }
    None // this ignores the noise ==> noting useful
}
