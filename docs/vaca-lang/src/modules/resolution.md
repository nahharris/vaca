# Module identifiers and resolution

This chapter specifies how Vaca resolves module identifiers to source files.

## Overview

Vaca modules are stored in source files. A module is identified by a **module specifier** that
appears in `use` forms.

The module system is designed to support:

- explicit library roots (toolchain configured)
- relative imports between project files
- predictable, portable resolution rules

## Library roots

Every conforming implementation MUST support a configuration containing one or more **library
roots** (directories).

Library roots are used to resolve imports whose specifier begins with `@`.

How library roots are configured (CLI flag, environment variable, project file) is
implementation-defined, but the *resolution semantics* are not.

## Module specifier syntax

A module specifier is a symbol whose textual form is one of:

- `@<path>` for library modules
- `<path>` for relative modules

where `<path>` is a dot-separated sequence of **segments**.

Examples:

- `@stl.io`
- `@stl.str`
- `my.app.main`
- `$.util.math`
- `$. $.foo` is invalid (whitespace splits forms; segments must be contiguous in one symbol)

### Segments

Segments are separated by `.`.

For **relative** modules only, the segment `$` denotes “parent directory”.

The segment `$` MUST NOT appear in library module specifiers (those beginning with `@`).

## Mapping specifiers to source paths

### File extension

The canonical source extension is `.vaca`.

### Relative modules

Let:

- `importer_dir` be the directory of the importing source file.
- `segments` be the dot-separated segments of the module specifier.

Resolution algorithm:

1. Start with `dir = importer_dir`.
2. For each segment except the last:
   - If the segment is `$`, set `dir = parent(dir)`.
     If `dir` has no parent, resolution fails.
   - Otherwise, set `dir = dir / segment`.
3. Let `file = last_segment + ".vaca"`.
4. The resolved module path is `dir / file`.

### Library modules

For a library module `@a.b.c`:

1. Remove the leading `@`.
2. For each configured library root `R`, construct the candidate path `R/a/b/c.vaca`.
3. The first existing candidate (in library-root order) is the resolved module.
4. If no candidate exists, resolution fails.

## Errors

Resolution MUST fail with a module-not-found error if:

- a relative import attempts to traverse beyond the filesystem root via `$`
- a path maps to no readable `.vaca` source file
- a library import cannot be found in any configured library root

The error MUST report:

- the importing module/file identity (if available)
- the attempted module specifier
- the candidate paths considered (at least in debug tooling; for user-facing errors, it SHOULD
  include the final attempted path(s))

## Caching

Implementations MAY cache resolved and loaded modules. Caching MUST NOT change semantics and MUST
NOT cause stale modules to be used when the implementation is configured for incremental or
interactive workflows.
