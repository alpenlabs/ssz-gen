# sizzle-parser

This is a parser for a the very narrow subset of Python used to define SSZ
schemas.

It supports:

* `class` defs
  * with only typed fields, but without default values
  * with(out) doc comments **(planned)**
  * without methods
  * with decorators for trait derivation (`@derive(...)`, `@module_derive(...)`)
* type alias assignments
* integer constant assignments
* comments (lines starting with `#`)

## Decorator Syntax

The parser supports Python-style decorators for configuring trait derivation:

```python
# Module-level derives (applies to all types in the file)
@module_derive(Clone, Debug)

# Per-type derives (overrides module-level for this specific type)
@derive(Clone, Debug, PartialEq, Eq)
class MyContainer(Container):
    x: uint8
    y: uint16
```

**Supported decorators:**
- `@module_derive(...)` - Module-level derives for all types
- `@derive(...)` - Per-type derives (overrides module-level)
- `#` - Comments (entire line is ignored)

## Design

The parsing uses a few non-textbook techniques in order to make it easier to
manage and reason about.

The first unusual feature is similar to what Python does for managing whitespace
significance.  The initial tokenizer pass tracks indentation and inserts special
`Indent` and `Deindent` tokens along the way.  It infers which kind of
indentation is used on the fly (ie. tabs or spaces (and how many)).  There is
also a `Newline` token, which might or might not be useful..

The second unusual feature is that we have a stage between the initial
tokenizing pass and the parser that generates the AST.  We have this second
lexer pass that matches paired tokens that signify the starts and ends of blocks
like `[`+`]`, `(`+`)`, indent+deindent.  We extract these to track blocks of
tokens and construct a "token tree".  This makes the parser a bit more
convenient to write, since it means that we don't have to do any weird tokentail
things in order to parse variable size structures.  Parsing a class def has only
a couple of constant-size forms.  This concept is borrowed from Rust, which uses
it in order to make writing macros easier.
