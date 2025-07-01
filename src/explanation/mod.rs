use crate::{ast::Expression, explanation::plaintext_format::TextFormatter};
pub mod plaintext_format;

pub trait SimplificationObserver {
    fn rule_applied(&mut self, rule_name: &str, before: &Expression, after: &Expression);
    fn step_started(&mut self, expression: &Expression);
    fn step_completed(&mut self, result: &Expression);
}

pub trait ExplanationFormatter {
    fn format_rule_applied(
        &self,
        rule_name: &str,
        before: &Expression,
        after: &Expression,
    ) -> String;
    fn format_step_started(&self, expression: &Expression) -> String;
    fn format_step_completed(&self, result: &Expression) -> String;
}

#[derive(Clone)]
pub enum OutputFormat {
    Text,
}

#[derive(Clone)]
pub struct FormattingObserver {
    explanations: Vec<String>,
    format: OutputFormat,
}

impl FormattingObserver {
    pub fn new(format: OutputFormat) -> Self {
        Self {
            explanations: Vec::new(),
            format,
        }
    }

    pub fn into_explanations(&self) -> &[String] {
        &self.explanations
    }

    pub fn rule_applied(&mut self, rule_name: &str, before: &Expression, after: &Expression) {
        self.explanations.push(match self.format {
            OutputFormat::Text => TextFormatter.format_rule_applied(rule_name, before, after),
        })
    }
    pub fn step_started(&mut self, expression: &Expression) {
        self.explanations.push(match self.format {
            OutputFormat::Text => TextFormatter.format_step_started(expression),
        })
    }

    pub fn step_completed(&mut self, result: &Expression) {
        self.explanations.push(match self.format {
            OutputFormat::Text => TextFormatter.format_step_completed(result),
        })
    }
}

// impl SimplificationObserver for FormattingObserver {
//     fn rule_applied(&mut self, rule_name: &str, before: &Expression, after: &Expression) {
//         self.explanations
//             .push(self.format_rule_applied(rule_name, before, after));
//     }

//     fn step_started(&mut self, expression: &Expression) {
//         self.explanations.push(self.format_step_started(expression));
//     }

//     fn step_completed(&mut self, result: &Expression) {
//         self.explanations.push(self.format_step_completed(result));
//     }
// }
