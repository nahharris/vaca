use super::{
    cursor::Cursor,
    error::{Error, ErrorKind, Span},
    value::{Keyword, Kind, Node, Number, NumberSuffix, Str, Symbol},
};

/// Parses all top-level EDN elements from `input`.
///
/// EDN has no mandatory top-level delimiter. This function therefore returns a
/// sequence of nodes.
///
/// # Annotated forms
/// Annotated elements (`#<form> <form>`) are preserved as [`Node::annotation`].
/// The parser never interprets annotations.
pub fn parse(input: &str) -> Result<Vec<Node<'_>>, Error> {
    Parser::new(input).parse_all()
}

/// Streaming EDN parser.
///
/// The parser reads directly from the input string (no token buffering) and
/// produces a borrowed AST.
///
/// Whitespace and commas are treated as separators. `;` starts a line comment.
#[derive(Debug, Clone)]
pub struct Parser<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Parser<'a> {
    /// Creates a new parser over `input`.
    pub fn new(input: &'a str) -> Self {
        Parser {
            cursor: Cursor::new(input),
        }
    }

    /// Parses all top-level elements until EOF.
    pub fn parse_all(mut self) -> Result<Vec<Node<'a>>, Error> {
        let mut nodes = Vec::new();
        loop {
            self.cursor.skip_ws_and_comments();
            if self.cursor.is_eof() {
                break;
            }
            if let Some(node) = self.parse_form()? {
                nodes.push(node);
            }
        }
        Ok(nodes)
    }

    fn parse_form(&mut self) -> Result<Option<Node<'a>>, Error> {
        self.cursor.skip_ws_and_comments();
        let Some(b) = self.cursor.peek() else {
            return Err(self.cursor.error_here(ErrorKind::UnexpectedEof));
        };

        match b {
            b'(' => Ok(Some(self.parse_list()?)),
            b'[' => Ok(Some(self.parse_vector()?)),
            b'{' => Ok(Some(self.parse_map()?)),
            b'%' => {
                if self.cursor.peek_next() == Some(b'{') {
                    Ok(Some(self.parse_set()?))
                } else {
                    Ok(Some(self.parse_token()?))
                }
            }
            b'"' => Ok(Some(self.parse_string()?)),
            b':' => Ok(Some(self.parse_keyword_node()?)),
            b'\\' => Ok(Some(self.parse_char()?)),
            b'#' => self.parse_dispatch(),
            _ => Ok(Some(self.parse_token()?)),
        }
    }

    /// Parses a single form without skipping leading separators.
    ///
    /// This is used for parsing the *annotation* part of `#<form> <form>`, where
    /// the annotation form must start immediately after `#`.
    fn parse_form_no_skip(&mut self) -> Result<Option<Node<'a>>, Error> {
        let Some(b) = self.cursor.peek() else {
            return Err(self.cursor.error_here(ErrorKind::UnexpectedEof));
        };

        match b {
            b'(' => Ok(Some(self.parse_list()?)),
            b'[' => Ok(Some(self.parse_vector()?)),
            b'{' => Ok(Some(self.parse_map()?)),
            b'%' => {
                if self.cursor.peek_next() == Some(b'{') {
                    Ok(Some(self.parse_set()?))
                } else {
                    Ok(Some(self.parse_token()?))
                }
            }
            b'"' => Ok(Some(self.parse_string()?)),
            b':' => Ok(Some(self.parse_keyword_node()?)),
            b'\\' => Ok(Some(self.parse_char()?)),
            b'#' => self.parse_dispatch(),
            _ => Ok(Some(self.parse_token()?)),
        }
    }

    /// Parses a list: `(<value>...)`.
    fn parse_list(&mut self) -> Result<Node<'a>, Error> {
        let start = self.cursor.index;
        self.cursor.bump();

        let mut values = Vec::new();
        loop {
            self.cursor.skip_ws_and_comments();
            match self.cursor.peek() {
                Some(b')') => {
                    self.cursor.bump();
                    break;
                }
                None => {
                    return Err(self.cursor.error_span(
                        ErrorKind::UnterminatedCollection { expected: ')' },
                        Span::new(start, self.cursor.index),
                    ));
                }
                _ => {
                    if let Some(v) = self.parse_form()? {
                        values.push(v);
                    }
                }
            }
        }

        Ok(Node::new(self.cursor.span_from(start), Kind::List(values)))
    }

    /// Parses a vector: `[<value>...]`.
    fn parse_vector(&mut self) -> Result<Node<'a>, Error> {
        let start = self.cursor.index;
        self.cursor.bump();

        let mut values = Vec::new();
        loop {
            self.cursor.skip_ws_and_comments();
            match self.cursor.peek() {
                Some(b']') => {
                    self.cursor.bump();
                    break;
                }
                None => {
                    return Err(self.cursor.error_span(
                        ErrorKind::UnterminatedCollection { expected: ']' },
                        Span::new(start, self.cursor.index),
                    ));
                }
                _ => {
                    if let Some(v) = self.parse_form()? {
                        values.push(v);
                    }
                }
            }
        }

        Ok(Node::new(
            self.cursor.span_from(start),
            Kind::Vector(values),
        ))
    }

    /// Parses a map: `{<key> <value> ...}`.
    ///
    /// EDN maps must contain an even number of forms.
    fn parse_map(&mut self) -> Result<Node<'a>, Error> {
        let start = self.cursor.index;
        self.cursor.bump();

        let mut items = Vec::new();
        loop {
            self.cursor.skip_ws_and_comments();
            match self.cursor.peek() {
                Some(b'}') => {
                    self.cursor.bump();
                    break;
                }
                None => {
                    return Err(self.cursor.error_span(
                        ErrorKind::UnterminatedCollection { expected: '}' },
                        Span::new(start, self.cursor.index),
                    ));
                }
                _ => {
                    if let Some(item) = self.parse_form()? {
                        items.push(item);
                    }
                }
            }
        }

        if items.len() % 2 != 0 {
            let last_start = items
                .last()
                .map(|n| n.span.start)
                .unwrap_or(self.cursor.index);
            return Err(self.cursor.error_span(
                ErrorKind::MapOddNumberOfForms,
                Span::new(last_start, self.cursor.index),
            ));
        }

        let mut entries = Vec::with_capacity(items.len() / 2);
        let mut iter = items.into_iter();
        while let (Some(k), Some(v)) = (iter.next(), iter.next()) {
            entries.push((k, v));
        }

        Ok(Node::new(self.cursor.span_from(start), Kind::Map(entries)))
    }

    /// Parses a set: `%{<form>*}`.
    fn parse_set(&mut self) -> Result<Node<'a>, Error> {
        let start = self.cursor.index;
        self.cursor.bump(); // '%'
        self.cursor.expect(b'{')?;

        let mut values = Vec::new();
        loop {
            self.cursor.skip_ws_and_comments();
            match self.cursor.peek() {
                Some(b'}') => {
                    self.cursor.bump();
                    break;
                }
                None => {
                    return Err(self.cursor.error_span(
                        ErrorKind::UnterminatedCollection { expected: '}' },
                        Span::new(start, self.cursor.index),
                    ));
                }
                _ => {
                    if let Some(v) = self.parse_form()? {
                        values.push(v);
                    }
                }
            }
        }

        Ok(Node::new(self.cursor.span_from(start), Kind::Set(values)))
    }

    /// Parses a `#` dispatch form.
    ///
    /// Supported dispatches:
    ///
    /// - `## <form>`: discard (reader discard)
    /// - `#<form> <form>`: annotation (preserved as [`Node::annotation`])
    fn parse_dispatch(&mut self) -> Result<Option<Node<'a>>, Error> {
        let start = self.cursor.index;
        self.cursor.bump(); // '#'

        match self.cursor.peek() {
            Some(b'#') => {
                // Reader discard: `## <form>`
                self.cursor.bump(); // second '#'
                self.cursor.skip_ws_and_comments();
                // Discard the next readable element.
                let _discarded = self.parse_form()?;
                Ok(None)
            }
            Some(b'_') => Err(self.cursor.error_here(ErrorKind::InvalidDispatch)),
            Some(b' ' | b'\t' | b'\r' | b'\n' | b',' | b';') => {
                // `#` must be immediately followed by a form.
                Err(self.cursor.error_here(ErrorKind::InvalidDispatch))
            }
            Some(_) => {
                // Annotation: `#<form> <form>`
                let Some(annotation) = self.parse_form_no_skip()? else {
                    return Err(self.cursor.error_here(ErrorKind::UnexpectedEof));
                };
                self.cursor.skip_ws_and_comments();
                if self.cursor.is_eof() {
                    return Err(self.cursor.error_here(ErrorKind::UnexpectedEof));
                }
                let Some(mut form) = self.parse_form()? else {
                    return Err(self.cursor.error_here(ErrorKind::UnexpectedEof));
                };

                // Expand the form span to include the whole `#... <form>` sequence.
                form.span = self.cursor.span_from(start);

                // Attach annotation. If the form is already annotated (e.g. `#a #b x`),
                // preserve both by collecting them into a list in source order.
                form.annotation = Some(Box::new(match form.annotation.take() {
                    None => annotation,
                    Some(prev) => {
                        let prev = *prev;
                        let span = Span::new(prev.span.start, annotation.span.end);
                        Node::new(span, Kind::List(vec![prev, annotation]))
                    }
                }));

                Ok(Some(form))
            }
            None => Err(self.cursor.error_here(ErrorKind::InvalidDispatch)),
        }
    }

    /// Parses a string literal.
    fn parse_string(&mut self) -> Result<Node<'a>, Error> {
        let start = self.cursor.index;
        self.cursor.bump(); // '"'
        let content_start = self.cursor.index;

        let mut has_escape = false;
        while let Some(b) = self.cursor.peek() {
            match b {
                b'"' => break,
                b'\\' => {
                    has_escape = true;
                    self.cursor.bump();
                    if self.cursor.bump().is_none() {
                        return Err(self.cursor.error_here(ErrorKind::UnterminatedString));
                    }
                }
                _ => {
                    self.cursor.bump();
                }
            }
        }

        if self.cursor.peek() != Some(b'"') {
            return Err(self.cursor.error_span(
                ErrorKind::UnterminatedString,
                Span::new(start, self.cursor.index),
            ));
        }

        let content_end = self.cursor.index;
        self.cursor.bump(); // closing '"'

        let raw = self.cursor.slice(content_start, content_end);
        let string = if !has_escape {
            Str::Borrowed(raw)
        } else {
            Str::Owned(unescape_string(raw).map_err(|kind| {
                self.cursor
                    .error_span(kind, Span::new(content_start, content_end))
            })?)
        };

        Ok(Node::new(
            self.cursor.span_from(start),
            Kind::String(string),
        ))
    }

    /// Parses a character literal.
    fn parse_char(&mut self) -> Result<Node<'a>, Error> {
        let start = self.cursor.index;
        self.cursor.bump(); // '\\'

        let token_start = self.cursor.index;
        let token = self.cursor.take_while(token_start, |b| !is_delim_or_ws(b));
        if token.is_empty() {
            return Err(self.cursor.error_span(
                ErrorKind::InvalidCharacterLiteral,
                Span::new(start, self.cursor.index),
            ));
        }

        let value = match token {
            "newline" => '\n',
            "return" => '\r',
            "space" => ' ',
            "tab" => '\t',
            _ if token.starts_with('u') && token.len() == 5 => {
                let hex = &token[1..];
                let code = u16::from_str_radix(hex, 16).map_err(|_| {
                    self.cursor.error_span(
                        ErrorKind::InvalidUnicodeEscape,
                        Span::new(token_start, self.cursor.index),
                    )
                })?;
                char::from_u32(code as u32).ok_or_else(|| {
                    self.cursor.error_span(
                        ErrorKind::InvalidUnicodeEscape,
                        Span::new(token_start, self.cursor.index),
                    )
                })?
            }
            _ => {
                let mut chars = token.chars();
                let ch = chars.next().ok_or_else(|| {
                    self.cursor.error_span(
                        ErrorKind::InvalidCharacterLiteral,
                        Span::new(token_start, self.cursor.index),
                    )
                })?;
                if chars.next().is_some() {
                    return Err(self.cursor.error_span(
                        ErrorKind::InvalidCharacterLiteral,
                        Span::new(token_start, self.cursor.index),
                    ));
                }
                ch
            }
        };

        Ok(Node::new(self.cursor.span_from(start), Kind::Char(value)))
    }

    /// Parses a keyword token starting with `:`.
    fn parse_keyword_node(&mut self) -> Result<Node<'a>, Error> {
        let start = self.cursor.index;
        let token_start = self.cursor.index;
        let token = self.cursor.take_while(token_start, |b| !is_delim_or_ws(b));

        let keyword = parse_keyword_token(token).map_err(|kind| {
            self.cursor
                .error_span(kind, Span::new(token_start, self.cursor.index))
        })?;

        Ok(Node::new(
            self.cursor.span_from(start),
            Kind::Keyword(keyword),
        ))
    }

    /// Parses a token that is not delimited by a special leading character.
    ///
    /// This includes: nil/bools, numbers, and symbols.
    fn parse_token(&mut self) -> Result<Node<'a>, Error> {
        let start = self.cursor.index;
        let token_start = self.cursor.index;
        let token = self.cursor.take_while(token_start, |b| !is_delim_or_ws(b));
        let span = self.cursor.span_from(start);

        match token {
            "nil" => Ok(Node::new(span, Kind::Nil)),
            "true" => Ok(Node::new(span, Kind::Bool(true))),
            "false" => Ok(Node::new(span, Kind::Bool(false))),
            _ => {
                // Trailing-colon keyword: `symbol:`
                if token.ends_with(':') {
                    let keyword = parse_keyword_token(token).map_err(|kind| {
                        self.cursor
                            .error_span(kind, Span::new(token_start, self.cursor.index))
                    })?;
                    return Ok(Node::new(span, Kind::Keyword(keyword)));
                }
                if let Ok(number) = parse_number(token) {
                    return Ok(Node::new(span, Kind::Number(number)));
                }
                let symbol = parse_symbol(token).map_err(|kind| {
                    self.cursor
                        .error_span(kind, Span::new(token_start, self.cursor.index))
                })?;
                Ok(Node::new(span, Kind::Symbol(symbol)))
            }
        }
    }
}

