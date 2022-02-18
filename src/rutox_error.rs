use super::scanner::src_location::SrcLocation;
use colored::*;

pub enum RutoxError {
    Programmer(String, SrcLocation),
    Syntax(String, SrcLocation),
    Runtime(String, SrcLocation),
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
        }
    }
}

impl RutoxError {
    pub fn location(&self) -> SrcLocation {
        match self {
            RutoxError::Syntax(_, location) => location.clone(),
            RutoxError::Programmer(_, location) => location.clone(),
            RutoxError::Runtime(_, location) => location.clone(),
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
        let code_column = format!(" {} |", self.location().line).blue().bold();
        let empty_column = format!(" {} |", " ".repeat(self.location().line.to_string().len()))
            .blue()
            .bold();

        let src_line = format!(
            "{} {}",
            code_column,
            self.extract_error_line(source_file_content)
        );
        let error_indication = format!(
            "{}{}{}",
            empty_column,
            " ".repeat(self.location().col),
            "^ the error occurred here"
        )
        .yellow()
        .bold();

        format!(
            "{}\n{}\n{}\n{}",
            empty_column, src_line, error_indication, empty_column
        )
    }

    fn extract_error_line(&self, source_file_content: &str) -> String {
        let error_line_index = self.location().line - 1;

        source_file_content
            .split('\n')
            .nth(error_line_index)
            .expect("Could not find error line in source file.")
            .to_string()
    }
}
