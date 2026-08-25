//! Context Layer — sits on top of Logos
//! Handles: collection types, nested fields,
//! duration/distance parsing, chain validation

use logos::Logos;
use super::logos_tokens::RawToken;
use super::token::{
    Token, CollectionKind,
    Duration, Distance, TimeUnit, DistUnit,
    DataType, Constraint,
};
use super::error::LexError;

pub struct ContextLayer {
    source: String,
}

impl ContextLayer {
    pub fn new(source: &str) -> Self {
        Self { source: source.to_string() }
    }

    pub fn tokenize(&self) -> Result<Vec<Token>, LexError> {
        let mut raw_tokens: Vec<(RawToken, &str)> = vec![];

        // Collect all logos tokens with their slices
        let mut lex = RawToken::lexer(&self.source);
        while let Some(result) = lex.next() {
            match result {
                Ok(tok) => raw_tokens.push((tok, lex.slice())),
                Err(_)  => {
                    return Err(LexError::UnknownToken {
                        token: lex.slice().to_string(),
                        line:  0,
                        col:   0,
                    })
                }
            }
        }

        // Context pass — convert raw → rich tokens
        let mut tokens  = vec![];
        let mut i       = 0;
        let     len     = raw_tokens.len();

        while i < len {
            let (ref tok, slice) = raw_tokens[i];

            match tok {
                // ── Identifier: could be collection or field ──────
                RawToken::Identifier => {
                    let name = slice.to_string();

                    // Peek ahead for [] or {}
                    if i + 2 < len {
                        match (&raw_tokens[i+1].0, &raw_tokens[i+2].0) {

                            // users[]  → Structured collection
                            (RawToken::LBracket, RawToken::RBracket) => {
                                tokens.push(Token::Collection {
                                    name,
                                    kind: CollectionKind::Structured,
                                });
                                i += 3; // consume name [ ]
                                continue;
                            }

                            // products{} → Unstructured collection
                            (RawToken::LBrace, RawToken::RBrace) => {
                                tokens.push(Token::Collection {
                                    name,
                                    kind: CollectionKind::Unstructured,
                                });
                                i += 3; // consume name { }
                                continue;
                            }

                            _ => {}
                        }
                    }

                    // Normal identifier
                    tokens.push(Token::Identifier(name));
                }

                // ── Duration: 7d, 30m, 2h ────────────────────────
                RawToken::Duration => {
                    tokens.push(parse_duration(slice)?);
                }

                // ── Distance: 5km, 500m ──────────────────────────
                RawToken::Distance => {
                    tokens.push(parse_distance(slice)?);
                }

                // ── Float ────────────────────────────────────────
                RawToken::Float => {
                    let f: f64 = slice.parse().unwrap_or(0.0);
                    tokens.push(Token::Float(f));
                }

                // ── Int ──────────────────────────────────────────
                RawToken::Int => {
                    let n: i64 = slice.parse().unwrap_or(0);
                    tokens.push(Token::Int(n));
                }

                // ── String ───────────────────────────────────────
                RawToken::Text => {
                    // Strip surrounding quotes
                    let inner = &slice[1..slice.len()-1];
                    tokens.push(Token::Text(inner.to_string()));
                }

                // ── Bool ─────────────────────────────────────────
                RawToken::True  => tokens.push(Token::Bool(true)),
                RawToken::False => tokens.push(Token::Bool(false)),
                RawToken::Null  => tokens.push(Token::Null),

                // ── Data Types ───────────────────────────────────
                RawToken::TyUuid     => tokens.push(Token::DataType(DataType::Uuid)),
                RawToken::TyText     => tokens.push(Token::DataType(DataType::Text)),
                RawToken::TyInt      => tokens.push(Token::DataType(DataType::Int)),
                RawToken::TyFloat    => tokens.push(Token::DataType(DataType::Float)),
                RawToken::TyBool     => tokens.push(Token::DataType(DataType::Bool)),
                RawToken::TyDatetime => tokens.push(Token::DataType(DataType::Datetime)),
                RawToken::TyGeopoint => tokens.push(Token::DataType(DataType::Geopoint)),
                RawToken::TyVector   => tokens.push(Token::DataType(DataType::Vector)),

                // ── Constraints ──────────────────────────────────
                RawToken::Primary => tokens.push(Token::Constraint(Constraint::Primary)),
                RawToken::Unique  => tokens.push(Token::Constraint(Constraint::Unique)),
                // NOT NULL handled together
                RawToken::Not => {
                    if i + 1 < len && raw_tokens[i+1].0 == RawToken::Null {
                        tokens.push(Token::Constraint(Constraint::NotNull));
                        i += 2;
                        continue;
                    }
                    tokens.push(Token::Bang);
                }

                // ── Chain Keywords ────────────────────────────────
                RawToken::Where  => tokens.push(Token::Where),
                RawToken::Give   => tokens.push(Token::Give),
                RawToken::Sort   => tokens.push(Token::Sort),
                RawToken::Cup    => tokens.push(Token::Cup),
                RawToken::Blend  => tokens.push(Token::Blend),
                RawToken::Mix    => tokens.push(Token::Mix),
                RawToken::Pour   => tokens.push(Token::Pour),
                RawToken::Refill => tokens.push(Token::Refill),
                RawToken::Spill  => tokens.push(Token::Spill),

                // ── Top Level ─────────────────────────────────────
                RawToken::Shot  => tokens.push(Token::Shot),
                RawToken::Grind => tokens.push(Token::Grind),
                RawToken::Menu  => tokens.push(Token::Menu),

                // ── Schema ───────────────────────────────────────
                RawToken::On     => tokens.push(Token::On),
                RawToken::As     => tokens.push(Token::As),
                RawToken::Asc    => tokens.push(Token::Asc),
                RawToken::Desc   => tokens.push(Token::Desc),
                RawToken::Flex   => tokens.push(Token::Flex),
                RawToken::Exists => tokens.push(Token::Exists),

                // ── Built-in Functions ────────────────────────────
                RawToken::FnUuid  => tokens.push(Token::FnUuid),
                RawToken::FnNow   => tokens.push(Token::FnNow),
                RawToken::FnToday => tokens.push(Token::FnToday),
                RawToken::FnCount => tokens.push(Token::FnCount),
                RawToken::FnSum   => tokens.push(Token::FnSum),
                RawToken::FnAvg   => tokens.push(Token::FnAvg),
                RawToken::FnMax   => tokens.push(Token::FnMax),
                RawToken::FnMin   => tokens.push(Token::FnMin),

                // ── Special Methods ───────────────────────────────
                RawToken::MethodNear      => tokens.push(Token::MethodNear),
                RawToken::MethodLike      => tokens.push(Token::MethodLike),
                RawToken::MethodHas       => tokens.push(Token::MethodHas),
                RawToken::MethodLast      => tokens.push(Token::MethodLast),
                RawToken::MethodThreshold => tokens.push(Token::MethodThreshold),

                // ── Symbols ──────────────────────────────────────
                RawToken::Dot      => tokens.push(Token::Dot),
                RawToken::Comma    => tokens.push(Token::Comma),
                RawToken::Colon    => tokens.push(Token::Colon),
                RawToken::LParen   => tokens.push(Token::LParen),
                RawToken::RParen   => tokens.push(Token::RParen),
                RawToken::LBrace   => tokens.push(Token::LBrace),
                RawToken::RBrace   => tokens.push(Token::RBrace),
                RawToken::LBracket => tokens.push(Token::LBracket),
                RawToken::RBracket => tokens.push(Token::RBracket),
                RawToken::Star     => tokens.push(Token::Wildcard),

                // ── Operators ────────────────────────────────────
                RawToken::Eq    => tokens.push(Token::Eq),
                RawToken::NotEq => tokens.push(Token::NotEq),
                RawToken::Gt    => tokens.push(Token::Gt),
                RawToken::Lt    => tokens.push(Token::Lt),
                RawToken::Gte   => tokens.push(Token::Gte),
                RawToken::Lte   => tokens.push(Token::Lte),
                RawToken::Pipe  => tokens.push(Token::Pipe),
                RawToken::Bang  => tokens.push(Token::Bang),
            }

            i += 1;
        }

        tokens.push(Token::Eof);
        Ok(tokens)
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn parse_duration(s: &str) -> Result<Token, LexError> {
    let (num_str, unit_str) = split_num_unit(s);
    let value: f64 = num_str.parse().map_err(|_| {
        LexError::InvalidDuration { raw: s.to_string() }
    })?;

    let unit = match unit_str {
        "s"  => TimeUnit::Second,
        "m"  => TimeUnit::Minute,
        "h"  => TimeUnit::Hour,
        "d"  => TimeUnit::Day,
        "w"  => TimeUnit::Week,
        "mo" => TimeUnit::Month,
        "y"  => TimeUnit::Year,
        _    => return Err(LexError::InvalidDuration { raw: s.to_string() }),
    };

    Ok(Token::Duration(Duration { value, unit }))
}

fn parse_distance(s: &str) -> Result<Token, LexError> {
    let (num_str, unit_str) = split_num_unit(s);
    let value: f64 = num_str.parse().map_err(|_| {
        LexError::InvalidDistance { raw: s.to_string() }
    })?;

    let unit = match unit_str {
        "m"  => DistUnit::Meter,
        "km" => DistUnit::Kilometer,
        "mi" => DistUnit::Mile,
        _    => return Err(LexError::InvalidDistance { raw: s.to_string() }),
    };

    Ok(Token::Distance(Distance { value, unit }))
}

fn split_num_unit(s: &str) -> (&str, &str) {
    let pos = s.find(|c: char| c.is_alphabetic()).unwrap_or(s.len());
    (&s[..pos], &s[pos..])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lex(src: &str) -> Vec<Token> {
        ContextLayer::new(src).tokenize().unwrap()
    }

    #[test]
    fn structured_collection() {
        let tokens = lex("users[]");
        assert_eq!(tokens[0], Token::Collection {
            name: "users".to_string(),
            kind: CollectionKind::Structured,
        });
    }

    #[test]
    fn unstructured_collection() {
        let tokens = lex("products{}");
        assert_eq!(tokens[0], Token::Collection {
            name: "products".to_string(),
            kind: CollectionKind::Unstructured,
        });
    }

    #[test]
    fn parses_duration() {
        let tokens = lex("7d");
        assert_eq!(tokens[0], Token::Duration(Duration {
            value: 7.0,
            unit: TimeUnit::Day,
        }));
    }

    #[test]
    fn parses_distance() {
        let tokens = lex("5km");
        assert_eq!(tokens[0], Token::Distance(Distance {
            value: 5.0,
            unit: DistUnit::Kilometer,
        }));
    }

    #[test]
    fn full_query_tokens() {
        let tokens = lex("users[]\n  .where(age > 18)\n  .give(name)\n  .cup(10)");
        // Should start with Collection token
        assert!(matches!(&tokens[0], Token::Collection { name, kind }
            if name == "users" && *kind == CollectionKind::Structured));
    }
}