fn is_delim_or_ws(b: u8) -> bool {
    matches!(
        b,
        b' ' | b'\t' | b'\r' | b'\n' | b',' | b'(' | b')' | b'[' | b']' | b'{' | b'}' | b'"' | b';'
    )
}

/// Unescapes the contents of an EDN string literal.
///
/// The input must not include the surrounding `"` quotes.
fn unescape_string(raw: &str) -> Result<String, ErrorKind> {
    let mut out = String::with_capacity(raw.len());
    let mut chars = raw.chars();
    while let Some(ch) = chars.next() {
        if ch != '\\' {
            out.push(ch);
            continue;
        }
        let Some(esc) = chars.next() else {
            return Err(ErrorKind::UnterminatedString);
        };
        match esc {
            't' => out.push('\t'),
            'r' => out.push('\r'),
            'n' => out.push('\n'),
            '\\' => out.push('\\'),
            '"' => out.push('"'),
            'u' => {
                let mut hex = String::with_capacity(4);
                for _ in 0..4 {
                    let Some(h) = chars.next() else {
                        return Err(ErrorKind::InvalidUnicodeEscape);
                    };
                    hex.push(h);
                }
                let code =
                    u16::from_str_radix(&hex, 16).map_err(|_| ErrorKind::InvalidUnicodeEscape)?;
                let c = char::from_u32(code as u32).ok_or(ErrorKind::InvalidUnicodeEscape)?;
                out.push(c);
            }
            _ => return Err(ErrorKind::UnterminatedString),
        }
    }
    Ok(out)
}

