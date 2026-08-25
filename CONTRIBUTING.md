# Contributing to coffeeql-lexer

## What you can work on

Issues labelled `lexer` in this repo.

Scope:
- Token definitions (`token.rs`)
- Span tracking — line, col, offset on every token
- Error messages — human-readable, with hints
- Keyword suggestions on typos
- Incremental lexing for IDE use
- Comment support (`//`)
- Test coverage for all error cases

## Setup

\`\`\`bash
git clone https://github.com/coffeeql/coffeeql-lexer
cd coffeeql-lexer
cargo build
cargo test
\`\`\`

## PR rules

- cargo test must pass
- cargo clippy must pass
- cargo fmt must be applied
- One issue per PR
- Reference the issue number in PR title: `fix #001 — improve error for invalid char`

## CLA

You will be prompted to sign the CLA on your first PR.
This allows CoffeeQL to relicense the code in future if needed.
