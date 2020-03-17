# Security considerations for the `wasi-crypto` API design

## Misuse resistance

While the `wasi-crypto` provides low-level operations and may not
prevent primitives from being used incorrectly, we should still try to
avoid insecure constructions to be instantiated, and mitigate the
implications of bugs in applications and implementations.

### Strong typing

Operations requiring any kind of key (including public keys) must only
accept the key as a type specifically defined for the given operation
and key type.

Even though these keys can be represented as raw bytes, such a
representation should not be directly accepted by any functions besides
the ones constructing a key object.

For example, a public key used for signature verification must be of
type `signature_publickey`.

This ensures that a key will not inadvertently be used for a different
operation, and enables type verification to be made during
compilation.

Internally, a key also stores the algorithm and parameters it is
designed to be used with. Using a key for the correct operation, but
the wrong algorithm or different parameters must result in a runtime
error.

### No insecure options

Applications instantiate a construction using a string representing
an algorithm as well as its main parameters.

An example is `RSA_PKCS1_2048_8192_SHA384`.

The set of possible algorithm and parameters must be well-defined, so
that insecure constructions cannot be instantiated.

### Bound checking

The linear memory model of WebAssembly and its current lack of support
for protected pages inside a memory region facilitates heartbleed-like
vulnerabilities.

As a result, `wasi-crypto` APIs must not trust applications for
providing correctly sized output buffers.

The following API would assume that `$tag` was properly allocated for
the expected authentication tag:

```text
(@interface func (export "crypto_aead_get_tag")
  (param $handle crypto_aead)
  (param $tag (@witx pointer u8))
  (result $error $crypto_errno)
)
```

However, such an API would be error prone, so the buffer size, as
actually allocated by the application, should be provided:

```text
(@interface func (export "crypto_aead_get_tag")
  (param $handle crypto_aead)
  (param $tag (@witx pointer u8))
  (param $tag_max_len $size)
  (result $error $crypto_errno)
)
```

If `$tag_max_len` is shorter than the expected length, the function
must return a `$crypto_errno.overflow` error code. `$tag` may be
filled with random bytes in order to mitigate the implications of
applications ignoring the error code.

### Side channels

Mitigations against side channels must be implemented for all
operations involving secrets.

In order to do so, the API must try to prevent applications from
performing comparisons themselves.

In particular, a MAC operation should not return raw bytes, but a
MAC object. MAC objects can be compared for equality using a dedicated
function, that will take care of avoiding side channels.

The API should also not leak internal data structures. For example,
there shouldn't be any ways to get the internal state of a cipher.

In the context of WebAssembly, it also means nothing directly derived
from pointers to host memory or external linear memories.

A secret should only be referred to using an opaque handle, and
exporting it should require a dedicated function.

Error codes may end up being sent to untrusted users, and should not
leak information about secrets either (ex: act as a padding oracle).

The same guarantees are expected from the underlying algorithm
implementations. Any application using the `wasi-crypto` module can
thus assume that side channels mitigations have been implemented
for every single operation exposed by the API.

### Nonces / IVs

Using a constant IV is one of the most common mistake made by
applications, and has catastrophic implications. In order to prevent
this, APIs should:

- APIs must not require an IV. Setting the IV should be an optional
operation, distinct from the creation of a cipher object.

- In absence of an explicit IV, the implementation should either
create a safe IV (e.g. by using a CSPRNG if the size is comfortable
enough to avoid collisions), or return an error when the application
tries to use the object to encrypt data.

### Secret zeroing

Keys are exposed as handles, that applications should close after use.

`wasi-crypto` implementations should try to wipe secret key material
from memory, as well as any information derived from it (ex: initial
state of block ciphers).

### Errors

Applications may not always correctly check for error codes. If secret
data should have been returned, but the operation failed, the output
buffer should not remain filled, even partially, with secret data.

For example, if a decryption operation fails, the output buffer
must be zeroed or filled with random data instead of containing
(partially) decrypted data.

### Absence of undefined behaviors

The API specification must explicitly define what happens on edge
cases on a per-algorithm basis.

For example, what happens when the internal counter of a stream/block
cipher wraps must be defined, and behave consistently across all
implementations.

This also includes the way public keys are validated. For example, the
specification may require implementations to prevent the creation of a
public key if the source representation of that key is not in canonical
form.
