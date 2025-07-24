# Crate Documentation

**Version:** 0.235.0

**Format Version:** 54

# Module `wit_parser`

## Modules

## Module `decoding`

**Attributes:**

- `Other("#[<cfg>(feature = \"decoding\")]")`

```rust
pub mod decoding { /* ... */ }
```

### Types

#### Enum `DecodedWasm`

Result of the [`decode`] function.

```rust
pub enum DecodedWasm {
    WitPackage(Resolve, PackageId),
    Component(Resolve, WorldId),
}
```

##### Variants

###### `WitPackage`

The input to [`decode`] was one or more binary-encoded WIT package(s).

The full resolve graph is here plus the identifier of the packages that
were encoded. Note that other packages may be within the resolve if any
of the main packages refer to other, foreign packages.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Resolve` |  |
| 1 | `PackageId` |  |

###### `Component`

The input to [`decode`] was a component and its interface is specified
by the world here.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Resolve` |  |
| 1 | `WorldId` |  |

##### Implementations

###### Methods

- ```rust
  pub fn resolve(self: &Self) -> &Resolve { /* ... */ }
  ```
  Returns the [`Resolve`] for WIT types contained.

- ```rust
  pub fn package(self: &Self) -> PackageId { /* ... */ }
  ```
  Returns the main packages of what was decoded.

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
### Functions

#### Function `decode_reader`

Decode for incremental reading

```rust
pub fn decode_reader</* synthetic */ impl Read: Read>(reader: impl Read) -> anyhow::Result<DecodedWasm> { /* ... */ }
```

#### Function `decode`

Decodes an in-memory WebAssembly binary into a WIT [`Resolve`] and
associated metadata.

The WebAssembly binary provided here can either be a
WIT-package-encoded-as-binary or an actual component itself. A [`Resolve`]
is always created and the return value indicates which was detected.

```rust
pub fn decode(bytes: &[u8]) -> anyhow::Result<DecodedWasm> { /* ... */ }
```

#### Function `decode_world`

Decodes the single component type `world` specified as a WIT world.

The `world` should be an exported component type. The `world` must have been
previously created via `encode_world` meaning that it is a component that
itself imports nothing and exports a single component, and the single
component export represents the world. The name of the export is also the
name of the package/world/etc.

```rust
pub fn decode_world(wasm: &[u8]) -> anyhow::Result<(Resolve, WorldId)> { /* ... */ }
```

## Module `abi`

```rust
pub mod abi { /* ... */ }
```

### Types

#### Struct `WasmSignature`

A core WebAssembly signature with params and results.

