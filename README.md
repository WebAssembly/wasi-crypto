# WASI Cryptography APIs

This repository is for development of Cryptography API proposals for the
[WASI Subgroup] of the [WebAssembly Community Group].

Please refer to those groups' documentation for more information on their
processes, goals, scope, and deliverables.

[WASI Subgroup]: https://github.com/WebAssembly/WASI
[WebAssembly Community Group]: https://www.w3.org/community/webassembly/

* [High-level goals](docs/HighLevelGoals.md)
* [Security design document](design/security.md)
* [Specification](docs/wasi-crypto.md)
* Interface definitions:
  * common types and functions ([witx](witx/codegen/wasi_ephemeral_crypto_common.witx), [doc](witx/codegen/wasi_ephemeral_crypto_common.md))
  * symmetric operations ([witx](witx/codegen/wasi_ephemeral_crypto_symmetric.witx), [doc](witx/codegen/wasi_ephemeral_crypto_symmetric.md))
  * common types and functions for asymmetric operations ([witx](witx/codegen/wasi_ephemeral_crypto_asymmetric_common.witx), [doc](witx/codegen/wasi_ephemeral_crypto_asymmetric_common.md))
  * signatures ([witx](witx/codegen/wasi_ephemeral_crypto_signatures.witx), [doc](witx/codegen/wasi_ephemeral_crypto_signatures.md))
  * key exchange ([witx](witx/codegen/wasi_ephemeral_crypto_kx.witx), [doc](witx/codegen/wasi_ephemeral_crypto_kx.md))
  * external secrets ([witx](witx/codegen/wasi_ephemeral_crypto_external_secrets.witx), [doc](witx/codegen/wasi_ephemeral_crypto_external_secrets.md))
* [Concise API overview](witx/codegen/wasi_ephemeral_crypto.txt)
* [Implementation for the WasmEdge runtime](https://wasmedge.org/book/en/dev/rust/wasicrypto.html)
* [Example AssemblyScript bindings](implementations/bindings/assemblyscript)
* [Example Rust bindings](implementations/bindings/rust)

Interested parties are welcome to join the working group meeting every 2 weeks on Tuesday.

* [Google Meet link](https://meet.google.com/yeh-kbzo-pfx)
* [Google calendar](https://calendar.google.com/calendar/u/0/embed?src=f67fde02694963243f5dafb70d61c64e715dacbcf1abba17824e672635f90f3e@group.calendar.google.com)
