# coffeeql-lexer

The tokenizer for CoffeeQL.

Converts a raw CoffeeQL query string into a typed token stream
that the parser consumes.

## What this does

\`\`\`
"users[].where(plan = \"pro\").cup(10)"
         ↓
[Collection("users", Structured), Dot, Where, LParen,
 Ident("plan"), Eq, Str("pro"), RParen, Dot, Cup,
 LParen, Int(10), RParen]
\`\`\`

## Crate structure

- `src/token.rs`  — Token enum and Span struct
- `src/mod.rs`    — Lexer implementation
- `src/error.rs`  — Lexer-specific errors
- `src/tests.rs`  — Test suite

## Usage

\`\`\`rust
use coffeeql_lexer::Lexer;

let tokens = Lexer::new("users[].where(plan = \"pro\").cup(10)").tokenize();
\`\`\`

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).
Good first issues are labelled `good-first-issue`.

## Part of CoffeeQL

This is the open-source syntax layer of CoffeeQL.
The execution engine (planner, executor, adapters)
ships as compiled binaries via npm and PyPI.

- npm: `npm install coffeeql`
- pip: `pip install coffeeql`
- pub: `dart pub add coffeeql`
- docs: [coffeeql.dev](https://coffeeql.dev)

## License

Apache 2.0 + Commons Clause — see [LICENSE](LICENSE)
