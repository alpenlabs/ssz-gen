# SSZ Codegen

A codegen tool that parses simplified Python SSZ (Simple Serialize) definitions using `sizzle-parser` and generates Rust code for it utilizing `ssz_derive`'s derive macros.

## Supported SSZ Syntax

### Constants
```python
val_x = 42
val_y = 64
```

### Aliases
```python
alias_uint_alias = uint16
alias_vec_a = Vector[uint8, 10]

alias_container = Container
alias_stable_container_10 = StableContainer[10]

class Foo(alias_container):
    a: byte

alias_foo = Foo
```

Built-in type aliases:
- `byte` → `uint8`
- `bit` → `boolean`
- `null` → `boolean`
- `BytesX` → `Vector[uint8, X]` (1..=64)

### Container Definitions for Container, StableContainer[N] and Profile[B]
```python
class Alpha(Container):
    a: uint8
    b: uint16
    c: Vector[uint8, 10]
```

### Comments

The parser supports four types of comments:

- **Docstrings** (`"""..."""`): Triple-quoted documentation strings that are preserved and converted to Rust doc comments. Docstrings support multi-line text and are cleaned up to remove common indentation.
  ```python
  class Point(Container):
      """
      This is a doc comment for the class
      It can span multiple lines
      """
      x: uint32
      y: uint32
  ```

- **Doc comments** (`###`): Documentation comments preserved in the generated code
  ```python
  ### This is a doc comment for the class
  ### It can span multiple lines
  class Point(Container):
      ### X coordinate of the point
      x: uint32
      ### Y coordinate of the point
      y: uint32
  ```
  
  **Merging docstrings and doc comments**: When both docstrings (`"""..."""`) and doc comments (`###`) are present on a class, they are merged in the generated Rust code with the docstring appearing first, followed by a blank line, then the doc comments:
  ```python
  ### This doc comment comes after the docstring
  class PointWithBoth(Container):
      """
      This docstring comes first.
      """
      x: uint32
  ```

- **Pragma comments** (`#~#`): Special directive comments that control code generation behavior. Pragmas can specify additional derive macros or custom attributes.
  
  **Class-level pragmas** are placed before class definitions:
  ```python
  #~# derive: Serialize, Deserialize
  #~# attr: #[repr(C)]
  class Point(Container):
      x: uint32
      y: uint32
  ```
  
  **Field-level pragmas** are placed before field definitions:
  ```python
  class Point(Container):
      #~# field_attr: #[serde(rename = "x_coord")]
      x: uint32
      y: uint32
  ```
  
  **Supported pragma formats:**
  - `derive: Trait1, Trait2, ...` - Adds additional derive macros to the generated type. These are merged with configured derives and required SSZ derives (Encode, Decode, TreeHash).
  - `attr: #[attribute]` - Adds struct-level attributes (e.g., `#[repr(C)]`, `#[cfg(test)]`).
  - `field_attr: #[attribute]` - Adds field-level attributes (e.g., `#[serde(rename = "field_name")]`).
  
  Multiple pragmas can be specified on separate lines:
  ```python
  #~# derive: Serialize, Deserialize
  #~# attr: #[repr(C)]
  #~# attr: #[derive(Default)]
  class Point(Container):
      x: uint32
  ```
  
  Pragmas are preserved through inheritance - child classes inherit parent pragmas and can add their own.

- **Regular comments** (`#`): Standard comments that are discarded during parsing
  ```python
  # This comment is ignored
  class Point(Container):
      x: uint32
  ```

Docstrings appear at the beginning of a class body and are attached to the class. Doc comments appearing before class definitions or field definitions are attached to those elements and preserved through the parsing pipeline. Multiple consecutive doc comment lines are merged with newlines preserved. Both docstrings and doc comments are emitted in the generated Rust code as `///` comments with 80-character line wrapping.

### Inheritance
```python
class Foo(StableContainer[5]):
    a: Optional[uint8]
    b: Optional[uint8]
    c: Optional[uint8]

class Bar(Foo):
    b: Optional[uint16]
    d: Optional[uint8]
```

this is equivalent to
```python
class Bar(StableContainer[5]):
    a: Optional[uint8] # Inherited
    b: Optional[uint16] # Replaced
    c: Optional[uint8] # Inherited
    d: Optional[uint8] # New
```

Keep in mind in inheritance the relative order of all fields must be preserved. In the example above
```python
class Bar(Foo):
    b: Optional[uint16]
    d: Optional[uint8]
    a: Optional[uint16]
```

Would not be allowed because you're overwriting the parent's `a` field but `a` is supposed to be the first field.
```python
class Bar(Foo):
    a: Optional[uint16]
    c: Optional[uint16]
    d: Optional[uint8]
```

Would be allowed since the relative order between all the defined fields is preserved. The above would be the same as
```python
class Bar(StableContainer[5]):
    a: Optional[uint16] # Replaced
    b: Optional[uint8] # Inherited
    c: Optional[uint16] # Replaced
    d: Optional[uint8] # New
```

### Union Types
```python
union_a = Union[uint8, uint16, uint32]
```

In Rust unions are implemented as enums. Because of this we need to be able to assign unique identifiers to the same unions. Because of this and in order to remove any confusion we disallow "anonymous" unions except for `Union[None, T]` which we treat as `Option<T>` in Rust.

Example:
```python
# Union has no "name" and is referenced by a field -> not allowed
class Foo(Container):
    a: Union[uint8, uint16]

# The inner union has no "name" -> not allowed
alias = Union[uint8, Union[uint8, uint16]]

# Union is of type Union[None, T] -> allowed, will use Option<u8> in rust
class Bar(Container):
    a: Union[None, uint8]

# The inner union is of type Union[None, T] -> allowed, will use Option<u16> in rust for the inner union
alias = Union[uint8, Union[None, uint16]]
```

The correct way to do the wrong examples from above would be:
```python
alias_union = Union[uint8, uint16]

class Foo(Container):
    a: alias_union

alias = Union[uint8, alias_union]
```

The variants within the enum are named `Selector{index}`.

### Imports
You can import definitions from other files using `import FILE` or `IMPORT FILE as IDENT` or from other crates using `IMPORT CRATE.MODULE`. To import from other crates the crate must be provided to `build_ssz_files` in the `crates` argument.
- In case of importing from local files:
    - The path to the file must be relative to the current module.
    - `..` for parent folder, `.` as the separator for folders
    - Example: 2 folders back, inside utils folder, in maths.ssz -> `import ....utils.maths as maths`
- In case of importing `ssz_types::containers::ContainerA` from external crate `ssz_types`:
    - Make sure `ssz_types` is included in the `crates` argument of `build_ssz_files`.
    - On top of your SSZ schema file add `import ssz_types.containers as external_containers`
    - Now you can use `ContainerA` the following way: `ImportedContainer = external_containers.ContainerA`

`common.ssz`:
```python
alias_uint8 = uint8
alias_union = Union[uint8, uint16]
class Foo(Container):
    a: alias_uint8
    b: alias_union
```

`file.ssz`:
```python
import common

class Bar(common.Foo):
    c: common.union
```

this is equivalent to
```python
class Bar(Container):
    a: uint8
    b: Union[uint8, uint16]
    c: Union[uint8, uint16]
```

# Example Input / Output
Input: [`tests/input/test_1.ssz`](/crates/ssz_codegen/tests/input/test_1.ssz)

Output: [`tests/expected_output/test_1.rs`](/crates/ssz_codegen/tests/expected_output/test_1.rs)
