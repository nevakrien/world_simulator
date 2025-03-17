use core::option::Option::None;

use super::error_reporter::ErrorReporter;
use crate::tokenizer::error_types::TokenizerError;

/// Represents the different token types supported by the tokenizer.
#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    // Literals
    Number(i32),
    Identifier(&'a str),
    String(String),

    // Single-character tokens
    Plus,      // +
    Minus,     // -
    Star,      // *
    Slash,     // /
    Percent,   // %
    Caret,     // ^
    And,       // &
    Or,        // |
    Not,       // !
    Tilde,     // ~
    Equal,     // =
    Semicolon, // ;
    Colon,     // :
    Comma,     // ,
    Dot,       // .
    Question,  // ?
    At,        // @

    // Grouping symbols
    LParen,   // (
    RParen,   // )
    LBrace,   // {
    RBrace,   // }
    LBracket, // [
    RBracket, // ]

    // Multi-character tokens (operators)
    DoubleEqual,     // ==
    NotEqual,        // !=
    Less,            // <
    LessEqual,       // <=
    Greater,         // >
    GreaterEqual,    // >=
    PlusEqual,       // +=
    MinusEqual,      // -=
    StarEqual,       // *=
    SlashEqual,      // /=
    PercentEqual,    // %=
    AndEqual,        // &=
    OrEqual,         // |=
    CaretEqual,      // ^=
    LeftShift,       // <<
    RightShift,      // >>
    LeftShiftEqual,  // <<=
    RightShiftEqual, // >>=

    // End-of-line and end-of-file markers
    EOL,
    EOF,
}

