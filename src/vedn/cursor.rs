use super::{Error, ErrorKind, Span};

/// Byte cursor over a UTF-8 source string.
///
/// This is a tiny helper used by the EDN parser. It is optimized for:
///
/// - Single-pass scanning.
/// - Cheap slicing by byte offsets.
/// - Tracking `line`/`column` for diagnostics.
#[derive(Debug, Clone)]
pub struct Cursor<'a> {
    /// Full input buffer.
    input: &'a str,
    /// Cached byte view of `input`.
    bytes: &'a [u8],
    /// Current byte index into `input`.
    pub index: usize,
    /// Current line number (1-based).
    pub line: u32,
    /// Current column number (1-based).
    pub column: u32,
}

impl<'a> Cursor<'a> {
    /// Creates a new cursor over the provided input.
    pub fn new(input: &'a str) -> Self {
        Cursor {
            input,
            bytes: input.as_bytes(),
            index: 0,
            line: 1,
            column: 1,
        }
    }

    /// Returns true when the cursor has reached the end of the input.
    pub fn is_eof(&self) -> bool {
        self.index >= self.bytes.len()
    }

    /// Returns the remaining input slice.
    pub fn remaining(&self) -> &'a str {
        &self.input[self.index..]
    }

    /// Returns the current byte without advancing.
    pub fn peek(&self) -> Option<u8> {
        self.bytes.get(self.index).copied()
    }

    pub fn peek_next(&self) -> Option<u8> {
        self.bytes.get(self.index + 1).copied()
    }

    /// Advances by one byte and returns it.
    ///
    /// Updates `line`/`column` counters when encountering `\n`.
    pub fn bump(&mut self) -> Option<u8> {
        let byte = self.peek()?;
        self.index += 1;
        if byte == b'\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(byte)
    }

    /// Creates a span from the given start offset to the current cursor position.
    pub fn span_from(&self, start: usize) -> Span {
        Span::new(start, self.index)
    }

    /// Constructs an [`Error`] at the current location.
    pub fn error_here(&self, kind: ErrorKind) -> Error {
        Error::new(
            kind,
            Span::new(self.index, self.index),
            self.line,
            self.column,
        )
    }

    /// Constructs an [`Error`] using an explicit span.
    pub fn error_span(&self, kind: ErrorKind, span: Span) -> Error {
        Error::new(kind, span, self.line, self.column)
    }

    /// Skips EDN whitespace, commas, and `;` line comments.
    pub fn skip_ws_and_comments(&mut self) {
        loop {
            self.skip_ws();
            if self.peek() == Some(b';') {
                while let Some(b) = self.bump() {
                    if b == b'\n' {
                        break;
                    }
                }
                continue;
            }
            break;
        }
    }

    /// Skips EDN whitespace and commas.
    pub fn skip_ws(&mut self) {
        while let Some(b) = self.peek() {
            match b {
                b' ' | b'\t' | b'\r' | b'\n' | b',' => {
                    self.bump();
                }
                _ => break,
            }
        }
    }

    /// Consumes the next byte and ensures it matches `expected`.
    pub fn expect(&mut self, expected: u8) -> Result<(), Error> {
        match self.bump() {
            Some(found) if found == expected => Ok(()),
            Some(found) => Err(self.error_here(ErrorKind::UnexpectedChar {
                found: found as char,
                expected: "delimiter",
            })),
            None => Err(self.error_here(ErrorKind::UnexpectedEof)),
        }
    }

    /// Returns a slice of the original input.
    pub fn slice(&self, start: usize, end: usize) -> &'a str {
        &self.input[start..end]
    }

    /// Consumes bytes while `predicate` returns true and returns the consumed slice.
    ///
    /// The returned slice is taken from `start..self.index`.
    pub fn take_while<F>(&mut self, start: usize, mut predicate: F) -> &'a str
    where
        F: FnMut(u8) -> bool,
    {
        while let Some(b) = self.peek() {
            if !predicate(b) {
                break;
            }
            self.bump();
        }
        self.slice(start, self.index)
    }

    /// Returns an error if the cursor is at EOF.
    pub fn require_non_eof(&self) -> Result<(), Error> {
        if self.is_eof() {
            Err(self.error_here(ErrorKind::UnexpectedEof))
        } else {
            Ok(())
        }
    }
}
