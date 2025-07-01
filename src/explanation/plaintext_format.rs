use crate::{ast::Expression, explanation::ExplanationFormatter};

pub struct TextFormatter;

impl ExplanationFormatter for TextFormatter {
    fn format_rule_applied(&self, rule_name: &str, before: &Expression, after: &Expression) -> String {
        format!("- {}\n  Before: {}\n  After:  {}", rule_name, before, after)
    }
    
    fn format_step_started(&self, expression: &Expression) -> String {
        format!("Simplifying expression: {}", expression)
    }
    
    fn format_step_completed(&self, result: &Expression) -> String {
        format!("Final result: {}", result)
    }
}