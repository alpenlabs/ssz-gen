# sizzle-parser

This is a parser for a the very narrow subset of Python used to define SSZ
schemas.

It supports:

* `class` defs
  * with only typed fields, but without default values
  * with doc comments and pragmas
  * without methods
  * without decorators
* type alias assignments
* integer constant assignments

## Comments

The parser supports three types of comments:

- **Doc comments** (`###`): Documentation comments that are preserved and attached to classes or fields
  ```python
  ### This is a doc comment for the class
  ### It can span multiple lines
  class Point(Container):
      ### X coordinate
      x: uint32
  ```

- **Pragma comments** (`#~#`): Special directive comments
  ```python
  #~# some-directive value
  class Point(Container):
      #~# field-pragma
      x: uint32
  ```

- **Regular comments** (`#`): Standard comments that are discarded during parsing
  ```python
  # This comment is ignored
  class Point(Container):
      x: uint32
  ```

Doc comments and pragmas appearing before a class definition are attached to the class. Similarly, comments appearing before field definitions within a class body are attached to those fields.

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
