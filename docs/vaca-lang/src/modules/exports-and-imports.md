# Exports and imports

This chapter specifies how modules expose bindings and how `use` imports them.

## The `use` form

The standard prelude defines `use` as a module directive.

### Syntax

```clojure
(use module)
(use module [import-spec ...])
```

- `module` is a module specifier symbol (see “Module identifiers and resolution”).
- The optional import vector selects and renames imported bindings.

### When `use` takes effect

`use` MUST take effect during module loading/compilation so that subsequent forms in the module can
refer to imported names.

Implementations MAY restrict `use` to module top-level; if so, it MUST be a compile-time error to
invoke `use` in an expression context that would require runtime-dependent imports.

## Export model

Vaca’s standard export model is:

- A module exports all top-level bindings introduced by the standard definition forms (`def`,
  `defn`, `defmacro`) **unless** they are marked private.

### Private definitions

The standard prelude MUST support a privacy marker on definitions. This specification defines the
canonical marker:

- A definition form MAY include a keyword option `:private true`.

Examples:

```clojure
(def secret 123 :private true)
(defn helper [x] (+ x 1) :private true)
```

Private definitions are not exported and cannot be imported from other modules.

## Import specifier syntax

An import vector contains a sequence of import specs.

### Import a name

```clojure
(use @stl.io [println])
```

This imports the exported binding named `println` and makes it available under the same name in
the importing module.

### Import with alias

```clojure
(use @stl.io [println :as p])
```

This imports `println` and binds it in the importing module under the local name `p`.

The alias syntax is:

```text
<name> :as <alias>
```

where `<name>` and `<alias>` are symbols.

## Name collisions

If an import would introduce a binding whose local name already exists in the importing module’s
scope, the import MUST fail with a name-collision error.

Implementations MAY offer an explicit “overwrite” mechanism, but it MUST NOT be the default.

## Cycles

If the module graph contains a cycle (directly or indirectly), module loading MUST fail with a
cycle error that reports the cycle path.

## Semantics of imported bindings

Imported bindings refer to the exporting module’s definitions.

Normative constraints:

- Importing a value MUST NOT copy or clone it in a way that changes identity-sensitive semantics.
- If the language exposes mutable references, importing MUST preserve reference identity.

For purely functional values, this typically means imported values are shared and immutable.