/// Parses and validates a keyword token.
///
/// `token` must be either `:<symbol>` or `<symbol>:`.
fn parse_keyword_token(token: &str) -> Result<Keyword<'_>, ErrorKind> {
    if token.is_empty() {
        return Err(ErrorKind::InvalidKeyword);
    }

    if token.starts_with(':') {
        // Leading-colon keywords: `:name`, `:ns/name`
        if token.starts_with("::") || token.starts_with(":/") {
            return Err(ErrorKind::InvalidKeyword);
        }

        let raw = token;
        let without_colon = &token[1..];
        let symbol = parse_symbol(without_colon).map_err(|_| ErrorKind::InvalidKeyword)?;
        return Ok(Keyword {
            raw,
            namespace: symbol.namespace,
            name: symbol.name,
        });
    }

    if token.ends_with(':') {
        // Trailing-colon keywords: `name:`, `ns/name:`
        let base = &token[..token.len() - 1];
        if base.is_empty() {
            return Err(ErrorKind::InvalidKeyword);
        }
        let symbol = parse_symbol(base).map_err(|_| ErrorKind::InvalidKeyword)?;
        return Ok(Keyword {
            raw: token,
            namespace: symbol.namespace,
            name: symbol.name,
        });
    }

    Err(ErrorKind::InvalidKeyword)
}

