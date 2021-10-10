
# Module: wasi_ephemeral_crypto_signatures

## Table of contents

### Types list:

[**[All](#types)**] - [_[`crypto_errno`](#crypto_errno)_] - [_[`keypair_encoding`](#keypair_encoding)_] - [_[`publickey_encoding`](#publickey_encoding)_] - [_[`secretkey_encoding`](#secretkey_encoding)_] - [_[`signature_encoding`](#signature_encoding)_] - [_[`algorithm_type`](#algorithm_type)_] - [_[`version`](#version)_] - [_[`size`](#size)_] - [_[`timestamp`](#timestamp)_] - [_[`u64`](#u64)_] - [_[`array_output`](#array_output)_] - [_[`options`](#options)_] - [_[`secrets_manager`](#secrets_manager)_] - [_[`keypair`](#keypair)_] - [_[`signature_state`](#signature_state)_] - [_[`signature`](#signature)_] - [_[`publickey`](#publickey)_] - [_[`secretkey`](#secretkey)_] - [_[`signature_verification_state`](#signature_verification_state)_] - [_[`symmetric_state`](#symmetric_state)_] - [_[`symmetric_key`](#symmetric_key)_] - [_[`symmetric_tag`](#symmetric_tag)_] - [_[`opt_options_u`](#opt_options_u)_] - [_[`opt_options`](#opt_options)_] - [_[`opt_symmetric_key_u`](#opt_symmetric_key_u)_] - [_[`opt_symmetric_key`](#opt_symmetric_key)_] - [_[`signature_keypair`](#signature_keypair)_] - [_[`signature_publickey`](#signature_publickey)_] - [_[`signature_secretkey`](#signature_secretkey)_]

### Functions list:

[**[All](#functions)**] - [[`signature_export()`](#signature_export)] - [[`signature_import()`](#signature_import)] - [[`signature_state_open()`](#signature_state_open)] - [[`signature_state_update()`](#signature_state_update)] - [[`signature_state_sign()`](#signature_state_sign)] - [[`signature_state_close()`](#signature_state_close)] - [[`signature_verification_state_open()`](#signature_verification_state_open)] - [[`signature_verification_state_update()`](#signature_verification_state_update)] - [[`signature_verification_state_verify()`](#signature_verification_state_verify)] - [[`signature_verification_state_close()`](#signature_verification_state_close)] - [[`signature_close()`](#signature_close)]

## Types

### _[`crypto_errno`](#crypto_errno)_

Enumeration with tag type: `u16`, and the following members:

* **`success`**: _[`crypto_errno`](#crypto_errno)_
* **`guest_error`**: _[`crypto_errno`](#crypto_errno)_
* **`not_implemented`**: _[`crypto_errno`](#crypto_errno)_
* **`unsupported_feature`**: _[`crypto_errno`](#crypto_errno)_
* **`prohibited_operation`**: _[`crypto_errno`](#crypto_errno)_
* **`unsupported_encoding`**: _[`crypto_errno`](#crypto_errno)_
* **`unsupported_algorithm`**: _[`crypto_errno`](#crypto_errno)_
* **`unsupported_option`**: _[`crypto_errno`](#crypto_errno)_
* **`invalid_key`**: _[`crypto_errno`](#crypto_errno)_
* **`invalid_length`**: _[`crypto_errno`](#crypto_errno)_
* **`verification_failed`**: _[`crypto_errno`](#crypto_errno)_
* **`rng_error`**: _[`crypto_errno`](#crypto_errno)_
* **`algorithm_failure`**: _[`crypto_errno`](#crypto_errno)_
* **`invalid_signature`**: _[`crypto_errno`](#crypto_errno)_
* **`closed`**: _[`crypto_errno`](#crypto_errno)_
* **`invalid_handle`**: _[`crypto_errno`](#crypto_errno)_
* **`overflow`**: _[`crypto_errno`](#crypto_errno)_
* **`internal_error`**: _[`crypto_errno`](#crypto_errno)_
* **`too_many_handles`**: _[`crypto_errno`](#crypto_errno)_
* **`key_not_supported`**: _[`crypto_errno`](#crypto_errno)_
* **`key_required`**: _[`crypto_errno`](#crypto_errno)_
* **`invalid_tag`**: _[`crypto_errno`](#crypto_errno)_
* **`invalid_operation`**: _[`crypto_errno`](#crypto_errno)_
* **`nonce_required`**: _[`crypto_errno`](#crypto_errno)_
* **`invalid_nonce`**: _[`crypto_errno`](#crypto_errno)_
* **`option_not_set`**: _[`crypto_errno`](#crypto_errno)_
* **`not_found`**: _[`crypto_errno`](#crypto_errno)_
* **`parameters_missing`**: _[`crypto_errno`](#crypto_errno)_
* **`in_progress`**: _[`crypto_errno`](#crypto_errno)_
* **`incompatible_keys`**: _[`crypto_errno`](#crypto_errno)_
* **`expired`**: _[`crypto_errno`](#crypto_errno)_

> Error codes.


---

### _[`keypair_encoding`](#keypair_encoding)_

Enumeration with tag type: `u16`, and the following members:

* **`raw`**: _[`keypair_encoding`](#keypair_encoding)_
* **`pkcs8`**: _[`keypair_encoding`](#keypair_encoding)_
* **`pem`**: _[`keypair_encoding`](#keypair_encoding)_
* **`local`**: _[`keypair_encoding`](#keypair_encoding)_

> Encoding to use for importing or exporting a key pair.


---

### _[`publickey_encoding`](#publickey_encoding)_

Enumeration with tag type: `u16`, and the following members:

* **`raw`**: _[`publickey_encoding`](#publickey_encoding)_
* **`pkcs8`**: _[`publickey_encoding`](#publickey_encoding)_
* **`pem`**: _[`publickey_encoding`](#publickey_encoding)_
* **`sec`**: _[`publickey_encoding`](#publickey_encoding)_
* **`compressed_sec`**: _[`publickey_encoding`](#publickey_encoding)_
* **`local`**: _[`publickey_encoding`](#publickey_encoding)_

> Encoding to use for importing or exporting a public key.


---

### _[`secretkey_encoding`](#secretkey_encoding)_

Enumeration with tag type: `u16`, and the following members:

* **`raw`**: _[`secretkey_encoding`](#secretkey_encoding)_
* **`pkcs8`**: _[`secretkey_encoding`](#secretkey_encoding)_
* **`pem`**: _[`secretkey_encoding`](#secretkey_encoding)_
* **`sec`**: _[`secretkey_encoding`](#secretkey_encoding)_
* **`compressed_sec`**: _[`secretkey_encoding`](#secretkey_encoding)_
* **`local`**: _[`secretkey_encoding`](#secretkey_encoding)_

> Encoding to use for importing or exporting a secret key.


---

### _[`signature_encoding`](#signature_encoding)_

Enumeration with tag type: `u16`, and the following members:

* **`raw`**: _[`signature_encoding`](#signature_encoding)_
* **`der`**: _[`signature_encoding`](#signature_encoding)_

> Encoding to use for importing or exporting a signature.


---

### _[`algorithm_type`](#algorithm_type)_

Enumeration with tag type: `u16`, and the following members:

* **`signatures`**: _[`algorithm_type`](#algorithm_type)_
* **`symmetric`**: _[`algorithm_type`](#algorithm_type)_
* **`key_exchange`**: _[`algorithm_type`](#algorithm_type)_

> An algorithm category.


---

### _[`version`](#version)_
Alias for `u64`.


> Version of a managed key.
> 
> A version can be an arbitrary `u64` integer, with the expection of some reserved values.


---

### _[`size`](#size)_
Alias for `usize`.


> Size of a value.


---

### _[`timestamp`](#timestamp)_
Alias for `u64`.


> A UNIX timestamp, in seconds since 01/01/1970.


---

### _[`u64`](#u64)_
Alias for `u64`.


> A 64-bit value


---

### _[`array_output`](#array_output)_
Alias for `handle`.


> Handle for functions returning output whose size may be large or not known in advance.
> 
> An `array_output` object contains a host-allocated byte array.
> 
> A guest can get the size of that array after a function returns in order to then allocate a buffer of the correct size.
> In addition, the content of such an object can be consumed by a guest in a streaming fashion.
> 
> An `array_output` handle is automatically closed after its full content has been consumed.


---

### _[`options`](#options)_
Alias for `handle`.


> A set of options.
> 
> This type is used to set non-default parameters.
> 
> The exact set of allowed options depends on the algorithm being used.


---

### _[`secrets_manager`](#secrets_manager)_
Alias for `handle`.


> A handle to the optional secrets management facilities offered by a host.
> 
> This is used to generate, retrieve and invalidate managed keys.


---

### _[`keypair`](#keypair)_
Alias for `handle`.


> A key pair.


---

### _[`signature_state`](#signature_state)_
Alias for `handle`.


> A state to absorb data to be signed.
> 
> After a signature has been computed or verified, the state remains valid for further operations.
> 
> A subsequent signature would sign all the data accumulated since the creation of the state object.


---

### _[`signature`](#signature)_
Alias for `handle`.


> A signature.


---

### _[`publickey`](#publickey)_
Alias for `handle`.


> A public key, for key exchange and signature verification.


---

### _[`secretkey`](#secretkey)_
Alias for `handle`.


> A secret key, for key exchange mechanisms.


---

### _[`signature_verification_state`](#signature_verification_state)_
Alias for `handle`.


> A state to absorb signed data to be verified.


---

### _[`symmetric_state`](#symmetric_state)_
Alias for `handle`.


> A state to perform symmetric operations.
> 
> The state is not reset nor invalidated after an option has been performed.
> Incremental updates and sessions are thus supported.


---

### _[`symmetric_key`](#symmetric_key)_
Alias for `handle`.


> A symmetric key.
> 
> The key can be imported from raw bytes, or can be a reference to a managed key.
> 
> If it was imported, the host will wipe it from memory as soon as the handle is closed.


---

### _[`symmetric_tag`](#symmetric_tag)_
Alias for `handle`.


> An authentication tag.
> 
> This is an object returned by functions computing authentication tags.
> 
> A tag can be compared against another tag (directly supplied as raw bytes) in constant time with the `symmetric_tag_verify()` function.
> 
> This object type can't be directly created from raw bytes. They are only returned by functions computing MACs.
> 
> The host is reponsible for securely wiping them from memory on close.


---

### _[`opt_options_u`](#opt_options_u)_

Enumeration with tag type: `u8`, and the following members:

* **`some`**: _[`opt_options_u`](#opt_options_u)_
* **`none`**: _[`opt_options_u`](#opt_options_u)_

> Options index, only required by the Interface Types translation layer.


---

### _[`opt_options`](#opt_options)_
Tagged union with tag type: `u8` and the following possibilities:

* **`some`**: _[`options`](#options)_
* **`none`**: _(empty)_

> An optional options set.
> 
> This union simulates an `Option\<Options\>` type to make the `options` parameter of some functions optional.


---

### _[`opt_symmetric_key_u`](#opt_symmetric_key_u)_

Enumeration with tag type: `u8`, and the following members:

* **`some`**: _[`opt_symmetric_key_u`](#opt_symmetric_key_u)_
* **`none`**: _[`opt_symmetric_key_u`](#opt_symmetric_key_u)_

> Symmetric key index, only required by the Interface Types translation layer.


---

### _[`opt_symmetric_key`](#opt_symmetric_key)_
Tagged union with tag type: `u8` and the following possibilities:

* **`some`**: _[`symmetric_key`](#symmetric_key)_
* **`none`**: _(empty)_

> An optional symmetric key.
> 
> This union simulates an `Option\<SymmetricKey\>` type to make the `symmetric_key` parameter of some functions optional.


---

### _[`signature_keypair`](#signature_keypair)_

Alias for `handle`.


> `$signature_keypair` is just an alias for `$keypair`
> 
> However, bindings may want to define a specialized type `signature_keypair` as a super class of `keypair`, with additional methods such as `sign`.


---

### _[`signature_publickey`](#signature_publickey)_

Alias for `handle`.


> `$signature_publickey` is just an alias for `$publickey`
> 
> However, bindings may want to define a specialized type `signature_publickey` as a super class of `publickey`, with additional methods such as `verify`.


---

### _[`signature_secretkey`](#signature_secretkey)_

Alias for `handle`.


> `$signature_secretkey` is just an alias for `$secretkey`
> 
> However, bindings may want to define a specialized type `signature_secretkey` as a super class of `secretkey`.


---

## Functions

### [`signature_export()`](#signature_export)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`signature`**: _[`signature`](#signature)_
* **`encoding`**: _[`signature_encoding`](#signature_encoding)_

#### Output:

* _[`array_output`](#array_output)_ mutable pointer

> Export a signature.
> 
> This function exports a signature object using the specified encoding.
> 
> May return `unsupported_encoding` if the signature cannot be encoded into the given format.


---

### [`signature_import()`](#signature_import)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`algorithm`**: `string`
* **`encoded`**: `u8` pointer
* **`encoded_len`**: _[`size`](#size)_
* **`encoding`**: _[`signature_encoding`](#signature_encoding)_

#### Output:

* _[`signature`](#signature)_ mutable pointer

> Create a signature object.
> 
> This object can be used along with a public key to verify an existing signature.
> 
> It may return `invalid_signature` if the signature is invalid or incompatible with the specified algorithm, as well as `unsupported_encoding` if the encoding is not compatible with the signature type.
> 
> The function may also return `unsupported_algorithm` if the algorithm is not supported by the host.
> 
> Example usage:
> 
> ```rust
> let signature_handle = ctx.signature_import("ECDSA_P256_SHA256", SignatureEncoding::DER, encoded)?;
> ```


---

### [`signature_state_open()`](#signature_state_open)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`kp`**: _[`signature_keypair`](#signature_keypair)_

#### Output:

* _[`signature_state`](#signature_state)_ mutable pointer

> Create a new state to collect data to compute a signature on.
> 
> This function allows data to be signed to be supplied in a streaming fashion.
> 
> The state is not closed and can be used after a signature has been computed, allowing incremental updates by calling `signature_state_update()` again afterwards.
> 
> Example usage - signature creation
> 
> ```rust
> let kp_handle = ctx.keypair_import(AlgorithmType::Signatures, "Ed25519ph", keypair, KeypairEncoding::Raw)?;
> let state_handle = ctx.signature_state_open(kp_handle)?;
> ctx.signature_state_update(state_handle, b"message part 1")?;
> ctx.signature_state_update(state_handle, b"message part 2")?;
> let sig_handle = ctx.signature_state_sign(state_handle)?;
> let raw_sig = ctx.signature_export(sig_handle, SignatureEncoding::Raw)?;
> ```


---

### [`signature_state_update()`](#signature_state_update)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`state`**: _[`signature_state`](#signature_state)_
* **`input`**: `u8` pointer
* **`input_len`**: _[`size`](#size)_

This function has no output.

> Absorb data into the signature state.
> 
> This function may return `unsupported_feature` is the selected algorithm doesn't support incremental updates.


---

### [`signature_state_sign()`](#signature_state_sign)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`state`**: _[`signature_state`](#signature_state)_

#### Output:

* _[`array_output`](#array_output)_ mutable pointer

> Compute a signature for all the data collected up to that point.
> 
> The function can be called multiple times for incremental signing.


---

### [`signature_state_close()`](#signature_state_close)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`state`**: _[`signature_state`](#signature_state)_

This function has no output.

> Destroy a signature state.
> 
> Objects are reference counted. It is safe to close an object immediately after the last function needing it is called.
> 
> Note that closing a signature state doesn't close or invalidate the key pair object, that be reused for further signatures.


---

### [`signature_verification_state_open()`](#signature_verification_state_open)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`kp`**: _[`signature_publickey`](#signature_publickey)_

#### Output:

* _[`signature_verification_state`](#signature_verification_state)_ mutable pointer

> Create a new state to collect data to verify a signature on.
> 
> This is the verification counterpart of `signature_state`.
> 
> Data can be injected using `signature_verification_state_update()`, and the state is not closed after a verification, allowing incremental verification.
> 
> Example usage - signature verification:
> 
> ```rust
> let pk_handle = ctx.publickey_import(AlgorithmType::Signatures, "ECDSA_P256_SHA256", encoded_pk, PublicKeyEncoding::CompressedSec)?;
> let signature_handle = ctx.signature_import(AlgorithmType::Signatures, "ECDSA_P256_SHA256", encoded_sig, SignatureEncoding::Der)?;
> let state_handle = ctx.signature_verification_state_open(pk_handle)?;
> ctx.signature_verification_state_update(state_handle, "message")?;
> ctx.signature_verification_state_verify(signature_handle)?;
> ```


---

### [`signature_verification_state_update()`](#signature_verification_state_update)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`state`**: _[`signature_verification_state`](#signature_verification_state)_
* **`input`**: `u8` pointer
* **`input_len`**: _[`size`](#size)_

This function has no output.

> Absorb data into the signature verification state.
> 
> This function may return `unsupported_feature` is the selected algorithm doesn't support incremental updates.


---

### [`signature_verification_state_verify()`](#signature_verification_state_verify)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`state`**: _[`signature_verification_state`](#signature_verification_state)_
* **`signature`**: _[`signature`](#signature)_

This function has no output.

> Check that the given signature is verifies for the data collected up to that point point.
> 
> The state is not closed and can absorb more data to allow for incremental verification.
> 
> The function returns `invalid_signature` if the signature doesn't appear to be valid.


---

### [`signature_verification_state_close()`](#signature_verification_state_close)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`state`**: _[`signature_verification_state`](#signature_verification_state)_

This function has no output.

> Destroy a signature verification state.
> 
> Objects are reference counted. It is safe to close an object immediately after the last function needing it is called.
> 
> Note that closing a signature state doesn't close or invalidate the public key object, that be reused for further verifications.


---

### [`signature_close()`](#signature_close)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`signature`**: _[`signature`](#signature)_

This function has no output.

> Destroy a signature.
> 
> Objects are reference counted. It is safe to close an object immediately after the last function needing it is called.


---

