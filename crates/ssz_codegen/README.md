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
  - `external_kind: <kind>` - Controls `Ref` type generation for external types. Can be used on fields, union variants, or type alias assignments.
    - `container`: Generates a Ref variant (e.g. `MyTypeRef`). Use this for external container types that need zero-copy views.
    - `primitive`: Uses the type directly without a Ref wrapper. Use this for external primitive types.
  
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

There are two ways to define union types:

#### Type Alias Syntax
```python
union_a = Union[uint8, uint16, uint32]
```

The variants within the enum are named using the inner type name (e.g., `MyType(MyType)`). If the type is a primitive or the name cannot be determined, it falls back to `Selector{index}` (e.g., `Selector0(u8)`).

Pragmas can be applied to type alias unions to control behavior for all variants:
```python
#~# external_kind: container
ExternalUnion = Union[external_ssz.Type1, external_ssz.Type2]
```

This applies the `external_kind: container` pragma to all external types in the union, generating appropriate `Ref` view types.

#### Class-Based Syntax (Named Variants)

For more control over variant names and to add per-variant pragmas or documentation, use the class-based syntax:

```python
### Represents different pending input types
class PendingInputEntry(Union):
    ### A deposit from the execution layer
    #~# external_kind: container
    Deposit: external_ssz.SubjectDepositData

    ### A withdrawal request
    Withdrawal: WithdrawalData
```

This generates a Rust enum where the field names become variant names:
```rust
/// Represents different pending input types
pub enum PendingInputEntry {
    /// A deposit from the execution layer
    Deposit(external_ssz::SubjectDepositData),
    /// A withdrawal request
    Withdrawal(WithdrawalData),
}
```

**When to use each syntax:**
- Use **type alias syntax** (`Union[T1, T2]`) for simple unions with primitive types, or unions with external types where all variants share the same pragma (e.g., all are containers)
- Use **class-based syntax** (`class Name(Union)`) when you need:
  - Explicit variant names (e.g., `Deposit` instead of `Selector0`)
  - Doc comments on individual variants
  - Different pragmas per variant (e.g., mixed container and primitive external types)
  - Better readability for complex union definitions

#### Anonymous Union Restrictions

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

- In case of importing from existing Rust modules (without `.ssz` files):
    - Works similarly to external crate imports but for modules within your workspace or dependencies that don't use `ssz-gen`.
    - The import path is treated as a Rust module path.
    - Example: `import my_crate.existing_module as existing` allows using `existing.MyType` which resolves to `my_crate::existing_module::MyType`.

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