/// Tokenizes the input source code into a stream of tokens.
/// This implementation is designed for high performance and robustness, handling
/// nested multi-line comments, various operators, literals, and error conditions.
///
/// # Parameters
///
/// - `input`: The source code as a string slice.
/// - `reporter`: A mutable reference to an ErrorReporter for recording errors.
///
/// # Returns
///
/// A vector of tokens representing the parsed input.
#[allow(dead_code)]
pub fn tokenize<'a>(input: &'a str, reporter: &mut ErrorReporter) -> Vec<Token<'a>> {
    // Pre-reserve capacity to reduce reallocations.
    let mut tokens = Vec::with_capacity(input.len() / 2);
    let mut chars = input.char_indices().peekable();
    let mut current_line = 1;
    let mut current_column = 1;

    while let Some((i, c)) = chars.next() {
        match c {
            // Skip spaces and tabs.
            ' ' | '\t' => {
                current_column += 1;
            }
            // Newline produces an EOL token.
            '\n' => {
                tokens.push(Token::EOL);
                current_line += 1;
                current_column = 1;
            }

            // Handle comments and division operator.
            '/' => {
                if let Some(&(_, next)) = chars.peek() {
                    if next == '/' {
                        // Single-line comment: simply skip characters until newline.
                        chars.next(); // Consume the second '/'
                        current_column += 2;
                        while let Some(&(_, ch)) = chars.peek() {
                            if ch == '\n' {
                                break;
                            }
                            chars.next();
                            current_column += 1;
                        }
                        continue; // Skip adding any token.
                    } else if next == '*' {
                        // Multi-line comment with nesting: skip entire comment block.
                        chars.next(); // Consume '*'
                        current_column += 2;
                        let comment_start_line = current_line;
                        let comment_start_column = current_column;
                        let mut depth = 1;
                        while let Some((_, ch)) = chars.next() {
                            if ch == '\n' {
                                current_line += 1;
                                current_column = 1;
                            } else {
                                current_column += 1;
                            }
                            // Increase nesting when "/*" is encountered.
                            if ch == '/' {
                                if let Some(&(_, next_ch)) = chars.peek() {
                                    if next_ch == '*' {
                                        chars.next();
                                        depth += 1;
                                        current_column += 1;
                                    }
                                }
                            }
                            // Decrease nesting when "*/" is encountered.
                            else if ch == '*' {
                                if let Some(&(_, next_ch)) = chars.peek() {
                                    if next_ch == '/' {
                                        chars.next();
                                        current_column += 1;
                                        depth -= 1;
                                        if depth == 0 {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        // If comment was not properly closed, report an error.
                        if depth != 0 {
                            reporter.add_error(TokenizerError::InvalidNestedComment(
                                comment_start_line,
                                comment_start_column,
                            ));
                        }
                        continue; // Skip producing a comment token.
                    }
                }
                // Handle '/' operator (or compound '/=' operator) normally.
                if let Some(&(_, next)) = chars.peek() {
                    if next == '=' {
                        chars.next();
                        tokens.push(Token::SlashEqual);
                        current_column += 2;
                        continue;
                    }
                }
                tokens.push(Token::Slash);
                current_column += 1;
            }

            // String literal handling.
            '"' => {
                current_column += 1; // Consume opening quote.
                let string_start_line = current_line;
                let string_start_column = current_column;
                let mut string_literal = String::new();
                let mut terminated = false;
                while let Some((_, ch)) = chars.next() {
                    if ch == '\\' {
                        // Process escape sequence.
                        if let Some((_, esc)) = chars.next() {
                            match esc {
                                '"' => string_literal.push('"'),
                                '\\' => string_literal.push('\\'),
                                'n' => string_literal.push('\n'),
                                't' => string_literal.push('\t'),
                                other => string_literal.push(other),
                            }
                            current_column += 2;
                        } else {
                            reporter.add_error(TokenizerError::UnterminatedString(
                                current_line,
                                current_column,
                            ));
                            break;
                        }
                    } else if ch == '"' {
                        terminated = true;
                        current_column += 1;
                        break;
                    } else {
                        string_literal.push(ch);
                        if ch == '\n' {
                            current_line += 1;
                            current_column = 1;
                        } else {
                            current_column += 1;
                        }
                    }
                }
                if !terminated {
                    reporter.add_error(TokenizerError::UnterminatedString(
                        string_start_line,
                        string_start_column,
                    ));
                }
                tokens.push(Token::String(string_literal));
            }

            // Number literal: collect consecutive digits.
            '0'..='9' => {
                let start = i;
                while let Some(&(_, ch)) = chars.peek() {
                    if ch.is_ascii_digit() {
                        chars.next();
                        current_column += 1;
                    } else {
                        break;
                    }
                }
                let end = match chars.peek() {
                    Some(&(j, _)) => j,
                    None => input.len(),
                };
                let num_str = &input[start..end];
                match num_str.parse::<i32>() {
                    Ok(num) => tokens.push(Token::Number(num)),
                    Err(_) => reporter.add_error(TokenizerError::InvalidCharacter(
                        num_str.chars().next().unwrap(),
                        current_line,
                        current_column,
                    )),
                }
            }

            // Identifier (or keyword) handling.
            'a'..='z' | 'A'..='Z' | '_' => {
                let start = i;
                while let Some(&(_, ch)) = chars.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        chars.next();
                        current_column += 1;
                    } else {
                        break;
                    }
                }
                let end = match chars.peek() {
                    Some(&(j, _)) => j,
                    None => input.len(),
                };
                tokens.push(Token::Identifier(&input[start..end]));
            }

            // Operators and punctuation.
            '+' => {
                if let Some(&(_, next)) = chars.peek() {
                    if next == '=' {
                        chars.next();
                        tokens.push(Token::PlusEqual);
                        current_column += 2;
                        continue;
                    }
                }
                tokens.push(Token::Plus);
                current_column += 1;
            }
            '-' => {
                if let Some(&(_, next)) = chars.peek() {
                    if next == '=' {
                        chars.next();
                        tokens.push(Token::MinusEqual);
                        current_column += 2;
                        continue;
                    }
                }
                tokens.push(Token::Minus);
                current_column += 1;
            }
            '*' => {
                // Check for unmatched comment closure: "*/" encountered outside a comment.
                if let Some(&(_, next)) = chars.peek() {
                    if next == '/' {
                        chars.next(); // Consume '/'
                        reporter.add_error(TokenizerError::UnmatchedCommentClosure(
                            current_line,
                            current_column,
                        ));
                        current_column += 2;
                        continue;
                    } else if next == '=' {
                        chars.next();
                        tokens.push(Token::StarEqual);
                        current_column += 2;
                        continue;
                    }
                }
                tokens.push(Token::Star);
                current_column += 1;
            }
            '%' => {
                if let Some(&(_, next)) = chars.peek() {
                    if next == '=' {
                        chars.next();
                        tokens.push(Token::PercentEqual);
                        current_column += 2;
                        continue;
                    }
                }
                tokens.push(Token::Percent);
                current_column += 1;
            }
            '^' => {
                if let Some(&(_, next)) = chars.peek() {
                    if next == '=' {
                        chars.next();
                        tokens.push(Token::CaretEqual);
                        current_column += 2;
                        continue;
                    }
                }
                tokens.push(Token::Caret);
                current_column += 1;
            }
            '&' => {
                if let Some(&(_, next)) = chars.peek() {
                    if next == '=' {
                        chars.next();
                        tokens.push(Token::AndEqual);
                        current_column += 2;
                        continue;
                    }
                }
                tokens.push(Token::And);
                current_column += 1;
            }
            '|' => {
                if let Some(&(_, next)) = chars.peek() {
                    if next == '=' {
                        chars.next();
                        tokens.push(Token::OrEqual);
                        current_column += 2;
                        continue;
                    }
                }
                tokens.push(Token::Or);
                current_column += 1;
            }
            '!' => {
                if let Some(&(_, next)) = chars.peek() {
                    if next == '=' {
                        chars.next();
                        tokens.push(Token::NotEqual);
                        current_column += 2;
                        continue;
                    }
                }
                tokens.push(Token::Not);
                current_column += 1;
            }
            '<' => {
                if let Some(&(_, next)) = chars.peek() {
                    if next == '=' {
                        chars.next();
                        tokens.push(Token::LessEqual);
                        current_column += 2;
                        continue;
                    } else if next == '<' {
                        chars.next();
                        if let Some(&(_, after)) = chars.peek() {
                            if after == '=' {
                                chars.next();
                                tokens.push(Token::LeftShiftEqual);
                                current_column += 3;
                                continue;
                            }
                        }
                        tokens.push(Token::LeftShift);
                        current_column += 2;
                        continue;
                    }
                }
                tokens.push(Token::Less);
                current_column += 1;
            }
            '>' => {
                if let Some(&(_, next)) = chars.peek() {
                    if next == '=' {
                        chars.next();
                        tokens.push(Token::GreaterEqual);
                        current_column += 2;
                        continue;
                    } else if next == '>' {
                        chars.next();
                        if let Some(&(_, after)) = chars.peek() {
                            if after == '=' {
                                chars.next();
                                tokens.push(Token::RightShiftEqual);
                                current_column += 3;
                                continue;
                            }
                        }
                        tokens.push(Token::RightShift);
                        current_column += 2;
                        continue;
                    }
                }
                tokens.push(Token::Greater);
                current_column += 1;
            }
            '=' => {
                if let Some(&(_, next)) = chars.peek() {
                    if next == '=' {
                        chars.next();
                        tokens.push(Token::DoubleEqual);
                        current_column += 2;
                        continue;
                    }
                }
                tokens.push(Token::Equal);
                current_column += 1;
            }
            ';' => {
                tokens.push(Token::Semicolon);
                current_column += 1;
            }
            ':' => {
                tokens.push(Token::Colon);
                current_column += 1;
            }
            ',' => {
                tokens.push(Token::Comma);
                current_column += 1;
            }
            '.' => {
                tokens.push(Token::Dot);
                current_column += 1;
            }
            '?' => {
                tokens.push(Token::Question);
                current_column += 1;
            }
            '@' => {
                tokens.push(Token::At);
                current_column += 1;
            }
            '(' => {
                tokens.push(Token::LParen);
                current_column += 1;
            }
            ')' => {
                tokens.push(Token::RParen);
                current_column += 1;
            }
            '{' => {
                tokens.push(Token::LBrace);
                current_column += 1;
            }
            '}' => {
                tokens.push(Token::RBrace);
                current_column += 1;
            }
            '[' => {
                tokens.push(Token::LBracket);
                current_column += 1;
            }
            ']' => {
                tokens.push(Token::RBracket);
                current_column += 1;
            }
            // Catch-all for any unrecognized character.
            _ => {
                reporter.add_error(TokenizerError::InvalidCharacter(
                    c,
                    current_line,
                    current_column,
                ));
                current_column += 1;
            }
        }
    }
    tokens.push(Token::EOF);
    tokens
}
