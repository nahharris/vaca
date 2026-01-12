# `stl.net`

`stl.net` defines low-level networking primitives.

This module is recommended for the standard library; if present, it MUST be specified precisely,
especially around blocking behavior and errors.

## Types

- `ip-addr`, `socket-addr`
- `tcp-listener`, `tcp-stream`
- `udp-socket`

## Core operations (recommended)

- `(tcp.listen addr)` → `(result tcp-listener net-error)`
- `(tcp.accept listener)` → `(result tcp-stream net-error)`
- `(tcp.connect addr)` → `(result tcp-stream net-error)`
- `(udp.bind addr)` → `(result udp-socket net-error)`

Streams/sockets SHOULD integrate with `stl.io` reader/writer abstractions.

## Timeouts and non-blocking I/O

If timeouts are supported, the API MUST specify:

- whether operations block by default
- how to configure timeouts
- how cancellation is expressed (if supported)
