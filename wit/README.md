## Made changes

### Automatic

| WITX Type / Construct                   | Target WIT Construct      | Transpilation Details & Notes                                                                                                               |
| --------------------------------------- | ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| **`TypeRef::Name`**                     | `Type::named(...)`        | Map using kebab-case identifier.                                                                                                            |
| **`BuiltinType::Char`**                 | `Type::Char`              | Translated directly.                                                                                                                        |
| **`BuiltinType::U8 / U16 / U32 / U64`** | `Type::U8` to `Type::U64` | Translated directly.                                                                                                                        |
| **`BuiltinType::S8 / S16 / S32 / S64`** | `Type::S8` to `Type::S64` | Translated directly (Signed integer).                                                                                                       |
| **`BuiltinType::F32 / F64`**            | `Type::F32` / `Type::F64` | Translated directly (Floats).                                                                                                               |
| **`Type::List(Type::Char)`**            | `Type::String`            | Special promotion applied (`list<char>` -> `string`).                                                                                       |
| **`Type::List(T)`**                     | `Type::list<T>`           | Mapped to a WIT list.                                                                                                                       |
| **`Type::Variant` (Option style)**      | `Type::option<T>`         | Recognized via `general_as_option`.                                                                                                         |
| **`Type::Variant` (Expected style)**    | `Type::result<T, E>`      | Recognized via `general_as_expected` (handles ok/err variations).                                                                           |
| **`Type::Variant` (Enum style)**        | `TypeDef::variant(...)`   | Custom map to variant cases.                                                                                                                |
| **`Type::Record` (Tuple style)**        | `Type::tuple(...)`        | Evaluated when `record.is_tuple()` is true.                                                                                                 |
| **`Type::Record` (Bitflags)**           | `TypeDef::flags(...)`     | Evaluated when `record.bitflags_repr().is_some()`.                                                                                          |
| **`Type::Record` (Standard)**           | `TypeDef::record(...)`    | Maps fields to a named structure.                                                                                                           |
| **`Type::Handle`**                      | `Type::named(...)`        | Resolves to the name of the resource handle.                                                                                                |
| **`witx_module.constants()`**           | `TypeDef::resource(...)`  | **Workaround**: Grouped by type and modeled as a resource containing static-like `ResourceFunc::method` calls tagged with a `TODO` comment. |
| **`witx_module.resources()`**           | `TypeDef::resource(...)`  | Map to an isolated empty WIT resource type block.                                                                                           |
| **`witx_module.funcs()`**               | `StandaloneFunc`          | Maps parameters and the single return output.                                                                                               |
| **`Type::Pointer`**                     | **Not Supported**         | ❌ Fails with `TranspileError::UnsupportedType`. Raw memory addresses are disallowed; recommend `list` or `resource`.                       |
| **`Type::ConstPointer`**                | **Not Supported**         | ❌ Fails with `TranspileError::UnsupportedType`. Raw memory addresses are disallowed; recommend `list` or `resource`.                       |

### Manual

| WITX Type / Construct                   | Target WIT Construct                  | Transpilation Details & Notes                                                                                                                                                            |
| --------------------------------------- | ------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **`Pointer + len` (Output buffer)**     | `list<u8>`                            | For `symmetric-state-encrypt`, `symmetric-state-encrypt-detached`, `symmetric-state-decrypt`, and `symmetric-state-decrypt-detached`; the ability to encrypt in-place had to be removed. |
| **`ConstPointer + len` (Input buffer)** | `list<u8>`                            | For `symmetric-state-encrypt`, `symmetric-state-encrypt-detached`, `symmetric-state-decrypt`, and `symmetric-state-decrypt-detached`; the ability to encrypt in-place had to be removed. |
| **Version constants**                   | `variant`                             | Mostly due to the lack of constants, with a variant being more suitable to display the same information than implicit values.                                                            |
| **`Variant`** without data              | `enum`                                | There is nothing stopping the transpiler from handling this case as well. I just didn't notice and fixing it by hand was easier.                                                         |
| `type ... = handle`                     | `resource`                            | This required manually adding `borrow<>` where required.                                                                                                                                 |
| **Imports**                             | `use wasi-ephemeral-crypto-common.{}` | This was done manually and verified using:                                                                                                                                               |
| `wasm-tools component wit .`.           |
