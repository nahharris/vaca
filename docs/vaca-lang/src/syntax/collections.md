# Collections

Collections are delimited forms that contain zero or more nested forms.

## Delimiters

The following delimiters form collections:

- Lists: `(` … `)`
- Vectors: `[` … `]`
- Maps: `{` … `}`
- Sets: `%{` … `}`

An unterminated collection is a read-time error.

## List

Syntax: `(<form>*)`

Lists are used both as code and as data.

## Vector

Syntax: `[<form>*]`

Vectors are the primary ordered collection literal.

## Map

Syntax: `{(<form> <form>)*}`

Maps MUST contain an even number of forms. If a map contains an odd number of forms, it is a
read-time error.

## Set

Syntax: `%{<form>*}`

