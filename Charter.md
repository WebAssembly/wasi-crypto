# WebAssembly System Interface Cryptography Subgroup Charter

The System Interface Subgroup is a sub-organization of the
[WebAssembly Community Group](https://www.w3.org/community/webassembly/) of the W3C.
As such, it is intended that its charter align with that of the CG. In particular, 
the sections of the [CG charter](https://webassembly.github.io/cg-charter/) relating to
[Community and Business Group Process](https://webassembly.github.io/cg-charter/#process),
[Contribution Mechanics](https://webassembly.github.io/cg-charter/#contrib),
[Transparency](https://webassembly.github.io/cg-charter/#transparency), and
[Decision Process](https://webassembly.github.io/cg-charter/#decision) also apply to the Subgroup.

## Goals

The mission of this sugbroup is to provide a forum for pre-standardization
collaboration on a cryptography API for WebAssembly programs.

## Scope

The Subgroup will consider topics related to system interface APIs, including:

- APIs for encryption, signatures, key exchange and hashing
- APIs for other transformations
  (particularly where hardware acceleration may be available on some platforms)
- APIs for key management
- APIs for elliptic curve arithmetic and modular arithmetic
- APIs for secure handling of secret material
- Strict definitions of how each API should behave for each algorithm and
  possible parameters, so that interoperability between implementations is
  guaranteed.

## Deliverables

### Specifications
The Subgroup may produce several kinds of specification-related work output:
- Creation of new specifications in standards bodies or working
groups (e.g. Wasm WG or TC39)
- Creation of new specifications outside of standards bodies

### Non-normative reports
The Subgroup may produce non-normative material such as requirements
documents, recommendations, and use cases.

### Software
The Subgroup may produce software related to Wasm system interface APIs (either
as standalone libraries, tooling, or integration of interface-related
functionality in existing CG software such as Binaryen or WABT). Capabilities may
include:
- Libraries implementing external standard APIs in terms of WebAssembly
  System Interface APIs
- Tools for producing code that uses WASI cryptography APIs
- Tools for implementing WebAssembly APIs
- Tools for debugging programs using WASI cryptography APIs
- Test vectors and test suites.

## Amendments to this Charter and Chair Selection

This charter may be amended, and Subgroup Chairs may be selected by vote of the full
WebAssembly Community Group.

