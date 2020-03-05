# WASI-crypto High-Level Goals

1. Define portable, modular, runtime-independent, and
WebAssembly-native APIs which can be used by WebAssembly code to
perform cryptographic operations while preserving the sandboxed nature
of WebAssembly.
2. Specify how public and secret keys are generated, verified,
imported, exported, stored, rotated and used. Some of these operations
may not be available. For example, in a HSM-like environment, keys may
not be exportable, or may need further transformations.
3. Define APIs commonly required for handling sensitive material:
constant-time comparison, secret key destruction, constant-time
encoding, common operations on large numbers.
4. Define APIs for generating secure random numbers, encryption, hashing,
password stretching, signatures, key derivation and padding.
5. Define APIs for modular arithmetic and elliptic curve arithmetic.

All these APIs must be designed for interoperability, with no undefined
behaviors.

Being core APIs, they should also be designed for stability. Namely,
these APIs should support current and futures primitives and
constructions without requiring breaking changes.
Current and future constructions include XOFs, session-based
encryption, and post-quantum key exchange and signatures.

## WASI-crypto non-goals

- Implementing high-level protocols (ex: Noise, TLS, PAKEs). However, the
WASI-crypto module should provide all the required pieces in order to
build these protocols.
- ASN.1 parsing or generation.
- Networking.

These can be implemented later on, as additional WASI modules.