```rust
pub struct WasmSignature {
    pub params: Vec<WasmType>,
    pub results: Vec<WasmType>,
    pub indirect_params: bool,
    pub retptr: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `params` | `Vec<WasmType>` | The WebAssembly parameters of this function. |
| `results` | `Vec<WasmType>` | The WebAssembly results of this function. |
| `indirect_params` | `bool` | Whether or not this signature is passing all of its parameters<br>indirectly through a pointer within `params`.<br><br>Note that `params` still reflects the true wasm parameters of this<br>function, this is auxiliary information for code generators if<br>necessary. |
| `retptr` | `bool` | Whether or not this signature is using a return pointer to store the<br>result of the function, which is reflected either in `params` or<br>`results` depending on the context this function is used (e.g. an import<br>or an export). |

##### Implementations

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> WasmSignature { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &WasmSignature) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Comparable**
  - ```rust
    fn compare(self: &Self, key: &K) -> Ordering { /* ... */ }
    ```

- **RefUnwindSafe**
- **Eq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &WasmSignature) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Freeze**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &WasmSignature) -> bool { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Enum `WasmType`

Enumerates wasm types used by interface types when lowering/lifting.

```rust
pub enum WasmType {
    I32,
    I64,
    F32,
    F64,
    Pointer,
    PointerOrI64,
    Length,
}
```

##### Variants

###### `I32`

###### `I64`

###### `F32`

###### `F64`

###### `Pointer`

A pointer type. In core Wasm this typically lowers to either `i32` or
`i64` depending on the index type of the exported linear memory,
however bindings can use different source-level types to preserve
provenance.

Users that don't do anything special for pointers can treat this as
`i32`.

###### `PointerOrI64`

A type for values which can be either pointers or 64-bit integers.
This occurs in variants, when pointers and non-pointers are unified.

Users that don't do anything special for pointers can treat this as
`i64`.

###### `Length`

An array length type. In core Wasm this lowers to either `i32` or `i64`
depending on the index type of the exported linear memory.

Users that don't do anything special for pointers can treat this as
`i32`.

##### Implementations

###### Trait Implementations

- **Freeze**
- **RefUnwindSafe**
- **Eq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> WasmType { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &WasmType) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Unpin**
- **Comparable**
  - ```rust
    fn compare(self: &Self, key: &K) -> Ordering { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &WasmType) -> bool { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Copy**
- **StructuralPartialEq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(i: Int) -> WasmType { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &WasmType) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Enum `AbiVariant`

We use a different ABI for wasm importing functions exported by the host
than for wasm exporting functions imported by the host.

Note that this reflects the flavor of ABI we generate, and not necessarily
the way the resulting bindings will be used by end users. See the comments
on the `Direction` enum in gen-core for details.

The bindings ABI has a concept of a "guest" and a "host". There are two
variants of the ABI, one specialized for the "guest" importing and calling
a function defined and exported in the "host", and the other specialized for
the "host" importing and calling a function defined and exported in the "guest".

```rust
pub enum AbiVariant {
    GuestImport,
    GuestExport,
    GuestImportAsync,
    GuestExportAsync,
    GuestExportAsyncStackful,
}
```

##### Variants

###### `GuestImport`

The guest is importing and calling the function.

###### `GuestExport`

The guest is defining and exporting the function.

###### `GuestImportAsync`

###### `GuestExportAsync`

###### `GuestExportAsyncStackful`

##### Implementations

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **StructuralPartialEq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Freeze**
- **Unpin**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Copy**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &AbiVariant) -> bool { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> AbiVariant { /* ... */ }
    ```

- **Eq**
- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `FlatTypes`

```rust
pub struct FlatTypes<''a> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn new(types: &''a mut [WasmType]) -> FlatTypes<''a> { /* ... */ }
  ```

- ```rust
  pub fn push(self: &mut Self, ty: WasmType) -> bool { /* ... */ }
  ```

- ```rust
  pub fn to_vec(self: &Self) -> Vec<WasmType> { /* ... */ }
  ```

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Send**
- **RefUnwindSafe**
- **Freeze**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

## Types

### Type Alias `WorldId`

```rust
pub type WorldId = id_arena::Id<World>;
```

### Type Alias `InterfaceId`

```rust
pub type InterfaceId = id_arena::Id<Interface>;
```

### Type Alias `TypeId`

```rust
pub type TypeId = id_arena::Id<TypeDef>;
```

### Struct `UnresolvedPackage`

Representation of a parsed WIT package which has not resolved external
dependencies yet.

This representation has performed internal resolution of the WIT package
itself, ensuring that all references internally are valid and the WIT was
syntactically valid and such.

The fields of this structure represent a flat list of arrays unioned from
all documents within the WIT package. This means, for example, that all
types from all documents are located in `self.types`. The fields of each
item can help splitting back out into packages/interfaces/etc as necessary.

Note that an `UnresolvedPackage` cannot be queried in general about
information such as size or alignment as that would require resolution of
foreign dependencies. Translations such as to-binary additionally are not
supported on an `UnresolvedPackage` due to the lack of knowledge about the
foreign types. This is intended to be an intermediate state which can be
inspected by embedders, if necessary, before quickly transforming to a
[`Resolve`] to fully work with a WIT package.

After an [`UnresolvedPackage`] is parsed it can be fully resolved with
[`Resolve::push`]. During this operation a dependency map is specified which
will connect the `foreign_deps` field of this structure to packages
previously inserted within the [`Resolve`]. Embedders are responsible for
performing this resolution themselves.

```rust
pub struct UnresolvedPackage {
    pub name: PackageName,
    pub worlds: id_arena::Arena<World>,
    pub interfaces: id_arena::Arena<Interface>,
    pub types: id_arena::Arena<TypeDef>,
    pub foreign_deps: indexmap::IndexMap<PackageName, indexmap::IndexMap<String, AstItem>>,
    pub docs: Docs,
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `PackageName` | The namespace, name, and version information for this package. |
| `worlds` | `id_arena::Arena<World>` | All worlds from all documents within this package.<br><br>Each world lists the document that it is from. |
| `interfaces` | `id_arena::Arena<Interface>` | All interfaces from all documents within this package.<br><br>Each interface lists the document that it is from. Interfaces are listed<br>in topological order as well so iteration through this arena will only<br>reference prior elements already visited when working with recursive<br>references. |
| `types` | `id_arena::Arena<TypeDef>` | All types from all documents within this package.<br><br>Each type lists the interface or world that defined it, or nothing if<br>it's an anonymous type. Types are listed in this arena in topological<br>order to ensure that iteration through this arena will only reference<br>other types transitively that are already iterated over. |
| `foreign_deps` | `indexmap::IndexMap<PackageName, indexmap::IndexMap<String, AstItem>>` | All foreign dependencies that this package depends on.<br><br>These foreign dependencies must be resolved to convert this unresolved<br>package into a `Resolve`. The map here is keyed by the name of the<br>foreign package that this depends on, and the sub-map is keyed by an<br>interface name followed by the identifier within `self.interfaces`. The<br>fields of `self.interfaces` describes the required types that are from<br>each foreign interface. |
| `docs` | `Docs` | Doc comments for this package. |
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> UnresolvedPackage { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Send**
- **UnwindSafe**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

### Struct `UnresolvedPackageGroup`

Tracks a set of packages, all pulled from the same group of WIT source files.

```rust
pub struct UnresolvedPackageGroup {
    pub main: UnresolvedPackage,
    pub nested: Vec<UnresolvedPackage>,
    pub source_map: SourceMap,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `main` | `UnresolvedPackage` | The "main" package in this package group which was found at the root of<br>the WIT files.<br><br>Note that this is required to be present in all WIT files. |
| `nested` | `Vec<UnresolvedPackage>` | Nested packages found while parsing `main`, if any. |
| `source_map` | `SourceMap` | A set of processed source files from which these packages have been parsed. |

#### Implementations

##### Methods

- ```rust
  pub fn parse</* synthetic */ impl AsRef<Path>: AsRef<Path>>(path: impl AsRef<Path>, contents: &str) -> Result<UnresolvedPackageGroup> { /* ... */ }
  ```
  Parses the given string as a wit document.

- ```rust
  pub fn parse_path</* synthetic */ impl AsRef<Path>: AsRef<Path>>(path: impl AsRef<Path>) -> Result<UnresolvedPackageGroup> { /* ... */ }
  ```
  Parse a WIT package at the provided path.

- ```rust
  pub fn parse_file</* synthetic */ impl AsRef<Path>: AsRef<Path>>(path: impl AsRef<Path>) -> Result<UnresolvedPackageGroup> { /* ... */ }
  ```
  Parses a WIT package from the file provided.

- ```rust
  pub fn parse_dir</* synthetic */ impl AsRef<Path>: AsRef<Path>>(path: impl AsRef<Path>) -> Result<UnresolvedPackageGroup> { /* ... */ }
  ```
  Parses a WIT package from the directory provided.

##### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> UnresolvedPackageGroup { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

### Enum `AstItem`

**Attributes:**

- `Other("#[<cfg_attr>(feature = \"serde\", derive(Serialize))]")`
- `Other("#[<cfg_attr>(feature = \"serde\", serde(rename_all = \"kebab-case\"))]")`
- `Other("#[serde(rename_all = \"kebab-case\")]")`

```rust
pub enum AstItem {
    Interface(InterfaceId),
    World(WorldId),
}
```

#### Variants

##### `Interface`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `InterfaceId` |  |

##### `World`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `WorldId` |  |

#### Implementations

##### Trait Implementations

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Copy**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> AstItem { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

### Struct `PackageName`

**Attributes:**

- `Other("#[<cfg_attr>(feature = \"serde\", derive(Serialize))]")`
- `Other("#[<cfg_attr>(feature = \"serde\", serde(into = \"String\"))]")`
- `Other("#[serde(into = \"String\")]")`

A structure used to keep track of the name of a package, containing optional
information such as a namespace and version information.

This is directly encoded as an "ID" in the binary component representation
with an interfaced tacked on as well.

```rust
pub struct PackageName {
    pub namespace: String,
    pub name: String,
    pub version: Option<semver::Version>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `namespace` | `String` | A namespace such as `wasi` in `wasi:foo/bar` |
| `name` | `String` | The kebab-name of this package, which is always specified. |
| `version` | `Option<semver::Version>` | Optional major/minor version information. |

#### Implementations

##### Methods

- ```rust
  pub fn interface_id(self: &Self, interface: &str) -> String { /* ... */ }
  ```
  Returns the ID that this package name would assign the `interface` name

- ```rust
  pub fn version_compat_track(version: &Version) -> Version { /* ... */ }
  ```
  Determines the "semver compatible track" for the given version.

- ```rust
  pub fn version_compat_track_string(version: &Version) -> String { /* ... */ }
  ```
  Returns the string corresponding to

##### Trait Implementations

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> PackageName { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &PackageName) -> bool { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(name: PackageName) -> String { /* ... */ }
    ```

- **RefUnwindSafe**
- **Comparable**
  - ```rust
    fn compare(self: &Self, key: &K) -> Ordering { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Sync**
- **Send**
- **Eq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &PackageName) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &PackageName) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

### Struct `World`

**Attributes:**

- `Other("#[<cfg_attr>(feature = \"serde\", derive(Serialize))]")`

```rust
pub struct World {
    pub name: String,
    pub imports: indexmap::IndexMap<WorldKey, WorldItem>,
    pub exports: indexmap::IndexMap<WorldKey, WorldItem>,
    pub package: Option<PackageId>,
    pub docs: Docs,
    pub stability: Stability,
    pub includes: Vec<(Stability, WorldId)>,
    pub include_names: Vec<Vec<IncludeName>>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `String` | The WIT identifier name of this world. |
| `imports` | `indexmap::IndexMap<WorldKey, WorldItem>` | All imported items into this interface, both worlds and functions. |
| `exports` | `indexmap::IndexMap<WorldKey, WorldItem>` | All exported items from this interface, both worlds and functions. |
| `package` | `Option<PackageId>` | The package that owns this world. |
| `docs` | `Docs` | Documentation associated with this world declaration. |
| `stability` | `Stability` | Stability annotation for this world itself. |
| `includes` | `Vec<(Stability, WorldId)>` | All the included worlds from this world. Empty if this is fully resolved |
| `include_names` | `Vec<Vec<IncludeName>>` | All the included worlds names. Empty if this is fully resolved |

#### Implementations

##### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> World { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

### Struct `IncludeName`

```rust
pub struct IncludeName {
    pub name: String,
    pub as_: String,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `String` | The name of the item |
| `as_` | `String` | The name to be replaced with |

#### Implementations

##### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> IncludeName { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

### Enum `WorldKey`

**Attributes:**

- `Other("#[<cfg_attr>(feature = \"serde\", derive(Serialize))]")`
- `Other("#[<cfg_attr>(feature = \"serde\", serde(into = \"String\"))]")`
- `Other("#[serde(into = \"String\")]")`

The key to the import/export maps of a world. Either a kebab-name or a
unique interface.

```rust
pub enum WorldKey {
    Name(String),
    Interface(InterfaceId),
}
```

#### Variants

##### `Name`

A kebab-name.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

##### `Interface`

An interface which is assigned no kebab-name.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `InterfaceId` |  |

#### Implementations

##### Methods

- ```rust
  pub fn unwrap_name(self: Self) -> String { /* ... */ }
  ```
  Asserts that this is `WorldKey::Name` and returns the name.

##### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> WorldKey { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Eq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Hash**
  - ```rust
    fn hash<H: Hasher>(self: &Self, hasher: &mut H) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &WorldKey) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(key: WorldKey) -> String { /* ... */ }
    ```

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

### Enum `WorldItem`

**Attributes:**

- `Other("#[<cfg_attr>(feature = \"serde\", derive(Serialize))]")`
- `Other("#[<cfg_attr>(feature = \"serde\", serde(rename_all = \"kebab-case\"))]")`
- `Other("#[serde(rename_all = \"kebab-case\")]")`

```rust
pub enum WorldItem {
    Interface {
        id: InterfaceId,
        stability: Stability,
    },
    Function(Function),
    Type(TypeId),
}
```

#### Variants

##### `Interface`

An interface is being imported or exported from a world, indicating that
it's a namespace of functions.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `id` | `InterfaceId` |  |
| `stability` | `Stability` |  |

##### `Function`

A function is being directly imported or exported from this world.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Function` |  |

##### `Type`

A type is being exported from this world.

Note that types are never imported into worlds at this time.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `TypeId` |  |

#### Implementations

##### Methods

- ```rust
  pub fn stability<''a>(self: &''a Self, resolve: &''a Resolve) -> &''a Stability { /* ... */ }
  ```

##### Trait Implementations

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> WorldItem { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &WorldItem) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
### Struct `Interface`

**Attributes:**

- `Other("#[<cfg_attr>(feature = \"serde\", derive(Serialize))]")`

```rust
pub struct Interface {
    pub name: Option<String>,
    pub types: indexmap::IndexMap<String, TypeId>,
    pub functions: indexmap::IndexMap<String, Function>,
    pub docs: Docs,
    pub stability: Stability,
    pub package: Option<PackageId>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `Option<String>` | Optionally listed name of this interface.<br><br>This is `None` for inline interfaces in worlds. |
| `types` | `indexmap::IndexMap<String, TypeId>` | Exported types from this interface.<br><br>Export names are listed within the types themselves. Note that the<br>export name here matches the name listed in the `TypeDef`. |
| `functions` | `indexmap::IndexMap<String, Function>` | Exported functions from this interface. |
| `docs` | `Docs` | Documentation associated with this interface. |
| `stability` | `Stability` | Stability attribute for this interface. |
| `package` | `Option<PackageId>` | The package that owns this interface. |

#### Implementations

##### Trait Implementations

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Interface { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
### Struct `TypeDef`

**Attributes:**

- `Other("#[<cfg_attr>(feature = \"serde\", derive(Serialize))]")`

```rust
pub struct TypeDef {
    pub name: Option<String>,
    pub kind: TypeDefKind,
    pub owner: TypeOwner,
    pub docs: Docs,
    pub stability: Stability,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `Option<String>` |  |
| `kind` | `TypeDefKind` |  |
| `owner` | `TypeOwner` |  |
| `docs` | `Docs` |  |
| `stability` | `Stability` | Stability attribute for this type. |

#### Implementations

##### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **UnwindSafe**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Freeze**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TypeDef { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TypeDef) -> bool { /* ... */ }
    ```

### Enum `TypeDefKind`

**Attributes:**

- `Other("#[<cfg_attr>(feature = \"serde\", derive(Serialize))]")`
- `Other("#[<cfg_attr>(feature = \"serde\", serde(rename_all = \"kebab-case\"))]")`
- `Other("#[serde(rename_all = \"kebab-case\")]")`

```rust
pub enum TypeDefKind {
    Record(Record),
    Resource,
    Handle(Handle),
    Flags(Flags),
    Tuple(Tuple),
    Variant(Variant),
    Enum(Enum),
    Option(Type),
    Result(Result_),
    List(Type),
    FixedSizeList(Type, u32),
    Future(Option<Type>),
    Stream(Option<Type>),
    Type(Type),
    Unknown,
}
```

#### Variants

##### `Record`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Record` |  |

##### `Resource`

##### `Handle`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Handle` |  |

##### `Flags`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Flags` |  |

##### `Tuple`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Tuple` |  |

##### `Variant`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Variant` |  |

##### `Enum`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Enum` |  |

##### `Option`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Type` |  |

##### `Result`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Result_` |  |

##### `List`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Type` |  |

##### `FixedSizeList`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Type` |  |
| 1 | `u32` |  |

##### `Future`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<Type>` |  |

##### `Stream`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<Type>` |  |

##### `Type`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Type` |  |

##### `Unknown`

This represents a type of unknown structure imported from a foreign
interface.

This variant is only used during the creation of `UnresolvedPackage` but
by the time a `Resolve` is created then this will not exist.

#### Implementations

##### Methods

- ```rust
  pub fn as_str(self: &Self) -> &''static str { /* ... */ }
  ```

##### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> TypeDefKind { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TypeDefKind) -> bool { /* ... */ }
    ```

- **Eq**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Enum `TypeOwner`

**Attributes:**

- `Other("#[<cfg_attr>(feature = \"serde\", derive(Serialize))]")`
- `Other("#[<cfg_attr>(feature = \"serde\", serde(rename_all = \"kebab-case\"))]")`
- `Other("#[serde(rename_all = \"kebab-case\")]")`

```rust
pub enum TypeOwner {
    World(WorldId),
    Interface(InterfaceId),
    None,
}
```

#### Variants

##### `World`

This type was defined within a `world` block.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `WorldId` |  |

##### `Interface`

This type was defined within an `interface` block.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `InterfaceId` |  |

##### `None`

This type wasn't inherently defined anywhere, such as a `list<T>`, which
doesn't need an owner.

#### Implementations

##### Trait Implementations

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Copy**
- **Unpin**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Send**
- **StructuralPartialEq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TypeOwner { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TypeOwner) -> bool { /* ... */ }
    ```

- **Freeze**
### Enum `Handle`

**Attributes:**

- `Other("#[<cfg_attr>(feature = \"serde\", derive(Serialize))]")`
- `Other("#[<cfg_attr>(feature = \"serde\", serde(rename_all = \"kebab-case\"))]")`
- `Other("#[serde(rename_all = \"kebab-case\")]")`

```rust
pub enum Handle {
    Own(TypeId),
    Borrow(TypeId),
}
```

#### Variants

##### `Own`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `TypeId` |  |

##### `Borrow`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `TypeId` |  |

#### Implementations

##### Trait Implementations

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Freeze**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Handle) -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **StructuralPartialEq**
- **Eq**
- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Handle { /* ... */ }
    ```

### Enum `Type`

```rust
pub enum Type {
    Bool,
    U8,
    U16,
    U32,
    U64,
    S8,
    S16,
    S32,
    S64,
    F32,
    F64,
    Char,
    String,
    ErrorContext,
    Id(TypeId),
}
```

#### Variants

##### `Bool`

##### `U8`

##### `U16`

##### `U32`

##### `U64`

##### `S8`

##### `S16`

##### `S32`

##### `S64`

##### `F32`

##### `F64`

##### `Char`

##### `String`

##### `ErrorContext`

##### `Id`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `TypeId` |  |

#### Implementations

##### Trait Implementations

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<S>(self: &Self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
where
    S: Serializer { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **StructuralPartialEq**
- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Type) -> bool { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Copy**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Type { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Send**
### Enum `Int`

```rust
pub enum Int {
    U8,
    U16,
    U32,
    U64,
}
```

#### Variants

##### `U8`

##### `U16`

##### `U32`

##### `U64`

#### Implementations

##### Trait Implementations

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(i: Int) -> WasmType { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Int) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Eq**
- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Int { /* ... */ }
    ```

- **Copy**
### Struct `Record`

**Attributes:**

- `Other("#[<cfg_attr>(feature = \"serde\", derive(Serialize))]")`

```rust
pub struct Record {
    pub fields: Vec<Field>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `fields` | `Vec<Field>` |  |

#### Implementations

##### Trait Implementations

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Record { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Freeze**
- **StructuralPartialEq**
- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Record) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
### Struct `Field`

**Attributes:**

- `Other("#[<cfg_attr>(feature = \"serde\", derive(Serialize))]")`

```rust
pub struct Field {
    pub name: String,
    pub ty: Type,
    pub docs: Docs,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `String` |  |
| `ty` | `Type` |  |
| `docs` | `Docs` |  |

#### Implementations

##### Trait Implementations

- **Send**
- **Sync**
- **Unpin**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Field { /* ... */ }
    ```

- **RefUnwindSafe**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Eq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Field) -> bool { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

### Struct `Flags`

**Attributes:**

- `Other("#[<cfg_attr>(feature = \"serde\", derive(Serialize))]")`

```rust
pub struct Flags {
    pub flags: Vec<Flag>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `flags` | `Vec<Flag>` |  |

#### Implementations

##### Methods

- ```rust
  pub fn repr(self: &Self) -> FlagsRepr { /* ... */ }
  ```

##### Trait Implementations

- **Send**
- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Flags { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Flags) -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **StructuralPartialEq**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Struct `Flag`

**Attributes:**

- `Other("#[<cfg_attr>(feature = \"serde\", derive(Serialize))]")`

```rust
pub struct Flag {
    pub name: String,
    pub docs: Docs,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `String` |  |
| `docs` | `Docs` |  |

#### Implementations

##### Trait Implementations

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Flag { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Flag) -> bool { /* ... */ }
    ```

- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Freeze**
- **Send**
- **UnwindSafe**
- **StructuralPartialEq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Enum `FlagsRepr`

```rust
pub enum FlagsRepr {
    U8,
    U16,
    U32(usize),
}
```

#### Variants

##### `U8`

##### `U16`

##### `U32`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `usize` |  |

#### Implementations

##### Methods

- ```rust
  pub fn count(self: &Self) -> usize { /* ... */ }
  ```

##### Trait Implementations

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> FlagsRepr { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &FlagsRepr) -> bool { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

### Struct `Tuple`

**Attributes:**

- `Other("#[<cfg_attr>(feature = \"serde\", derive(Serialize))]")`

```rust
pub struct Tuple {
    pub types: Vec<Type>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `types` | `Vec<Type>` |  |

#### Implementations

##### Trait Implementations

- **Unpin**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Tuple) -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Tuple { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
### Struct `Variant`

**Attributes:**

- `Other("#[<cfg_attr>(feature = \"serde\", derive(Serialize))]")`

```rust
pub struct Variant {
    pub cases: Vec<Case>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `cases` | `Vec<Case>` |  |

#### Implementations

##### Methods

- ```rust
  pub fn tag(self: &Self) -> Int { /* ... */ }
  ```

##### Trait Implementations

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Eq**
- **UnwindSafe**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Variant { /* ... */ }
    ```

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Variant) -> bool { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

### Struct `Case`

**Attributes:**

- `Other("#[<cfg_attr>(feature = \"serde\", derive(Serialize))]")`

```rust
pub struct Case {
    pub name: String,
    pub ty: Option<Type>,
    pub docs: Docs,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `String` |  |
| `ty` | `Option<Type>` |  |
| `docs` | `Docs` |  |

#### Implementations

##### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Unpin**
- **RefUnwindSafe**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **StructuralPartialEq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Case { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **UnwindSafe**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Case) -> bool { /* ... */ }
    ```

### Struct `Enum`

**Attributes:**

- `Other("#[<cfg_attr>(feature = \"serde\", derive(Serialize))]")`

```rust
pub struct Enum {
    pub cases: Vec<EnumCase>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `cases` | `Vec<EnumCase>` |  |

#### Implementations

##### Methods

- ```rust
  pub fn tag(self: &Self) -> Int { /* ... */ }
  ```

##### Trait Implementations

- **StructuralPartialEq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Enum) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Enum { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

### Struct `EnumCase`

**Attributes:**

- `Other("#[<cfg_attr>(feature = \"serde\", derive(Serialize))]")`

```rust
pub struct EnumCase {
    pub name: String,
    pub docs: Docs,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `String` |  |
| `docs` | `Docs` |  |

#### Implementations

##### Trait Implementations

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> EnumCase { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **StructuralPartialEq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Eq**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Freeze**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &EnumCase) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

### Struct `Result_`

**Attributes:**

- `Other("#[<cfg_attr>(feature = \"serde\", derive(Serialize))]")`

```rust
pub struct Result_ {
    pub ok: Option<Type>,
    pub err: Option<Type>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `ok` | `Option<Type>` |  |
| `err` | `Option<Type>` |  |

#### Implementations

##### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Result_) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Result_ { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Eq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
### Struct `Docs`

**Attributes:**

- `Other("#[<cfg_attr>(feature = \"serde\", derive(Serialize))]")`

```rust
pub struct Docs {
    pub contents: Option<String>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `contents` | `Option<String>` |  |

#### Implementations

##### Methods

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```

##### Trait Implementations

- **Freeze**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Docs { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Docs { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Eq**
- **RefUnwindSafe**
- **UnwindSafe**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Docs) -> bool { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Struct `Function`

**Attributes:**

- `Other("#[<cfg_attr>(feature = \"serde\", derive(Serialize))]")`

```rust
pub struct Function {
    pub name: String,
    pub kind: FunctionKind,
    pub params: Vec<(String, Type)>,
    pub result: Option<Type>,
    pub docs: Docs,
    pub stability: Stability,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `String` |  |
| `kind` | `FunctionKind` |  |
| `params` | `Vec<(String, Type)>` |  |
| `result` | `Option<Type>` |  |
| `docs` | `Docs` |  |
| `stability` | `Stability` | Stability attribute for this function. |

#### Implementations

##### Methods

- ```rust
  pub fn item_name(self: &Self) -> &str { /* ... */ }
  ```

- ```rust
  pub fn parameter_and_result_types(self: &Self) -> impl Iterator<Item = Type> + ''_ { /* ... */ }
  ```
  Returns an iterator over the types used in parameters and results.

- ```rust
  pub fn standard32_core_export_name<''a>(self: &''a Self, interface: Option<&str>) -> Cow<''a, str> { /* ... */ }
  ```
  Gets the core export name for this function.

- ```rust
  pub fn legacy_core_export_name<''a>(self: &''a Self, interface: Option<&str>) -> Cow<''a, str> { /* ... */ }
  ```

- ```rust
  pub fn core_export_name<''a>(self: &''a Self, interface: Option<&str>, mangling: Mangling) -> Cow<''a, str> { /* ... */ }
  ```
  Gets the core export name for this function.

- ```rust
  pub fn find_futures_and_streams(self: &Self, resolve: &Resolve) -> Vec<TypeId> { /* ... */ }
  ```
  Collect any future and stream types appearing in the signature of this

##### Trait Implementations

- **RefUnwindSafe**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Eq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Function { /* ... */ }
    ```

- **StructuralPartialEq**
- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Function) -> bool { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
### Enum `FunctionKind`

**Attributes:**

- `Other("#[<cfg_attr>(feature = \"serde\", derive(Serialize))]")`
- `Other("#[<cfg_attr>(feature = \"serde\", serde(rename_all = \"kebab-case\"))]")`
- `Other("#[serde(rename_all = \"kebab-case\")]")`

```rust
pub enum FunctionKind {
    Freestanding,
    AsyncFreestanding,
    Method(TypeId),
    AsyncMethod(TypeId),
    Static(TypeId),
    AsyncStatic(TypeId),
    Constructor(TypeId),
}
```

#### Variants

##### `Freestanding`

A freestanding function.

```wit
interface foo {
    the-func: func();
}
```

##### `AsyncFreestanding`

An async freestanding function.

```wit
interface foo {
    the-func: async func();
}
```

##### `Method`

A resource method where the first parameter is implicitly
`borrow<T>`.

```wit
interface foo {
    resource r {
        the-func: func();
    }
}
```

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `TypeId` |  |

##### `AsyncMethod`

An async resource method where the first parameter is implicitly
`borrow<T>`.

```wit
interface foo {
    resource r {
        the-func: async func();
    }
}
```

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `TypeId` |  |

##### `Static`

A static resource method.

```wit
interface foo {
    resource r {
        the-func: static func();
    }
}
```

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `TypeId` |  |

##### `AsyncStatic`

An async static resource method.

```wit
interface foo {
    resource r {
        the-func: static async func();
    }
}
```

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `TypeId` |  |

##### `Constructor`

A resource constructor where the return value is implicitly `own<T>`.

```wit
interface foo {
    resource r {
        constructor();
    }
}
```

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `TypeId` |  |

#### Implementations

##### Methods

- ```rust
  pub fn resource(self: &Self) -> Option<TypeId> { /* ... */ }
  ```
  Returns the resource, if present, that this function kind refers to.

- ```rust
  pub fn resource_mut(self: &mut Self) -> Option<&mut TypeId> { /* ... */ }
  ```
  Returns the resource, if present, that this function kind refers to.

##### Trait Implementations

- **RefUnwindSafe**
- **StructuralPartialEq**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> FunctionKind { /* ... */ }
    ```

- **Send**
- **Sync**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &FunctionKind) -> bool { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Eq**
### Enum `Mangling`

Possible forms of name mangling that are supported by this crate.

```rust
pub enum Mangling {
    Standard32,
    Legacy,
}
```

#### Variants

##### `Standard32`

The "standard" component model mangling format for 32-bit linear
memories. This is specified in WebAssembly/component-model#378

##### `Legacy`

The "legacy" name mangling supported in versions 218-and-prior for this
crate. This is the original support for how components were created from
core wasm modules and this does not correspond to any standard. This is
preserved for now while tools transition to the new scheme.

#### Implementations

##### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Mangling) -> bool { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Eq**
- **Unpin**
- **Send**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Mangling { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Copy**
- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **FromStr**
  - ```rust
    fn from_str(s: &str) -> Result<Mangling> { /* ... */ }
    ```

### Enum `LiftLowerAbi`

Possible lift/lower ABI choices supported when mangling names.

```rust
pub enum LiftLowerAbi {
    Sync,
    AsyncCallback,
    AsyncStackful,
}
```

#### Variants

##### `Sync`

Both imports and exports will use the synchronous ABI.

##### `AsyncCallback`

Both imports and exports will use the async ABI (with a callback for
each export).

##### `AsyncStackful`

Both imports and exports will use the async ABI (with no callbacks for
exports).

#### Implementations

##### Methods

- ```rust
  pub fn import_variant(self: Self) -> AbiVariant { /* ... */ }
  ```
  Get the import [`AbiVariant`] corresponding to this [`LiftLowerAbi`]

- ```rust
  pub fn export_variant(self: Self) -> AbiVariant { /* ... */ }
  ```
  Get the export [`AbiVariant`] corresponding to this [`LiftLowerAbi`]

##### Trait Implementations

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> LiftLowerAbi { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &LiftLowerAbi) -> bool { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **StructuralPartialEq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Copy**
- **Eq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Enum `ManglingAndAbi`

Combination of [`Mangling`] and [`LiftLowerAbi`].

```rust
pub enum ManglingAndAbi {
    Standard32,
    Legacy(LiftLowerAbi),
}
```

#### Variants

##### `Standard32`

See [`Mangling::Standard32`].

As of this writing, the standard name mangling only supports the
synchronous ABI.

##### `Legacy`

See [`Mangling::Legacy`] and [`LiftLowerAbi`].

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `LiftLowerAbi` |  |

#### Implementations

##### Methods

- ```rust
  pub fn import_variant(self: Self) -> AbiVariant { /* ... */ }
  ```
  Get the import [`AbiVariant`] corresponding to this [`ManglingAndAbi`]

- ```rust
  pub fn export_variant(self: Self) -> AbiVariant { /* ... */ }
  ```
  Get the export [`AbiVariant`] corresponding to this [`ManglingAndAbi`]

- ```rust
  pub fn sync(self: Self) -> Self { /* ... */ }
  ```
  Switch the ABI to be sync if it's async.

- ```rust
  pub fn is_async(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether this is an async ABI

##### Trait Implementations

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ManglingAndAbi { /* ... */ }
    ```

- **UnwindSafe**
- **Copy**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ManglingAndAbi) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **StructuralPartialEq**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Eq**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

### Enum `Stability`

**Attributes:**

- `Other("#[<cfg_attr>(feature = \"serde\", derive(serde_derive::Deserialize, Serialize))]")`
- `Other("#[<cfg_attr>(feature = \"serde\", serde(rename_all = \"kebab-case\"))]")`
- `Other("#[serde(rename_all = \"kebab-case\")]")`

Representation of the stability attributes associated with a world,
interface, function, or type.

This is added for WebAssembly/component-model#332 where @since and @unstable
annotations were added to WIT.

The order of the of enum values is significant since it is used with Ord and PartialOrd

```rust
pub enum Stability {
    Unknown,
    Unstable {
        feature: String,
        deprecated: Option<semver::Version>,
    },
    Stable {
        since: semver::Version,
        deprecated: Option<semver::Version>,
    },
}
```

#### Variants

##### `Unknown`

This item does not have either `@since` or `@unstable`.

##### `Unstable`

`@unstable(feature = foo)`

This item is explicitly tagged `@unstable`. A feature name is listed and
this item is excluded by default in `Resolve` unless explicitly enabled.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `feature` | `String` |  |
| `deprecated` | `Option<semver::Version>` |  |

##### `Stable`

`@since(version = 1.2.3)`

This item is explicitly tagged with `@since` as stable since the
specified version.  This may optionally have a feature listed as well.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `since` | `semver::Version` |  |
| `deprecated` | `Option<semver::Version>` |  |

#### Implementations

##### Methods

- ```rust
  pub fn is_unknown(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether this is `Stability::Unknown`.

- ```rust
  pub fn is_stable(self: &Self) -> bool { /* ... */ }
  ```

##### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Stability { /* ... */ }
    ```

- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **StructuralPartialEq**
- **Comparable**
  - ```rust
    fn compare(self: &Self, key: &K) -> Ordering { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Send**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Stability) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Stability) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **UnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Stability) -> bool { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **DeserializeOwned**
- **RefUnwindSafe**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Sync**
- **Default**
  - ```rust
    fn default() -> Stability { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

## Functions

### Function `validate_id`

Checks if the given string is a legal identifier in wit.

```rust
pub fn validate_id(s: &str) -> anyhow::Result<()> { /* ... */ }
```

## Re-exports

### Re-export `PackageMetadata`

**Attributes:**

- `Other("#[<cfg>(feature = \"decoding\")]")`

```rust
pub use metadata::PackageMetadata;
```

### Re-export `SourceMap`

```rust
pub use ast::SourceMap;
```

### Re-export `ParsedUsePath`

```rust
pub use ast::ParsedUsePath;
```

### Re-export `parse_use_path`

```rust
pub use ast::parse_use_path;
```

### Re-export `LiveTypes`

```rust
pub use live::LiveTypes;
```

### Re-export `TypeIdVisitor`

```rust
pub use live::TypeIdVisitor;
```

### Re-export `sizealign::*`

```rust
pub use sizealign::*;
```

### Re-export `resolve::*`

```rust
pub use resolve::*;
```

