# Complete grammar (EBNF)

This appendix specifies the reader-level grammar for VEDN. It describes how a source file is read
as a sequence of forms, including separators, collections, annotations, and discard.

Lexical details for atoms (numbers, symbols, keywords, strings, characters) are EDN-like unless
explicitly extended elsewhere in this specification.

```text
Document        ::= Sep* (Form Sep*)*

Sep             ::= Whitespace | Comma | Comment
Whitespace      ::= " " | "\t" | "\n" | "\r"
Comma           ::= ","
Comment         ::= ";" (not_newline)* ("\n" | EOF)

Form            ::= 
                  | List
                  | Vector
                  | Map
                  | Set
                  | Annotation
                  | Discard
                  | Nil
                  | Boolean
                  | Number
                  | String
                  | Character
                  | Keyword
                  | Symbol

List            ::= "(" Sep* (Form Sep*)* ")"
Vector          ::= "[" Sep* (Form Sep*)* "]"
Map             ::= "{" Sep* (Form Sep* Form Sep*)* "}"
Set             ::= "%{" Sep* (Form Sep*)* "}"

Annotation      ::= "#" Form Sep* Form
Discard         ::= "##" Sep* Form

Nil             ::= "nil"
Boolean         ::= "true" | "false"

Number          ::= Integer | FloatingPoint

Integer         ::= Int | Int "N"
Int             ::= Digit
                  | NonZeroDigit Digits
                  | "+" Digit
                  | "+" NonZeroDigit Digits
                  | "-" Digit
                  | "-" NonZeroDigit Digits
Digit           ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
NonZeroDigit    ::= "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
Digits          ::= Digit | Digit Digits

FloatingPoint   ::= Int "M"
                  | Int Frac
                  | Int Exp
                  | Int Frac Exp
Frac            ::= "." Digits
Exp             ::= Ex Digits
Ex              ::= "e" | "e+" | "e-" | "E" | "E+" | "E-"

String          ::= "\"" { StringChar } "\""
StringChar      ::= StringEscaped | StringUnescaped
StringEscaped   ::= "\\" ( "t" | "r" | "n" | "\\" | "\"" | "u" Hex Hex Hex Hex )
StringUnescaped ::= any_char_except_backslash_or_doublequote

Character       ::= "\\" ( NamedChar | AnyNonWhitespaceChar | "u" Hex Hex Hex Hex )
NamedChar       ::= "newline" | "return" | "space" | "tab"
AnyNonWhitespaceChar ::= any_char_except_whitespace
Hex             ::= Digit | "a" | "b" | "c" | "d" | "e" | "f" | "A" | "B" | "C" | "D" | "E" | "F"
Keyword         ::= ":" Symbol | Symbol ":"

Symbol          ::= SymbolChar { SymbolChar }
SymbolChar      ::= Alpha
                  | Digit
                  | "." | "*" | "+" | "!" | "-" | "_" | "?" | "$" | "%" | "&" | "=" | "<" | ">"
                  | ":" | "#"
                  | "/"
Alpha           ::= "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K" | "L" | "M"
                  | "N" | "O" | "P" | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X" | "Y" | "Z"
                  | "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m"
                  | "n" | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z"

```

