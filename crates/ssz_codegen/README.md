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
You can import definitions from other files using `import FILE` or `IMPORT FILE as IDENT`.
- The path to the file must be relative to the current module.
- `..` for parent folder, `.` as the separator for folders
- Example: 2 folders back, inside utils folder, in maths.ssz -> `import ....utils.maths as maths`

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
