//! CoffeeQL Lexer Errors — Coffee themed ☕

use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum LexError {
    #[error("☕ Spilled query at line {line}, column {col}!\n\
             Unknown ingredient: '{token}'\n\
             Hint: Check your spelling.")]
    UnknownToken {
        token: String,
        line:  usize,
        col:   usize,
    },

    #[error("☕ Your string is never closed!\n\
             Started at line {line}.\n\
             Hint: Add a closing '\"'")]
    UnclosedString {
        line: usize,
    },

    #[error("☕ '{name}' needs a type!\n\
             Hint: Use {name}[] for table\n\
                   or {name}{{}} for collection")]
    MissingCollectionType {
        name: String,
    },

    #[error("☕ Invalid duration '{raw}'\n\
             Hint: Use 7d, 30m, 2h, 1w, 3mo, 1y")]
    InvalidDuration {
        raw: String,
    },

    #[error("☕ Invalid distance '{raw}'\n\
             Hint: Use 5km, 500m, 10mi")]
    InvalidDistance {
        raw: String,
    },
}
