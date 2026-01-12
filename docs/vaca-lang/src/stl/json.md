# `stl.json`

`stl.json` defines JSON parsing and generation.

## Data mapping

The module MUST specify how JSON maps onto Vaca values, for example:

- JSON object → map with string keys (or keywords; MUST choose)
- JSON array → vector
- JSON string → string
- JSON number → `int` or `float` (MUST specify exact rules)
- JSON boolean → bool
- JSON null → nil

## API

- `(json.parse s)` → `(result any json-error)`
- `(json.stringify x)` → `(result string json-error)`

If pretty-printing is supported, it MUST be specified.
