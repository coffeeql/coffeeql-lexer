//! CoffeeQL Token Types

#[derive(Debug, Clone, PartialEq)]
pub enum CollectionKind {
    Structured,    // []
    Unstructured,  // {}
}

#[derive(Debug, Clone, PartialEq)]
pub enum TimeUnit {
    Second, Minute, Hour, Day, Week, Month, Year,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DistUnit {
    Meter, Kilometer, Mile,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Duration {
    pub value: f64,
    pub unit:  TimeUnit,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Distance {
    pub value: f64,
    pub unit:  DistUnit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SortDir { Asc, Desc }

#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Uuid, Text, Int, Float, Bool,
    Datetime, Geopoint, Vector,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Constraint {
    Primary, Unique, NotNull,
}

/// Full CoffeeQL Token set
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // ── Collection
    /// users[] or products{}
    Collection {
        name: String,
        kind: CollectionKind,
    },

    // ── Identifiers & Literals
    Identifier(String),
    Int(i64),
    Float(f64),
    Text(String),
    Bool(bool),
    Null,

    // ── Special Values
    Duration(Duration),
    Distance(Distance),

    // ── Chain Keywords
    Where, Give, Sort, Cup,
    Blend, Mix, Pour, Refill, Spill,

    // ── Top-level Keywords
    Shot, Grind, Menu,

    // ── Schema Keywords 
    DataType(DataType),
    Constraint(Constraint),
    Flex,
    On,
    As,

    // ── Sort Direction 
    Asc, Desc,

    // ── Built-in Functions
    FnUuid, FnNow, FnToday,
    FnCount, FnSum, FnAvg, FnMax, FnMin,

    // ── Special Methods
    MethodNear, MethodLike, MethodHas,
    MethodLast, MethodThreshold,

    // ── Exists 
    Exists,

    // ── Symbols 
    Dot, Comma, Colon,
    LParen, RParen,
    LBrace, RBrace,
    LBracket, RBracket,

    // ── Operators
    Eq, NotEq, Gt, Lt, Gte, Lte,
    Or, Bang, Pipe,

    // ── Special
    Wildcard, // *

    // ── End
    Eof,
}
