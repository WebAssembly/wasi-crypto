# WASI-crypto High-Level Goals

## Motivations behind WASI-crypto

While optimizing compilers could allow efficient implementation of
cryptographic features in WebAssembly, there are several occasions as
below where a host implementation is more desirable. WASI-crypto aims
to fill those gaps by defining a standard interface as a set of APIs.

1. Hardware acceleration: Modern computing devices are equipped with
dedicated hardware support for major cryptographic primitives, such as
AES and SHA-2. Leveraging those accelerators is important for
performance critical applications.
2. System level secret isolation: Smartcards and Hardware Security
Modules (HSM) are becoming popular as they can properly isolate users'
private keys from the rest of the system or cloud. To operate on those
private keys, applications need to use the standard interface provided
at the system level.
3. Software certification: Governments and enterprise often require
certification of crypto implementations. Notable examples are
FIPS-140, Common Criteria, and PCI-DSS. Usually, such certifications
are given to the common system libraries, as it is impractical to
certify every program.
4. Resilience against side channel attacks: In recent years, several
attacks had been developed by utilizing side channels in the crypto
implementations, such as the use of non constant-time operations and
non-uniform cache access. At WebAssembly level, it is currently not
possible to ensure non-existence of side channels until the final
machine code is generated for the target architecture.

## WASI-crypto goals

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

- Implementing high-level protocols (ex: Noise, TLS, PAKEs, HPKE). However,
the WASI-crypto module should provide all the required pieces in order to
build these protocols.
- ASN.1 parsing or generation.
- Networking.

These can be implemented later on, as additional WASI modules.
