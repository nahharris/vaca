# `stl.fs`

`stl.fs` defines filesystem operations.

The filesystem API MUST be explicit about errors; `result` is recommended.

## Types

- `file`: an opaque handle to an open file
- `file-mode`: an enum of modes such as `:read`, `:write`, `:append`, binary variants, etc.
- `fs-error`: an error type (enum or struct) describing filesystem failures

## Core operations

- `(file.open path mode)` → `(result file fs-error)`
- `(file.close f)` → `void`
- `(file.read-to-end f)` → `(result (vec uint) fs-error)` (recommended)
- `(file.read-to-string f)` → `(result string fs-error)` (recommended)
- `(file.write f bytes-or-string)` → `(result void fs-error)`

## Path operations

- `(path.join a b)` → `string`
- `(path.parent p)` → `(option string)`
- `(path.exists? p)` → `bool`
- `(path.is-dir? p)` → `bool`
- `(path.is-file? p)` → `bool`

## Directory operations

- `(dir.list path)` → `(result (vec string) fs-error)`
- `(dir.create path)` → `(result void fs-error)`
- `(dir.remove path)` → `(result void fs-error)` (MUST specify recursive vs non-recursive)

## Atomicity and portability

The module MUST specify:

- whether writes are atomic
- how path separators are handled
- behavior on permissions and non-UTF8 paths
