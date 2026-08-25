//! CoffeeQL Lexer
//! Pipeline: Raw String → PreProcessor → Logos → ContextLayer → Token Stream

pub mod pre;
pub mod logos_tokens;
pub mod context;
pub mod error;
pub mod token;

use pre::PreProcessor;
use context::ContextLayer;
pub use token::Token;
pub use error::LexError;

pub struct Lexer {
    source: String,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self { source: source.to_string() }
    }

    pub fn tokenize(&self) -> Result<Vec<Token>, LexError> {
        let cleaned = PreProcessor::new(&self.source).process();
        let tokens  = ContextLayer::new(&cleaned).tokenize()?;
        Ok(tokens)
    }
}
