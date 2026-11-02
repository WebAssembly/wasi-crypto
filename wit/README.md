# WASI Crypto for WASI 0.2+

This directory contains the WIT API for WASI 0.2+.

## Status

This API is a moving target and is not yet stable.
It may receive breaking changes.

For production use today, use the stable WASI Preview 1 (WASI-core) API in
[`witx/witx-0.10`](../witx/witx-0.10/).
That API is production ready and will not receive breaking changes.

## Interfaces

A host implements `common` plus any subset of the algorithm families it supports.
No family depends on another family or on the secrets manager, so a host can, for example, offer only `symmetric`.

| Interface            | Required?                  | Uses                                      |
| -------------------- | -------------------------- | ----------------------------------------- |
| `common`             | Yes                        | -                                         |
| `symmetric`          | Per family                 | `common`                                  |
| `signatures`         | Per family                 | `common`                                  |
| `kx`                 | Per family                 | `common`                                  |
| `secrets-manager`    | Optional                   | `common`                                  |
| `symmetric-managed`  | Optional                   | `common`, `secrets-manager`, `symmetric`  |
| `signatures-managed` | Optional                   | `common`, `secrets-manager`, `signatures` |
| `kx-managed`         | Optional                   | `common`, `secrets-manager`, `kx`         |
| `external-secrets`   | Optional                   | `common`, `secrets-manager`               |
| `symmetric-batch`    | Optional, not in the world | `common`, `symmetric`                     |
| `signatures-batch`   | Optional, not in the world | `common`, `signatures`                    |

The `imports` world in [`world.wit`](world.wit) imports every interface except the batch ones.

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

| WITX Type / Construct                                              | Target WIT Construct                          | Transpilation Details & Notes                                                                                                                                                            |
| ------------------------------------------------------------------ | --------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **`Pointer + len` (Output buffer)**                                | `list<u8>`                                    | For `symmetric-state.encrypt`, `symmetric-state.encrypt-detached`, `symmetric-state.decrypt`, and `symmetric-state.decrypt-detached`; the ability to encrypt in-place had to be removed. |
| **`ConstPointer + len` (Input buffer)**                            | `list<u8>`                                    | For `symmetric-state.encrypt`, `symmetric-state.encrypt-detached`, `symmetric-state.decrypt`, and `symmetric-state.decrypt-detached`; the ability to encrypt in-place had to be removed. |
| **Version constants**                                              | `variant`                                     | Mostly due to the lack of constants, with a variant being more suitable to display the same information than implicit values.                                                            |
| **`Variant`** without data                                         | `enum`                                        | There is nothing stopping the transpiler from handling this case as well. I just didn't notice and fixing it by hand was easier.                                                         |
| `type ... = handle`                                                | `resource`                                    | This required manually adding `borrow<>` where required.                                                                                                                                 |
| **Imports**                                                        | `use common.{}`, `use signatures.{}`, ...     | This was done manually and verified using:                                                                                                                                               |
| **`array-output.pull` / `symmetric-tag.pull` parameter ownership** | `borrow<T>` (WITX host closes as side-effect) | `T` (owned; Component Model destroys on function return)                                                                                                                                 | In WITX the host explicitly closes the handle inside `pull`, enforcing single-use. WIT `borrow<T>` prevents the host from doing so. Taking ownership restores the invariant. The regression is minimal: WITX allowed retrying `pull` with a larger buffer on `Overflow`, but the WIT version already returns `list<u8>` (host-allocated), making `Overflow` impossible and retries unnecessary. |
| **`$foo_close: func(...) -> result<_, crypto-errno>`**             | *(removed)*                                   | Every `foo-close` function was dropped. `resource` already gives each handle a destructor that the Component Model invokes when a guest drops its handle, so the explicit close functions were redundant. |
| **Modules `wasi_ephemeral_crypto_*`**                              | Interfaces `common`, `symmetric`, `signatures`, `kx`, ... | The package name `wasi:crypto` already namespaces the interfaces, so the prefix was dropped, as in other WASI proposals (`wasi:http/types`, `wasi:keyvalue/store`). The package version was bumped to 0.12.0 because this is a breaking change. |
| **Handle types declared in `common`, functions in other modules**  | Resources declared in the interface that implements them | A WIT resource's methods must be declared in the same interface as the resource, so each resource moved next to its behaviour, as in `wasi:http/types` and `wasi:sockets/types`. `common` keeps only shared types and `array-output`. |
| **Free functions taking a handle as first argument** (`symmetric_state_absorb(state, ...)`) | Methods (`symmetric-state.absorb(...)`) | Constructors became named `static func`s returning `result` (`symmetric-state.open`, `signature-keypair.generate`), as in `wasi:sockets` (`tcp-socket.create`). Functions consuming their handle (`array-output.pull`, `symmetric-tag.pull`) are `static func(this: T)`, as in `wasi:http` (`incoming-body.finish`). |
| **`$keypair`, `$publickey`, `$secretkey` with `$kx_*` / `$signature_*` aliases** | Distinct resources `kx-keypair`, `kx-publickey`, `kx-secretkey` and `signature-keypair`, `signature-publickey`, `signature-secretkey` | A WIT type alias is the same type as its target and can't carry extra methods. Distinct resources give the strong typing asked for in [`design/security.md`](../design/security.md): a key-exchange key can no longer be passed to a signature function. A key never served both families, since its algorithm is fixed at creation. `asymmetric_common` is gone, and its functions exist once per family without the `algorithm_type` parameter. `kx_dh` became `kx-secretkey.dh(pk)`. |
| **`$options` and `options_open(algorithm_type)`**                  | `symmetric-options`, `signature-options`, `kx-options`, `secrets-manager-options` with `constructor()` | Options are "tied to algorithm type" ([`docs/wasi-crypto.md`](../docs/wasi-crypto.md)), which is now enforced by the type system. The secrets manager gets its own options instead of borrowing an algorithm category. `algorithm_type` is no longer needed and was removed. |
| **`$opt_options`**                                                 | *(removed)*                                   | It aliased the generic `options`, which no longer exists. `opt-symmetric-key` moved to `symmetric`. |
| **Optional managed-key functions** (`keypair_generate_managed`, `symmetric_key_id`, ...) | Optional interfaces `symmetric-managed`, `signatures-managed`, `kx-managed`; `secrets-manager` and its options in the optional `secrets-manager` interface | WIT can't mark a function as optional, so optional functionality lives in separate interfaces, as in `wasi:keyvalue` (`atomics`, `batch`). These functions take resources from two interfaces, so they stay free functions. |
| **Module `wasi_ephemeral_crypto_external_secrets`** (`external_secret_store(secrets_manager, ...)`, ...) | Optional interface `external-secrets` with free functions `store`, `replace`, `from-id`, `invalidate`, `encapsulate`, `decapsulate` | These functions only take a `secrets-manager`, so they could be its methods. They stay in their own interface so that external secrets remain optional on their own: a host can offer a secrets manager for managed keys without external secret storage, as `wasi:keyvalue` keeps `atomics` and `batch` apart from `store`. The `external-secret-` prefix was dropped because the interface name already provides it. |
