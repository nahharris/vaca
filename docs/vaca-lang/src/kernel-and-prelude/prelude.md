# The Standard Prelude

The **Standard Prelude** is a set of canonical definitions (functions, macros, and types) written
in Vaca that establish the language’s everyday surface.

This specification treats these definitions as normative: a conforming toolchain MUST provide a
prelude with the specified behavior, either by:

- implicitly loading it for every module, or
- making it available as a well-known library module (e.g. `@prelude`) and requiring modules to
  import it by convention.

## Prelude surface: required bindings

At minimum, the prelude MUST provide the following constructs with the semantics described in this
book:

- **Definitions**: `def`, `defn`
- **Functions**: `fn`
- **Binding and control**: `let`, `if`, `do`
- **Quotation**: `quote` (and optionally quasiquote facilities)
- **Pipelines**: `|>`
- **Iteration**: `loop`, `recur`
- **Macros**: `defmacro` (macro definition facility)

These names denote *library-level constructs* (typically macros) even if a particular toolchain
chooses to implement them with VM opcodes internally.

## Definitions: `def` and `defn`

### `(def name value ...)`

`def` introduces a top-level binding in the current module.

Normative behavior:

- `name` MUST be a symbol (possibly typed via `#T name`).
- The value expression MUST be evaluated exactly once, and the resulting value bound to `name`.
- The resulting value of the `def` form is the value bound.
- `def` MUST support an optional docstring field.

### `(defn name params body...)`

`defn` defines a named function and binds it at top level.

Normative behavior:

- `name` MUST be a symbol (possibly typed via `#T name`).
- `params` MUST be a vector of parameter bindings (possibly typed).
- The function body is a sequence of forms evaluated in order; the function result is the value of
  the last form.
- A `defn` MUST support an optional `:doc` string (syntax is specified by the prelude; this book
  requires that the docstring be attachable and retrievable via introspection APIs).

## Anonymous functions: `fn`

`fn` constructs a closure that captures the lexical environment.

- Parameter binding and arity rules are specified in the evaluation and stdlib chapters.

## Binding and control: `let`, `if`, `do`

### `(let [name1 val1 name2 val2 ...] body...)`

- Bindings are introduced left-to-right.
- Each `vali` is evaluated in the environment extended by previous bindings.
- The body is evaluated in the extended environment.

### `(if cond then else)`

- `cond` is evaluated.
- If `cond` is truthy (see “Truthiness”), `then` is evaluated and returned.
- Otherwise, `else` is evaluated and returned.

### `(do form1 form2 ... formN)`

- Each form is evaluated left-to-right.
- The value of the `do` expression is the value of the last form (or `nil` if there are none).

## Quotation: `quote`

### `(quote form)`

- `quote` returns `form` as data without evaluating it.

## Pipeline: `|>`

`|>` is a convenience macro for left-to-right function application.

Normative behavior:

```clojure
(|> x
    (f a)
    g
    (h b c))
```

expands to a form equivalent to:

```clojure
(h (g (f x a)) b c)
```

More precisely:

- Start with the value expression `x`.
- For each subsequent step:
  - If the step is a list form `(op arg...)`, it becomes `(op <acc> arg...)`.
  - Otherwise it becomes `(<step> <acc>)`.
- The final accumulated form is the expansion.

## Iteration: `loop` and `recur`

`loop` and `recur` define a portable, constant-space iteration model.

### Tail position

A form is in **tail position** if, after it is evaluated, the surrounding construct performs no
additional computation before returning that value.

The prelude MUST define tail position rules at least for:

- function bodies (`fn` / `defn`)
- `do`
- `if` (both branches are tail positions if the `if` itself is in tail position)
- `let` (the body is tail position if the `let` is)
- `loop` bodies

### `(loop [name1 init1 name2 init2 ...] body...)`

- Establishes local bindings, similar to `let`.
- Additionally establishes a **re-entry point** for `recur`.

### `(recur expr1 expr2 ...)`

- `recur` MUST be used only in tail position within a `loop` body or function body that defines a
  re-entry point.
- It evaluates each `expri` and then requests iteration by rebinding loop/function parameters to
  the new values, transferring control to the re-entry point **without growing the stack**.

If `recur` appears outside a valid tail position or without an enclosing re-entry point, it is a
program error.

The kernel provides the underlying tail-call facility (see “Kernel requirements”).