/// Parses and validates a symbol token according to EDN's strict rules.
fn parse_symbol(token: &str) -> Result<Symbol<'_>, ErrorKind> {
    if token.is_empty() {
        return Err(ErrorKind::InvalidSymbol);
    }

    // Special-case: '/' alone is allowed.
    if token == "/" {
        return Ok(Symbol {
            raw: token,
            namespace: None,
            name: token,
        });
    }

    if token.starts_with(':') {
        return Err(ErrorKind::InvalidSymbol);
    }

    let slash_count = token.as_bytes().iter().filter(|b| **b == b'/').count();
    if slash_count > 1 {
        return Err(ErrorKind::InvalidSymbol);
    }

    let (namespace, name) = if let Some((ns, nm)) = token.split_once('/') {
        if ns.is_empty() || nm.is_empty() {
            return Err(ErrorKind::InvalidSymbol);
        }
        (Some(ns), nm)
    } else {
        (None, token)
    };

    validate_symbol_component(name)?;
    if let Some(ns) = namespace {
        validate_symbol_component(ns)?;
    }

    Ok(Symbol {
        raw: token,
        namespace,
        name,
    })
}

fn validate_symbol_component(s: &str) -> Result<(), ErrorKind> {
    let mut chars = s.chars();
    let Some(first) = chars.next() else {
        return Err(ErrorKind::InvalidSymbol);
    };

    // first character cannot be numeric.
    if first.is_ascii_digit() {
        return Err(ErrorKind::InvalidSymbol);
    }

    // If first is '-', '+', or '.', second char (if any) must be non-numeric.
    if matches!(first, '-' | '+' | '.') {
        if let Some(second) = chars.clone().next() {
            if second.is_ascii_digit() {
                return Err(ErrorKind::InvalidSymbol);
            }
        }
    }

    // Validate all chars.
    for ch in s.chars() {
        if is_symbol_char(ch) {
            continue;
        }
        return Err(ErrorKind::InvalidSymbol);
    }

    Ok(())
}

