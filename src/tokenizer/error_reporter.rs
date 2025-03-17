use colored::Colorize;

use super::error_types::TokenizerError;
/// Collects and formats multiple tokenizer errors into a single report.
pub struct ErrorReporter {
    errors: Vec<TokenizerError>,
}

impl ErrorReporter {
    /// Creates a new `ErrorReporter` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// let reporter = ErrorReporter::new();
    /// ```
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    /// Adds an error to the report.
    ///
    /// # Parameters
    ///
    /// - `error`: The `TokenizerError` to add.
    pub fn add_error(&mut self, error: TokenizerError) {
        self.errors.push(error);
    }

    /// Checks if there are any recorded errors.
    ///
    /// # Returns
    ///
    /// `true` if there is at least one error, `false` otherwise.
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Generates a detailed, beautifully formatted error report.
    ///
    /// The report includes:
    /// - A header with an attention-grabbing title.
    /// - A list of error messages with each error's details.
    /// - Decorative borders and separators.
    ///
    /// # Returns
    ///
    /// A multi-line string containing the formatted error report.
    pub fn generate_report(&self, input: &str) -> String {
        let mut report = String::new();
        // Split the input into lines for reference (1-indexed)
        let lines: Vec<&str> = input.lines().collect();

        for error in &self.errors {
            // Determine error details based on its variant.
            let (error_type, line, col, explanation) = match error {
                TokenizerError::InvalidCharacter(ch, line, col) => {
                    ("Invalid Character".red(), *line, *col, format!("Encountered an invalid character: '{}'", ch))
                }
                TokenizerError::UnterminatedString(line, col) => {
                    ("Unterminated String".red(), *line, *col, "String literal is missing a closing quote".to_string())
                }
                TokenizerError::UnexpectedEOF(token, line, col) => {
                    ("Unexpected EOF".red(), *line, *col, format!("Unexpected end-of-file while processing '{}'", token.red()))
                }
                TokenizerError::UnmatchedCommentClosure(line, col) => {
                    ("Unmatched Comment Closure".red(), *line, *col, "Found a comment closure '*/' without a matching '/*'".to_string())
                }
                TokenizerError::InvalidNestedComment(line, col) => {
                    ("Invalid Nested Comment".red(), *line, *col, "Improperly nested or unclosed comment".to_string())
                }
                TokenizerError::ExpectedToken(expected, line, col) => {
                    ("Expected Token Missing".red(), *line, *col, format!("Expected token '{}' is missing", expected.red()))
                }
            };

            // Retrieve the content of the error's line (lines are 1-indexed)
            let line_content = if line > 0 && (line as usize) <= lines.len() {
                lines[line as usize - 1]
            } else {
                &"<line not found>".bright_green()
            };

            // Highlight the error character at the given column (if possible)
            let highlighted_line = if col > 0 && (col as usize) <= line_content.len() {
                let index = (col - 1) as usize;
                let (before, rest) = line_content.split_at(index);
                let (err_char, after) = if rest.is_empty() {
                    ("", "")
                } else {
                    rest.split_at(1)
                };
                format!("{}{}{}", before.bright_green(), err_char.red().bold(), after.bright_green())
            } else {
                line_content.bright_green().to_string()
            };

            // Build the error block.
            report.push_str(&format!(
                "┌[{}] Error at line {}, column {}\n",
                error_type.bold(),
                line.to_string().bright_cyan(),
                col.to_string().bright_cyan()
            ));
            report.push_str(&format!("├ {}\n", explanation.bright_yellow()));
            report.push_str(&format!("├ Code Piece ~ \"{}\"\n", highlighted_line));
            report.push_str("└─[***]\n");
        }
        report
    }

    /// Prints the formatted error report to standard output.
    pub fn print_report(&self, input: &str) {
        println!("{}", self.generate_report(input));
    }
}
