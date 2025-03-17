/// Represents different types of errors that can occur during tokenization.
#[derive(Debug, PartialEq)]
pub enum TokenizerError {
    /// Encountered an unknown or invalid character that is not part of the language.
    /// Example: `@` in Java source code where it is not allowed.
    InvalidCharacter(char, usize, usize), // (Character, Line, Column)

    /// Encountered an unterminated string literal.
    /// Example: `"Hello` without a closing `"`
    UnterminatedString(usize, usize), // (Line, Column)

    /// Found an unexpected end-of-file while processing a token.
    UnexpectedEOF(String, usize, usize), // (Token Type, Line, Column)

    /// Unmatched comment closure (e.g., `*/` without a matching `/*`).
    UnmatchedCommentClosure(usize, usize), // (Line, Column)

    /// Nested comments exceeded allowed depth or were improperly closed.
    InvalidNestedComment(usize, usize), // (Line, Column)

    /// Missing expected token in context.
    /// Example: `int x =` without a terminating value.
    ExpectedToken(String, usize, usize), // (Expected Token, Line, Column)
}
