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
  * common types and functions ([witx](witx/proposal_common.witx), [doc](witx/proposal_common.md))
  * symmetric operations ([witx](witx/proposal_siymmetric.witx), [doc](witx/proposal_symmetric.md))
  * common types and functions for asymmetric operations ([witx](witx/proposal_asymmetric_common.witx), [doc](witx/proposal_asymmetric_common.md))
  * signatures ([witx](witx/proposal_signatures.witx), [doc](witx/proposal_signatures.md))
  * key exchange ([witx](witx/proposal_kx.witx), [doc](witx/proposal_kx.md))
  * external secrets ([witx](witx/proposal_external_secrets.witx), [doc](witx/proposal_external_secrets.md))
* [Concise API overview](witx/wasi_ephemeral_crypto.txt)
* [Example AssemblyScript bindings](implementations/bindings/assemblyscript)
* [Example Rust bindings](implementations/bindings/rust)
