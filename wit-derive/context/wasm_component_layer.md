# Crate Documentation

**Version:** 0.1.18

**Format Version:** 54

# Module `wasm_component_layer`

`wasm_component_layer` is a runtime agnostic implementation of the [WebAssembly component model](https://github.com/WebAssembly/component-model).
It supports loading and linking WASM components, inspecting and generating component interface types at runtime, and more atop any WebAssembly backend. The implementation is based upon the [`wasmtime`](https://github.com/bytecodealliance/wasmtime), [`js-component-bindgen`](https://github.com/bytecodealliance/jco), and [`wit-parser`](https://github.com/bytecodealliance/wasm-tools/tree/main) crates.

## Usage

To use `wasm_component_layer`, a runtime is required. The [`wasm_runtime_layer`](https://github.com/DouglasDwyer/wasm_runtime_layer) crate provides the common interface used for WebAssembly runtimes, so when using this crate it must also be added to the `Cargo.toml` file with the appropriate runtime selected. For instance, the examples in this repository use the [`wasmi_runtime_layer`](https://crates.io/crates/wasmi_runtime_layer) runtime:

```toml
wasm_component_layer = "0.1.16"
wasmi_runtime_layer = "0.31.0"
# wasmtime_runtime_layer = "21.0.0"
# js_wasm_runtime_layer = "0.4.0"
```

The following is a small overview of `wasm_component_layer`'s API. The complete example may be found in the [examples folder](/examples). Consider a WASM component with the following WIT:

```wit
package test:guest

interface foo {
    // Selects the item in position n within list x
    select-nth: func(x: list<string>, n: u32) -> string
}

world guest {
    export foo
}
```

The component can be loaded into `wasm_component_layer` and invoked as follows:

```ignore
use wasm_component_layer::*;

// The bytes of the component.
const WASM: &[u8] = include_bytes!("single_component/component.wasm");

pub fn main() {
    // Create a new engine for instantiating a component.
    let engine = Engine::new(wasmi::Engine::default());

    // Create a store for managing WASM data and any custom user-defined state.
    let mut store = Store::new(&engine, ());

    // Parse the component bytes and load its imports and exports.
    let component = Component::new(&engine, WASM).unwrap();
    // Create a linker that will be used to resolve the component's imports, if any.
    let linker = Linker::default();
    // Create an instance of the component using the linker.
    let instance = linker.instantiate(&mut store, &component).unwrap();

    // Get the interface that the interface exports.
    let interface = instance.exports().instance(&"test:guest/foo".try_into().unwrap()).unwrap();
    // Get the function for selecting a list element.
    let select_nth = interface.func("select-nth").unwrap().typed::<(Vec<String>, u32), String>().unwrap();

    // Create an example list to test upon.
    let example = ["a", "b", "c"].iter().map(ToString::to_string).collect::<Vec<_>>();

    println!("Calling select-nth({example:?}, 1) == {}", select_nth.call(&mut store, (example.clone(), 1)).unwrap());
    // Prints 'Calling select-nth(["a", "b", "c"], 1) == b'
}
```

## Features

`wasm_component_layer` supports the following major features:

- Parsing and instantiating WASM component binaries
- Runtime generation of component interface types
- Specialized list types for faster lifting/lowering
- Structural equality of component interface types, as mandated by the spec
- Support for guest resources
- Support for strongly-typed host resources with destructors

The following features have yet to be implemented:

- A macro for generating host bindings
- More comprehensive tests
- Subtyping

## Types

### Struct `Component`

A parsed and validated WebAssembly component, which may be used to instantiate [`Instance`]s.

```rust
pub struct Component(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn new<E: backend::WasmEngine>(engine: &Engine<E>, bytes: &[u8]) -> Result<Self> { /* ... */ }
  ```
  Creates a new component with the given engine and binary data.

- ```rust
  pub fn exports(self: &Self) -> &ComponentTypes { /* ... */ }
  ```
  The types and interfaces exported by this component.

- ```rust
  pub fn imports(self: &Self) -> &ComponentTypes { /* ... */ }
  ```
  The types and interfaces imported by this component. To instantiate

- ```rust
  pub fn package(self: &Self) -> &PackageIdentifier { /* ... */ }
  ```
  The root package of this component.

##### Trait Implementations

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Component { /* ... */ }
    ```

- **Freeze**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **UnwindSafe**
### Struct `ComponentTypes`

Details a set of types within a component.

```rust
pub struct ComponentTypes {
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
  pub fn root(self: &Self) -> &ComponentTypesInstance { /* ... */ }
  ```
  Gets the root instance.

- ```rust
  pub fn instance(self: &Self, name: &InterfaceIdentifier) -> Option<&ComponentTypesInstance> { /* ... */ }
  ```
  Gets the instance with the specified name, if any.

- ```rust
  pub fn instances(self: &Self) -> impl Iterator<Item = (&InterfaceIdentifier, &ComponentTypesInstance)> { /* ... */ }
  ```
  Gets an iterator over all instances by identifier.

##### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Send**
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

- **UnwindSafe**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

### Struct `ComponentTypesInstance`

Represents a specific interface from a component.

```rust
pub struct ComponentTypesInstance {
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
  pub fn func</* synthetic */ impl AsRef<str>: AsRef<str>>(self: &Self, name: impl AsRef<str>) -> Option<crate::types::FuncType> { /* ... */ }
  ```
  Gets the associated function by name, if any.

- ```rust
  pub fn funcs(self: &Self) -> impl Iterator<Item = (&str, crate::types::FuncType)> { /* ... */ }
  ```
  Iterates over all associated functions by name.

- ```rust
  pub fn resource</* synthetic */ impl AsRef<str>: AsRef<str>>(self: &Self, name: impl AsRef<str>) -> Option<ResourceType> { /* ... */ }
  ```
  Gets the associated abstract resource by name, if any.

- ```rust
  pub fn resources(self: &Self) -> impl Iterator<Item = (&str, crate::types::ResourceType)> { /* ... */ }
  ```
  Iterates over all associated functions by name.

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **Send**
- **UnwindSafe**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Struct `Linker`

Provides the ability to define imports for a component and create [`Instance`]s of it.

```rust
pub struct Linker {
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
  pub fn root(self: &Self) -> &LinkerInstance { /* ... */ }
  ```
  Immutably obtains the root interface for this linker.

- ```rust
  pub fn root_mut(self: &mut Self) -> &mut LinkerInstance { /* ... */ }
  ```
  Mutably obtains the root interface for this linker.

- ```rust
  pub fn define_instance(self: &mut Self, name: InterfaceIdentifier) -> Result<&mut LinkerInstance> { /* ... */ }
  ```
  Creates a new instance in the linker with the provided name. Returns an

- ```rust
  pub fn instance(self: &Self, name: &InterfaceIdentifier) -> Option<&LinkerInstance> { /* ... */ }
  ```
  Immutably obtains the instance with the given name, if any.

- ```rust
  pub fn instance_mut(self: &mut Self, name: &InterfaceIdentifier) -> Option<&mut LinkerInstance> { /* ... */ }
  ```
  Mutably obtains the instance with the given name, if any.

- ```rust
  pub fn instances(self: &Self) -> impl ExactSizeIterator<Item = (&InterfaceIdentifier, &LinkerInstance)> { /* ... */ }
  ```
  Gets an immutable iterator over all instances defined in this linker.

- ```rust
  pub fn instances_mut(self: &mut Self) -> impl ExactSizeIterator<Item = (&InterfaceIdentifier, &mut LinkerInstance)> { /* ... */ }
  ```
  Gets a mutable iterator over all instances defined in this linker.

- ```rust
  pub fn instantiate</* synthetic */ impl AsContextMut: AsContextMut>(self: &Self, ctx: impl AsContextMut, component: &Component) -> Result<Instance> { /* ... */ }
  ```
  Instantiates a component for the provided store, filling in its imports with externals

##### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Default**
  - ```rust
    fn default() -> Linker { /* ... */ }
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

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Linker { /* ... */ }
    ```

### Struct `LinkerInstance`

Describes a concrete interface which components may import.

```rust
pub struct LinkerInstance {
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
  pub fn define_func</* synthetic */ impl Into<Arc<str>>: Into<Arc<str>>>(self: &mut Self, name: impl Into<Arc<str>>, func: crate::func::Func) -> Result<()> { /* ... */ }
  ```
  Defines a new function for this interface with the provided name.

- ```rust
  pub fn func</* synthetic */ impl AsRef<str>: AsRef<str>>(self: &Self, name: impl AsRef<str>) -> Option<crate::func::Func> { /* ... */ }
  ```
  Gets the function in this interface with the given name, if any.

- ```rust
  pub fn define_resource</* synthetic */ impl Into<Arc<str>>: Into<Arc<str>>>(self: &mut Self, name: impl Into<Arc<str>>, resource_ty: ResourceType) -> Result<()> { /* ... */ }
  ```
  Defines a new resource type for this interface with the provided name.

- ```rust
  pub fn resource</* synthetic */ impl AsRef<str>: AsRef<str>>(self: &Self, name: impl AsRef<str>) -> Option<ResourceType> { /* ... */ }
  ```
  Gets the resource in this interface with the given name, if any.

- ```rust
  pub fn funcs(self: &Self) -> impl Iterator<Item = (&str, crate::func::Func)> { /* ... */ }
  ```
  Iterates over all associated functions by name.

- ```rust
  pub fn resources(self: &Self) -> impl Iterator<Item = (&str, ResourceType)> { /* ... */ }
  ```
  Iterates over all associated functions by name.

##### Trait Implementations

- **Default**
  - ```rust
    fn default() -> LinkerInstance { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> LinkerInstance { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

### Struct `Instance`

An instantiated WebAssembly component.

```rust
pub struct Instance(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn component(self: &Self) -> &Component { /* ... */ }
  ```
  Gets the component associated with this instance.

- ```rust
  pub fn exports(self: &Self) -> &Exports { /* ... */ }
  ```
  Gets the exports of this instance.

- ```rust
  pub fn drop<T, E: backend::WasmEngine>(self: &Self, ctx: &mut Store<T, E>) -> Result<Vec<Error>> { /* ... */ }
  ```
  Drops the instance and all of its owned resources, removing its data from the given store.

##### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **UnwindSafe**
- **Freeze**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Instance { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

### Struct `Exports`

Provides the exports for an instance.

```rust
pub struct Exports {
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
  pub fn root(self: &Self) -> &ExportInstance { /* ... */ }
  ```
  Gets the root instance.

- ```rust
  pub fn instance(self: &Self, name: &InterfaceIdentifier) -> Option<&ExportInstance> { /* ... */ }
  ```
  Gets the instance with the specified name, if any.

- ```rust
  pub fn instances(self: &Self) -> impl Iterator<Item = (&InterfaceIdentifier, &ExportInstance)> { /* ... */ }
  ```
  Gets an iterator over all instances by identifier.

##### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
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

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Send**
### Struct `ExportInstance`

Represents a specific interface from a instance.

```rust
pub struct ExportInstance {
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
  pub fn func</* synthetic */ impl AsRef<str>: AsRef<str>>(self: &Self, name: impl AsRef<str>) -> Option<crate::func::Func> { /* ... */ }
  ```
  Gets the associated function by name, if any.

- ```rust
  pub fn funcs(self: &Self) -> impl Iterator<Item = (&str, crate::func::Func)> { /* ... */ }
  ```
  Iterates over all associated functions by name.

- ```rust
  pub fn resource</* synthetic */ impl AsRef<str>: AsRef<str>>(self: &Self, name: impl AsRef<str>) -> Option<ResourceType> { /* ... */ }
  ```
  Gets the associated abstract resource by name, if any.

- ```rust
  pub fn resources(self: &Self) -> impl Iterator<Item = (&str, ResourceType)> { /* ... */ }
  ```
  Iterates over all associated functions by name.

##### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **Freeze**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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
pub struct Store<T, E: backend::WasmEngine> {
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

- **Freeze**
- **AsContextMut**
  - ```rust
    fn as_context_mut(self: &mut Self) -> StoreContextMut<''_, <Self as >::UserState, <Self as >::Engine> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **AsContext**
  - ```rust
    fn as_context(self: &Self) -> StoreContext<''_, <Self as >::UserState, <Self as >::Engine> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
### Struct `StoreContext`

A temporary handle to a [`&Store<T>`][`Store`].

This type is suitable for [`AsContext`] trait bounds on methods if desired.
For more information, see [`Store`].

```rust
pub struct StoreContext<''a, T: ''a, E: backend::WasmEngine> {
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

- **Unpin**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **AsContext**
  - ```rust
    fn as_context(self: &Self) -> StoreContext<''_, <Self as >::UserState, <Self as >::Engine> { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
### Struct `StoreContextMut`

A temporary handle to a [`&mut Store<T>`][`Store`].

This type is suitable for [`AsContextMut`] or [`AsContext`] trait bounds on methods if desired.
For more information, see [`Store`].

```rust
pub struct StoreContextMut<''a, T: ''a, E: backend::WasmEngine> {
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

- ```rust
  pub fn data_mut(self: &mut Self) -> &mut T { /* ... */ }
  ```
  Access the underlying data owned by this store.

##### Trait Implementations

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **AsContext**
  - ```rust
    fn as_context(self: &Self) -> StoreContext<''_, <Self as >::UserState, <Self as >::Engine> { /* ... */ }
    ```

- **RefUnwindSafe**
- **AsContextMut**
  - ```rust
    fn as_context_mut(self: &mut Self) -> StoreContextMut<''_, <Self as >::UserState, <Self as >::Engine> { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Sync**
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

- `Store<T, E>` with <T, E: backend::WasmEngine>
- `&T` with <T: AsContext>
- `&mut T` with <T: AsContext>
- `StoreContext<''a, T, E>` with <''a, T: ''a, E: backend::WasmEngine>
- `StoreContextMut<''a, T, E>` with <''a, T: ''a, E: backend::WasmEngine>

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

- `Store<T, E>` with <T, E: backend::WasmEngine>
- `&mut T` with <T: AsContextMut>
- `StoreContextMut<''a, T, E>` with <''a, T: ''a, E: backend::WasmEngine>

## Re-exports

### Re-export `Engine`

```rust
pub use wasm_runtime_layer::Engine;
```

### Re-export `Func`

```rust
pub use crate::func::Func;
```

### Re-export `PackageName`

```rust
pub use crate::identifier::PackageName;
```

### Re-export `FuncType`

```rust
pub use crate::types::FuncType;
```

### Re-export `ValueType`

```rust
pub use crate::types::ValueType;
```

### Re-export `VariantCase`

```rust
pub use crate::types::VariantCase;
```

### Re-export `Enum`

```rust
pub use crate::values::Enum;
```

### Re-export `Flags`

```rust
pub use crate::values::Flags;
```

### Re-export `Record`

```rust
pub use crate::values::Record;
```

### Re-export `Tuple`

```rust
pub use crate::values::Tuple;
```

### Re-export `Value`

```rust
pub use crate::values::Value;
```

### Re-export `Variant`

```rust
pub use crate::values::Variant;
```

### Re-export `crate::func::*`

```rust
pub use crate::func::*;
```

### Re-export `crate::identifier::*`

```rust
pub use crate::identifier::*;
```

### Re-export `crate::types::*`

```rust
pub use crate::types::*;
```

### Re-export `crate::values::*`

```rust
pub use crate::values::*;
```