fn is_symbol_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric()
        || matches!(
            ch,
            '.' | '*' | '+' | '!' | '-' | '_' | '?' | '$' | '%' | '&' | '=' | '<' | '>' | ':' | '#'
        )
}

/// Parses and validates a number token.
///
/// The returned number preserves the original lexeme.
fn parse_number(token: &str) -> Result<Number<'_>, ErrorKind> {
    if token.is_empty() {
        return Err(ErrorKind::InvalidNumber);
    }

    let (core, suffix) = match token.as_bytes().last().copied() {
        Some(b'N') => (&token[..token.len() - 1], NumberSuffix::BigInt),
        Some(b'M') => (&token[..token.len() - 1], NumberSuffix::BigDecimal),
        _ => (token, NumberSuffix::None),
    };

    if core.is_empty() {
        return Err(ErrorKind::InvalidNumber);
    }

    if is_int(core) {
        return Ok(Number::Int {
            lexeme: token,
            suffix,
        });
    }

    if is_float(core) {
        return Ok(Number::Float {
            lexeme: token,
            suffix,
        });
    }

    Err(ErrorKind::InvalidNumber)
}

fn is_int(s: &str) -> bool {
    let s = s.strip_prefix('+').unwrap_or(s);
    let Some(rest) = s.strip_prefix('-').or(Some(s)) else {
        return false;
    };

    if rest.is_empty() {
        return false;
    }

    if rest.len() > 1 && rest.starts_with('0') {
        return false;
    }

    rest.chars().all(|c| c.is_ascii_digit())
}

