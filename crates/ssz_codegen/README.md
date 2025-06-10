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

class Bar(Foo):
    b: Optional[uint8]
```
this is equivalent to
```python
class Bar(StableContainer[5]):
    a: Optional[uint8]
    b: Optional[uint8]
```

### Union Types
```python
union_a = Union[uint8, uint16, uint32]
```

In Rust unions are implemented as enums. Because of this we need to be able to assign unique identifiers to the same unions. We do this by hashing all the types within a union and naming the union `Union_{HASH}`.
The variants within the enum are named `Selector{index}`.

Equivalent aliases being used in a union will not result in a new enum creation. `Union[byte]` and `Union[uint8]` will both be treated as the exact same union.

# Example Input / Output
Input: [tests/input/test_1.ssz](/crates/ssz_codegen/tests/input/test_1.ssz)

Output: [tests/expected_output/test_1.rs](/crates/ssz_codegen/tests/expected_output/test_1.rs)
