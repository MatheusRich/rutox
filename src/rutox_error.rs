use super::scanner::src_location::SrcLocation;
use colored::*;

pub enum RutoxError {
    Programmer(String, SrcLocation),
    Syntax(String, SrcLocation),
    Runtime(String, SrcLocation),
    Multiple(Vec<RutoxError>),
}

impl std::fmt::Display for RutoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RutoxError::Syntax(msg, _) => {
                let error_type = "SyntaxError".red();
                let msg = format!("{error_type}: {msg}.").bold();

                write!(f, "{}", msg)
            }
            RutoxError::Runtime(msg, _) => {
                let error_type = "RuntimeError".red();
                let msg = format!("{error_type}: {msg}.").bold();

                write!(f, "{}", msg)
            }
            RutoxError::Programmer(msg, _) => {
                let error_type = "ProgrammerError".red();
                let msg = format!(
                    "{error_type}: {msg}.\nThis is a bug in rutox. Please report it at https://github.com/MatheusRich/rutox."
                )
                .bold();

                write!(f, "{}", msg)
            }
            RutoxError::Multiple(errors) => {
                let mut error_string = String::new();

                for error in errors {
                    error_string.push_str(&format!("{}\n", error));
                }

                write!(f, "{}", error_string)
            }
        }
    }
}

impl RutoxError {
    pub fn location(&self) -> SrcLocation {
        match self {
            RutoxError::Syntax(_, location) => location.clone(),
            RutoxError::Programmer(_, location) => location.clone(),
            RutoxError::Runtime(_, location) => location.clone(),
            RutoxError::Multiple(errors) => errors
                .first()
                .expect("There should be at least one error")
                .location()
                .clone(),
        }
    }

    pub fn details(&self, source_file_path: &str, source_file_content: &str) -> String {
        format!(
            "{}\n{}",
            self.source_location(source_file_path),
            self.code_excerpt(source_file_content)
        )
    }

    fn source_location(&self, source_file_path: &str) -> String {
        let arrow = "  -->".blue().bold();

        format!("{} {}:{}", arrow, source_file_path, self.location())
    }

    fn code_excerpt(&self, source_file_content: &str) -> String {
        let error_line_and_surroundings =
            self.extract_error_line_and_surroundings(source_file_content);
        let biggest_line_num_digit_count = error_line_and_surroundings
            .iter()
            .last()
            .expect("There should be at least one error line")
            .0
            .to_string()
            .len();

        let mut code_excerpt = String::new();
        for (line, code, should_highlight) in error_line_and_surroundings {
            code_excerpt.push_str(
                &self
                    .code_column(line, biggest_line_num_digit_count)
                    .to_string(),
            );
            if should_highlight {
                code_excerpt.push_str(&code.bold().white().to_string());
            } else {
                code_excerpt.push_str(&code);
            }
            if line == self.location().line as i64 {
                code_excerpt.push('\n');
                code_excerpt.push_str(
                    &self
                        .error_indication(biggest_line_num_digit_count)
                        .to_string(),
                );
            }
            code_excerpt.push('\n');
        }
        format!(
            "{}\n{}{}",
            self.empty_column(biggest_line_num_digit_count),
            code_excerpt,
            self.empty_column(biggest_line_num_digit_count)
        )
    }

    fn code_column(&self, line_num: i64, biggest_line_num: usize) -> ColoredString {
        let padding = " ".repeat(biggest_line_num - (line_num.to_string().len()));

        format!(" {}{} | ", padding, line_num).blue().bold()
    }

    fn empty_column(&self, biggest_line_num: usize) -> ColoredString {
        let padding = " ".repeat(biggest_line_num);

        format!(" {} | ", padding).blue().bold()
    }

    fn error_indication(&self, biggest_line_num: usize) -> ColoredString {
        let error_col = if self.location().col == 0 {
            0
        } else {
            self.location().col - 1
        };

        format!(
            "{}{}{}",
            self.empty_column(biggest_line_num),
            " ".repeat(error_col),
            "^ the error occurred here"
        )
        .yellow()
        .bold()
    }

    fn extract_error_line_and_surroundings(
        &self,
        source_file_content: &str,
    ) -> Vec<(i64, String, bool)> {
        let error_line_index = self.location().line as i64 - 1;
        let line_before_error_index = error_line_index - 1;
        let line_after_error_index = error_line_index + 1;

        source_file_content
            .split('\n')
            .map(|l| l.to_string())
            .enumerate()
            .map(|(i, line)| match i {
                v if v as i64 == line_before_error_index => {
                    (Some(line_before_error_index + 1), line, false)
                }
                v if v as i64 == error_line_index => (Some(error_line_index + 1), line, true),
                v if v as i64 == line_after_error_index => {
                    (Some(line_after_error_index + 1), line, false)
                }
                _ => (None, "".to_string(), false),
            })
            .filter(|(i, _, _)| i.is_some())
            .map(|(i, line, should_highlight)| (i.unwrap(), line, should_highlight))
            .collect()
    }
}
