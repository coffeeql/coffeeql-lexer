//! Pre-Processor
//! Removes comments, tracks line/column numbers.

pub struct PreProcessor {
    source: String,
}

impl PreProcessor {
    pub fn new(source: &str) -> Self {
        Self { source: source.to_string() }
    }

    /// Remove comments, normalize whitespace
    pub fn process(&self) -> String {
        let mut result = String::new();
        let mut chars  = self.source.chars().peekable();

        while let Some(ch) = chars.next() {
            match ch {
                // Single-line comment: // or --
                '/' if chars.peek() == Some(&'/') => {
                    // Skip until newline
                    while let Some(&c) = chars.peek() {
                        chars.next();
                        if c == '\n' { break; }
                    }
                    result.push('\n');
                }
                '-' if chars.peek() == Some(&'-') => {
                    while let Some(&c) = chars.peek() {
                        chars.next();
                        if c == '\n' { break; }
                    }
                    result.push('\n');
                }
                // Multi-line comment: /* ... */
                '/' if chars.peek() == Some(&'*') => {
                    chars.next(); // consume *
                    loop {
                        match chars.next() {
                            Some('*') if chars.peek() == Some(&'/') => {
                                chars.next(); // consume /
                                break;
                            }
                            Some('\n') => result.push('\n'),
                            None => break,
                            _ => {}
                        }
                    }
                }
                other => result.push(other),
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removes_single_line_comments() {
        let src = "users[] // this is a comment\n  .where(age > 18)";
        let out = PreProcessor::new(src).process();
        assert!(out.contains(".where(age > 18)"));
        assert!(!out.contains("this is a comment"));
    }

    #[test]
    fn removes_dash_comments() {
        let src = "users[] -- SQL style\n  .give(name)";
        let out = PreProcessor::new(src).process();
        assert!(!out.contains("SQL style"));
    }

    #[test]
    fn removes_block_comments() {
        let src = "users[] /* block\n comment */ .give(name)";
        let out = PreProcessor::new(src).process();
        assert!(!out.contains("block"));
        assert!(out.contains(".give(name)"));
    }
}
