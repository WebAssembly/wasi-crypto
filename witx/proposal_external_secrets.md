# Types
## <a href="#crypto_errno" name="crypto_errno"></a> `crypto_errno`: `Variant`
Error codes.

Size: 2

Alignment: 2

### Variant cases
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

- <a href="#crypto_errno.invalid_nonce" name="crypto_errno.invalid_nonce"></a> `invalid_nonce`
The provided nonce doesn't have a correct size for the given cipher.

- <a href="#crypto_errno.option_not_set" name="crypto_errno.option_not_set"></a> `option_not_set`
The named option was not set.

The caller tried to read the value of an option that was not set.
This error is used to make the distinction between an empty option, and an option that was not set and left to its default value.

- <a href="#crypto_errno.not_found" name="crypto_errno.not_found"></a> `not_found`
A key or key pair matching the requested identifier cannot be found using the supplied information.

This error is returned by a secrets manager via the `keypair_from_id()` function.

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

- <a href="#crypto_errno.expired" name="crypto_errno.expired"></a> `expired`
A managed key or secret expired and cannot be used any more.

## <a href="#keypair_encoding" name="keypair_encoding"></a> `keypair_encoding`: `Variant`
Encoding to use for importing or exporting a key pair.

Size: 2

Alignment: 2

### Variant cases
- <a href="#keypair_encoding.raw" name="keypair_encoding.raw"></a> `raw`
Raw bytes.

- <a href="#keypair_encoding.pkcs8" name="keypair_encoding.pkcs8"></a> `pkcs8`
PCSK8/DER encoding.

- <a href="#keypair_encoding.pem" name="keypair_encoding.pem"></a> `pem`
PEM encoding.

- <a href="#keypair_encoding.local" name="keypair_encoding.local"></a> `local`
Implementation-defined encoding.

## <a href="#publickey_encoding" name="publickey_encoding"></a> `publickey_encoding`: `Variant`
Encoding to use for importing or exporting a public key.

Size: 2

Alignment: 2

### Variant cases
- <a href="#publickey_encoding.raw" name="publickey_encoding.raw"></a> `raw`
Raw bytes.

- <a href="#publickey_encoding.pkcs8" name="publickey_encoding.pkcs8"></a> `pkcs8`
PKCS8/DER encoding.

- <a href="#publickey_encoding.pem" name="publickey_encoding.pem"></a> `pem`
PEM encoding.

- <a href="#publickey_encoding.sec" name="publickey_encoding.sec"></a> `sec`
SEC encoding.

- <a href="#publickey_encoding.compressed_sec" name="publickey_encoding.compressed_sec"></a> `compressed_sec`
Compressed SEC encoding.

- <a href="#publickey_encoding.local" name="publickey_encoding.local"></a> `local`
Implementation-defined encoding.

## <a href="#secretkey_encoding" name="secretkey_encoding"></a> `secretkey_encoding`: `Variant`
Encoding to use for importing or exporting a secret key.

Size: 2

Alignment: 2

### Variant cases
- <a href="#secretkey_encoding.raw" name="secretkey_encoding.raw"></a> `raw`
Raw bytes.

- <a href="#secretkey_encoding.pkcs8" name="secretkey_encoding.pkcs8"></a> `pkcs8`
PKCS8/DER encoding.

- <a href="#secretkey_encoding.pem" name="secretkey_encoding.pem"></a> `pem`
PEM encoding.

- <a href="#secretkey_encoding.sec" name="secretkey_encoding.sec"></a> `sec`
SEC encoding.

- <a href="#secretkey_encoding.compressed_sec" name="secretkey_encoding.compressed_sec"></a> `compressed_sec`
Compressed SEC encoding.

- <a href="#secretkey_encoding.local" name="secretkey_encoding.local"></a> `local`
Implementation-defined encoding.

## <a href="#signature_encoding" name="signature_encoding"></a> `signature_encoding`: `Variant`
Encoding to use for importing or exporting a signature.

Size: 2

Alignment: 2

