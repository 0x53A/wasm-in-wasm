# Crate Documentation

**Version:** 0.5.0

**Format Version:** 54

# Module `wasm_runtime_layer`

`wasm_runtime_layer` creates a thin abstraction over WebAssembly runtimes, allowing for backend-agnostic host code. The interface is based upon the `wasmtime` and `wasmi` crates, but may be implemented for any runtime.

## Usage

To use this crate, first instantiate a backend runtime. The runtime may be any
value that implements `backend::WasmEngine`. Some runtimes are already implemented as additional crates.
Then, one can create an `Engine` from the backend runtime, and use it to initialize modules and instances:

```rust
# use wasm_runtime_layer::*;
// 1. Instantiate a runtime
let engine = Engine::new(wasmi_runtime_layer::Engine::default());
let mut store = Store::new(&engine, ());

// 2. Create modules and instances, similar to other runtimes
let module_bin = wat::parse_str(
    r#"
(module
(type $t0 (func (param i32) (result i32)))
(func $add_one (export "add_one") (type $t0) (param $p0 i32) (result i32)
    local.get $p0
    i32.const 1
    i32.add))
"#,
)
.unwrap();

let module = Module::new(&engine, &module_bin).unwrap();
let instance = Instance::new(&mut store, &module, &Imports::default()).unwrap();

let add_one = instance
    .get_export(&store, "add_one")
    .unwrap()
    .into_func()
    .unwrap();
         
let mut result = [Value::I32(0)];
add_one
    .call(&mut store, &[Value::I32(42)], &mut result)
    .unwrap();

assert_eq!(result[0], Value::I32(43));
```

## Backends

* **wasmi_runtime_layer** - Implements the `WasmEngine` trait for wrappers around `wasmi::Engine` instances.
* **wasmtime_runtime_layer** - Implements the `WasmEngine` trait for wrappers around `wasmtime::Engine` instances.
* **js_wasm_runtime_layer** - Implements a wasm engine targeting the browser's WebAssembly API on `wasm32-unknown-unknown` targets.
* **pyodide-webassembly-runtime-layer** - Implements a wasm engine targeting the browser's WebAssembly API when running as a Python extension module inside Pyodide.

Contributions for additional backend implementations are welcome!

## Testing

To run the tests for wasmi and wasmtime, run:

```sh
cargo test
```

For the *wasm32* target, you can use the slower interpreter *wasmi*, or the native JIT accelerated browser backend.

To test the backends, you need to install [`wasm-pack`](https://github.com/rustwasm/wasm-pack).

You can then run:
```sh
wasm-pack test --node
```

## Modules

## Module `backend`

Provides traits for implementing runtime backends.

```rust
pub mod backend { /* ... */ }
```

### Types

#### Enum `Value`

Runtime representation of a value.

```rust
pub enum Value<E: WasmEngine> {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    FuncRef(Option<<E as >::Func>),
    ExternRef(Option<<E as >::ExternRef>),
}
```

##### Variants

###### `I32`

Value of 32-bit signed or unsigned integer.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i32` |  |

###### `I64`

Value of 64-bit signed or unsigned integer.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i64` |  |

###### `F32`

Value of 32-bit floating point number.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `f32` |  |

###### `F64`

Value of 64-bit floating point number.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `f64` |  |

###### `FuncRef`

An optional function reference.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<<E as >::Func>` |  |

###### `ExternRef`

An optional external reference.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<<E as >::ExternRef>` |  |

##### Implementations

###### Methods

- ```rust
  pub const fn ty(self: &Self) -> ValueType { /* ... */ }
  ```
  Returns the [`ValueType`] for this [`Value`].

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: &Value) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &crate::backend::Value<E>) -> Self { /* ... */ }
    ```

- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
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

- **Send**
- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Value<E> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
#### Enum `Extern`

An external item to a WebAssembly module.

This is returned from [`Instance::exports`](crate::Instance::exports)
or [`Instance::get_export`](crate::Instance::get_export).

```rust
pub enum Extern<E: WasmEngine> {
    Global(<E as >::Global),
    Table(<E as >::Table),
    Memory(<E as >::Memory),
    Func(<E as >::Func),
}
```

##### Variants

###### `Global`

A WebAssembly global which acts like a [`Cell<T>`] of sorts, supporting `get` and `set` operations.

[`Cell<T>`]: https://doc.rust-lang.org/core/cell/struct.Cell.html

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `<E as >::Global` |  |

###### `Table`

A WebAssembly table which is an array of funtion references.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `<E as >::Table` |  |

###### `Memory`

A WebAssembly linear memory.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `<E as >::Memory` |  |

###### `Func`

A WebAssembly function which can be called.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `<E as >::Func` |  |

##### Implementations

###### Methods

- ```rust
  pub fn into_global(self: Self) -> Option<<E as >::Global> { /* ... */ }
  ```
  Returns the underlying global variable if `self` is a global variable.

- ```rust
  pub fn into_table(self: Self) -> Option<<E as >::Table> { /* ... */ }
  ```
  Returns the underlying table if `self` is a table.

- ```rust
  pub fn into_memory(self: Self) -> Option<<E as >::Memory> { /* ... */ }
  ```
  Returns the underlying linear memory if `self` is a linear memory.

- ```rust
  pub fn into_func(self: Self) -> Option<<E as >::Func> { /* ... */ }
  ```
  Returns the underlying function if `self` is a function.

- ```rust
  pub fn ty</* synthetic */ impl AsContext<E>: AsContext<E>>(self: &Self, ctx: impl AsContext<E>) -> ExternType { /* ... */ }
  ```
  Returns the type associated with this [`Extern`].

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: &crate::backend::Extern<E>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &Extern) -> Self { /* ... */ }
    ```

