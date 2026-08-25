//! Logos-based raw tokenizer — fast regex matching layer

use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\r\n]+")] // Skip whitespace
pub enum RawToken {
    // ── Chain Keywords ──────────────────────────────────────
    #[token("where")]
    Where,
    #[token("give")]
    Give,
    #[token("sort")]
    Sort,
    #[token("cup")]
    Cup,
    #[token("blend")]
    Blend,
    #[token("mix")]
    Mix,
    #[token("pour")]
    Pour,
    #[token("refill")]
    Refill,
    #[token("spill")]
    Spill,

    // ── Top Level ───────────────────────────────────────────
    #[token("shot")]
    Shot,
    #[token("grind")]
    Grind,
    #[token("menu")]
    Menu,

    // ── Schema ──────────────────────────────────────────────
    // ON — accept both lowercase and uppercase
    #[token("on")]
    #[token("ON")]
    On,

    #[token("as")]
    As,
    #[token("ASC")]
    Asc,
    #[token("DESC")]
    Desc,
    #[token("FLEX")]
    Flex,
    #[token("EXISTS")]
    Exists,
    #[token("NOT")]
    Not,
    #[token("NULL")]
    Null,
    #[token("PRIMARY")]
    Primary,
    #[token("UNIQUE")]
    Unique,

    // ── Data Types ──────────────────────────────────────────
    #[token("UUID")]
    TyUuid,
    #[token("TEXT")]
    TyText,
    // INT and INTEGER both accepted
    #[token("INT")]
    #[token("INTEGER")]
    TyInt,
    #[token("FLOAT")]
    TyFloat,
    #[token("BOOL")]
    TyBool,
    #[token("DATETIME")]
    TyDatetime,
    #[token("GEOPOINT")]
    TyGeopoint,
    #[token("VECTOR")]
    TyVector,

    // ── Built-in Functions ──────────────────────────────────
    #[token("uuid")]
    FnUuid,
    #[token("now")]
    FnNow,
    #[token("today")]
    FnToday,
    #[token("COUNT")]
    FnCount,
    #[token("SUM")]
    FnSum,
    #[token("AVG")]
    FnAvg,
    #[token("MAX")]
    FnMax,
    #[token("MIN")]
    FnMin,

    // ── Special Methods ─────────────────────────────────────
    #[token("near")]
    MethodNear,
    #[token("like")]
    MethodLike,
    #[token("has")]
    MethodHas,
    #[token("last")]
    MethodLast,
    #[token("threshold")]
    MethodThreshold,

    // ── Bool Literals ───────────────────────────────────────
    #[token("true")]
    True,
    #[token("false")]
    False,

    // ── Number Literals ─────────────────────────────────────
    // Float first — more specific
    #[regex(r"[0-9]+\.[0-9]+")]
    Float,

    // Duration: 7d, 30m, 2h, 1w, 3mo, 1y
    #[regex(r"[0-9]+(mo|s|h|d|w|y)", priority = 5)]
    Duration,

    #[regex(r"[0-9]+(km|mi|m)", priority = 4)]
    Distance,

    // Plain integer
    #[regex(r"[0-9]+")]
    Int,

    // ── String Literals ─────────────────────────────────────
    #[regex(r#""([^"\\]|\\.)*""#)]
    Text,

    // ── Identifiers ─────────────────────────────────────────
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    // ── Symbols ─────────────────────────────────────────────
    #[token(".")]
    Dot,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("*")]
    Star,

    // ── Operators (longer first) ────────────────────────────
    #[token(">=")]
    Gte,
    #[token("<=")]
    Lte,
    #[token("!=")]
    NotEq,
    #[token(">")]
    Gt,
    #[token("<")]
    Lt,
    #[token("=")]
    Eq,
    #[token("|")]
    Pipe,
    #[token("!")]
    Bang,
}
