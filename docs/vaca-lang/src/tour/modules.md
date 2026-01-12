# Modules and libraries

Real programs are split into multiple files and share code via the module system.

## Importing names

Vaca uses `use` to import names from another module:

```clojure
(use @stl.io [println])
```

In this example, the module identifier begins with `@`, meaning it is resolved from configured
library search roots.

## Relative modules

If the module identifier does not begin with `@`, it is resolved relative to the importing file.
The marker `$` denotes the parent directory segment.

Conceptually:

- `foo.bar` refers to `./foo/bar.vaca` (relative)
- `$.foo.bar` refers to `../foo/bar.vaca` (one parent)
- `@stl.io` refers to a library module found under a lib root

The module system is specified precisely in the “Module system” part of this book.