- **Unpin**
- **Freeze**
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

- **UnwindSafe**
- **RefUnwindSafe**
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
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

#### Struct `Export`

A descriptor for an exported WebAssembly value of an [`Instance`].

This type is primarily accessed from the [`Instance::exports`] method and describes
what names are exported from a Wasm [`Instance`] and the type of the item that is exported.

```rust
pub struct Export<E: WasmEngine> {
    pub name: alloc::string::String,
    pub value: Extern<E>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `alloc::string::String` | The name by which the export is known. |
| `value` | `Extern<E>` | The value of the exported item. |

##### Implementations

###### Trait Implementations

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Export<E> { /* ... */ }
    ```

- **UnwindSafe**
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

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: crate::backend::Export<E>) -> Self { /* ... */ }
    ```

#### Struct `Imports`

All of the import data used when instantiating.

```rust
pub struct Imports<E: WasmEngine> {
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
  pub fn new() -> Self { /* ... */ }
  ```
  Create a new `Imports`.

- ```rust
  pub fn get_export(self: &Self, module: &str, name: &str) -> Option<Extern<E>> { /* ... */ }
  ```
  Gets an export given a module and a name

- ```rust
  pub fn exists(self: &Self, module: &str, name: &str) -> bool { /* ... */ }
  ```
  Returns if an export exist for a given module and name.

- ```rust
  pub fn contains_namespace(self: &Self, name: &str) -> bool { /* ... */ }
  ```
  Returns true if the Imports contains namespace with the provided name.

- ```rust
  pub fn register_namespace</* synthetic */ impl IntoIterator<Item = (String, Extern<E>)>: IntoIterator<Item = (String, Extern<E>)>>(self: &mut Self, ns: &str, contents: impl IntoIterator<Item = (String, Extern<E>)>) { /* ... */ }
  ```
  Register a list of externs into a namespace.

- ```rust
  pub fn define</* synthetic */ impl Into<Extern<E>>: Into<Extern<E>>>(self: &mut Self, ns: &str, name: &str, val: impl Into<Extern<E>>) { /* ... */ }
  ```
  Add a single import with a namespace `ns` and name `name`.

- ```rust
  pub fn iter(self: &Self) -> ImportsIterator<''_, E> { /* ... */ }
  ```
  Iterates through all the imports in this structure

###### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Extend**
  - ```rust
    fn extend<T: IntoIterator<Item = ((String, String), Extern<E>)>>(self: &mut Self, iter: T) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Imports<E> { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **Send**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
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

#### Struct `ImportsIterator`

An iterator over imports.

```rust
pub struct ImportsIterator<''a, E: WasmEngine> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Freeze**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

### Traits

#### Trait `WasmEngine`

Provides a backing implementation for a WebAssembly runtime.

```rust
pub trait WasmEngine: ''static + Clone + Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `ExternRef`: The external reference type.
- `Func`: The function type.
- `Global`: The global type.
- `Instance`: The instance type.
- `Memory`: The memory type.
- `Module`: The module type.
- `Store`: The store type.
- `StoreContext`: The store context type.
- `StoreContextMut`: The mutable store context type.
- `Table`: The table type.

#### Trait `WasmExternRef`

Provides an opaque reference to any data within WebAssembly.

```rust
pub trait WasmExternRef<E: WasmEngine>: Clone + Sized + Send + Sync {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `new`: Creates a new reference wrapping the given value.
- `downcast`: Returns a shared reference to the underlying data.

#### Trait `WasmFunc`

Provides a Wasm or host function reference.

```rust
pub trait WasmFunc<E: WasmEngine>: Clone + Sized + Send + Sync {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `new`: Creates a new function with the given arguments.
- `ty`: Gets the function type of this object.
- `call`: Calls the object with the given arguments.

#### Trait `WasmGlobal`

Provides a Wasm global variable reference.

```rust
pub trait WasmGlobal<E: WasmEngine>: Clone + Sized + Send + Sync {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `new`: Creates a new global variable to the store.
- `ty`: Returns the type of the global variable.
- `set`: Sets the value of the global variable.
- `get`: Gets the value of the global variable.

#### Trait `WasmMemory`

Provides a Wasm linear memory reference.

```rust
pub trait WasmMemory<E: WasmEngine>: Clone + Sized + Send + Sync {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `new`: Creates a new linear memory to the store.
- `ty`: Returns the memory type of the linear memory.
- `grow`: Grows the linear memory by the given amount of new pages.
- `current_pages`: Returns the amount of pages in use by the linear memory.
- `read`: Reads `n` bytes from `memory[offset..offset+n]` into `buffer`
- `write`: Writes `n` bytes to `memory[offset..offset+n]` from `buffer`

#### Trait `WasmTable`

Provides a Wasm table reference.

```rust
pub trait WasmTable<E: WasmEngine>: Clone + Sized + Send + Sync {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `new`: Creates a new table to the store.
- `ty`: Returns the type and limits of the table.
- `size`: Returns the current size of the table.
- `grow`: Grows the table by the given amount of elements.
- `get`: Returns the table element value at `index`.
- `set`: Sets the value of this table at `index`.

#### Trait `WasmInstance`

Provides an instantiated WASM module.

```rust
pub trait WasmInstance<E: WasmEngine>: Clone + Sized + Send + Sync {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `new`: Creates a new instance.
- `exports`: Gets the exports of this instance.
- `get_export`: Gets the export of the given name, if any, from this instance.

#### Trait `WasmModule`

Provides a parsed and validated WASM module.

```rust
pub trait WasmModule<E: WasmEngine>: Clone + Sized + Send + Sync {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `new`: Creates a new module from the given byte slice.
- `exports`: Gets the export types of the module.
- `get_export`: Gets the export type of the given name, if any, from this module.
- `imports`: Gets the import types of the module.

#### Trait `WasmStore`

Provides all of the global state that can be manipulated by WASM programs.

```rust
pub trait WasmStore<T, E: WasmEngine>: AsContext<E, UserState = T> + AsContextMut<E, UserState = T> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `new`: Creates a new store atop the given engine.
- `engine`: Gets the engine associated with this store.
- `data`: Gets an immutable reference to the underlying stored data.
- `data_mut`: Gets a mutable reference to the underlying stored data.
- `into_data`: Consumes `self` and returns its user provided data.

#### Trait `WasmStoreContext`

Provides a temporary immutable handle to a store.

```rust
pub trait WasmStoreContext<''a, T, E: WasmEngine>: AsContext<E, UserState = T> {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `engine`: Gets the engine associated with this store.
- `data`: Gets an immutable reference to the underlying stored data.

#### Trait `WasmStoreContextMut`

Provides a temporary mutable handle to a store.

```rust
pub trait WasmStoreContextMut<''a, T, E: WasmEngine>: WasmStoreContext<''a, T, E> + AsContextMut<E, UserState = T> {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `data_mut`: Gets a mutable reference to the underlying stored data.

#### Trait `AsContext`

A trait used to get shared access to a store.

```rust
pub trait AsContext<E: WasmEngine> {
    /* Associated items */
}
```

##### Required Items

###### Associated Types

- `UserState`: The type of data associated with the store.

###### Required Methods

- `as_context`: Returns the store context that this type provides access to.

##### Implementations

This trait is implemented for the following types:

- `C` with <E, C>

#### Trait `AsContextMut`

A trait used to get mutable access to a store.

```rust
pub trait AsContextMut<E: WasmEngine>: AsContext<E> {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `as_context_mut`: Returns the store context that this type provides access to.

##### Implementations

This trait is implemented for the following types:

- `C` with <E, C>

## Types

### Enum `ValueType`

Type of a value.

```rust
pub enum ValueType {
    I32,
    I64,
    F32,
    F64,
    FuncRef,
    ExternRef,
}
```

#### Variants

##### `I32`

32-bit signed or unsigned integer.

##### `I64`

64-bit signed or unsigned integer.

##### `F32`

32-bit floating point number.

##### `F64`

64-bit floating point number.

##### `FuncRef`

An optional function reference.

##### `ExternRef`

An optional external reference.

#### Implementations

##### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

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

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Send**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **StructuralPartialEq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Eq**
- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ValueType { /* ... */ }
    ```

- **Freeze**
- **Copy**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ValueType) -> bool { /* ... */ }
    ```

### Struct `GlobalType`

The type of a global variable.

```rust
pub struct GlobalType {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub fn new(content: ValueType, mutable: bool) -> Self { /* ... */ }
  ```
  Creates a new [`GlobalType`] from the given [`ValueType`] and mutability.

- ```rust
  pub fn content(self: &Self) -> ValueType { /* ... */ }
  ```
  Returns the [`ValueType`] of the global variable.

- ```rust
  pub fn mutable(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether the global variable is mutable.

##### Trait Implementations

- **Eq**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(global: GlobalType) -> Self { /* ... */ }
    ```

- **StructuralPartialEq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &GlobalType) -> bool { /* ... */ }
    ```

- **Sync**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> GlobalType { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Copy**
- **Send**
- **RefUnwindSafe**
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

### Struct `TableType`

A descriptor for a [`Table`] instance.

```rust
pub struct TableType {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub fn new(element: ValueType, min: u32, max: Option<u32>) -> Self { /* ... */ }
  ```
  Creates a new [`TableType`].

- ```rust
  pub fn element(self: &Self) -> ValueType { /* ... */ }
  ```
  Returns the [`ValueType`] of elements stored in the [`Table`].

- ```rust
  pub fn minimum(self: &Self) -> u32 { /* ... */ }
  ```
  Returns minimum number of elements the [`Table`] must have.

- ```rust
  pub fn maximum(self: &Self) -> Option<u32> { /* ... */ }
  ```
  The optional maximum number of elements the [`Table`] can have.

##### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TableType { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(table: TableType) -> Self { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **StructuralPartialEq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Eq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TableType) -> bool { /* ... */ }
    ```

- **Copy**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Struct `MemoryType`

The memory type of a linear memory.

```rust
pub struct MemoryType {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub fn new(initial: u32, maximum: Option<u32>) -> Self { /* ... */ }
  ```
  Creates a new memory type with initial and optional maximum pages.

- ```rust
  pub fn initial_pages(self: Self) -> u32 { /* ... */ }
  ```
  Returns the initial pages of the memory type.

- ```rust
  pub fn maximum_pages(self: Self) -> Option<u32> { /* ... */ }
  ```
  Returns the maximum pages of the memory type.

##### Trait Implementations

- **UnwindSafe**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Copy**
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

  - ```rust
    fn from(memory: MemoryType) -> Self { /* ... */ }
    ```

- **Send**
- **Unpin**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> MemoryType { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MemoryType) -> bool { /* ... */ }
    ```

- **Freeze**
- **StructuralPartialEq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **RefUnwindSafe**
### Struct `FuncType`

A function type representing a function's parameter and result types.

# Note

Can be cloned cheaply.

```rust
pub struct FuncType {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub fn new<P, R>(params: P, results: R) -> Self
where
    P: IntoIterator<Item = ValueType>,
    R: IntoIterator<Item = ValueType> { /* ... */ }
  ```
  Creates a new [`FuncType`].

- ```rust
  pub fn params(self: &Self) -> &[ValueType] { /* ... */ }
  ```
  Returns the parameter types of the function type.

- ```rust
  pub fn results(self: &Self) -> &[ValueType] { /* ... */ }
  ```
  Returns the result types of the function type.

- ```rust
  pub fn with_name</* synthetic */ impl Into<Arc<str>>: Into<Arc<str>>>(self: Self, name: impl Into<Arc<str>>) -> Self { /* ... */ }
  ```
  Set the debug name of the function

##### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> FuncType { /* ... */ }
    ```

- **Freeze**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(func: FuncType) -> Self { /* ... */ }
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

### Enum `ExternType`

The type of an [`Extern`] item.

A list of all possible types which can be externally referenced from a WebAssembly module.

```rust
pub enum ExternType {
    Global(GlobalType),
    Table(TableType),
    Memory(MemoryType),
    Func(FuncType),
}
```

#### Variants

##### `Global`

The type of an [`Extern::Global`].

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `GlobalType` |  |

##### `Table`

The type of an [`Extern::Table`].

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `TableType` |  |

##### `Memory`

The type of an [`Extern::Memory`].

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `MemoryType` |  |

##### `Func`

The type of an [`Extern::Func`].

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `FuncType` |  |

#### Implementations

##### Methods

- ```rust
  pub fn global(self: &Self) -> Option<&GlobalType> { /* ... */ }
  ```
  Returns the underlying [`GlobalType`] or `None` if it is of a different type.

- ```rust
  pub fn table(self: &Self) -> Option<&TableType> { /* ... */ }
  ```
  Returns the underlying [`TableType`] or `None` if it is of a different type.

- ```rust
  pub fn memory(self: &Self) -> Option<&MemoryType> { /* ... */ }
  ```
  Returns the underlying [`MemoryType`] or `None` if it is of a different type.

- ```rust
  pub fn func(self: &Self) -> Option<&FuncType> { /* ... */ }
  ```
  Returns the underlying [`FuncType`] or `None` if it is of a different type.

- ```rust
  pub fn try_into_func(self: Self) -> core::result::Result<FuncType, Self> { /* ... */ }
  ```
  Return the underlying [`FuncType`] if the types match

- ```rust
  pub fn try_into_table(self: Self) -> Result<TableType, Self> { /* ... */ }
  ```
  Return the underlying [`TableType`] if the types match

- ```rust
  pub fn try_into_global(self: Self) -> Result<GlobalType, Self> { /* ... */ }
  ```
  Return the underlying [`GlobalType`] if the types match

- ```rust
  pub fn try_into_memory(self: Self) -> Result<MemoryType, Self> { /* ... */ }
  ```
  Return the underlying [`MemoryType`] if the types match

##### Trait Implementations

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ExternType { /* ... */ }
    ```

- **Unpin**
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

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

  - ```rust
    fn from(global: GlobalType) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(table: TableType) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(memory: MemoryType) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(func: FuncType) -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
### Struct `ExportType`

A descriptor for an exported WebAssembly value of a [`Module`].

This type is primarily accessed from the [`Module::exports`] method and describes
what names are exported from a Wasm [`Module`] and the type of the item that is exported.

```rust
pub struct ExportType<''module> {
    pub name: &''module str,
    pub ty: ExternType,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `&''module str` | The name by which the export is known. |
| `ty` | `ExternType` | The type of the exported item. |

#### Implementations

##### Trait Implementations

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ExportType<''module> { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
### Struct `ImportType`

A descriptor for an imported value into a Wasm [`Module`].

This type is primarily accessed from the [`Module::imports`] method.
Each [`ImportType`] describes an import into the Wasm module with the `module/name`
that it is imported from as well as the type of item that is being imported.

```rust
pub struct ImportType<''module> {
    pub module: &''module str,
    pub name: &''module str,
    pub ty: ExternType,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `module` | `&''module str` | The module import name. |
| `name` | `&''module str` | The name of the imported item. |
| `ty` | `ExternType` | The external item type. |

#### Implementations

##### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ImportType<''module> { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

### Enum `Extern`

An external item to a WebAssembly module.

This is returned from [`Instance::exports`](crate::Instance::exports)
or [`Instance::get_export`](crate::Instance::get_export).

```rust
pub enum Extern {
    Global(Global),
    Table(Table),
    Memory(Memory),
    Func(Func),
}
```

#### Variants

##### `Global`

A WebAssembly global which acts like a [`Cell<T>`] of sorts, supporting `get` and `set` operations.

[`Cell<T>`]: https://doc.rust-lang.org/core/cell/struct.Cell.html

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Global` |  |

##### `Table`

A WebAssembly table which is an array of funtion references.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Table` |  |

##### `Memory`

A WebAssembly linear memory.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Memory` |  |

##### `Func`

A WebAssembly function which can be called.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Func` |  |

#### Implementations

##### Methods

- ```rust
  pub fn into_global(self: Self) -> Option<Global> { /* ... */ }
  ```
  Returns the underlying global variable if `self` is a global variable.

- ```rust
  pub fn into_table(self: Self) -> Option<Table> { /* ... */ }
  ```
  Returns the underlying table if `self` is a table.

- ```rust
  pub fn into_memory(self: Self) -> Option<Memory> { /* ... */ }
  ```
  Returns the underlying linear memory if `self` is a linear memory.

- ```rust
  pub fn into_func(self: Self) -> Option<Func> { /* ... */ }
  ```
  Returns the underlying function if `self` is a function.

- ```rust
  pub fn ty</* synthetic */ impl AsContext: AsContext>(self: &Self, ctx: impl AsContext) -> ExternType { /* ... */ }
  ```
  Returns the type associated with this [`Extern`].

##### Trait Implementations

- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Extern { /* ... */ }
    ```

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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: &crate::backend::Extern<E>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &Extern) -> Self { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
### Struct `Export`

A descriptor for an exported WebAssembly value of an [`Instance`].

This type is primarily accessed from the [`Instance::exports`] method and describes
what names are exported from a Wasm [`Instance`] and the type of the item that is exported.

```rust
pub struct Export {
    pub name: alloc::string::String,
    pub value: Extern,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `alloc::string::String` | The name by which the export is known. |
| `value` | `Extern` | The value of the exported item. |

#### Implementations

##### Trait Implementations

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
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

  - ```rust
    fn from(value: crate::backend::Export<E>) -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
### Struct `Imports`

All of the import data used when instantiating.

```rust
pub struct Imports {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Create a new `Imports`.

- ```rust
  pub fn get_export(self: &Self, module: &str, name: &str) -> Option<Extern> { /* ... */ }
  ```
  Gets an export given a module and a name

- ```rust
  pub fn exists(self: &Self, module: &str, name: &str) -> bool { /* ... */ }
  ```
  Returns if an export exist for a given module and name.

- ```rust
  pub fn contains_namespace(self: &Self, name: &str) -> bool { /* ... */ }
  ```
  Returns true if the Imports contains namespace with the provided name.

- ```rust
  pub fn register_namespace</* synthetic */ impl IntoIterator<Item = (String, Extern)>: IntoIterator<Item = (String, Extern)>>(self: &mut Self, ns: &str, contents: impl IntoIterator<Item = (String, Extern)>) { /* ... */ }
  ```
  Register a list of externs into a namespace.

- ```rust
  pub fn define</* synthetic */ impl Into<Extern>: Into<Extern>>(self: &mut Self, ns: &str, name: &str, val: impl Into<Extern>) { /* ... */ }
  ```
  Add a single import with a namespace `ns` and name `name`.

- ```rust
  pub fn iter(self: &Self) -> ImportsIterator<''_> { /* ... */ }
  ```
  Iterates through all the imports in this structure

##### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Imports { /* ... */ }
    ```

- **Unpin**
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

- **Freeze**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Send**
- **Extend**
  - ```rust
    fn extend<T: IntoIterator<Item = ((String, String), Extern)>>(self: &mut Self, iter: T) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

### Struct `ImportsIterator`

An iterator over imports.

```rust
pub struct ImportsIterator<''a> {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **UnwindSafe**
- **Send**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
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

### Struct `Engine`

**Attributes:**

- `Repr(AttributeRepr { kind: Transparent, align: None, packed: None, int: None })`

The backing engine for a WebAssembly runtime.

```rust
pub struct Engine<E: WasmEngine> {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub fn new(backend: E) -> Self { /* ... */ }
  ```
  Creates a new engine using the specified backend.

- ```rust
  pub fn into_backend(self: Self) -> E { /* ... */ }
  ```
  Unwraps this instance into the core backend engine.

##### Trait Implementations

- **Unpin**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefCast**
  - ```rust
    fn ref_cast(_from: &<Self as >::From) -> &Self { /* ... */ }
    ```

  - ```rust
    fn ref_cast_mut(_from: &mut <Self as >::From) -> &mut Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Engine<E> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

### Struct `Store`

The store represents all global state that can be manipulated by
WebAssembly programs. It consists of the runtime representation
of all instances of functions, tables, memories, and globals that
have been allocated during the lifetime of the abstract machine.

The `Store` holds the engine (that is —amongst many things— used to compile
the Wasm bytes into a valid module artifact).

Spec: <https://webassembly.github.io/spec/core/exec/runtime.html#store>

```rust
pub struct Store<T, E: WasmEngine> {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub fn new(engine: &Engine<E>, data: T) -> Self { /* ... */ }
  ```
  Creates a new [`Store`] with a specific [`Engine`].

- ```rust
  pub fn engine(self: &Self) -> &Engine<E> { /* ... */ }
  ```
  Returns the [`Engine`] that this store is associated with.

- ```rust
  pub fn data(self: &Self) -> &T { /* ... */ }
  ```
  Returns a shared reference to the user provided data owned by this [`Store`].

- ```rust
  pub fn data_mut(self: &mut Self) -> &mut T { /* ... */ }
  ```
  Returns an exclusive reference to the user provided data owned by this [`Store`].

- ```rust
  pub fn into_data(self: Self) -> T { /* ... */ }
  ```
  Consumes `self` and returns its user provided data.

##### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **AsContext**
  - ```rust
    fn as_context(self: &Self) -> <E as WasmEngine>::StoreContext<''_, <C as AsContext<E>>::UserState> { /* ... */ }
    ```

  - ```rust
    fn as_context(self: &Self) -> StoreContext<''_, <Self as >::UserState, <Self as >::Engine> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **AsContextMut**
  - ```rust
    fn as_context_mut(self: &mut Self) -> <E as WasmEngine>::StoreContextMut<''_, <C as AsContext<E>>::UserState> { /* ... */ }
    ```

  - ```rust
    fn as_context_mut(self: &mut Self) -> StoreContextMut<''_, <Self as >::UserState, <Self as >::Engine> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **RefUnwindSafe**
### Struct `StoreContext`

A temporary handle to a [`&Store<T>`][`Store`].

This type is suitable for [`AsContext`] trait bounds on methods if desired.
For more information, see [`Store`].

```rust
pub struct StoreContext<''a, T: ''a, E: WasmEngine> {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub fn engine(self: &Self) -> &Engine<E> { /* ... */ }
  ```
  Returns the underlying [`Engine`] this store is connected to.

- ```rust
  pub fn data(self: &Self) -> &T { /* ... */ }
  ```
  Access the underlying data owned by this store.

##### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **AsContext**
  - ```rust
    fn as_context(self: &Self) -> <E as WasmEngine>::StoreContext<''_, <C as AsContext<E>>::UserState> { /* ... */ }
    ```

  - ```rust
    fn as_context(self: &Self) -> StoreContext<''_, <Self as >::UserState, <Self as >::Engine> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Unpin**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

### Struct `StoreContextMut`

A temporary handle to a [`&mut Store<T>`][`Store`].

This type is suitable for [`AsContextMut`] or [`AsContext`] trait bounds on methods if desired.
For more information, see [`Store`].

```rust
pub struct StoreContextMut<''a, T: ''a, E: WasmEngine> {
    pub inner: <E as >::StoreContextMut<''a, T>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `inner` | `<E as >::StoreContextMut<''a, T>` | The backing implementation. |

#### Implementations

##### Methods

- ```rust
  pub fn engine(self: &Self) -> &Engine<E> { /* ... */ }
  ```
  Returns the underlying [`Engine`] this store is connected to.

- ```rust
  pub fn data(self: &Self) -> &T { /* ... */ }
  ```
  Access the underlying data owned by this store.

- ```rust
  pub fn data_mut(self: &mut Self) -> &mut T { /* ... */ }
  ```
  Access the underlying data owned by this store.

##### Trait Implementations

- **Unpin**
- **AsContextMut**
  - ```rust
    fn as_context_mut(self: &mut Self) -> <E as WasmEngine>::StoreContextMut<''_, <C as AsContext<E>>::UserState> { /* ... */ }
    ```

  - ```rust
    fn as_context_mut(self: &mut Self) -> StoreContextMut<''_, <Self as >::UserState, <Self as >::Engine> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **AsContext**
  - ```rust
    fn as_context(self: &Self) -> <E as WasmEngine>::StoreContext<''_, <C as AsContext<E>>::UserState> { /* ... */ }
    ```

  - ```rust
    fn as_context(self: &Self) -> StoreContext<''_, <Self as >::UserState, <Self as >::Engine> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

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

- **UnwindSafe**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
### Enum `Value`

Runtime representation of a value.

Wasm code manipulate values of the four basic value types:
integers and floating-point data of 32 or 64 bit width each, respectively.

There is no distinction between signed and unsigned integer types. Instead, integers are
interpreted by respective operations as either unsigned or signed in two’s complement representation.

```rust
pub enum Value {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    FuncRef(Option<Func>),
    ExternRef(Option<ExternRef>),
}
```

#### Variants

##### `I32`

Value of 32-bit signed or unsigned integer.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i32` |  |

##### `I64`

Value of 64-bit signed or unsigned integer.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i64` |  |

##### `F32`

Value of 32-bit floating point number.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `f32` |  |

##### `F64`

Value of 64-bit floating point number.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `f64` |  |

##### `FuncRef`

An optional function reference.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<Func>` |  |

##### `ExternRef`

An optional external reference.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<ExternRef>` |  |

#### Implementations

##### Methods

- ```rust
  pub const fn ty(self: &Self) -> ValueType { /* ... */ }
  ```
  Returns the [`ValueType`] for this [`Value`].

##### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Value { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: &Value) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &crate::backend::Value<E>) -> Self { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, o: &Self) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

### Struct `ExternRef`

Represents an opaque reference to any data within WebAssembly.

```rust
pub struct ExternRef {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub fn new<T: ''static + Send + Sync, C: AsContextMut>(ctx: C, object: T) -> Self { /* ... */ }
  ```
  Creates a new [`ExternRef`] wrapping the given value.

- ```rust
  pub fn downcast<''a, ''s: ''a, T: ''static, S: ''s, E: WasmEngine>(self: &''a Self, ctx: StoreContext<''s, S, E>) -> Result<&''a T> { /* ... */ }
  ```
  Returns a shared reference to the underlying data for this [`ExternRef`].

##### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ExternRef { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Send**
### Struct `Func`

A Wasm or host function reference.

```rust
pub struct Func {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub fn new<C: AsContextMut, /* synthetic */ impl 'static + Send + Sync + Fn(StoreContextMut<'_, C::UserState, C::Engine>, &[Value], &mut [Value]) -> Result<()>: ''static + Send + Sync + Fn(StoreContextMut<''_, <C as >::UserState, <C as >::Engine>, &[Value], &mut [Value]) -> Result<()>>(ctx: C, ty: FuncType, func: impl ''static + Send + Sync + Fn(StoreContextMut<''_, <C as >::UserState, <C as >::Engine>, &[Value], &mut [Value]) -> Result<()>) -> Self { /* ... */ }
  ```
  Creates a new [`Func`] with the given arguments.

- ```rust
  pub fn ty<C: AsContext>(self: &Self, ctx: C) -> FuncType { /* ... */ }
  ```
  Returns the function type of the [`Func`].

- ```rust
  pub fn call<C: AsContextMut>(self: &Self, ctx: C, args: &[Value], results: &mut [Value]) -> Result<()> { /* ... */ }
  ```
  Calls the Wasm or host function with the given inputs.

##### Trait Implementations

- **Freeze**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Func { /* ... */ }
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
### Struct `Global`

A Wasm global variable reference.

```rust
pub struct Global {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub fn new<C: AsContextMut>(ctx: C, initial_value: Value, mutable: bool) -> Self { /* ... */ }
  ```
  Creates a new global variable to the store.

- ```rust
  pub fn ty<C: AsContext>(self: &Self, ctx: C) -> GlobalType { /* ... */ }
  ```
  Returns the [`GlobalType`] of the global variable.

- ```rust
  pub fn get<C: AsContextMut>(self: &Self, ctx: C) -> Value { /* ... */ }
  ```
  Returns the current value of the global variable.

- ```rust
  pub fn set<C: AsContextMut>(self: &Self, ctx: C, new_value: Value) -> Result<()> { /* ... */ }
  ```
  Sets a new value to the global variable.

##### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Unpin**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Global { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Struct `Module`

A parsed and validated WebAssembly module.

```rust
pub struct Module {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub fn new<E: WasmEngine>(engine: &Engine<E>, bytes: &[u8]) -> Result<Self> { /* ... */ }
  ```
  Creates a new Wasm [`Module`] from the given byte slice.

- ```rust
  pub fn exports<E: WasmEngine>(self: &Self, engine: &Engine<E>) -> impl ''_ + Iterator<Item = ExportType<''_>> { /* ... */ }
  ```
  Returns an iterator over the exports of the [`Module`].

- ```rust
  pub fn get_export<E: WasmEngine>(self: &Self, engine: &Engine<E>, name: &str) -> Option<ExternType> { /* ... */ }
  ```
  Looks up an export in this [`Module`] by its `name`.

- ```rust
  pub fn imports<E: WasmEngine>(self: &Self, engine: &Engine<E>) -> impl ''_ + Iterator<Item = ImportType<''_>> { /* ... */ }
  ```
  Returns an iterator over the imports of the [`Module`].

##### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Module { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

### Struct `Instance`

An instantiated WebAssembly [`Module`].

This type represents an instantiation of a [`Module`].
It primarily allows to access its [`exports`](Instance::exports)
to call functions, get or set globals, read or write memory, etc.

When interacting with any Wasm code you will want to create an
[`Instance`] in order to execute anything.

```rust
pub struct Instance {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub fn new<C: AsContextMut>(ctx: C, module: &Module, imports: &Imports) -> Result<Self> { /* ... */ }
  ```
  Creates a new [`Instance`] which runs code from the provided module against the given import set.

- ```rust
  pub fn exports<C: AsContext>(self: &Self, ctx: C) -> impl Iterator<Item = Export> { /* ... */ }
  ```
  Returns an iterator over the exports of the [`Instance`].

- ```rust
  pub fn get_export<C: AsContext>(self: &Self, ctx: C, name: &str) -> Option<Extern> { /* ... */ }
  ```
  Returns the value exported to the given `name` if any.

##### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Instance { /* ... */ }
    ```

### Struct `Memory`

A Wasm linear memory reference.

```rust
pub struct Memory {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub fn new<C: AsContextMut>(ctx: C, ty: MemoryType) -> Result<Self> { /* ... */ }
  ```
  Creates a new linear memory to the store.

- ```rust
  pub fn ty<C: AsContext>(self: &Self, ctx: C) -> MemoryType { /* ... */ }
  ```
  Returns the memory type of the linear memory.

- ```rust
  pub fn grow<C: AsContextMut>(self: &Self, ctx: C, additional: u32) -> Result<u32> { /* ... */ }
  ```
  Grows the linear memory by the given amount of new pages.

- ```rust
  pub fn current_pages<C: AsContext>(self: &Self, ctx: C) -> u32 { /* ... */ }
  ```
  Returns the amount of pages in use by the linear memory.

- ```rust
  pub fn read<C: AsContext>(self: &Self, ctx: C, offset: usize, buffer: &mut [u8]) -> Result<()> { /* ... */ }
  ```
  Reads `n` bytes from `memory[offset..offset+n]` into `buffer`

- ```rust
  pub fn write<C: AsContextMut>(self: &Self, ctx: C, offset: usize, buffer: &[u8]) -> Result<()> { /* ... */ }
  ```
  Writes `n` bytes to `memory[offset..offset+n]` from `buffer`

##### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Memory { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
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

- **Freeze**
- **Sync**
### Struct `Table`

A Wasm table reference.

```rust
pub struct Table {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub fn new<C: AsContextMut>(ctx: C, ty: TableType, init: Value) -> Result<Self> { /* ... */ }
  ```
  Creates a new table to the store.

- ```rust
  pub fn ty<C: AsContext>(self: &Self, ctx: C) -> TableType { /* ... */ }
  ```
  Returns the type and limits of the table.

- ```rust
  pub fn size<C: AsContext>(self: &Self, ctx: C) -> u32 { /* ... */ }
  ```
  Returns the current size of the [`Table`].

- ```rust
  pub fn grow<C: AsContextMut>(self: &Self, ctx: C, delta: u32, init: Value) -> Result<u32> { /* ... */ }
  ```
  Grows the table by the given amount of elements.

- ```rust
  pub fn get<C: AsContextMut>(self: &Self, ctx: C, index: u32) -> Option<Value> { /* ... */ }
  ```
  Returns the [`Table`] element value at `index`.

- ```rust
  pub fn set<C: AsContextMut>(self: &Self, ctx: C, index: u32, value: Value) -> Result<()> { /* ... */ }
  ```
  Sets the [`Value`] of this [`Table`] at `index`.

##### Trait Implementations

- **Freeze**
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Table { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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
- **UnwindSafe**
- **Send**
## Traits

### Trait `AsContext`

A trait used to get shared access to a [`Store`].

```rust
pub trait AsContext {
    /* Associated items */
}
```

#### Required Items

##### Associated Types

- `Engine`: The engine type associated with the context.
- `UserState`: The user state associated with the [`Store`], aka the `T` in `Store<T>`.

##### Required Methods

- `as_context`: Returns the store context that this type provides access to.

#### Implementations

This trait is implemented for the following types:

- `Store<T, E>` with <T, E: WasmEngine>
- `&T` with <T: AsContext>
- `&mut T` with <T: AsContext>
- `StoreContext<''a, T, E>` with <''a, T: ''a, E: WasmEngine>
- `StoreContextMut<''a, T, E>` with <''a, T: ''a, E: WasmEngine>

### Trait `AsContextMut`

A trait used to get exclusive access to a [`Store`].

```rust
pub trait AsContextMut: AsContext {
    /* Associated items */
}
```

#### Required Items

##### Required Methods

- `as_context_mut`: Returns the store context that this type provides access to.

#### Implementations

This trait is implemented for the following types:

- `Store<T, E>` with <T, E: WasmEngine>
- `&mut T` with <T: AsContextMut>
- `StoreContextMut<''a, T, E>` with <''a, T: ''a, E: WasmEngine>

