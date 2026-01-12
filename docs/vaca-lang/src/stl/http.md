# `stl.http`

`stl.http` defines HTTP client and server APIs.

This module is recommended for the standard library. If included in the normative distribution, it
MUST specify:

- HTTP versions supported
- request/response types
- streaming bodies vs buffered bodies
- TLS integration (if present)
- error model

## Client (recommended)

- `(http.get url)` → `(result response http-error)`
- `(http.post url body)` → `(result response http-error)`

## Server (recommended)

- `(http.serve addr handler)` where `handler` is a function mapping request → response.
