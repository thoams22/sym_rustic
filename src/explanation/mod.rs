use crate::{ast::Expression, explanation::plaintext_format::TextFormatter};
pub mod plaintext_format;

pub trait ExplanationFormatter {
    fn format_rule_applied(
        &self,
        rule_name: &str,
        before: &Expression,
        after: &Expression,
    ) -> String;
    fn format_simplify_step_started(&self, expression: &Expression) -> String;
    fn format_simplify_step_completed(&self, result: &Expression) -> String;
    
    fn format_solve_step_started(&self, expression: &Expression) -> String;
    fn format_solve_step_completed(&self, result: &Expression) -> String;
}

#[derive(Clone)]
pub enum OutputFormat {
    Text,
}

#[derive(PartialEq, Clone)]
pub enum LastStep {
    Start,
    Step,
    End,
    Unknown
}

#[derive(Clone)]
pub struct FormattingObserver {
    explanations: Vec<String>,
    format: OutputFormat,
    last_step: LastStep
}

impl FormattingObserver {
    pub fn new(format: OutputFormat) -> Self {
        Self {
            explanations: Vec::new(),
            format,
            last_step: LastStep::Unknown
        }
    }

    pub fn into_explanations(&self) -> &[String] {
        &self.explanations
    }

    pub fn rule_applied(&mut self, rule_name: &str, before: &Expression, after: &Expression) {
        self.explanations.push(match self.format {
            OutputFormat::Text => TextFormatter.format_rule_applied(rule_name, before, after),
        });
        self.last_step = LastStep::Step;
    }

    pub fn simplify_step_started(&mut self, expression: &Expression) {
        self.explanations.push(match self.format {
            OutputFormat::Text => TextFormatter.format_simplify_step_started(expression),
        });
        self.last_step = LastStep::Start;
    }
    
    pub fn simplify_step_completed(&mut self, result: &Expression) {
        // If the last explanation is a start then there was no simplification so it is removed
        if self.last_step == LastStep::Start {
            self.explanations.pop();
            self.last_step = LastStep::Start;
        } else {
            self.explanations.push(match self.format {
                OutputFormat::Text => TextFormatter.format_simplify_step_completed(result),
            });
            self.last_step = LastStep::End;
        }
    }

    pub fn open_explaination(&mut self, explanation: String) {
        self.explanations.push(explanation);
    }

    pub fn solve_step_started(&mut self, expression: &Expression) {
        self.explanations.push(match self.format {
            OutputFormat::Text => TextFormatter.format_solve_step_started(expression),
        });
    }
    
    pub fn solve_step_completed(&mut self, result: &Expression) {

            self.explanations.push(match self.format {
                OutputFormat::Text => TextFormatter.format_solve_step_completed(result),
            });
    }
}