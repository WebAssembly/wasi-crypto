# Types
## <a href="#crypto_errno" name="crypto_errno"></a> `crypto_errno`: Enum(`u16`)
Error codes.

Size: 2

Alignment: 2

### Variants
- <a href="#crypto_errno.success" name="crypto_errno.success"></a> `success`
Operation succeeded.

- <a href="#crypto_errno.guest_error" name="crypto_errno.guest_error"></a> `guest_error`
An error occurred when trying to during a conversion from a host type to a guest type.

Only an internal bug can throw this error.

- <a href="#crypto_errno.not_implemented" name="crypto_errno.not_implemented"></a> `not_implemented`
The requested operation is valid, but not implemented by the host.

- <a href="#crypto_errno.unsupported_feature" name="crypto_errno.unsupported_feature"></a> `unsupported_feature`
The requested feature is not supported by the chosen algorithm.

- <a href="#crypto_errno.prohibited_operation" name="crypto_errno.prohibited_operation"></a> `prohibited_operation`
The requested operation is valid, but was administratively prohibited.

- <a href="#crypto_errno.unsupported_encoding" name="crypto_errno.unsupported_encoding"></a> `unsupported_encoding`
Unsupported encoding for an import or export operation.

- <a href="#crypto_errno.unsupported_algorithm" name="crypto_errno.unsupported_algorithm"></a> `unsupported_algorithm`
The requested algorithm is not supported by the host.

- <a href="#crypto_errno.unsupported_option" name="crypto_errno.unsupported_option"></a> `unsupported_option`
The requested option is not supported by the currently selected algorithm.

- <a href="#crypto_errno.invalid_key" name="crypto_errno.invalid_key"></a> `invalid_key`
An invalid or incompatible key was supplied.

The key may not be valid, or was generated for a different algorithm or parameters set.

- <a href="#crypto_errno.invalid_length" name="crypto_errno.invalid_length"></a> `invalid_length`
The currently selected algorithm doesn't support the requested output length.

This error is thrown by non-extensible hash functions, when requesting an output size larger than they produce out of a single block.

- <a href="#crypto_errno.verification_failed" name="crypto_errno.verification_failed"></a> `verification_failed`
A signature or authentication tag verification failed.

- <a href="#crypto_errno.rng_error" name="crypto_errno.rng_error"></a> `rng_error`
A secure random numbers generator is not available.

The requested operation requires random numbers, but the host cannot securely generate them at the moment.

- <a href="#crypto_errno.algorithm_failure" name="crypto_errno.algorithm_failure"></a> `algorithm_failure`
An error was returned by the underlying cryptography library.

The host may be running out of memory, parameters may be incompatible with the chosen implementation of an algorithm or another unexpected error may have happened.

Ideally, the specification should provide enough details and guidance to make this error impossible to ever be thrown.

Realistically, the WASI crypto module cannot possibly cover all possible error types implementations can return, especially since some of these may be language-specific.
This error can thus be thrown when other error types are not suitable, and when the original error comes from the cryptographic primitives themselves and not from the WASI module.

- <a href="#crypto_errno.invalid_signature" name="crypto_errno.invalid_signature"></a> `invalid_signature`
The supplied signature is invalid, or incompatible with the chosen algorithm.

- <a href="#crypto_errno.closed" name="crypto_errno.closed"></a> `closed`
An attempt was made to close a handle that was already closed.

- <a href="#crypto_errno.invalid_handle" name="crypto_errno.invalid_handle"></a> `invalid_handle`
A function was called with an unassigned handle, a closed handle, or handle of an unexpected type.

- <a href="#crypto_errno.overflow" name="crypto_errno.overflow"></a> `overflow`
The host needs to copy data to a guest-allocated buffer, but that buffer is too small.

- <a href="#crypto_errno.internal_error" name="crypto_errno.internal_error"></a> `internal_error`
An internal error occurred.

This error is reserved to internal consistency checks, and must only be sent if the internal state of the host remains safe after an inconsistency was detected.

- <a href="#crypto_errno.too_many_handles" name="crypto_errno.too_many_handles"></a> `too_many_handles`
Too many handles are currently open, and a new one cannot be created.

Implementations are free to represent handles as they want, and to enforce limits to limit resources usage.

- <a href="#crypto_errno.key_not_supported" name="crypto_errno.key_not_supported"></a> `key_not_supported`
A key was provided, but the chosen algorithm doesn't support keys.

This is returned by symmetric operations.

Many hash functions, in particular, do not support keys without being used in particular constructions.
Blindly ignoring a key provided by mistake while trying to open a context for such as function could cause serious security vulnerabilities.

These functions must refuse to create the context and return this error instead.

- <a href="#crypto_errno.key_required" name="crypto_errno.key_required"></a> `key_required`
A key is required for the chosen algorithm, but none was given.

- <a href="#crypto_errno.invalid_tag" name="crypto_errno.invalid_tag"></a> `invalid_tag`
The provided authentication tag is invalid or incompatible with the current algorithm.

This error is returned by decryption functions and tag verification functions.

Unlike `verification_failed`, this error code is returned when the tag cannot possibly verify for any input.

- <a href="#crypto_errno.invalid_operation" name="crypto_errno.invalid_operation"></a> `invalid_operation`
The requested operation is incompatible with the current scheme.

For example, the `symmetric_state_encrypt()` function cannot complete if the selected construction is a key derivation function.
This error code will be returned instead.

- <a href="#crypto_errno.nonce_required" name="crypto_errno.nonce_required"></a> `nonce_required`
A nonce is required.

Most encryption schemes require a nonce.

In the absence of a nonce, the WASI cryptography module can automatically generate one, if that can be done safely. The nonce can be retrieved later with the `symmetric_state_option_get()` function using the `nonce` parameter.
If automatically generating a nonce cannot be done safely, the module never falls back to an insecure option and requests an explicit nonce by throwing that error.

- <a href="#crypto_errno.option_not_set" name="crypto_errno.option_not_set"></a> `option_not_set`
The named option was not set.

The caller tried to read the value of an option that was not set.
This error is used to make the distinction between an empty option, and an option that was not set and left to its default value.

- <a href="#crypto_errno.key_not_found" name="crypto_errno.key_not_found"></a> `key_not_found`
A key or key pair matching the requested identifier cannot be found using the supplied information.

This error is returned by a key manager via the `keypair_from_id()` function.

- <a href="#crypto_errno.parameters_missing" name="crypto_errno.parameters_missing"></a> `parameters_missing`
The algorithm requires parameters that haven't been set.