### Variant cases
- <a href="#signature_encoding.raw" name="signature_encoding.raw"></a> `raw`
Raw bytes.

- <a href="#signature_encoding.der" name="signature_encoding.der"></a> `der`
DER encoding.

## <a href="#algorithm_type" name="algorithm_type"></a> `algorithm_type`: `Variant`
An algorithm category.

Size: 2

Alignment: 2

### Variant cases
- <a href="#algorithm_type.signatures" name="algorithm_type.signatures"></a> `signatures`

- <a href="#algorithm_type.symmetric" name="algorithm_type.symmetric"></a> `symmetric`

- <a href="#algorithm_type.key_exchange" name="algorithm_type.key_exchange"></a> `key_exchange`

## <a href="#version" name="version"></a> `version`: `u64`
Version of a managed key.

A version can be an arbitrary [`u64`](#u64) integer, with the expection of some reserved values.

Size: 8

Alignment: 8

### Constants
- <a href="#version.unspecified" name="version.unspecified"></a> `unspecified`

- <a href="#version.latest" name="version.latest"></a> `latest`

- <a href="#version.all" name="version.all"></a> `all`

## <a href="#size" name="size"></a> `size`: `usize`
Size of a value.

Size: 4

Alignment: 4

## <a href="#timestamp" name="timestamp"></a> `timestamp`: `u64`
A UNIX timestamp, in seconds since 01/01/1970.

Size: 8

Alignment: 8

## <a href="#array_output" name="array_output"></a> `array_output`: `Handle`
Handle for functions returning output whose size may be large or not known in advance.

An [`array_output`](#array_output) object contains a host-allocated byte array.

A guest can get the size of that array after a function returns in order to then allocate a buffer of the correct size.
In addition, the content of such an object can be consumed by a guest in a streaming fashion.

An [`array_output`](#array_output) handle is automatically closed after its full content has been consumed.

Size: 4

Alignment: 4

### Supertypes
## <a href="#options" name="options"></a> `options`: `Handle`
A set of options.

This type is used to set non-default parameters.

The exact set of allowed options depends on the algorithm being used.

Size: 4

Alignment: 4

### Supertypes
## <a href="#secrets_manager" name="secrets_manager"></a> `secrets_manager`: `Handle`
A handle to the optional secrets management facilities offered by a host.

This is used to generate, retrieve and invalidate managed keys.

Size: 4

Alignment: 4

### Supertypes
## <a href="#keypair" name="keypair"></a> `keypair`: `Handle`
A key pair.

Size: 4

Alignment: 4

### Supertypes
## <a href="#signature_state" name="signature_state"></a> `signature_state`: `Handle`
A state to absorb data to be signed.

After a signature has been computed or verified, the state remains valid for further operations.

A subsequent signature would sign all the data accumulated since the creation of the state object.

Size: 4

Alignment: 4

### Supertypes
## <a href="#signature" name="signature"></a> `signature`: `Handle`
A signature.

Size: 4

Alignment: 4

### Supertypes
## <a href="#publickey" name="publickey"></a> `publickey`: `Handle`
A public key, for key exchange and signature verification.

Size: 4

Alignment: 4

### Supertypes
## <a href="#secretkey" name="secretkey"></a> `secretkey`: `Handle`
A secret key, for key exchange mechanisms.

Size: 4

Alignment: 4

### Supertypes
## <a href="#signature_verification_state" name="signature_verification_state"></a> `signature_verification_state`: `Handle`
A state to absorb signed data to be verified.

Size: 4

Alignment: 4

### Supertypes
## <a href="#symmetric_state" name="symmetric_state"></a> `symmetric_state`: `Handle`
A state to perform symmetric operations.

The state is not reset nor invalidated after an option has been performed.
Incremental updates and sessions are thus supported.

Size: 4

Alignment: 4

### Supertypes
## <a href="#symmetric_key" name="symmetric_key"></a> `symmetric_key`: `Handle`
A symmetric key.

The key can be imported from raw bytes, or can be a reference to a managed key.

If it was imported, the host will wipe it from memory as soon as the handle is closed.

Size: 4

Alignment: 4

### Supertypes
## <a href="#symmetric_tag" name="symmetric_tag"></a> `symmetric_tag`: `Handle`
An authentication tag.

This is an object returned by functions computing authentication tags.

A tag can be compared against another tag (directly supplied as raw bytes) in constant time with the `symmetric_tag_verify()` function.

This object type can't be directly created from raw bytes. They are only returned by functions computing MACs.

The host is reponsible for securely wiping them from memory on close.

Size: 4

Alignment: 4

### Supertypes
## <a href="#opt_options_u" name="opt_options_u"></a> `opt_options_u`: `Variant`
Options index, only required by the Interface Types translation layer.

Size: 1

Alignment: 1

### Variant cases
- <a href="#opt_options_u.some" name="opt_options_u.some"></a> `some`

- <a href="#opt_options_u.none" name="opt_options_u.none"></a> `none`

## <a href="#opt_options" name="opt_options"></a> `opt_options`: `Variant`
An optional options set.

This union simulates an `Option<Options>` type to make the [`options`](#options) parameter of some functions optional.

Size: 8

Alignment: 4

### Variant Layout
- size: 8
- align: 4
- tag_size: 1
### Variant cases
- <a href="#opt_options.some" name="opt_options.some"></a> `some`: [`options`](#options)

- <a href="#opt_options.none" name="opt_options.none"></a> `none`

## <a href="#opt_symmetric_key_u" name="opt_symmetric_key_u"></a> `opt_symmetric_key_u`: `Variant`
Symmetric key index, only required by the Interface Types translation layer.

Size: 1

Alignment: 1

### Variant cases
- <a href="#opt_symmetric_key_u.some" name="opt_symmetric_key_u.some"></a> `some`

- <a href="#opt_symmetric_key_u.none" name="opt_symmetric_key_u.none"></a> `none`

## <a href="#opt_symmetric_key" name="opt_symmetric_key"></a> `opt_symmetric_key`: `Variant`
An optional symmetric key.

This union simulates an `Option<SymmetricKey>` type to make the [`symmetric_key`](#symmetric_key) parameter of some functions optional.

Size: 8

Alignment: 4

### Variant Layout
- size: 8
- align: 4
- tag_size: 1
### Variant cases
- <a href="#opt_symmetric_key.some" name="opt_symmetric_key.some"></a> `some`: [`symmetric_key`](#symmetric_key)

- <a href="#opt_symmetric_key.none" name="opt_symmetric_key.none"></a> `none`

## <a href="#u64" name="u64"></a> `u64`: `u64`

Size: 8

Alignment: 8

# Modules
## <a href="#wasi_ephemeral_crypto_common" name="wasi_ephemeral_crypto_common"></a> wasi_ephemeral_crypto_common
### Imports
#### Memory
### Functions

---

#### <a href="#options_open" name="options_open"></a> `options_open(algorithm_type: algorithm_type) -> Result<options, crypto_errno>`
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
- <a href="#options_open.error" name="options_open.error"></a> `error`: `Result<options, crypto_errno>`

###### Variant Layout
- size: 8
- align: 4
- tag_size: 4
###### Variant cases
- <a href="#options_open.error.ok" name="options_open.error.ok"></a> `ok`: [`options`](#options)

- <a href="#options_open.error.err" name="options_open.error.err"></a> `err`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#options_close" name="options_close"></a> `options_close(handle: options) -> Result<(), crypto_errno>`
Destroy an options object.

Objects are reference counted. It is safe to close an object immediately after the last function needing it is called.

##### Params
- <a href="#options_close.handle" name="options_close.handle"></a> `handle`: [`options`](#options)

##### Results
- <a href="#options_close.error" name="options_close.error"></a> `error`: `Result<(), crypto_errno>`

###### Variant Layout
- size: 8
- align: 4
- tag_size: 4
###### Variant cases
- <a href="#options_close.error.ok" name="options_close.error.ok"></a> `ok`

- <a href="#options_close.error.err" name="options_close.error.err"></a> `err`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#options_set" name="options_set"></a> `options_set(handle: options, name: string, value: ConstPointer<u8>, value_len: size) -> Result<(), crypto_errno>`
Set or update an option.

This is used to set algorithm-specific parameters, but also to provide credentials for the secrets management facilities, if required.

This function may return `unsupported_option` if an option that doesn't exist for any implemented algorithms is specified.

##### Params
- <a href="#options_set.handle" name="options_set.handle"></a> `handle`: [`options`](#options)

- <a href="#options_set.name" name="options_set.name"></a> `name`: `string`

- <a href="#options_set.value" name="options_set.value"></a> `value`: `ConstPointer<u8>`

- <a href="#options_set.value_len" name="options_set.value_len"></a> `value_len`: [`size`](#size)

##### Results
- <a href="#options_set.error" name="options_set.error"></a> `error`: `Result<(), crypto_errno>`

###### Variant Layout
- size: 8
- align: 4
- tag_size: 4
###### Variant cases
- <a href="#options_set.error.ok" name="options_set.error.ok"></a> `ok`

- <a href="#options_set.error.err" name="options_set.error.err"></a> `err`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#options_set_u64" name="options_set_u64"></a> `options_set_u64(handle: options, name: string, value: u64) -> Result<(), crypto_errno>`
Set or update an integer option.

This is used to set algorithm-specific parameters.

This function may return `unsupported_option` if an option that doesn't exist for any implemented algorithms is specified.

##### Params
- <a href="#options_set_u64.handle" name="options_set_u64.handle"></a> `handle`: [`options`](#options)

- <a href="#options_set_u64.name" name="options_set_u64.name"></a> `name`: `string`

- <a href="#options_set_u64.value" name="options_set_u64.value"></a> `value`: `u64`

##### Results
- <a href="#options_set_u64.error" name="options_set_u64.error"></a> `error`: `Result<(), crypto_errno>`

###### Variant Layout
- size: 8
- align: 4
- tag_size: 4
###### Variant cases
- <a href="#options_set_u64.error.ok" name="options_set_u64.error.ok"></a> `ok`

- <a href="#options_set_u64.error.err" name="options_set_u64.error.err"></a> `err`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#options_set_guest_buffer" name="options_set_guest_buffer"></a> `options_set_guest_buffer(handle: options, name: string, buffer: Pointer<u8>, buffer_len: size) -> Result<(), crypto_errno>`
Set or update a guest-allocated memory that the host can use or return data into.

This is for example used to set the scratch buffer required by memory-hard functions.

This function may return `unsupported_option` if an option that doesn't exist for any implemented algorithms is specified.

##### Params
- <a href="#options_set_guest_buffer.handle" name="options_set_guest_buffer.handle"></a> `handle`: [`options`](#options)

- <a href="#options_set_guest_buffer.name" name="options_set_guest_buffer.name"></a> `name`: `string`

- <a href="#options_set_guest_buffer.buffer" name="options_set_guest_buffer.buffer"></a> `buffer`: `Pointer<u8>`

- <a href="#options_set_guest_buffer.buffer_len" name="options_set_guest_buffer.buffer_len"></a> `buffer_len`: [`size`](#size)

##### Results
- <a href="#options_set_guest_buffer.error" name="options_set_guest_buffer.error"></a> `error`: `Result<(), crypto_errno>`

###### Variant Layout
- size: 8
- align: 4
- tag_size: 4
###### Variant cases
- <a href="#options_set_guest_buffer.error.ok" name="options_set_guest_buffer.error.ok"></a> `ok`

- <a href="#options_set_guest_buffer.error.err" name="options_set_guest_buffer.error.err"></a> `err`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#array_output_len" name="array_output_len"></a> `array_output_len(array_output: array_output) -> Result<size, crypto_errno>`
Return the length of an [`array_output`](#array_output) object.

This allows a guest to allocate a buffer of the correct size in order to copy the output of a function returning this object type.

##### Params
- <a href="#array_output_len.array_output" name="array_output_len.array_output"></a> `array_output`: [`array_output`](#array_output)

##### Results
- <a href="#array_output_len.error" name="array_output_len.error"></a> `error`: `Result<size, crypto_errno>`

###### Variant Layout
- size: 8
- align: 4
- tag_size: 4
###### Variant cases
- <a href="#array_output_len.error.ok" name="array_output_len.error.ok"></a> `ok`: [`size`](#size)

- <a href="#array_output_len.error.err" name="array_output_len.error.err"></a> `err`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#array_output_pull" name="array_output_pull"></a> `array_output_pull(array_output: array_output, buf: Pointer<u8>, buf_len: size) -> Result<size, crypto_errno>`
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
- <a href="#array_output_pull.error" name="array_output_pull.error"></a> `error`: `Result<size, crypto_errno>`

###### Variant Layout
- size: 8
- align: 4
- tag_size: 4
###### Variant cases
- <a href="#array_output_pull.error.ok" name="array_output_pull.error.ok"></a> `ok`: [`size`](#size)

- <a href="#array_output_pull.error.err" name="array_output_pull.error.err"></a> `err`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#secrets_manager_open" name="secrets_manager_open"></a> `secrets_manager_open(options: opt_options) -> Result<secrets_manager, crypto_errno>`
__(optional)__
Create a context to use a secrets manager.

The set of required and supported options is defined by the host.

The function returns the `unsupported_feature` error code if secrets management facilities are not supported by the host.
This is also an optional import, meaning that the function may not even exist.

##### Params
- <a href="#secrets_manager_open.options" name="secrets_manager_open.options"></a> `options`: [`opt_options`](#opt_options)

##### Results
- <a href="#secrets_manager_open.error" name="secrets_manager_open.error"></a> `error`: `Result<secrets_manager, crypto_errno>`

###### Variant Layout
- size: 8
- align: 4
- tag_size: 4
###### Variant cases
- <a href="#secrets_manager_open.error.ok" name="secrets_manager_open.error.ok"></a> `ok`: [`secrets_manager`](#secrets_manager)

- <a href="#secrets_manager_open.error.err" name="secrets_manager_open.error.err"></a> `err`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#secrets_manager_close" name="secrets_manager_close"></a> `secrets_manager_close(secrets_manager: secrets_manager) -> Result<(), crypto_errno>`
__(optional)__
Destroy a secrets manager context.

The function returns the `unsupported_feature` error code if secrets management facilities are not supported by the host.
This is also an optional import, meaning that the function may not even exist.

##### Params
- <a href="#secrets_manager_close.secrets_manager" name="secrets_manager_close.secrets_manager"></a> `secrets_manager`: [`secrets_manager`](#secrets_manager)

##### Results
- <a href="#secrets_manager_close.error" name="secrets_manager_close.error"></a> `error`: `Result<(), crypto_errno>`

###### Variant Layout
- size: 8
- align: 4
- tag_size: 4
###### Variant cases
- <a href="#secrets_manager_close.error.ok" name="secrets_manager_close.error.ok"></a> `ok`

- <a href="#secrets_manager_close.error.err" name="secrets_manager_close.error.err"></a> `err`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#secrets_manager_invalidate" name="secrets_manager_invalidate"></a> `secrets_manager_invalidate(secrets_manager: secrets_manager, key_id: ConstPointer<u8>, key_id_len: size, key_version: version) -> Result<(), crypto_errno>`
__(optional)__
Invalidate a managed key or key pair given an identifier and a version.

This asks the secrets manager to delete or revoke a stored key, a specific version of a key.

`key_version` can be set to a version number, to [`version.latest`](#version.latest) to invalidate the current version, or to [`version.all`](#version.all) to invalidate all versions of a key.

The function returns `unsupported_feature` if this operation is not supported by the host, and `not_found` if the identifier and version don't match any existing key.

This is an optional import, meaning that the function may not even exist.

##### Params
- <a href="#secrets_manager_invalidate.secrets_manager" name="secrets_manager_invalidate.secrets_manager"></a> `secrets_manager`: [`secrets_manager`](#secrets_manager)

- <a href="#secrets_manager_invalidate.key_id" name="secrets_manager_invalidate.key_id"></a> `key_id`: `ConstPointer<u8>`

- <a href="#secrets_manager_invalidate.key_id_len" name="secrets_manager_invalidate.key_id_len"></a> `key_id_len`: [`size`](#size)

- <a href="#secrets_manager_invalidate.key_version" name="secrets_manager_invalidate.key_version"></a> `key_version`: [`version`](#version)

##### Results
- <a href="#secrets_manager_invalidate.error" name="secrets_manager_invalidate.error"></a> `error`: `Result<(), crypto_errno>`

###### Variant Layout
- size: 8
- align: 4
- tag_size: 4
###### Variant cases
- <a href="#secrets_manager_invalidate.error.ok" name="secrets_manager_invalidate.error.ok"></a> `ok`

- <a href="#secrets_manager_invalidate.error.err" name="secrets_manager_invalidate.error.err"></a> `err`: [`crypto_errno`](#crypto_errno)

## <a href="#wasi_ephemeral_crypto_exernal_secrets" name="wasi_ephemeral_crypto_exernal_secrets"></a> wasi_ephemeral_crypto_exernal_secrets
### Imports
#### Memory
### Functions

---

#### <a href="#external_secret_store" name="external_secret_store"></a> `external_secret_store(secrets_manager: secrets_manager, secret: ConstPointer<u8>, secret_len: size, expiration: timestamp, secret_id: Pointer<u8>, secret_id_max_len: size) -> Result<(), crypto_errno>`
Store an external secret into the secrets manager.

`$expiration` is the expiration date of the secret as a UNIX timestamp, in seconds.
An expiration date is mandatory.

On success, the secret identifier is put into `$secret_id` if it fits into `$secret_id_max_len` bytes.
If the supplied ouptut buffer is too small, `$overflow` is returned.

If this function is not supported by the host the `$unsupported_feature` error is returned.

##### Params
- <a href="#external_secret_store.secrets_manager" name="external_secret_store.secrets_manager"></a> `secrets_manager`: [`secrets_manager`](#secrets_manager)

- <a href="#external_secret_store.secret" name="external_secret_store.secret"></a> `secret`: `ConstPointer<u8>`

- <a href="#external_secret_store.secret_len" name="external_secret_store.secret_len"></a> `secret_len`: [`size`](#size)

- <a href="#external_secret_store.expiration" name="external_secret_store.expiration"></a> `expiration`: [`timestamp`](#timestamp)

- <a href="#external_secret_store.secret_id" name="external_secret_store.secret_id"></a> `secret_id`: `Pointer<u8>`

- <a href="#external_secret_store.secret_id_max_len" name="external_secret_store.secret_id_max_len"></a> `secret_id_max_len`: [`size`](#size)

##### Results
- <a href="#external_secret_store.error" name="external_secret_store.error"></a> `error`: `Result<(), crypto_errno>`

###### Variant Layout
- size: 8
- align: 4
- tag_size: 4
###### Variant cases
- <a href="#external_secret_store.error.ok" name="external_secret_store.error.ok"></a> `ok`

- <a href="#external_secret_store.error.err" name="external_secret_store.error.err"></a> `err`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#external_secret_replace" name="external_secret_replace"></a> `external_secret_replace(secrets_manager: secrets_manager, secret: ConstPointer<u8>, secret_len: size, expiration: timestamp, secret_id: ConstPointer<u8>, secret_id_len: size) -> Result<version, crypto_errno>`
Replace a managed external with a new version.

`$expiration` is the expiration date of the secret as a UNIX timestamp, in seconds.
An expiration date is mandatory.

On success, a new version is created and returned.

If this function is not supported by the host the `$unsupported_feature` error is returned.

##### Params
- <a href="#external_secret_replace.secrets_manager" name="external_secret_replace.secrets_manager"></a> `secrets_manager`: [`secrets_manager`](#secrets_manager)

- <a href="#external_secret_replace.secret" name="external_secret_replace.secret"></a> `secret`: `ConstPointer<u8>`

- <a href="#external_secret_replace.secret_len" name="external_secret_replace.secret_len"></a> `secret_len`: [`size`](#size)

- <a href="#external_secret_replace.expiration" name="external_secret_replace.expiration"></a> `expiration`: [`timestamp`](#timestamp)

- <a href="#external_secret_replace.secret_id" name="external_secret_replace.secret_id"></a> `secret_id`: `ConstPointer<u8>`

- <a href="#external_secret_replace.secret_id_len" name="external_secret_replace.secret_id_len"></a> `secret_id_len`: [`size`](#size)

##### Results
- <a href="#external_secret_replace.error" name="external_secret_replace.error"></a> `error`: `Result<version, crypto_errno>`

###### Variant Layout
- size: 16
- align: 8
- tag_size: 4
###### Variant cases
- <a href="#external_secret_replace.error.ok" name="external_secret_replace.error.ok"></a> `ok`: [`version`](#version)

- <a href="#external_secret_replace.error.err" name="external_secret_replace.error.err"></a> `err`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#external_secret_from_id" name="external_secret_from_id"></a> `external_secret_from_id(secrets_manager: secrets_manager, secret_id: ConstPointer<u8>, secret_id_len: size, secret_version: version) -> Result<array_output, crypto_errno>`
Get a copy of an external secret given an identifier and version.

`secret_version` can be set to a version number, or to [`version.latest`](#version.latest) to retrieve the most recent version of a secret.

On success, a copy of the secret is returned.

The function returns `$unsupported_feature` if this operation is not supported by the host, and `not_found` if the identifier and version don't match any existing secret.

##### Params
- <a href="#external_secret_from_id.secrets_manager" name="external_secret_from_id.secrets_manager"></a> `secrets_manager`: [`secrets_manager`](#secrets_manager)

- <a href="#external_secret_from_id.secret_id" name="external_secret_from_id.secret_id"></a> `secret_id`: `ConstPointer<u8>`

- <a href="#external_secret_from_id.secret_id_len" name="external_secret_from_id.secret_id_len"></a> `secret_id_len`: [`size`](#size)

- <a href="#external_secret_from_id.secret_version" name="external_secret_from_id.secret_version"></a> `secret_version`: [`version`](#version)

##### Results
- <a href="#external_secret_from_id.error" name="external_secret_from_id.error"></a> `error`: `Result<array_output, crypto_errno>`

###### Variant Layout
- size: 8
- align: 4
- tag_size: 4
###### Variant cases
- <a href="#external_secret_from_id.error.ok" name="external_secret_from_id.error.ok"></a> `ok`: [`array_output`](#array_output)

- <a href="#external_secret_from_id.error.err" name="external_secret_from_id.error.err"></a> `err`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#external_secret_invalidate" name="external_secret_invalidate"></a> `external_secret_invalidate(secrets_manager: secrets_manager, secret_id: ConstPointer<u8>, secret_id_len: size, secret_version: version) -> Result<(), crypto_errno>`
Invalidate an external secret given an identifier and a version.

This asks the secrets manager to delete or revoke a stored secret, a specific version of a secret.

`secret_version` can be set to a version number, or to [`version.latest`](#version.latest) to invalidate the current version, or to [`version.all`](#version.all) to invalidate all versions of a secret.

The function returns `$unsupported_feature` if this operation is not supported by the host, and `not_found` if the identifier and version don't match any existing secret.

##### Params
- <a href="#external_secret_invalidate.secrets_manager" name="external_secret_invalidate.secrets_manager"></a> `secrets_manager`: [`secrets_manager`](#secrets_manager)

- <a href="#external_secret_invalidate.secret_id" name="external_secret_invalidate.secret_id"></a> `secret_id`: `ConstPointer<u8>`

- <a href="#external_secret_invalidate.secret_id_len" name="external_secret_invalidate.secret_id_len"></a> `secret_id_len`: [`size`](#size)

- <a href="#external_secret_invalidate.secret_version" name="external_secret_invalidate.secret_version"></a> `secret_version`: [`version`](#version)

##### Results
- <a href="#external_secret_invalidate.error" name="external_secret_invalidate.error"></a> `error`: `Result<(), crypto_errno>`

###### Variant Layout
- size: 8
- align: 4
- tag_size: 4
###### Variant cases
- <a href="#external_secret_invalidate.error.ok" name="external_secret_invalidate.error.ok"></a> `ok`

- <a href="#external_secret_invalidate.error.err" name="external_secret_invalidate.error.err"></a> `err`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#external_secret_encapsulate" name="external_secret_encapsulate"></a> `external_secret_encapsulate(secrets_manager: secrets_manager, secret: ConstPointer<u8>, secret_len: size, expiration: timestamp) -> Result<array_output, crypto_errno>`
Encrypt an external secret.

Applications don't have access to the encryption key, and the secrets manager is free to choose any suitable algorithm.

However, the returned ciphertext must include and authenticate both the secret and the expiration date.

On success, the ciphertext is returned.

##### Params
- <a href="#external_secret_encapsulate.secrets_manager" name="external_secret_encapsulate.secrets_manager"></a> `secrets_manager`: [`secrets_manager`](#secrets_manager)

- <a href="#external_secret_encapsulate.secret" name="external_secret_encapsulate.secret"></a> `secret`: `ConstPointer<u8>`

- <a href="#external_secret_encapsulate.secret_len" name="external_secret_encapsulate.secret_len"></a> `secret_len`: [`size`](#size)

- <a href="#external_secret_encapsulate.expiration" name="external_secret_encapsulate.expiration"></a> `expiration`: [`timestamp`](#timestamp)

##### Results
- <a href="#external_secret_encapsulate.error" name="external_secret_encapsulate.error"></a> `error`: `Result<array_output, crypto_errno>`

###### Variant Layout
- size: 8
- align: 4
- tag_size: 4
###### Variant cases
- <a href="#external_secret_encapsulate.error.ok" name="external_secret_encapsulate.error.ok"></a> `ok`: [`array_output`](#array_output)

- <a href="#external_secret_encapsulate.error.err" name="external_secret_encapsulate.error.err"></a> `err`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#external_secret_decapsulate" name="external_secret_decapsulate"></a> `external_secret_decapsulate(secrets_manager: secrets_manager, encrypted_secret: ConstPointer<u8>, encrypted_secret_len: size) -> Result<array_output, crypto_errno>`
Decrypt an external secret previously encrypted by the secrets manager.

Returns the original secret if the ciphertext is valid.
Returns `$expired` if the current date is past the stored expiration date.
Returns `$verification_failed` if the ciphertext format is invalid or if its authentication tag couldn't be verified.

##### Params
- <a href="#external_secret_decapsulate.secrets_manager" name="external_secret_decapsulate.secrets_manager"></a> `secrets_manager`: [`secrets_manager`](#secrets_manager)

- <a href="#external_secret_decapsulate.encrypted_secret" name="external_secret_decapsulate.encrypted_secret"></a> `encrypted_secret`: `ConstPointer<u8>`

- <a href="#external_secret_decapsulate.encrypted_secret_len" name="external_secret_decapsulate.encrypted_secret_len"></a> `encrypted_secret_len`: [`size`](#size)

##### Results
- <a href="#external_secret_decapsulate.error" name="external_secret_decapsulate.error"></a> `error`: `Result<array_output, crypto_errno>`

###### Variant Layout
- size: 8
- align: 4
- tag_size: 4
###### Variant cases
- <a href="#external_secret_decapsulate.error.ok" name="external_secret_decapsulate.error.ok"></a> `ok`: [`array_output`](#array_output)

- <a href="#external_secret_decapsulate.error.err" name="external_secret_decapsulate.error.err"></a> `err`: [`crypto_errno`](#crypto_errno)