fn is_float(s: &str) -> bool {
    // EDN float forms require an integer part.
    // (int frac), (int exp), (int frac exp)

    // Split exponent if present.
    let (mantissa, exp) = split_exp(s);
    if let Some(exp) = exp {
        if exp.is_empty() {
            return false;
        }
        let exp = exp
            .strip_prefix('+')
            .or_else(|| exp.strip_prefix('-'))
            .unwrap_or(exp);
        if exp.is_empty() || !exp.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
    }

    // mantissa is an int or int.frac
    if let Some((int_part, frac_part)) = mantissa.split_once('.') {
        if int_part.is_empty() {
            return false;
        }
        if frac_part.is_empty() {
            return false;
        }
        if !is_int(int_part) {
            return false;
        }
        if !frac_part.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        true
    } else {
        // no dot => must have exponent
        exp.is_some() && is_int(mantissa)
    }
}

fn split_exp(s: &str) -> (&str, Option<&str>) {
    for (i, b) in s.as_bytes().iter().enumerate() {
        if matches!(*b, b'e' | b'E') {
            let mantissa = &s[..i];
            let exp = &s[i + 1..];
            return (mantissa, Some(exp));
        }
    }
    (s, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_symbol(node: &Node<'_>, raw: &str) {
        let Kind::Symbol(sym) = &node.kind else {
            panic!("expected Symbol, got: {:?}", node.kind);
        };
        assert_eq!(sym.raw, raw);
    }

    fn assert_keyword(node: &Node<'_>, raw: &str) {
        let Kind::Keyword(kw) = &node.kind else {
            panic!("expected Keyword");
        };
        assert_eq!(kw.raw, raw);
    }

    #[test]
    fn parse_multiple_top_level() {
        let values = parse("1 2 3").unwrap();
        assert_eq!(values.len(), 3);
    }

    #[test]
    fn parse_whitespace_commas_and_comments() {
        let values = parse("1,2 ; comment\n 3").unwrap();
        assert_eq!(values.len(), 3);
    }

    #[test]
    fn parse_nil_and_bools() {
        let values = parse("nil true false").unwrap();
        assert!(matches!(values[0].kind, Kind::Nil));
        assert!(matches!(values[1].kind, Kind::Bool(true)));
        assert!(matches!(values[2].kind, Kind::Bool(false)));
    }

    #[test]
    fn parse_strings_borrowed_and_escaped() {
        let values = parse("\"abc\" \"a\\nb\"").unwrap();
        let Kind::String(Str::Borrowed(s)) = &values[0].kind else {
            panic!("expected borrowed string");
        };
        assert_eq!(*s, "abc");

        let Kind::String(Str::Owned(s)) = &values[1].kind else {
            panic!("expected owned string");
        };
        assert_eq!(s, "a\nb");
    }

    #[test]
    fn parse_chars() {
        let values = parse("\\c \\newline \\u0041").unwrap();
        assert!(matches!(values[0].kind, Kind::Char('c')));
        assert!(matches!(values[1].kind, Kind::Char('\n')));
        assert!(matches!(values[2].kind, Kind::Char('A')));
    }

    #[test]
    fn parse_numbers() {
        let values = parse("0 -0 +1 42N 1.5 1e9 2.0M").unwrap();
        assert!(matches!(values[0].kind, Kind::Number(Number::Int { .. })));
        assert!(matches!(values[1].kind, Kind::Number(Number::Int { .. })));
        assert!(matches!(values[2].kind, Kind::Number(Number::Int { .. })));
        assert!(matches!(
            values[3].kind,
            Kind::Number(Number::Int {
                suffix: NumberSuffix::BigInt,
                ..
            })
        ));
        assert!(matches!(values[4].kind, Kind::Number(Number::Float { .. })));
        assert!(matches!(values[5].kind, Kind::Number(Number::Float { .. })));
        assert!(matches!(
            values[6].kind,
            Kind::Number(Number::Float {
                suffix: NumberSuffix::BigDecimal,
                ..
            })
        ));
    }

    #[test]
    fn parse_collections() {
        let values = parse("(a 1) [a 1] {:a 1, :b 2} %{a b}").unwrap();
        let Kind::List(list) = &values[0].kind else {
            panic!("expected list");
        };
        assert_symbol(&list[0], "a");

        let Kind::Vector(vec) = &values[1].kind else {
            panic!("expected vector");
        };
        assert_symbol(&vec[0], "a");

        let Kind::Map(map) = &values[2].kind else {
            panic!("expected map");
        };
        assert_keyword(&map[0].0, ":a");

        let Kind::Set(set) = &values[3].kind else {
            panic!("expected set");
        };
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn parse_trailing_colon_keywords() {
        let values = parse("{:x 1, ns/name: 2}").unwrap();
        let Kind::Map(map) = &values[0].kind else {
            panic!("expected map");
        };
        assert_keyword(&map[0].0, ":x");
        assert_keyword(&map[1].0, "ns/name:");
    }

    #[test]
    fn parse_annotated_symbol() {
        let values = parse("#inst \"2020-01-01\"").unwrap();
        let Some(annotation) = &values[0].annotation else {
            panic!("expected annotation");
        };
        let Kind::Symbol(ann) = &annotation.kind else {
            panic!("expected symbol annotation");
        };
        assert_eq!(ann.name, "inst");
        let Kind::String(s) = &values[0].kind else {
            panic!("expected string");
        };
        assert_eq!(s.as_str(), "2020-01-01");
    }

    #[test]
    fn parse_annotated_list() {
        let values = parse("#(vec int) [1 2]").unwrap();
        let Some(annotation) = &values[0].annotation else {
            panic!("expected annotation");
        };
        let Kind::List(ann_list) = &annotation.kind else {
            panic!("expected list annotation");
        };
        assert_symbol(&ann_list[0], "vec");
        assert_symbol(&ann_list[1], "int");

        let Kind::Vector(v) = &values[0].kind else {
            panic!("expected vector value");
        };
        assert_eq!(v.len(), 2);
    }

    #[test]
    fn parse_nested_annotated_list() {
        let values = parse("#(vec (vec int)) [ [1] [2] ]").unwrap();
        let Some(annotation) = &values[0].annotation else {
            panic!("expected annotation");
        };
        let Kind::List(ann_list) = &annotation.kind else {
            panic!("expected list annotation");
        };
        assert_symbol(&ann_list[0], "vec");

        let Kind::List(inner) = &ann_list[1].kind else {
            panic!("expected nested list annotation");
        };
        assert_symbol(&inner[0], "vec");
        assert_symbol(&inner[1], "int");
    }

    #[test]
    fn parse_annotated_list_with_multiple_items() {
        let values = parse("#(map keyword int) {:a 1}").unwrap();
        let Some(annotation) = &values[0].annotation else {
            panic!("expected annotation");
        };
        let Kind::List(ann_list) = &annotation.kind else {
            panic!("expected list annotation");
        };
        assert_symbol(&ann_list[0], "map");
        assert_symbol(&ann_list[1], "keyword");
        assert_symbol(&ann_list[2], "int");

        let Kind::Map(entries) = &values[0].kind else {
            panic!("expected map value");
        };
        assert_eq!(entries.len(), 1);
        assert_keyword(&entries[0].0, ":a");
    }

    #[test]
    fn parse_annotation_can_be_keyword() {
        let values = parse("#:ann 1").unwrap();
        let Some(annotation) = &values[0].annotation else {
            panic!("expected annotation");
        };
        assert_keyword(annotation, ":ann");
        assert!(matches!(values[0].kind, Kind::Number(Number::Int { .. })));
    }

    #[test]
    fn parse_annotation_can_be_vector() {
        let values = parse("#[1 2] foo").unwrap();
        let Some(annotation) = &values[0].annotation else {
            panic!("expected annotation");
        };
        let Kind::Vector(items) = &annotation.kind else {
            panic!("expected vector annotation");
        };
        assert_eq!(items.len(), 2);
        assert!(matches!(items[0].kind, Kind::Number(Number::Int { .. })));
        assert!(matches!(items[1].kind, Kind::Number(Number::Int { .. })));
        assert_symbol(&values[0], "foo");
    }

    #[test]
    fn parse_annotation_can_be_string() {
        let values = parse("#\"ann\" 1").unwrap();
        let Some(annotation) = &values[0].annotation else {
            panic!("expected annotation");
        };
        let Kind::String(s) = &annotation.kind else {
            panic!("expected string annotation");
        };
        assert_eq!(s.as_str(), "ann");
    }

    #[test]
    fn parse_annotation_can_be_number() {
        let values = parse("#42 foo").unwrap();
        let Some(annotation) = &values[0].annotation else {
            panic!("expected annotation");
        };
        assert!(matches!(annotation.kind, Kind::Number(Number::Int { .. })));
        assert_symbol(&values[0], "foo");
    }

    #[test]
    fn parse_annotation_can_be_nil_and_bool() {
        let values = parse("#nil 1 #true 2").unwrap();
        let Some(a0) = &values[0].annotation else {
            panic!("expected annotation");
        };
        assert!(matches!(a0.kind, Kind::Nil));

        let Some(a1) = &values[1].annotation else {
            panic!("expected annotation");
        };
        assert!(matches!(a1.kind, Kind::Bool(true)));
    }

    #[test]
    fn parse_annotation_can_be_char() {
        let values = parse("#\\c foo").unwrap();
        let Some(annotation) = &values[0].annotation else {
            panic!("expected annotation");
        };
        assert!(matches!(annotation.kind, Kind::Char('c')));
        assert_symbol(&values[0], "foo");
    }

    #[test]
    fn parse_discard() {
        let values = parse("[a ## foo 42]").unwrap();
        let Kind::Vector(v) = &values[0].kind else {
            panic!("expected vector");
        };
        assert_eq!(v.len(), 2);
        assert_symbol(&v[0], "a");
        assert!(matches!(v[1].kind, Kind::Number(Number::Int { .. })));
    }

    #[test]
    fn parse_set_percent_syntax() {
        let values = parse("%{a b}").unwrap();
        let Kind::Set(items) = &values[0].kind else {
            panic!("expected set");
        };
        assert_eq!(items.len(), 2);
        assert_symbol(&items[0], "a");
        assert_symbol(&items[1], "b");
    }

    #[test]
    fn parse_discard_can_discard_multiple_and_at_end_of_collection() {
        let values = parse("[1 ## 2 ## ## 3]").unwrap();
        let Kind::Vector(v) = &values[0].kind else {
            panic!("expected vector");
        };
        assert_eq!(v.len(), 1);
        assert!(matches!(v[0].kind, Kind::Number(Number::Int { .. })));
    }

    #[test]
    fn parse_vaca_sample_hello_world() {
        let input = include_str!("../samples/hello_world.vaca");
        let forms = parse(input).unwrap();
        assert_eq!(forms.len(), 3);

        // (defn #int sum [#int a #int b] ...)
        let Kind::List(defn_list) = &forms[1].kind else {
            panic!("expected list");
        };

        assert_symbol(&defn_list[0], "defn");

        // In the sample the function name is *annotated*:
        // `(defn #int sum ...)` is read as `Symbol(sum)` annotated with `Symbol(int)`.
        let Some(name_annotation) = &defn_list[1].annotation else {
            panic!("expected name annotation");
        };
        let Kind::Symbol(ann) = &name_annotation.kind else {
            panic!("expected symbol name annotation");
        };
        assert_eq!(ann.name, "int");

        let Kind::Symbol(name) = &defn_list[1].kind else {
            panic!("expected function name symbol");
        };
        assert_eq!(name.name, "sum");

        // Parameters are a *typed vector* in the sample: `[#int a #int b]`.
        let Kind::Vector(params) = &defn_list[2].kind else {
            panic!("expected params vector");
        };

        let Some(param0_annotation) = &params[0].annotation else {
            panic!("expected param annotation");
        };
        let Kind::Symbol(ann) = &param0_annotation.kind else {
            panic!("expected symbol param annotation");
        };
        assert_eq!(ann.name, "int");

        let Kind::Symbol(param0_name) = &params[0].kind else {
            panic!("expected param name symbol");
        };
        assert_eq!(param0_name.name, "a");
    }

    #[test]
    fn strict_invalid_keyword_rejected() {
        assert!(parse("::foo").is_err());
        assert!(parse(":/foo").is_err());
    }

    #[test]
    fn strict_invalid_symbol_rejected() {
        assert!(parse("1foo").is_err());
        assert!(parse("foo/bar/baz").is_err());
        assert!(parse("-1").is_ok()); // number token
        assert!(parse("+1").is_ok());
        assert!(parse(".1").is_err());
    }
}
