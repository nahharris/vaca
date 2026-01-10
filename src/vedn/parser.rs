use super::{
    cursor::Cursor,
    error::{Error, ErrorKind, Span},
    value::{Keyword, Kind, Node, Number, NumberSuffix, Str, Symbol, Typed},
};

/// Parses all top-level EDN elements from `input`.
///
/// EDN has no mandatory top-level delimiter. This function therefore returns a
/// sequence of nodes.
///
/// # Tags
/// Typed elements (`#tag <value>`) are preserved as [`Kind::Typed`]. The
/// parser never invokes tag handlers.
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
            nodes.push(self.parse_node()?);
        }
        Ok(nodes)
    }

    fn parse_node(&mut self) -> Result<Node<'a>, Error> {
        self.cursor.skip_ws_and_comments();
        let _start = self.cursor.index;
        let Some(b) = self.cursor.peek() else {
            return Err(self.cursor.error_here(ErrorKind::UnexpectedEof));
        };

        let node = match b {
            b'(' => self.parse_list()?,
            b'[' => self.parse_vector()?,
            b'{' => self.parse_map()?,
            b'"' => self.parse_string()?,
            b':' => self.parse_keyword()?,
            b'\\' => self.parse_char()?,
            b'#' => self.parse_dispatch()?,
            _ => self.parse_token()?,
        };

        // In the presence of discard (`#_`), parsing may advance to the next
        // element and return it; the returned node then starts after `start`.
        Ok(node)
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
                _ => values.push(self.parse_node()?),
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
                _ => values.push(self.parse_node()?),
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

        let mut entries = Vec::new();
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
                    let key = self.parse_node()?;
                    self.cursor.skip_ws_and_comments();
                    if matches!(self.cursor.peek(), Some(b'}') | None) {
                        return Err(self.cursor.error_span(
                            ErrorKind::MapOddNumberOfForms,
                            Span::new(key.span.start, self.cursor.index),
                        ));
                    }
                    let value = self.parse_node()?;
                    entries.push((key, value));
                }
            }
        }

        Ok(Node::new(self.cursor.span_from(start), Kind::Map(entries)))
    }

    /// Parses a set: `#{<value>...}`.
    fn parse_set(&mut self, start: usize) -> Result<Node<'a>, Error> {
        // we already consumed '#', the next byte must be '{'
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
                _ => values.push(self.parse_node()?),
            }
        }

        Ok(Node::new(self.cursor.span_from(start), Kind::Set(values)))
    }

    /// Parses a `#` dispatch form.
    ///
    /// Supported dispatches:
    ///
    /// - `#{ ... }`: sets
    /// - `#_ <value>`: discard
    /// - `#tag <value>`: typed elements (EDN-strict tags start with an alphabetic character)
    fn parse_dispatch(&mut self) -> Result<Node<'a>, Error> {
        let start = self.cursor.index;
        self.cursor.bump(); // '#'

        match self.cursor.peek() {
            Some(b'{') => self.parse_set(start),
            Some(b'_') => {
                self.cursor.bump();
                self.cursor.skip_ws_and_comments();
                // Discard the next readable element.
                let _discarded = self.parse_node()?;
                // After discarding, parse the next element in the caller.
                // Here we return a synthetic nil node so callers can keep a single-value API.
                // The parent parser handles discard by not having this return path.
                // So: this function is only called from parse_node, therefore we need to
                // represent discard as "no node". We implement it by re-parsing and returning
                // the next node.
                self.cursor.skip_ws_and_comments();
                self.parse_node()
            }
            Some(b) if is_ascii_alpha(b) => {
                let ty = self.parse_tag_symbol()?;
                self.cursor.skip_ws_and_comments();
                if self.cursor.is_eof() {
                    return Err(self.cursor.error_here(ErrorKind::UnexpectedEof));
                }
                let value = self.parse_node()?;
                Ok(Node::new(
                    self.cursor.span_from(start),
                    Kind::Typed(Typed {
                        ty,
                        value: Box::new(value),
                    }),
                ))
            }
            _ => Err(self.cursor.error_here(ErrorKind::InvalidDispatch)),
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

    /// Parses a keyword token.
    fn parse_keyword(&mut self) -> Result<Node<'a>, Error> {
        let start = self.cursor.index;
        let token_start = self.cursor.index;
        let token = self.cursor.take_while(token_start, |b| !is_delim_or_ws(b));

        let keyword = parse_keyword(token).map_err(|kind| {
            self.cursor
                .error_span(kind, Span::new(token_start, self.cursor.index))
        })?;

        Ok(Node::new(
            self.cursor.span_from(start),
            Kind::Keyword(keyword),
        ))
    }

    /// Parses the symbol following `#` in a typed element.
    fn parse_tag_symbol(&mut self) -> Result<Symbol<'a>, Error> {
        let token_start = self.cursor.index;
        let token = self.cursor.take_while(token_start, |b| !is_delim_or_ws(b));
        let symbol = parse_symbol(token).map_err(|kind| {
            self.cursor
                .error_span(kind, Span::new(token_start, self.cursor.index))
        })?;
        Ok(symbol)
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

fn is_ascii_alpha(b: u8) -> bool {
    b.is_ascii_alphabetic()
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
/// `token` must include the leading `:`.
fn parse_keyword(token: &str) -> Result<Keyword<'_>, ErrorKind> {
    if !token.starts_with(':') {
        return Err(ErrorKind::InvalidKeyword);
    }
    if token.starts_with("::") {
        return Err(ErrorKind::InvalidKeyword);
    }
    if token.starts_with(":/") {
        return Err(ErrorKind::InvalidKeyword);
    }

    let raw = token;
    let without_colon = &token[1..];
    let symbol = parse_symbol(without_colon).map_err(|_| ErrorKind::InvalidKeyword)?;
    Ok(Keyword {
        raw,
        namespace: symbol.namespace,
        name: symbol.name,
    })
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
        let values = parse("(a 1) [a 1] {:a 1, :b 2} #{a b}").unwrap();
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
    fn parse_tags_as_typing_syntax() {
        let values = parse("#inst \"2020-01-01\"").unwrap();
        let Kind::Typed(typed) = &values[0].kind else {
            panic!("expected typed");
        };
        assert_eq!(typed.ty.name, "inst");
        let Kind::String(s) = &typed.value.kind else {
            panic!("expected string");
        };
        assert_eq!(s.as_str(), "2020-01-01");
    }

    #[test]
    fn parse_discard() {
        let values = parse("[a #_foo 42]").unwrap();
        let Kind::Vector(v) = &values[0].kind else {
            panic!("expected vector");
        };
        assert_eq!(v.len(), 2);
        assert_symbol(&v[0], "a");
        assert!(matches!(v[1].kind, Kind::Number(Number::Int { .. })));
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

        // In the sample the function name is *typed*:
        // `(defn #int sum ...)` is read as `Tagged(tag=int, value=Symbol(sum))`.
        let Kind::Typed(typed_name) = &defn_list[1].kind else {
            panic!("expected typed function name");
        };
        assert_eq!(typed_name.ty.name, "int");

        let Kind::Symbol(name) = &typed_name.value.kind else {
            panic!("expected function name symbol");
        };
        assert_eq!(name.name, "sum");

        // Parameters are a *typed vector* in the sample: `[#int a #int b]`.
        let Kind::Vector(params) = &defn_list[2].kind else {
            panic!("expected params vector");
        };

        let Kind::Typed(param0_type) = &params[0].kind else {
            panic!("expected typed param");
        };
        assert_eq!(param0_type.ty.name, "int");

        let Kind::Symbol(param0_name) = &param0_type.value.kind else {
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