Non-generic options are required and must be given by building an [`options`](#options) set and giving that object to functions instantiating that algorithm.

- <a href="#crypto_errno.in_progress" name="crypto_errno.in_progress"></a> `in_progress`
A requested computation is not done yet, and additional calls to the function are required.

Some functions, such as functions generating key pairs and password stretching functions, can take a long time to complete.

In order to avoid a host call to be blocked for too long, these functions can return prematurely, requiring additional calls with the same parameters until they complete.

- <a href="#crypto_errno.incompatible_keys" name="crypto_errno.incompatible_keys"></a> `incompatible_keys`
Multiple keys have been provided, but they do not share the same type.

This error is returned when trying to build a key pair from a public key and a secret key that were created for different and incompatible algorithms.

## <a href="#keypair_encoding" name="keypair_encoding"></a> `keypair_encoding`: Enum(`u16`)
Encoding to use for importing or exporting a key pair.

Size: 2

Alignment: 2

### Variants
- <a href="#keypair_encoding.raw" name="keypair_encoding.raw"></a> `raw`
Raw bytes.

- <a href="#keypair_encoding.pkcs8" name="keypair_encoding.pkcs8"></a> `pkcs8`
PCSK8 encoding.

- <a href="#keypair_encoding.der" name="keypair_encoding.der"></a> `der`
DER encoding.

- <a href="#keypair_encoding.pem" name="keypair_encoding.pem"></a> `pem`
PEM encoding.

## <a href="#publickey_encoding" name="publickey_encoding"></a> `publickey_encoding`: Enum(`u16`)
Encoding to use for importing or exporting a public key.

Size: 2

Alignment: 2

### Variants
- <a href="#publickey_encoding.raw" name="publickey_encoding.raw"></a> `raw`
Raw bytes.

- <a href="#publickey_encoding.der" name="publickey_encoding.der"></a> `der`
DER encoding.

- <a href="#publickey_encoding.pem" name="publickey_encoding.pem"></a> `pem`
PEM encoding.

- <a href="#publickey_encoding.sec" name="publickey_encoding.sec"></a> `sec`
SEC encoding.

- <a href="#publickey_encoding.compressed_sec" name="publickey_encoding.compressed_sec"></a> `compressed_sec`
Compressed SEC encoding.

## <a href="#secretkey_encoding" name="secretkey_encoding"></a> `secretkey_encoding`: Enum(`u16`)
Encoding to use for importing or exporting a secret key.

Size: 2

Alignment: 2

### Variants
- <a href="#secretkey_encoding.raw" name="secretkey_encoding.raw"></a> `raw`
Raw bytes.

- <a href="#secretkey_encoding.der" name="secretkey_encoding.der"></a> `der`
DER encoding.

- <a href="#secretkey_encoding.pem" name="secretkey_encoding.pem"></a> `pem`
PEM encoding.

- <a href="#secretkey_encoding.sec" name="secretkey_encoding.sec"></a> `sec`
SEC encoding.

- <a href="#secretkey_encoding.compressed_sec" name="secretkey_encoding.compressed_sec"></a> `compressed_sec`
Compressed SEC encoding.

## <a href="#signature_encoding" name="signature_encoding"></a> `signature_encoding`: Enum(`u16`)
Encoding to use for importing or exporting a signature.

Size: 2

Alignment: 2

### Variants
- <a href="#signature_encoding.raw" name="signature_encoding.raw"></a> `raw`
Raw bytes.

- <a href="#signature_encoding.der" name="signature_encoding.der"></a> `der`
DER encoding.

## <a href="#algorithm_type" name="algorithm_type"></a> `algorithm_type`: Enum(`u16`)
An algorithm category.

Size: 2

Alignment: 2

### Variants
- <a href="#algorithm_type.signatures" name="algorithm_type.signatures"></a> `signatures`

- <a href="#algorithm_type.symmetric" name="algorithm_type.symmetric"></a> `symmetric`

## <a href="#version" name="version"></a> `version`: Int(`u64`)
Version of a managed key.

A version can be an arbitrary `u64` integer, with the expection of some reserved values.

Size: 8

Alignment: 8

### Consts
- <a href="#version.unspecified" name="version.unspecified"></a> `unspecified`
Key doesn't support versioning.

- <a href="#version.latest" name="version.latest"></a> `latest`
Use the latest version of a key.

- <a href="#version.all" name="version.all"></a> `all`
Perform an operation over all versions of a key.

## <a href="#size" name="size"></a> `size`: `usize`
Size of a value.

Size: 4

Alignment: 4

## <a href="#array_output" name="array_output"></a> `array_output`
Handle for functions returning output whose size may be large or not known in advance.

An [`array_output`](#array_output) object contains a host-allocated byte array.

A guest can get the size of that array after a function returns in order to then allocate a buffer of the correct size.
In addition, the content of such an object can be consumed by a guest in a streaming fashion.

An [`array_output`](#array_output) handle is automatically closed after its full content has been consumed.

Size: 4

Alignment: 4

### Supertypes
## <a href="#options" name="options"></a> `options`
A set of options.

This type is used to set non-default parameters.

The exact set of allowed options depends on the algorithm being used.

Size: 4

Alignment: 4

### Supertypes
## <a href="#key_manager" name="key_manager"></a> `key_manager`
A handle to the optional key management facilities offered by a host.

This is used to generate, retrieve and invalidate managed keys.

Size: 4

Alignment: 4

### Supertypes
## <a href="#keypair" name="keypair"></a> `keypair`
A key pair.

Size: 4

Alignment: 4

### Supertypes
## <a href="#signature_state" name="signature_state"></a> `signature_state`
A state to absorb data to be signed.

After a signature has been computed or verified, the state remains valid for further operations.

A subsequent signature would sign all the data accumulated since the creation of the state object.

Size: 4

Alignment: 4

### Supertypes
## <a href="#signature" name="signature"></a> `signature`
A signature.

Size: 4

Alignment: 4

### Supertypes
## <a href="#publickey" name="publickey"></a> `publickey`
A public key, for key exchange and signature verification.

Size: 4

Alignment: 4

### Supertypes
## <a href="#secretkey" name="secretkey"></a> `secretkey`
A secret key, for key exchange mechanisms.

Size: 4

Alignment: 4

### Supertypes
## <a href="#signature_verification_state" name="signature_verification_state"></a> `signature_verification_state`
A state to absorb signed data to be verified.

Size: 4

Alignment: 4

### Supertypes
## <a href="#symmetric_state" name="symmetric_state"></a> `symmetric_state`
A state to perform symmetric operations.

The state is not reset nor invalidated after an option has been performed.
Incremental updates and sessions are thus supported.

Size: 4

Alignment: 4

### Supertypes
## <a href="#symmetric_key" name="symmetric_key"></a> `symmetric_key`
A symmetric key.

The key can be imported from raw bytes, or can be a reference to a managed key.

If it was imported, the host will wipe it from memory as soon as the handle is closed.

Size: 4

Alignment: 4

### Supertypes
## <a href="#symmetric_tag" name="symmetric_tag"></a> `symmetric_tag`
An authentication tag.

This is an object returned by functions computing authentication tags.

A tag can be compared against another tag (directly supplied as raw bytes) in constant time with the `symmetric_tag_verify()` function.

This object type can't be directly created from raw bytes. They are only returned by functions computing MACs.

The host is reponsible for securely wiping them from memory on close.

Size: 4

Alignment: 4

### Supertypes
## <a href="#opt_options_u" name="opt_options_u"></a> `opt_options_u`: Enum(`u8`)
Options index, only required by the Interface Types translation layer.

Size: 1

Alignment: 1

### Variants
- <a href="#opt_options_u.some" name="opt_options_u.some"></a> `some`

- <a href="#opt_options_u.none" name="opt_options_u.none"></a> `none`

## <a href="#opt_options" name="opt_options"></a> `opt_options`: Union
An optional options set.

This union simulates an `Option<Options>` type to make the [`options`](#options) parameter of some functions optional.

Size: 8

Alignment: 4

### Union Layout
- tag_size: 1
- tag_align: 1
- contents_offset: 4
- contents_size: 4
- contents_align: 4
### Union variants
- <a href="#opt_options.some" name="opt_options.some"></a> `some`: [`options`](#options)

- <a href="#opt_options.none" name="opt_options.none"></a> `none`

## <a href="#opt_symmetric_key_u" name="opt_symmetric_key_u"></a> `opt_symmetric_key_u`: Enum(`u8`)
Symmetric key index, only required by the Interface Types translation layer.

Size: 1

Alignment: 1

### Variants
- <a href="#opt_symmetric_key_u.some" name="opt_symmetric_key_u.some"></a> `some`

- <a href="#opt_symmetric_key_u.none" name="opt_symmetric_key_u.none"></a> `none`

## <a href="#opt_symmetric_key" name="opt_symmetric_key"></a> `opt_symmetric_key`: Union
An optional symmetric key.

This union simulates an `Option<SymmetricKey>` type to make the [`symmetric_key`](#symmetric_key) parameter of some functions optional.

Size: 8

Alignment: 4

### Union Layout
- tag_size: 1
- tag_align: 1
- contents_offset: 4
- contents_size: 4
- contents_align: 4
### Union variants
- <a href="#opt_symmetric_key.some" name="opt_symmetric_key.some"></a> `some`: [`symmetric_key`](#symmetric_key)

- <a href="#opt_symmetric_key.none" name="opt_symmetric_key.none"></a> `none`

## <a href="#signature_keypair" name="signature_keypair"></a> `signature_keypair`: [`keypair`](#keypair)
`$signature_keypair` is just an alias for `$keypair`

However, bindings may want to define a specialized type [`signature_keypair`](#signature_keypair) as a super class of [`keypair`](#keypair), with additional methods such as `sign`.

Size: 4

Alignment: 4

## <a href="#signature_publickey" name="signature_publickey"></a> `signature_publickey`: [`publickey`](#publickey)
`$signature_publickey` is just an alias for `$publickey`

However, bindings may want to define a specialized type [`signature_publickey`](#signature_publickey) as a super class of [`publickey`](#publickey), with additional methods such as `verify`.

Size: 4

Alignment: 4

# Modules
## <a href="#wasi_ephemeral_crypto_common" name="wasi_ephemeral_crypto_common"></a> wasi_ephemeral_crypto_common
### Imports
#### Memory
### Functions

---

#### <a href="#options_open" name="options_open"></a> `options_open(algorithm_type: algorithm_type) -> (crypto_errno, options)`
Create a new object to set non-default options.

Example usage:

```rust
let options_handle = options_open(AlgorithmType::Symmetric)?;
options_set(options_handle, "context", context)?;
options_set_u64(options_handle, "threads", 4)?;
let state = symmetric_state_open("BLAKE3", None, Some(options_handle))?;
options_close(options_handle)?;
```

##### Params
- <a href="#options_open.algorithm_type" name="options_open.algorithm_type"></a> `algorithm_type`: [`algorithm_type`](#algorithm_type)

##### Results
- <a href="#options_open.error" name="options_open.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#options_open.handle" name="options_open.handle"></a> `handle`: [`options`](#options)


---

#### <a href="#options_close" name="options_close"></a> `options_close(handle: options) -> crypto_errno`
Destroy an options object.

Objects are reference counted. It is safe to close an object immediately after the last function needing it is called.

##### Params
- <a href="#options_close.handle" name="options_close.handle"></a> `handle`: [`options`](#options)

##### Results
- <a href="#options_close.error" name="options_close.error"></a> `error`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#options_set" name="options_set"></a> `options_set(handle: options, name: string, value: ConstPointer<u8>, value_len: size) -> crypto_errno`
Set or update an option.

This is used to set algorithm-specific parameters, but also to provide credentials for the key management facilities, if required.

This function may return `unsupported_option` if an option that doesn't exist for any implemented algorithms is specified.

##### Params
- <a href="#options_set.handle" name="options_set.handle"></a> `handle`: [`options`](#options)

- <a href="#options_set.name" name="options_set.name"></a> `name`: `string`

- <a href="#options_set.value" name="options_set.value"></a> `value`: `ConstPointer<u8>`

- <a href="#options_set.value_len" name="options_set.value_len"></a> `value_len`: [`size`](#size)

##### Results
- <a href="#options_set.error" name="options_set.error"></a> `error`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#options_set_u64" name="options_set_u64"></a> `options_set_u64(handle: options, name: string, value: u64) -> crypto_errno`
Set or update an integer option.

This is used to set algorithm-specific parameters.

This function may return `unsupported_option` if an option that doesn't exist for any implemented algorithms is specified.

##### Params
- <a href="#options_set_u64.handle" name="options_set_u64.handle"></a> `handle`: [`options`](#options)

- <a href="#options_set_u64.name" name="options_set_u64.name"></a> `name`: `string`

- <a href="#options_set_u64.value" name="options_set_u64.value"></a> `value`: `u64`

##### Results
- <a href="#options_set_u64.error" name="options_set_u64.error"></a> `error`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#options_set_guest_buffer" name="options_set_guest_buffer"></a> `options_set_guest_buffer(handle: options, name: string, buffer: Pointer<u8>, buffer_len: size) -> crypto_errno`
Set or update a guest-allocated memory that the host can use or return data into.

This is for example used to set the scratch buffer required by memory-hard functions.

This function may return `unsupported_option` if an option that doesn't exist for any implemented algorithms is specified.

##### Params
- <a href="#options_set_guest_buffer.handle" name="options_set_guest_buffer.handle"></a> `handle`: [`options`](#options)

- <a href="#options_set_guest_buffer.name" name="options_set_guest_buffer.name"></a> `name`: `string`

- <a href="#options_set_guest_buffer.buffer" name="options_set_guest_buffer.buffer"></a> `buffer`: `Pointer<u8>`

- <a href="#options_set_guest_buffer.buffer_len" name="options_set_guest_buffer.buffer_len"></a> `buffer_len`: [`size`](#size)

##### Results
- <a href="#options_set_guest_buffer.error" name="options_set_guest_buffer.error"></a> `error`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#array_output_len" name="array_output_len"></a> `array_output_len(array_output: array_output) -> (crypto_errno, size)`
Return the length of an [`array_output`](#array_output) object.

This allows a guest to allocate a buffer of the correct size in order to copy the output of a function returning this object type.

##### Params
- <a href="#array_output_len.array_output" name="array_output_len.array_output"></a> `array_output`: [`array_output`](#array_output)

##### Results
- <a href="#array_output_len.error" name="array_output_len.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#array_output_len.len" name="array_output_len.len"></a> `len`: [`size`](#size)


---

#### <a href="#array_output_pull" name="array_output_pull"></a> `array_output_pull(array_output: array_output, buf: Pointer<u8>, buf_len: size) -> (crypto_errno, size)`
Copy the content of an [`array_output`](#array_output) object into an application-allocated buffer.

Multiple calls to that function can be made in order to consume the data in a streaming fashion, if necessary.

The function returns the number of bytes that were actually copied. `0` means that the end of the stream has been reached. The total size always matches the output of `array_output_len()`.

The handle is automatically closed after all the data has been consumed.

Example usage:

```rust
let len = array_output_len(output_handle)?;
let mut out = vec![0u8; len];
array_output_pull(output_handle, &mut out)?;
```

##### Params
- <a href="#array_output_pull.array_output" name="array_output_pull.array_output"></a> `array_output`: [`array_output`](#array_output)

- <a href="#array_output_pull.buf" name="array_output_pull.buf"></a> `buf`: `Pointer<u8>`

- <a href="#array_output_pull.buf_len" name="array_output_pull.buf_len"></a> `buf_len`: [`size`](#size)

##### Results
- <a href="#array_output_pull.error" name="array_output_pull.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#array_output_pull.len" name="array_output_pull.len"></a> `len`: [`size`](#size)


---

#### <a href="#key_manager_open" name="key_manager_open"></a> `key_manager_open(options: opt_options) -> (crypto_errno, key_manager)`
__(optional)__
Create a context to use a key manager.

The set of required and supported options is defined by the host.

The function returns the `unsupported_feature` error code if key management facilities are not supported by the host.
This is also an optional import, meaning that the function may not even exist.

##### Params
- <a href="#key_manager_open.options" name="key_manager_open.options"></a> `options`: [`opt_options`](#opt_options)

##### Results
- <a href="#key_manager_open.error" name="key_manager_open.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#key_manager_open.handle" name="key_manager_open.handle"></a> `handle`: [`key_manager`](#key_manager)


---

#### <a href="#key_manager_close" name="key_manager_close"></a> `key_manager_close(key_manager: key_manager) -> crypto_errno`
__(optional)__
Destroy a key manager context.

The function returns the `unsupported_feature` error code if key management facilities are not supported by the host.
This is also an optional import, meaning that the function may not even exist.

##### Params
- <a href="#key_manager_close.key_manager" name="key_manager_close.key_manager"></a> `key_manager`: [`key_manager`](#key_manager)

##### Results
- <a href="#key_manager_close.error" name="key_manager_close.error"></a> `error`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#key_manager_invalidate" name="key_manager_invalidate"></a> `key_manager_invalidate(key_manager: key_manager, key_id: ConstPointer<u8>, key_id_len: size, key_version: version) -> crypto_errno`
__(optional)__
Invalidate a managed key or key pair given an identifier and a version.

This asks the key manager to delete or revoke a stored key, a specific version of a key..

`key_version` can be set to a version number, to [`version.latest`](#version.latest) to invalidate the current version, or to [`version.all`](#version.all) to invalidate all versions of a key.

The function returns `unsupported_feature` if this operation is not supported by the host, and `key_not_found` if the identifier and version don't match any existing key.

This is an optional import, meaning that the function may not even exist.

##### Params
- <a href="#key_manager_invalidate.key_manager" name="key_manager_invalidate.key_manager"></a> `key_manager`: [`key_manager`](#key_manager)

- <a href="#key_manager_invalidate.key_id" name="key_manager_invalidate.key_id"></a> `key_id`: `ConstPointer<u8>`

- <a href="#key_manager_invalidate.key_id_len" name="key_manager_invalidate.key_id_len"></a> `key_id_len`: [`size`](#size)

- <a href="#key_manager_invalidate.key_version" name="key_manager_invalidate.key_version"></a> `key_version`: [`version`](#version)

##### Results
- <a href="#key_manager_invalidate.error" name="key_manager_invalidate.error"></a> `error`: [`crypto_errno`](#crypto_errno)

## <a href="#wasi_ephemeral_crypto_asymmetric_common" name="wasi_ephemeral_crypto_asymmetric_common"></a> wasi_ephemeral_crypto_asymmetric_common
### Imports
#### Memory
### Functions

---

#### <a href="#keypair_generate" name="keypair_generate"></a> `keypair_generate(algorithm_type: algorithm_type, algorithm: string, options: opt_options) -> (crypto_errno, keypair)`
Generate a new key pair.

Internally, a key pair stores the supplied algorithm and optional parameters.

Trying to use that key pair with different parameters will throw an `invalid_key` error.

This function may return `$crypto_errno.unsupported_feature` if key generation is not supported by the host for the chosen algorithm.

The function may also return `unsupported_algorithm` if the algorithm is not supported by the host.

Finally, if generating that type of key pair is an expensive operation, the function may return `in_progress`.
In that case, the guest should retry with the same parameters until the function completes.

Example usage:

```rust
let kp_handle = ctx.keypair_generate(AlgorithmType::Signatures, "RSA_PKCS1_2048_8192_SHA512", None)?;
```

##### Params
- <a href="#keypair_generate.algorithm_type" name="keypair_generate.algorithm_type"></a> `algorithm_type`: [`algorithm_type`](#algorithm_type)

- <a href="#keypair_generate.algorithm" name="keypair_generate.algorithm"></a> `algorithm`: `string`

- <a href="#keypair_generate.options" name="keypair_generate.options"></a> `options`: [`opt_options`](#opt_options)

##### Results
- <a href="#keypair_generate.error" name="keypair_generate.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#keypair_generate.handle" name="keypair_generate.handle"></a> `handle`: [`keypair`](#keypair)


---

#### <a href="#keypair_import" name="keypair_import"></a> `keypair_import(algorithm_type: algorithm_type, algorithm: string, encoded: ConstPointer<u8>, encoded_len: size, encoding: keypair_encoding) -> (crypto_errno, keypair)`
Import a key pair.

This function creates a [`keypair`](#keypair) object from existing material.

It may return `unsupported_algorithm` if the encoding scheme is not supported, or `invalid_key` if the key cannot be decoded.

The function may also return `unsupported_algorithm` if the algorithm is not supported by the host.

Example usage:

```rust
let kp_handle = ctx.keypair_import(AlgorithmType::Signatures, "RSA_PKCS1_2048_8192_SHA512", KeypairEncoding::PKCS8)?;
```

##### Params
- <a href="#keypair_import.algorithm_type" name="keypair_import.algorithm_type"></a> `algorithm_type`: [`algorithm_type`](#algorithm_type)

- <a href="#keypair_import.algorithm" name="keypair_import.algorithm"></a> `algorithm`: `string`

- <a href="#keypair_import.encoded" name="keypair_import.encoded"></a> `encoded`: `ConstPointer<u8>`

- <a href="#keypair_import.encoded_len" name="keypair_import.encoded_len"></a> `encoded_len`: [`size`](#size)

- <a href="#keypair_import.encoding" name="keypair_import.encoding"></a> `encoding`: [`keypair_encoding`](#keypair_encoding)

##### Results
- <a href="#keypair_import.error" name="keypair_import.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#keypair_import.handle" name="keypair_import.handle"></a> `handle`: [`keypair`](#keypair)


---

#### <a href="#keypair_generate_managed" name="keypair_generate_managed"></a> `keypair_generate_managed(key_manager: key_manager, algorithm_type: algorithm_type, algorithm: string, options: opt_options) -> (crypto_errno, keypair)`
__(optional)__
Generate a new managed key pair.

The key pair is generated and stored by the key management facilities.

It may be used through its identifier, but the host may not allow it to be exported.

The function returns the `unsupported_feature` error code if key management facilities are not supported by the host,
or `unsupported_algorithm` if a key cannot be created for the chosen algorithm.

The function may also return `unsupported_algorithm` if the algorithm is not supported by the host.

This is also an optional import, meaning that the function may not even exist.

##### Params
- <a href="#keypair_generate_managed.key_manager" name="keypair_generate_managed.key_manager"></a> `key_manager`: [`key_manager`](#key_manager)

- <a href="#keypair_generate_managed.algorithm_type" name="keypair_generate_managed.algorithm_type"></a> `algorithm_type`: [`algorithm_type`](#algorithm_type)

- <a href="#keypair_generate_managed.algorithm" name="keypair_generate_managed.algorithm"></a> `algorithm`: `string`

- <a href="#keypair_generate_managed.options" name="keypair_generate_managed.options"></a> `options`: [`opt_options`](#opt_options)

##### Results
- <a href="#keypair_generate_managed.error" name="keypair_generate_managed.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#keypair_generate_managed.handle" name="keypair_generate_managed.handle"></a> `handle`: [`keypair`](#keypair)


---

#### <a href="#keypair_id" name="keypair_id"></a> `keypair_id(kp: keypair, kp_id: Pointer<u8>, kp_id_max_len: size) -> (crypto_errno, size, version)`
__(optional)__
Return the key pair identifier and version of a managed key pair.

If the key pair is not managed, `unsupported_feature` is returned instead.

This is an optional import, meaning that the function may not even exist.

##### Params
- <a href="#keypair_id.kp" name="keypair_id.kp"></a> `kp`: [`keypair`](#keypair)

- <a href="#keypair_id.kp_id" name="keypair_id.kp_id"></a> `kp_id`: `Pointer<u8>`

- <a href="#keypair_id.kp_id_max_len" name="keypair_id.kp_id_max_len"></a> `kp_id_max_len`: [`size`](#size)

##### Results
- <a href="#keypair_id.error" name="keypair_id.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#keypair_id.kp_id_len" name="keypair_id.kp_id_len"></a> `kp_id_len`: [`size`](#size)

- <a href="#keypair_id.version" name="keypair_id.version"></a> `version`: [`version`](#version)


---

#### <a href="#keypair_from_id" name="keypair_from_id"></a> `keypair_from_id(key_manager: key_manager, kp_id: ConstPointer<u8>, kp_id_len: size, kp_version: version) -> (crypto_errno, keypair)`
__(optional)__
Return a managed key pair from a key identifier.

`kp_version` can be set to `version_latest` to retrieve the most recent version of a key pair.

If no key pair matching the provided information is found, `key_not_found` is returned instead.

This is an optional import, meaning that the function may not even exist.
``
##### Params
- <a href="#keypair_from_id.key_manager" name="keypair_from_id.key_manager"></a> `key_manager`: [`key_manager`](#key_manager)

- <a href="#keypair_from_id.kp_id" name="keypair_from_id.kp_id"></a> `kp_id`: `ConstPointer<u8>`

- <a href="#keypair_from_id.kp_id_len" name="keypair_from_id.kp_id_len"></a> `kp_id_len`: [`size`](#size)

- <a href="#keypair_from_id.kp_version" name="keypair_from_id.kp_version"></a> `kp_version`: [`version`](#version)

##### Results
- <a href="#keypair_from_id.error" name="keypair_from_id.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#keypair_from_id.handle" name="keypair_from_id.handle"></a> `handle`: [`keypair`](#keypair)


---

#### <a href="#keypair_from_pk_and_sk" name="keypair_from_pk_and_sk"></a> `keypair_from_pk_and_sk(publickey: publickey, secretkey: secretkey) -> (crypto_errno, keypair)`
Create a key pair from a public key and a secret key.

##### Params
- <a href="#keypair_from_pk_and_sk.publickey" name="keypair_from_pk_and_sk.publickey"></a> `publickey`: [`publickey`](#publickey)

- <a href="#keypair_from_pk_and_sk.secretkey" name="keypair_from_pk_and_sk.secretkey"></a> `secretkey`: [`secretkey`](#secretkey)

##### Results
- <a href="#keypair_from_pk_and_sk.error" name="keypair_from_pk_and_sk.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#keypair_from_pk_and_sk.handle" name="keypair_from_pk_and_sk.handle"></a> `handle`: [`keypair`](#keypair)


---

#### <a href="#keypair_export" name="keypair_export"></a> `keypair_export(kp: keypair, encoding: keypair_encoding) -> (crypto_errno, array_output)`
Export a key pair as the given encoding format.

May return `prohibited_operation` if this operation is denied or `unsupported_encoding` if the encoding is not supported.

##### Params
- <a href="#keypair_export.kp" name="keypair_export.kp"></a> `kp`: [`keypair`](#keypair)

- <a href="#keypair_export.encoding" name="keypair_export.encoding"></a> `encoding`: [`keypair_encoding`](#keypair_encoding)

##### Results
- <a href="#keypair_export.error" name="keypair_export.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#keypair_export.encoded" name="keypair_export.encoded"></a> `encoded`: [`array_output`](#array_output)


---

#### <a href="#keypair_publickey" name="keypair_publickey"></a> `keypair_publickey(kp: keypair) -> (crypto_errno, publickey)`
Get the public key of a key pair.

##### Params
- <a href="#keypair_publickey.kp" name="keypair_publickey.kp"></a> `kp`: [`keypair`](#keypair)

##### Results
- <a href="#keypair_publickey.error" name="keypair_publickey.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#keypair_publickey.pk" name="keypair_publickey.pk"></a> `pk`: [`publickey`](#publickey)


---

#### <a href="#keypair_secretkey" name="keypair_secretkey"></a> `keypair_secretkey(kp: keypair) -> (crypto_errno, secretkey)`
Get the secret key of a key pair.

##### Params
- <a href="#keypair_secretkey.kp" name="keypair_secretkey.kp"></a> `kp`: [`keypair`](#keypair)

##### Results
- <a href="#keypair_secretkey.error" name="keypair_secretkey.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#keypair_secretkey.sk" name="keypair_secretkey.sk"></a> `sk`: [`secretkey`](#secretkey)


---

#### <a href="#keypair_close" name="keypair_close"></a> `keypair_close(kp: keypair) -> crypto_errno`
Destroy a key pair.

The host will automatically wipe traces of the secret key from memory.

If this is a managed key, the key will not be removed from persistent storage, and can be reconstructed later using the key identifier.

##### Params
- <a href="#keypair_close.kp" name="keypair_close.kp"></a> `kp`: [`keypair`](#keypair)

##### Results
- <a href="#keypair_close.error" name="keypair_close.error"></a> `error`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#publickey_import" name="publickey_import"></a> `publickey_import(algorithm_type: algorithm_type, algorithm: string, encoded: ConstPointer<u8>, encoded_len: size, encoding: publickey_encoding) -> (crypto_errno, publickey)`
Import a public key.

The function may return `unsupported_encoding` if importing from the given format is not implemented or incompatible with the key type.

It may also return `invalid_key` if the key doesn't appear to match the supplied algorithm.

Finally, the function may return `unsupported_algorithm` if the algorithm is not supported by the host.

Example usage:

```rust
let pk_handle = ctx.publickey_import(AlgorithmType::Signatures, encoded, PublicKeyEncoding::Sec)?;
```

##### Params
- <a href="#publickey_import.algorithm_type" name="publickey_import.algorithm_type"></a> `algorithm_type`: [`algorithm_type`](#algorithm_type)

- <a href="#publickey_import.algorithm" name="publickey_import.algorithm"></a> `algorithm`: `string`

- <a href="#publickey_import.encoded" name="publickey_import.encoded"></a> `encoded`: `ConstPointer<u8>`

- <a href="#publickey_import.encoded_len" name="publickey_import.encoded_len"></a> `encoded_len`: [`size`](#size)

- <a href="#publickey_import.encoding" name="publickey_import.encoding"></a> `encoding`: [`publickey_encoding`](#publickey_encoding)

##### Results
- <a href="#publickey_import.error" name="publickey_import.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#publickey_import.pk" name="publickey_import.pk"></a> `pk`: [`publickey`](#publickey)


---

#### <a href="#publickey_export" name="publickey_export"></a> `publickey_export(pk: publickey, encoding: publickey_encoding) -> (crypto_errno, array_output)`
Export a public key as the given encoding format.

May return `unsupported_encoding` if the encoding is not supported.

##### Params
- <a href="#publickey_export.pk" name="publickey_export.pk"></a> `pk`: [`publickey`](#publickey)

- <a href="#publickey_export.encoding" name="publickey_export.encoding"></a> `encoding`: [`publickey_encoding`](#publickey_encoding)

##### Results
- <a href="#publickey_export.error" name="publickey_export.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#publickey_export.encoded" name="publickey_export.encoded"></a> `encoded`: [`array_output`](#array_output)


---

#### <a href="#publickey_verify" name="publickey_verify"></a> `publickey_verify(pk: publickey) -> crypto_errno`
Check that a public key is valid and in canonical form.

This function may perform stricter checks than those made during importation at the expense of additional CPU cycles.

The function returns `invalid_key` if the public key didn't pass the checks.

##### Params
- <a href="#publickey_verify.pk" name="publickey_verify.pk"></a> `pk`: [`publickey`](#publickey)

##### Results
- <a href="#publickey_verify.error" name="publickey_verify.error"></a> `error`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#publickey_from_secretkey" name="publickey_from_secretkey"></a> `publickey_from_secretkey(sk: secretkey) -> (crypto_errno, publickey)`
Compute the public key for a secret key.

##### Params
- <a href="#publickey_from_secretkey.sk" name="publickey_from_secretkey.sk"></a> `sk`: [`secretkey`](#secretkey)

##### Results
- <a href="#publickey_from_secretkey.error" name="publickey_from_secretkey.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#publickey_from_secretkey.pk" name="publickey_from_secretkey.pk"></a> `pk`: [`publickey`](#publickey)


---

#### <a href="#publickey_close" name="publickey_close"></a> `publickey_close(pk: publickey) -> crypto_errno`
Destroy a public key.

Objects are reference counted. It is safe to close an object immediately after the last function needing it is called.

##### Params
- <a href="#publickey_close.pk" name="publickey_close.pk"></a> `pk`: [`publickey`](#publickey)

##### Results
- <a href="#publickey_close.error" name="publickey_close.error"></a> `error`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#secretkey_import" name="secretkey_import"></a> `secretkey_import(algorithm_type: algorithm_type, algorithm: string, encoded: ConstPointer<u8>, encoded_len: size, encoding: secretkey_encoding) -> (crypto_errno, secretkey)`
Import a secret key.

The function may return `unsupported_encoding` if importing from the given format is not implemented or incompatible with the key type.

It may also return `invalid_key` if the key doesn't appear to match the supplied algorithm.

Finally, the function may return `unsupported_algorithm` if the algorithm is not supported by the host.

Example usage:

```rust
let pk_handle = ctx.secretkey_import(AlgorithmType::KX, encoded, SecretKeyEncoding::Raw)?;
```

##### Params
- <a href="#secretkey_import.algorithm_type" name="secretkey_import.algorithm_type"></a> `algorithm_type`: [`algorithm_type`](#algorithm_type)

- <a href="#secretkey_import.algorithm" name="secretkey_import.algorithm"></a> `algorithm`: `string`

- <a href="#secretkey_import.encoded" name="secretkey_import.encoded"></a> `encoded`: `ConstPointer<u8>`

- <a href="#secretkey_import.encoded_len" name="secretkey_import.encoded_len"></a> `encoded_len`: [`size`](#size)

- <a href="#secretkey_import.encoding" name="secretkey_import.encoding"></a> `encoding`: [`secretkey_encoding`](#secretkey_encoding)

##### Results
- <a href="#secretkey_import.error" name="secretkey_import.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#secretkey_import.sk" name="secretkey_import.sk"></a> `sk`: [`secretkey`](#secretkey)


---

#### <a href="#secretkey_export" name="secretkey_export"></a> `secretkey_export(sk: secretkey, encoding: secretkey_encoding) -> (crypto_errno, array_output)`
Export a secret key as the given encoding format.

May return `unsupported_encoding` if the encoding is not supported.

##### Params
- <a href="#secretkey_export.sk" name="secretkey_export.sk"></a> `sk`: [`secretkey`](#secretkey)

- <a href="#secretkey_export.encoding" name="secretkey_export.encoding"></a> `encoding`: [`secretkey_encoding`](#secretkey_encoding)

##### Results
- <a href="#secretkey_export.error" name="secretkey_export.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#secretkey_export.encoded" name="secretkey_export.encoded"></a> `encoded`: [`array_output`](#array_output)


---

#### <a href="#secretkey_close" name="secretkey_close"></a> `secretkey_close(sk: secretkey) -> crypto_errno`
Destroy a secret key.

Objects are reference counted. It is safe to close an object immediately after the last function needing it is called.

##### Params
- <a href="#secretkey_close.sk" name="secretkey_close.sk"></a> `sk`: [`secretkey`](#secretkey)

##### Results
- <a href="#secretkey_close.error" name="secretkey_close.error"></a> `error`: [`crypto_errno`](#crypto_errno)

## <a href="#wasi_ephemeral_crypto_signatures" name="wasi_ephemeral_crypto_signatures"></a> wasi_ephemeral_crypto_signatures
### Imports
#### Memory
### Functions

---

#### <a href="#signature_export" name="signature_export"></a> `signature_export(signature: signature, encoding: signature_encoding) -> (crypto_errno, array_output)`
Export a signature.

This function exports a signature object using the specified encoding.

May return `unsupported_encoding` if the signature cannot be encoded into the given format.

##### Params
- <a href="#signature_export.signature" name="signature_export.signature"></a> `signature`: [`signature`](#signature)

- <a href="#signature_export.encoding" name="signature_export.encoding"></a> `encoding`: [`signature_encoding`](#signature_encoding)

##### Results
- <a href="#signature_export.error" name="signature_export.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#signature_export.encoded" name="signature_export.encoded"></a> `encoded`: [`array_output`](#array_output)


---

#### <a href="#signature_import" name="signature_import"></a> `signature_import(algorithm: string, encoding: signature_encoding, encoded: ConstPointer<u8>, encoded_len: size) -> (crypto_errno, signature)`
Create a signature object.

This object can be used along with a public key to verify an existing signature.

It may return `invalid_signature` if the signature is invalid or incompatible with the specified algorithm, as well as `unsupported_encoding` if the encoding is not compatible with the signature type.

The function may also return `unsupported_algorithm` if the algorithm is not supported by the host.

Example usage:

```rust
let signature_handle = ctx.signature_import("ECDSA_P256_SHA256", SignatureEncoding::DER, encoded)?;
```

##### Params
- <a href="#signature_import.algorithm" name="signature_import.algorithm"></a> `algorithm`: `string`

- <a href="#signature_import.encoding" name="signature_import.encoding"></a> `encoding`: [`signature_encoding`](#signature_encoding)

- <a href="#signature_import.encoded" name="signature_import.encoded"></a> `encoded`: `ConstPointer<u8>`

- <a href="#signature_import.encoded_len" name="signature_import.encoded_len"></a> `encoded_len`: [`size`](#size)

##### Results
- <a href="#signature_import.error" name="signature_import.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#signature_import.signature" name="signature_import.signature"></a> `signature`: [`signature`](#signature)


---

#### <a href="#signature_state_open" name="signature_state_open"></a> `signature_state_open(kp: signature_keypair) -> (crypto_errno, signature_state)`
Create a new state to collect data to compute a signature on.

This function allows data to be signed to be supplied in a streaming fashion.

The state is not closed and can be used after a signature has been computed, allowing incremental updates by calling `signature_state_update()` again afterwards.

Example usage - signature creation

```rust
let kp_handle = ctx.keypair_import(AlgorithmType::Signatures, "Ed25519ph", keypair, KeypairEncoding::Raw)?;
let state_handle = ctx.signature_state_open(kp_handle)?;
ctx.signature_state_update(state_handle, b"message part 1")?;
ctx.signature_state_update(state_handle, b"message part 2")?;
let sig_handle = ctx.signature_state_sign(state_handle)?;
let raw_sig = ctx.signature_export(sig_handle, SignatureEncoding::Raw)?;
```

##### Params
- <a href="#signature_state_open.kp" name="signature_state_open.kp"></a> `kp`: [`signature_keypair`](#signature_keypair)

##### Results
- <a href="#signature_state_open.error" name="signature_state_open.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#signature_state_open.state" name="signature_state_open.state"></a> `state`: [`signature_state`](#signature_state)


---

#### <a href="#signature_state_update" name="signature_state_update"></a> `signature_state_update(state: signature_state, input: ConstPointer<u8>, input_len: size) -> crypto_errno`
Absorb data into the signature state.

This function may return `unsupported_feature` is the selected algorithm doesn't support incremental updates.

##### Params
- <a href="#signature_state_update.state" name="signature_state_update.state"></a> `state`: [`signature_state`](#signature_state)

- <a href="#signature_state_update.input" name="signature_state_update.input"></a> `input`: `ConstPointer<u8>`

- <a href="#signature_state_update.input_len" name="signature_state_update.input_len"></a> `input_len`: [`size`](#size)

##### Results
- <a href="#signature_state_update.error" name="signature_state_update.error"></a> `error`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#signature_state_sign" name="signature_state_sign"></a> `signature_state_sign(state: signature_state) -> (crypto_errno, array_output)`
Compute a signature for all the data collected up to that point.

The function can be called multiple times for incremental signing.

##### Params
- <a href="#signature_state_sign.state" name="signature_state_sign.state"></a> `state`: [`signature_state`](#signature_state)

##### Results
- <a href="#signature_state_sign.error" name="signature_state_sign.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#signature_state_sign.signature" name="signature_state_sign.signature"></a> `signature`: [`array_output`](#array_output)


---

#### <a href="#signature_state_close" name="signature_state_close"></a> `signature_state_close(state: signature_state) -> crypto_errno`
Destroy a signature state.

Objects are reference counted. It is safe to close an object immediately after the last function needing it is called.

Note that closing a signature state doesn't close or invalidate the key pair object, that be reused for further signatures.

##### Params
- <a href="#signature_state_close.state" name="signature_state_close.state"></a> `state`: [`signature_state`](#signature_state)

##### Results
- <a href="#signature_state_close.error" name="signature_state_close.error"></a> `error`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#signature_verification_state_open" name="signature_verification_state_open"></a> `signature_verification_state_open(kp: signature_publickey) -> (crypto_errno, signature_verification_state)`
Create a new state to collect data to verify a signature on.

This is the verification counterpart of [`signature_state`](#signature_state).

Data can be injected using `signature_verification_state_update()`, and the state is not closed after a verification, allowing incremental verification.

Example usage - signature verification:

```rust
let pk_handle = ctx.publickey_import(AlgorithmType::Signatures, "ECDSA_P256_SHA256", encoded_pk, PublicKeyEncoding::CompressedSec)?;
let signature_handle = ctx.signature_import(AlgorithmType::Signatures, "ECDSA_P256_SHA256", encoded_sig, PublicKeyEncoding::Der)?;
let state_handle = ctx.signature_verification_state_open(pk_handle)?;
ctx.signature_verification_state_update(state_handle, "message")?;
ctx.signature_verification_state_verify(signature_handle)?;
```

##### Params
- <a href="#signature_verification_state_open.kp" name="signature_verification_state_open.kp"></a> `kp`: [`signature_publickey`](#signature_publickey)

##### Results
- <a href="#signature_verification_state_open.error" name="signature_verification_state_open.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#signature_verification_state_open.state" name="signature_verification_state_open.state"></a> `state`: [`signature_verification_state`](#signature_verification_state)


---

#### <a href="#signature_verification_state_update" name="signature_verification_state_update"></a> `signature_verification_state_update(state: signature_verification_state, input: ConstPointer<u8>, input_len: size) -> crypto_errno`
Absorb data into the signature verification state.

This function may return `unsupported_feature` is the selected algorithm doesn't support incremental updates.

##### Params
- <a href="#signature_verification_state_update.state" name="signature_verification_state_update.state"></a> `state`: [`signature_verification_state`](#signature_verification_state)

- <a href="#signature_verification_state_update.input" name="signature_verification_state_update.input"></a> `input`: `ConstPointer<u8>`

- <a href="#signature_verification_state_update.input_len" name="signature_verification_state_update.input_len"></a> `input_len`: [`size`](#size)

##### Results
- <a href="#signature_verification_state_update.error" name="signature_verification_state_update.error"></a> `error`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#signature_verification_state_verify" name="signature_verification_state_verify"></a> `signature_verification_state_verify(state: signature_verification_state, signature: signature) -> crypto_errno`
Check that the given signature is verifies for the data collected up to that point point.

The state is not closed and can absorb more data to allow for incremental verification.

The function returns `invalid_signature` if the signature doesn't appear to be valid.

##### Params
- <a href="#signature_verification_state_verify.state" name="signature_verification_state_verify.state"></a> `state`: [`signature_verification_state`](#signature_verification_state)

- <a href="#signature_verification_state_verify.signature" name="signature_verification_state_verify.signature"></a> `signature`: [`signature`](#signature)

##### Results
- <a href="#signature_verification_state_verify.error" name="signature_verification_state_verify.error"></a> `error`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#signature_verification_state_close" name="signature_verification_state_close"></a> `signature_verification_state_close(state: signature_verification_state) -> crypto_errno`
Destroy a signature verification state.

Objects are reference counted. It is safe to close an object immediately after the last function needing it is called.

Note that closing a signature state doesn't close or invalidate the public key object, that be reused for further verifications.

##### Params
- <a href="#signature_verification_state_close.state" name="signature_verification_state_close.state"></a> `state`: [`signature_verification_state`](#signature_verification_state)

##### Results
- <a href="#signature_verification_state_close.error" name="signature_verification_state_close.error"></a> `error`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#signature_close" name="signature_close"></a> `signature_close(signature: signature) -> crypto_errno`
Destroy a signature.

Objects are reference counted. It is safe to close an object immediately after the last function needing it is called.

##### Params
- <a href="#signature_close.signature" name="signature_close.signature"></a> `signature`: [`signature`](#signature)

##### Results
- <a href="#signature_close.error" name="signature_close.error"></a> `error`: [`crypto_errno`](#crypto_errno)

## <a href="#wasi_ephemeral_crypto_symmetric" name="wasi_ephemeral_crypto_symmetric"></a> wasi_ephemeral_crypto_symmetric
### Imports
#### Memory
### Functions

---

#### <a href="#symmetric_key_generate" name="symmetric_key_generate"></a> `symmetric_key_generate(algorithm: string, options: opt_options) -> (crypto_errno, symmetric_key)`
Generate a new symmetric key for a given algorithm.

[`options`](#options) can be `None` to use the default parameters, or an algoritm-specific set of parameters to override.

This function may return `unsupported_feature` if key generation is not supported by the host for the chosen algorithm, or `unsupported_algorithm` if the algorithm is not supported by the host.

##### Params
- <a href="#symmetric_key_generate.algorithm" name="symmetric_key_generate.algorithm"></a> `algorithm`: `string`

- <a href="#symmetric_key_generate.options" name="symmetric_key_generate.options"></a> `options`: [`opt_options`](#opt_options)

##### Results
- <a href="#symmetric_key_generate.error" name="symmetric_key_generate.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#symmetric_key_generate.handle" name="symmetric_key_generate.handle"></a> `handle`: [`symmetric_key`](#symmetric_key)


---

#### <a href="#symmetric_key_import" name="symmetric_key_import"></a> `symmetric_key_import(algorithm: string, raw: ConstPointer<u8>, raw_len: size) -> (crypto_errno, symmetric_key)`
Create a symmetric key from raw material.

The algorithm is internally stored along with the key, and trying to use the key with an operation expecting a different algorithm will return `invalid_key`.

The function may also return `unsupported_algorithm` if the algorithm is not supported by the host.

##### Params
- <a href="#symmetric_key_import.algorithm" name="symmetric_key_import.algorithm"></a> `algorithm`: `string`

- <a href="#symmetric_key_import.raw" name="symmetric_key_import.raw"></a> `raw`: `ConstPointer<u8>`

- <a href="#symmetric_key_import.raw_len" name="symmetric_key_import.raw_len"></a> `raw_len`: [`size`](#size)

##### Results
- <a href="#symmetric_key_import.error" name="symmetric_key_import.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#symmetric_key_import.handle" name="symmetric_key_import.handle"></a> `handle`: [`symmetric_key`](#symmetric_key)


---

#### <a href="#symmetric_key_export" name="symmetric_key_export"></a> `symmetric_key_export(symmetric_key: symmetric_key) -> (crypto_errno, array_output)`
Export a symmetric key as raw material.

This is mainly useful to export a managed key.

May return `prohibited_operation` if this operation is denied.

##### Params
- <a href="#symmetric_key_export.symmetric_key" name="symmetric_key_export.symmetric_key"></a> `symmetric_key`: [`symmetric_key`](#symmetric_key)

##### Results
- <a href="#symmetric_key_export.error" name="symmetric_key_export.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#symmetric_key_export.encoded" name="symmetric_key_export.encoded"></a> `encoded`: [`array_output`](#array_output)


---

#### <a href="#symmetric_key_close" name="symmetric_key_close"></a> `symmetric_key_close(symmetric_key: symmetric_key) -> crypto_errno`
Destroy a symmetric key.

Objects are reference counted. It is safe to close an object immediately after the last function needing it is called.

##### Params
- <a href="#symmetric_key_close.symmetric_key" name="symmetric_key_close.symmetric_key"></a> `symmetric_key`: [`symmetric_key`](#symmetric_key)

##### Results
- <a href="#symmetric_key_close.error" name="symmetric_key_close.error"></a> `error`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#symmetric_key_generate_managed" name="symmetric_key_generate_managed"></a> `symmetric_key_generate_managed(key_manager: key_manager, algorithm: string, options: opt_options) -> (crypto_errno, symmetric_key)`
__(optional)__
Generate a new managed symmetric key.

The key is generated and stored by the key management facilities.

It may be used through its identifier, but the host may not allow it to be exported.

The function returns the `unsupported_feature` error code if key management facilities are not supported by the host,
or `unsupported_algorithm` if a key cannot be created for the chosen algorithm.

The function may also return `unsupported_algorithm` if the algorithm is not supported by the host.

This is also an optional import, meaning that the function may not even exist.

##### Params
- <a href="#symmetric_key_generate_managed.key_manager" name="symmetric_key_generate_managed.key_manager"></a> `key_manager`: [`key_manager`](#key_manager)

- <a href="#symmetric_key_generate_managed.algorithm" name="symmetric_key_generate_managed.algorithm"></a> `algorithm`: `string`

- <a href="#symmetric_key_generate_managed.options" name="symmetric_key_generate_managed.options"></a> `options`: [`opt_options`](#opt_options)

##### Results
- <a href="#symmetric_key_generate_managed.error" name="symmetric_key_generate_managed.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#symmetric_key_generate_managed.handle" name="symmetric_key_generate_managed.handle"></a> `handle`: [`symmetric_key`](#symmetric_key)


---

#### <a href="#symmetric_key_id" name="symmetric_key_id"></a> `symmetric_key_id(symmetric_key: symmetric_key, symmetric_key_id: Pointer<u8>, symmetric_key_id_max_len: size) -> (crypto_errno, size, version)`
__(optional)__
Return the key identifier and version of a managed symmetric key pair.

If the key is not managed, `unsupported_feature` is returned instead.

This is an optional import, meaning that the function may not even exist.

##### Params
- <a href="#symmetric_key_id.symmetric_key" name="symmetric_key_id.symmetric_key"></a> `symmetric_key`: [`symmetric_key`](#symmetric_key)

- <a href="#symmetric_key_id.symmetric_key_id" name="symmetric_key_id.symmetric_key_id"></a> `symmetric_key_id`: `Pointer<u8>`

- <a href="#symmetric_key_id.symmetric_key_id_max_len" name="symmetric_key_id.symmetric_key_id_max_len"></a> `symmetric_key_id_max_len`: [`size`](#size)

##### Results
- <a href="#symmetric_key_id.error" name="symmetric_key_id.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#symmetric_key_id.symmetric_key_id_len" name="symmetric_key_id.symmetric_key_id_len"></a> `symmetric_key_id_len`: [`size`](#size)

- <a href="#symmetric_key_id.version" name="symmetric_key_id.version"></a> `version`: [`version`](#version)


---

#### <a href="#symmetric_key_from_id" name="symmetric_key_from_id"></a> `symmetric_key_from_id(key_manager: key_manager, symmetric_key_id: ConstPointer<u8>, symmetric_key_id_len: size, symmetric_key_version: version) -> (crypto_errno, symmetric_key)`
__(optional)__
Return a managed symmetric key from a key identifier.

`kp_version` can be set to `version_latest` to retrieve the most recent version of a key pair.

If no key matching the provided information is found, `key_not_found` is returned instead.

This is an optional import, meaning that the function may not even exist.

##### Params
- <a href="#symmetric_key_from_id.key_manager" name="symmetric_key_from_id.key_manager"></a> `key_manager`: [`key_manager`](#key_manager)

- <a href="#symmetric_key_from_id.symmetric_key_id" name="symmetric_key_from_id.symmetric_key_id"></a> `symmetric_key_id`: `ConstPointer<u8>`

- <a href="#symmetric_key_from_id.symmetric_key_id_len" name="symmetric_key_from_id.symmetric_key_id_len"></a> `symmetric_key_id_len`: [`size`](#size)

- <a href="#symmetric_key_from_id.symmetric_key_version" name="symmetric_key_from_id.symmetric_key_version"></a> `symmetric_key_version`: [`version`](#version)

##### Results
- <a href="#symmetric_key_from_id.error" name="symmetric_key_from_id.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#symmetric_key_from_id.handle" name="symmetric_key_from_id.handle"></a> `handle`: [`symmetric_key`](#symmetric_key)


---

#### <a href="#symmetric_state_open" name="symmetric_state_open"></a> `symmetric_state_open(algorithm: string, key: opt_symmetric_key, options: opt_options) -> (crypto_errno, symmetric_state)`
Create a new state to aborb and produce data using symmetric operations.

The state remains valid after every operation in order to support incremental updates.

The function has two optional parameters: a key and an options set.

It will fail with a `key_not_supported` error code if a key was provided but the chosen algorithm doesn't natively support keying.

On the other hand, if a key is required, but was not provided, a `key_required` error will be thrown.

Some algorithms may require additional parameters. They have to be supplied as an options set:

```rust
let options_handle = ctx.options_open()?;
ctx.options_set("context", b"My application")?;
ctx.options_set_u64("fanout", 16)?;
let state_handle = ctx.symmetric_state_open("BLAKE2b-512", None, Some(options_handle))?;
```

If some parameters are mandatory but were not set, the `parameters_missing` error code will be returned.

A notable exception is the `nonce` parameter, that is common to most AEAD constructions.

If a nonce is required but was not supplied:

- If it is safe to do so, the host will automatically generate a nonce. This is true for nonces that are large enough to be randomly generated, or if the host is able to maintain a global counter.
- If not, the function will fail and return the dedicated `nonce_required` error code.

A nonce that was automatically generated can be retrieved after the function returns with `symmetric_state_get(state_handle, "nonce")`.

**Sample usage patterns:**

- **Hashing**

```rust
let mut out = [0u8; 64];
let state_handle = ctx.symmetric_state_open("SHAKE-128", None, None)?;
ctx.symmetric_state_absorb(state_handle, b"data")?;
ctx.symmetric_state_absorb(state_handle, b"more_data")?;
ctx.symmetric_state_squeeze(state_handle, &mut out)?;
```

- **MAC**

```rust
let mut raw_tag = [0u8; 64];
let key_handle = ctx.symmetric_key_import("HMAC/SHA-512", b"key")?;
let state_handle = ctx.symmetric_state_open("HMAC/SHA-512", Some(key_handle), None)?;
ctx.symmetric_state_absorb(state_handle, b"data")?;
ctx.symmetric_state_absorb(state_handle, b"more_data")?;
let computed_tag_handle = ctx.symmetric_state_squeeze_tag(state_handle)?;
ctx.symmetric_tag_pull(computed_tag_handle, &mut raw_tag)?;
```

Verification:

```rust
let state_handle = ctx.symmetric_state_open("HMAC/SHA-512", Some(key_handle), None)?;
ctx.symmetric_state_absorb(state_handle, b"data")?;
ctx.symmetric_state_absorb(state_handle, b"more_data")?;
let computed_tag_handle = ctx.symmetric_state_squeeze_tag(state_handle)?;
ctx.symmetric_tag_verify(computed_tag_handle, expected_raw_tag)?;
```

- **Tuple hashing**

```rust
let mut out = [0u8; 64];
let state_handle = ctx.symmetric_state_open("TupleHashXOF256", None, None)?;
ctx.symmetric_state_absorb(state_handle, b"value 1")?;
ctx.symmetric_state_absorb(state_handle, b"value 2")?;
ctx.symmetric_state_absorb(state_handle, b"value 3")?;
ctx.symmetric_state_squeeze(state_handle, &mut out)?;
```
Unlike MACs and regular hash functions, inputs are domain separated instead of being concatenated.

- **Key derivation using extract-and-expand**

Extract:

```rust
let mut prk = vec![0u8; 64];
let key_handle = ctx.symmetric_key_import("HKDF-EXTRACT/SHA-512", b"key")?;
let state_handle = ctx.symmetric_state_open("HKDF-EXTRACT/SHA-512", Some(key_handle), None)?;
ctx.symmetric_state_absorb(state_handle, b"salt")?;
let prk_handle = ctx.symmetric_state_squeeze_key(state_handle, "HKDF-EXPAND/SHA-512")?;
```

Expand:

```rust
let mut subkey = vec![0u8; 32];
let state_handle = ctx.symmetric_state_open("HKDF-EXPAND/SHA-512", Some(prk_handle), None)?;
ctx.symmetric_state_absorb(state_handle, b"info")?;
ctx.symmetric_state_squeeze(state_handle, &mut subkey)?;
```

- **Key derivation using a XOF**

```rust
let mut subkey1 = vec![0u8; 32];
let mut subkey2 = vec![0u8; 32];
let key_handle = ctx.symmetric_key_import("BLAKE3", b"key")?;
let state_handle = ctx.symmetric_state_open("BLAKE3", Some(key_handle), None)?;
ctx.symmetric_absorb(state_handle, b"context")?;
ctx.squeeze(state_handle, &mut subkey1)?;
ctx.squeeze(state_handle, &mut subkey2)?;
```

- **Password hashing**

```rust
let mut memory = vec![0u8; 1_000_000_000];
let options_handle = ctx.symmetric_options_open()?;
ctx.symmetric_options_set_guest_buffer(options_handle, "memory", &mut memory)?;
ctx.symmetric_options_set_u64(options_handle, "opslimit", 5)?;
ctx.symmetric_options_set_u64(options_handle, "parallelism", 8)?;

let state_handle = ctx.symmetric_state_open("ARGON2-ID-13", None, Some(options))?;
ctx.symmtric_state_absorb(state_handle, b"password")?;

let pw_str_handle = ctx.symmetric_state_squeeze_tag(state_handle)?;
let mut pw_str = vec![0u8; ctx.symmetric_tag_len(pw_str_handle)?];
ctx.symmetric_tag_pull(pw_str_handle, &mut pw_str)?;
```

- **AEAD encryption with an explicit nonce**

```rust
let key_handle = ctx.symmetric_key_generate("AES-256-GCM", None)?;
let message = b"test";

let options_handle = ctx.symmetric_options_open()?;
ctx.symmetric_options_set(options_handle, "nonce", nonce)?;

let state_handle = ctx.symmetric_state_open("AES-256-GCM", Some(key_handle), Some(options_handle))?;
let mut ciphertext = vec![0u8; message.len() + ctx.symmetric_state_max_tag_len(state_handle)?];
ctx.symmetric_state_absorb(state_handle, "additional data")?;
ctx.symmetric_state_encrypt(state_handle, &mut ciphertext, message)?;
```

- **AEAD encryption with automatic nonce generation**

```rust
let key_handle = ctx.symmetric_key_generate("AES-256-GCM-SIV", None)?;
let message = b"test";
let mut nonce = [0u8; 24];

let state_handle = ctx.symmetric_state_open("AES-256-GCM-SIV", Some(key_handle), None)?;

let nonce_handle = ctx.symmetric_state_options_get(state_handle, "nonce")?;
ctx.array_output_pull(nonce_handle, &mut nonce)?;

let mut ciphertext = vec![0u8; message.len() + ctx.symmetric_state_max_tag_len(state_handle)?];
ctx.symmetric_state_absorb(state_handle, "additional data")?;
ctx.symmetric_state_encrypt(state_handle, &mut ciphertext, message)?;
```

- **Session authenticated modes**

```rust
let mut out = [0u8; 16];
let mut out2 = [0u8; 16];
let mut ciphertext = [0u8; 20];
let key_handle = ctx.symmetric_key_generate("Xoodyak-256", None)?;
let state_handle = ctx.symmetric_state_open("Xoodyak-256", Some(key_handle), None)?;
ctx.symmetric_state_absorb(state_handle, b"data")?;
ctx.symmetric_state_encrypt(state_handle, &mut ciphertext, b"abcd")?;
ctx.symmetric_state_absorb(state_handle, b"more data")?;
ctx.symmetric_state_squeeze(state_handle, &mut out)?;
ctx.symmetric_state_squeeze(state_handle, &mut out2)?;
ctx.symmetric_state_ratchet(state_handle)?;
ctx.symmetric_state_absorb(state_handle, b"more data")?;
let next_key_handle = ctx.symmetric_state_squeeze_key(state_handle, "Xoodyak-256")?;
// ...
```

##### Params
- <a href="#symmetric_state_open.algorithm" name="symmetric_state_open.algorithm"></a> `algorithm`: `string`

- <a href="#symmetric_state_open.key" name="symmetric_state_open.key"></a> `key`: [`opt_symmetric_key`](#opt_symmetric_key)

- <a href="#symmetric_state_open.options" name="symmetric_state_open.options"></a> `options`: [`opt_options`](#opt_options)

##### Results
- <a href="#symmetric_state_open.error" name="symmetric_state_open.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#symmetric_state_open.symmetric_state" name="symmetric_state_open.symmetric_state"></a> `symmetric_state`: [`symmetric_state`](#symmetric_state)


---

#### <a href="#symmetric_state_options_get" name="symmetric_state_options_get"></a> `symmetric_state_options_get(handle: symmetric_state, name: string, value: Pointer<u8>, value_max_len: size) -> (crypto_errno, size)`
Retrieve a parameter from the current state.

In particular, `symmetric_state_options_get("nonce")` can be used to get a nonce that as automatically generated.

The function may return `options_not_set` if an option was not set, which is different from an empty value.

It may also return `unsupported_option` if the option doesn't exist for the chosen algorithm.

##### Params
- <a href="#symmetric_state_options_get.handle" name="symmetric_state_options_get.handle"></a> `handle`: [`symmetric_state`](#symmetric_state)

- <a href="#symmetric_state_options_get.name" name="symmetric_state_options_get.name"></a> `name`: `string`

- <a href="#symmetric_state_options_get.value" name="symmetric_state_options_get.value"></a> `value`: `Pointer<u8>`

- <a href="#symmetric_state_options_get.value_max_len" name="symmetric_state_options_get.value_max_len"></a> `value_max_len`: [`size`](#size)

##### Results
- <a href="#symmetric_state_options_get.error" name="symmetric_state_options_get.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#symmetric_state_options_get.value_len" name="symmetric_state_options_get.value_len"></a> `value_len`: [`size`](#size)


---

#### <a href="#symmetric_state_options_get_u64" name="symmetric_state_options_get_u64"></a> `symmetric_state_options_get_u64(handle: symmetric_state, name: string) -> (crypto_errno, u64)`
Retrieve an integer parameter from the current state.

In particular, `symmetric_state_options_get("nonce")` can be used to get a nonce that as automatically generated.

The function may return `options_not_set` if an option was not set.

It may also return `unsupported_option` if the option doesn't exist for the chosen algorithm.

##### Params
- <a href="#symmetric_state_options_get_u64.handle" name="symmetric_state_options_get_u64.handle"></a> `handle`: [`symmetric_state`](#symmetric_state)

- <a href="#symmetric_state_options_get_u64.name" name="symmetric_state_options_get_u64.name"></a> `name`: `string`

##### Results
- <a href="#symmetric_state_options_get_u64.error" name="symmetric_state_options_get_u64.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#symmetric_state_options_get_u64.value" name="symmetric_state_options_get_u64.value"></a> `value`: `u64`


---

#### <a href="#symmetric_state_close" name="symmetric_state_close"></a> `symmetric_state_close(handle: symmetric_state) -> crypto_errno`
Destroy a symmetric state.

Objects are reference counted. It is safe to close an object immediately after the last function needing it is called.

##### Params
- <a href="#symmetric_state_close.handle" name="symmetric_state_close.handle"></a> `handle`: [`symmetric_state`](#symmetric_state)

##### Results
- <a href="#symmetric_state_close.error" name="symmetric_state_close.error"></a> `error`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#symmetric_state_absorb" name="symmetric_state_absorb"></a> `symmetric_state_absorb(handle: symmetric_state, data: ConstPointer<u8>, data_len: size) -> crypto_errno`
Absorb data into the state.

- **Hash functions:** adds data to be hashed.
- **MAC functions:** adds data to be authenticated.
- **Tuplehash-like constructions:** adds a new tuple to the state.
- **Key derivation functions:** adds to the IKM or to the subkey information.
- **AEAD constructions:** adds additional data to be authenticated.
- **Stateful hash objects, permutation-based constructions:** absorbs.

If the chosen algorithm doesn't accept input data, the `invalid_operation` error code is returned.

If too much data has been fed for the algorithm, `overflow` may be thrown.

##### Params
- <a href="#symmetric_state_absorb.handle" name="symmetric_state_absorb.handle"></a> `handle`: [`symmetric_state`](#symmetric_state)

- <a href="#symmetric_state_absorb.data" name="symmetric_state_absorb.data"></a> `data`: `ConstPointer<u8>`

- <a href="#symmetric_state_absorb.data_len" name="symmetric_state_absorb.data_len"></a> `data_len`: [`size`](#size)

##### Results
- <a href="#symmetric_state_absorb.error" name="symmetric_state_absorb.error"></a> `error`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#symmetric_state_squeeze" name="symmetric_state_squeeze"></a> `symmetric_state_squeeze(handle: symmetric_state, out: Pointer<u8>, out_len: size) -> crypto_errno`
Squeeze bytes from the state.

- **Hash functions:** this tries to output an `out_len` bytes digest from the absorbed data. The hash function output will be truncated if necessary. If the requested size is too large, the `invalid_len` error code is returned.
- **Key derivation functions:** : outputs an arbitrary-long derived key.
- **RNGs, DRBGs, stream ciphers:**: outputs arbitrary-long data.
- **Stateful hash objects, permutation-based constructions:** squeeze.

Other kinds of algorithms may return `invalid_operation` instead.

For password-stretching functions, the function may return `in_progress`.
In that case, the guest should retry with the same parameters until the function completes.

##### Params
- <a href="#symmetric_state_squeeze.handle" name="symmetric_state_squeeze.handle"></a> `handle`: [`symmetric_state`](#symmetric_state)

- <a href="#symmetric_state_squeeze.out" name="symmetric_state_squeeze.out"></a> `out`: `Pointer<u8>`

- <a href="#symmetric_state_squeeze.out_len" name="symmetric_state_squeeze.out_len"></a> `out_len`: [`size`](#size)

##### Results
- <a href="#symmetric_state_squeeze.error" name="symmetric_state_squeeze.error"></a> `error`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#symmetric_state_squeeze_tag" name="symmetric_state_squeeze_tag"></a> `symmetric_state_squeeze_tag(handle: symmetric_state) -> (crypto_errno, symmetric_tag)`
Compute and return a tag for all the data injected into the state so far.

- **MAC functions**: returns a tag authenticating the absorbed data.
- **Tuplehash-like constructions:** returns a tag authenticating all the absorbed tuples.
- **Password-hashing functions:** returns a standard string containing all the required parameters for password verification.

Other kinds of algorithms may return `invalid_operation` instead.

For password-stretching functions, the function may return `in_progress`.
In that case, the guest should retry with the same parameters until the function completes.

##### Params
- <a href="#symmetric_state_squeeze_tag.handle" name="symmetric_state_squeeze_tag.handle"></a> `handle`: [`symmetric_state`](#symmetric_state)

##### Results
- <a href="#symmetric_state_squeeze_tag.error" name="symmetric_state_squeeze_tag.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#symmetric_state_squeeze_tag.symmetric_tag" name="symmetric_state_squeeze_tag.symmetric_tag"></a> `symmetric_tag`: [`symmetric_tag`](#symmetric_tag)


---

#### <a href="#symmetric_state_squeeze_key" name="symmetric_state_squeeze_key"></a> `symmetric_state_squeeze_key(handle: symmetric_state, alg_str: string) -> (crypto_errno, symmetric_key)`
Use the current state to produce a key for a target algorithm.

For extract-then-expand constructions, this returns the PRK.
For session-base authentication encryption, this returns a key that can be used to resume a session without storing a nonce.

`invalid_operation` is returned for algorithms not supporting this operation.

##### Params
- <a href="#symmetric_state_squeeze_key.handle" name="symmetric_state_squeeze_key.handle"></a> `handle`: [`symmetric_state`](#symmetric_state)

- <a href="#symmetric_state_squeeze_key.alg_str" name="symmetric_state_squeeze_key.alg_str"></a> `alg_str`: `string`

##### Results
- <a href="#symmetric_state_squeeze_key.error" name="symmetric_state_squeeze_key.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#symmetric_state_squeeze_key.symmetric_key" name="symmetric_state_squeeze_key.symmetric_key"></a> `symmetric_key`: [`symmetric_key`](#symmetric_key)


---

#### <a href="#symmetric_state_max_tag_len" name="symmetric_state_max_tag_len"></a> `symmetric_state_max_tag_len(handle: symmetric_state) -> (crypto_errno, size)`
Return the maximum length of an authentication tag for the current algorithm.

This allows guests to compute the size required to store a ciphertext along with its authentication tag.

The returned length may include the encryption mode's padding requirements in addition to the actual tag.

For an encryption operation, the size of the output buffer should be `input_len + symmetric_state_max_tag_len()`.

For a decryption operation, the size of the buffer that will store the decrypted data can be reduced to `ciphertext_len - symmetric_state_max_tag_len()` only if the algorithm is known to have a fixed tag length.

##### Params
- <a href="#symmetric_state_max_tag_len.handle" name="symmetric_state_max_tag_len.handle"></a> `handle`: [`symmetric_state`](#symmetric_state)

##### Results
- <a href="#symmetric_state_max_tag_len.error" name="symmetric_state_max_tag_len.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#symmetric_state_max_tag_len.len" name="symmetric_state_max_tag_len.len"></a> `len`: [`size`](#size)


---

#### <a href="#symmetric_state_encrypt" name="symmetric_state_encrypt"></a> `symmetric_state_encrypt(handle: symmetric_state, out: Pointer<u8>, out_len: size, data: ConstPointer<u8>, data_len: size) -> (crypto_errno, size)`
Encrypt data with an attached tag.

- **Stream cipher:** adds the input to the stream cipher output. `out_len` and `data_len` can be equal, as no authentication tags will be added.
- **AEAD:** encrypts `data` into `out`, including the authentication tag to the output. Additional data must have been previously absorbed using `symmetric_state_absorb()`. The `symmetric_state_max_tag_len()` function can be used to retrieve the overhead of adding the tag, as well as padding if necessary.
- **SHOE, Xoodyak, Strobe:** encrypts data, squeezes a tag and appends it to the output.

If `out` and `data` are the same address, encryption may happen in-place.

The function returns the actual size of the ciphertext along with the tag.

`invalid_operation` is returned for algorithms not supporting encryption.

##### Params
- <a href="#symmetric_state_encrypt.handle" name="symmetric_state_encrypt.handle"></a> `handle`: [`symmetric_state`](#symmetric_state)

- <a href="#symmetric_state_encrypt.out" name="symmetric_state_encrypt.out"></a> `out`: `Pointer<u8>`

- <a href="#symmetric_state_encrypt.out_len" name="symmetric_state_encrypt.out_len"></a> `out_len`: [`size`](#size)

- <a href="#symmetric_state_encrypt.data" name="symmetric_state_encrypt.data"></a> `data`: `ConstPointer<u8>`

- <a href="#symmetric_state_encrypt.data_len" name="symmetric_state_encrypt.data_len"></a> `data_len`: [`size`](#size)

##### Results
- <a href="#symmetric_state_encrypt.error" name="symmetric_state_encrypt.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#symmetric_state_encrypt.actual_out_len" name="symmetric_state_encrypt.actual_out_len"></a> `actual_out_len`: [`size`](#size)


---

#### <a href="#symmetric_state_encrypt_detached" name="symmetric_state_encrypt_detached"></a> `symmetric_state_encrypt_detached(handle: symmetric_state, out: Pointer<u8>, out_len: size, data: ConstPointer<u8>, data_len: size) -> (crypto_errno, symmetric_tag)`
Encrypt data, with a detached tag.

- **Stream cipher:** returns `invalid_operation` since stream ciphers do not include authentication tags.
- **AEAD:** encrypts `data` into `out` and returns the tag separately. Additional data must have been previously absorbed using `symmetric_state_absorb()`. The output and input buffers can be of the same length.
- **SHOE, Xoodyak, Strobe:** encrypts data and squeezes a tag.

If `out` and `data` are the same address, encryption may happen in-place.

The function returns the tag.

`invalid_operation` is returned for algorithms not supporting encryption.

##### Params
- <a href="#symmetric_state_encrypt_detached.handle" name="symmetric_state_encrypt_detached.handle"></a> `handle`: [`symmetric_state`](#symmetric_state)

- <a href="#symmetric_state_encrypt_detached.out" name="symmetric_state_encrypt_detached.out"></a> `out`: `Pointer<u8>`

- <a href="#symmetric_state_encrypt_detached.out_len" name="symmetric_state_encrypt_detached.out_len"></a> `out_len`: [`size`](#size)

- <a href="#symmetric_state_encrypt_detached.data" name="symmetric_state_encrypt_detached.data"></a> `data`: `ConstPointer<u8>`

- <a href="#symmetric_state_encrypt_detached.data_len" name="symmetric_state_encrypt_detached.data_len"></a> `data_len`: [`size`](#size)

##### Results
- <a href="#symmetric_state_encrypt_detached.error" name="symmetric_state_encrypt_detached.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#symmetric_state_encrypt_detached.symmetric_tag" name="symmetric_state_encrypt_detached.symmetric_tag"></a> `symmetric_tag`: [`symmetric_tag`](#symmetric_tag)


---

#### <a href="#symmetric_state_decrypt" name="symmetric_state_decrypt"></a> `symmetric_state_decrypt(handle: symmetric_state, out: Pointer<u8>, out_len: size, data: ConstPointer<u8>, data_len: size) -> (crypto_errno, size)`
- **Stream cipher:** adds the input to the stream cipher output. `out_len` and `data_len` can be equal, as no authentication tags will be added.
- **AEAD:** decrypts `data` into `out`. Additional data must have been previously absorbed using `symmetric_state_absorb()`.
- **SHOE, Xoodyak, Strobe:** decrypts data, squeezes a tag and verify that it matches the one that was appended to the ciphertext.

If `out` and `data` are the same address, decryption may happen in-place.

The function returns the actual size of the decrypted message.

`invalid_tag` is returned if the tag didn't verify.

`invalid_operation` is returned for algorithms not supporting encryption.

##### Params
- <a href="#symmetric_state_decrypt.handle" name="symmetric_state_decrypt.handle"></a> `handle`: [`symmetric_state`](#symmetric_state)

- <a href="#symmetric_state_decrypt.out" name="symmetric_state_decrypt.out"></a> `out`: `Pointer<u8>`

- <a href="#symmetric_state_decrypt.out_len" name="symmetric_state_decrypt.out_len"></a> `out_len`: [`size`](#size)

- <a href="#symmetric_state_decrypt.data" name="symmetric_state_decrypt.data"></a> `data`: `ConstPointer<u8>`

- <a href="#symmetric_state_decrypt.data_len" name="symmetric_state_decrypt.data_len"></a> `data_len`: [`size`](#size)

##### Results
- <a href="#symmetric_state_decrypt.error" name="symmetric_state_decrypt.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#symmetric_state_decrypt.actual_out_len" name="symmetric_state_decrypt.actual_out_len"></a> `actual_out_len`: [`size`](#size)


---

#### <a href="#symmetric_state_decrypt_detached" name="symmetric_state_decrypt_detached"></a> `symmetric_state_decrypt_detached(handle: symmetric_state, out: Pointer<u8>, out_len: size, data: ConstPointer<u8>, data_len: size, raw_tag: ConstPointer<u8>, raw_tag_len: size) -> (crypto_errno, size)`
- **Stream cipher:** returns `invalid_operation` since stream ciphers do not include authentication tags.
- **AEAD:** decrypts `data` into `out`. Additional data must have been previously absorbed using `symmetric_state_absorb()`.
- **SHOE, Xoodyak, Strobe:** decrypts data, squeezes a tag and verify that it matches the expected one.

`raw_tag` is the expected tag, as raw bytes.

If `out` and `data` are the same address, decryption may happen in-place.

The function returns the actual size of the decrypted message.

`invalid_tag` is returned if the tag verification failed.

`invalid_operation` is returned for algorithms not supporting encryption.

##### Params
- <a href="#symmetric_state_decrypt_detached.handle" name="symmetric_state_decrypt_detached.handle"></a> `handle`: [`symmetric_state`](#symmetric_state)

- <a href="#symmetric_state_decrypt_detached.out" name="symmetric_state_decrypt_detached.out"></a> `out`: `Pointer<u8>`

- <a href="#symmetric_state_decrypt_detached.out_len" name="symmetric_state_decrypt_detached.out_len"></a> `out_len`: [`size`](#size)

- <a href="#symmetric_state_decrypt_detached.data" name="symmetric_state_decrypt_detached.data"></a> `data`: `ConstPointer<u8>`

- <a href="#symmetric_state_decrypt_detached.data_len" name="symmetric_state_decrypt_detached.data_len"></a> `data_len`: [`size`](#size)

- <a href="#symmetric_state_decrypt_detached.raw_tag" name="symmetric_state_decrypt_detached.raw_tag"></a> `raw_tag`: `ConstPointer<u8>`

- <a href="#symmetric_state_decrypt_detached.raw_tag_len" name="symmetric_state_decrypt_detached.raw_tag_len"></a> `raw_tag_len`: [`size`](#size)

##### Results
- <a href="#symmetric_state_decrypt_detached.error" name="symmetric_state_decrypt_detached.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#symmetric_state_decrypt_detached.actual_out_len" name="symmetric_state_decrypt_detached.actual_out_len"></a> `actual_out_len`: [`size`](#size)


---

#### <a href="#symmetric_state_ratchet" name="symmetric_state_ratchet"></a> `symmetric_state_ratchet(handle: symmetric_state) -> crypto_errno`
Make it impossible to recover the previous state.

This operation is supported by some systems keeping a rolling state over an entire session, for forward security.

`invalid_operation` is returned for algorithms not supporting ratcheting.

##### Params
- <a href="#symmetric_state_ratchet.handle" name="symmetric_state_ratchet.handle"></a> `handle`: [`symmetric_state`](#symmetric_state)

##### Results
- <a href="#symmetric_state_ratchet.error" name="symmetric_state_ratchet.error"></a> `error`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#symmetric_tag_len" name="symmetric_tag_len"></a> `symmetric_tag_len(symmetric_tag: symmetric_tag) -> (crypto_errno, size)`
Return the length of an authentication tag.

This function can be used by a guest to allocate the correct buffer size to copy a computed authentication tag.

##### Params
- <a href="#symmetric_tag_len.symmetric_tag" name="symmetric_tag_len.symmetric_tag"></a> `symmetric_tag`: [`symmetric_tag`](#symmetric_tag)

##### Results
- <a href="#symmetric_tag_len.error" name="symmetric_tag_len.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#symmetric_tag_len.len" name="symmetric_tag_len.len"></a> `len`: [`size`](#size)


---

#### <a href="#symmetric_tag_pull" name="symmetric_tag_pull"></a> `symmetric_tag_pull(symmetric_tag: symmetric_tag, buf: Pointer<u8>, buf_len: size) -> (crypto_errno, size)`
Copy an authentication tag into a guest-allocated buffer.

The handle automatically becomes invalid after this operation. Manually closing it is not required.

Example usage:

```rust
let mut raw_tag = [0u8; 16];
ctx.symmetric_tag_pull(raw_tag_handle, &mut raw_tag)?;
```

The function returns `overflow` if the supplied buffer is too small to copy the tag.

Otherwise, it returns the number of bytes that have been copied.

##### Params
- <a href="#symmetric_tag_pull.symmetric_tag" name="symmetric_tag_pull.symmetric_tag"></a> `symmetric_tag`: [`symmetric_tag`](#symmetric_tag)

- <a href="#symmetric_tag_pull.buf" name="symmetric_tag_pull.buf"></a> `buf`: `Pointer<u8>`

- <a href="#symmetric_tag_pull.buf_len" name="symmetric_tag_pull.buf_len"></a> `buf_len`: [`size`](#size)

##### Results
- <a href="#symmetric_tag_pull.error" name="symmetric_tag_pull.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#symmetric_tag_pull.len" name="symmetric_tag_pull.len"></a> `len`: [`size`](#size)


---

#### <a href="#symmetric_tag_verify" name="symmetric_tag_verify"></a> `symmetric_tag_verify(symmetric_tag: symmetric_tag, expected_raw_tag_ptr: ConstPointer<u8>, expected_raw_tag_len: size) -> crypto_errno`
Verify that a computed authentication tag matches the expected value, in constant-time.

The expected tag must be provided as a raw byte string.

The function returns `invalid_tag` if the tags don't match.

Example usage:

```rust
let key_handle = ctx.symmetric_key_import("HMAC/SHA-256", b"key")?;
let state_handle = ctx.symmetric_state_open("HMAC/SHA-256", Some(key_handle), None)?;
ctx.symmetric_state_absorb(state_handle, b"data")?;
let computed_tag_handle = ctx.symmetric_state_squeeze_tag(state_handle)?;
ctx.symmetric_tag_verify(computed_tag_handle, expected_raw_tag)?;
```

##### Params
- <a href="#symmetric_tag_verify.symmetric_tag" name="symmetric_tag_verify.symmetric_tag"></a> `symmetric_tag`: [`symmetric_tag`](#symmetric_tag)

- <a href="#symmetric_tag_verify.expected_raw_tag_ptr" name="symmetric_tag_verify.expected_raw_tag_ptr"></a> `expected_raw_tag_ptr`: `ConstPointer<u8>`

- <a href="#symmetric_tag_verify.expected_raw_tag_len" name="symmetric_tag_verify.expected_raw_tag_len"></a> `expected_raw_tag_len`: [`size`](#size)

##### Results
- <a href="#symmetric_tag_verify.error" name="symmetric_tag_verify.error"></a> `error`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#symmetric_tag_close" name="symmetric_tag_close"></a> `symmetric_tag_close(symmetric_tag: symmetric_tag) -> crypto_errno`
Explicitly destroy an unused authentication tag.

This is usually not necessary, as `symmetric_tag_pull()` automatically closes a tag after it has been copied.

Objects are reference counted. It is safe to close an object immediately after the last function needing it is called.

##### Params
- <a href="#symmetric_tag_close.symmetric_tag" name="symmetric_tag_close.symmetric_tag"></a> `symmetric_tag`: [`symmetric_tag`](#symmetric_tag)

##### Results
- <a href="#symmetric_tag_close.error" name="symmetric_tag_close.error"></a> `error`: [`crypto_errno`](#crypto_errno)

