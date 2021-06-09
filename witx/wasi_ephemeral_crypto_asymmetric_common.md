
# Module: wasi_ephemeral_crypto_asymmetric_common

## Table of contents

### Types list:

[**[All](#types)**] - [_[`crypto_errno`](#crypto_errno)_] - [_[`keypair_encoding`](#keypair_encoding)_] - [_[`publickey_encoding`](#publickey_encoding)_] - [_[`secretkey_encoding`](#secretkey_encoding)_] - [_[`signature_encoding`](#signature_encoding)_] - [_[`algorithm_type`](#algorithm_type)_] - [_[`version`](#version)_] - [_[`size`](#size)_] - [_[`timestamp`](#timestamp)_] - [_[`u64`](#u64)_] - [_[`array_output`](#array_output)_] - [_[`options`](#options)_] - [_[`secrets_manager`](#secrets_manager)_] - [_[`keypair`](#keypair)_] - [_[`signature_state`](#signature_state)_] - [_[`signature`](#signature)_] - [_[`publickey`](#publickey)_] - [_[`secretkey`](#secretkey)_] - [_[`signature_verification_state`](#signature_verification_state)_] - [_[`symmetric_state`](#symmetric_state)_] - [_[`symmetric_key`](#symmetric_key)_] - [_[`symmetric_tag`](#symmetric_tag)_] - [_[`opt_options_u`](#opt_options_u)_] - [_[`opt_options`](#opt_options)_] - [_[`opt_symmetric_key_u`](#opt_symmetric_key_u)_] - [_[`opt_symmetric_key`](#opt_symmetric_key)_]

### Functions list:

[**[All](#functions)**] - [[`keypair_generate()`](#keypair_generate)] - [[`keypair_import()`](#keypair_import)] - [[`keypair_generate_managed()`](#keypair_generate_managed)] - [[`keypair_store_managed()`](#keypair_store_managed)] - [[`keypair_replace_managed()`](#keypair_replace_managed)] - [[`keypair_id()`](#keypair_id)] - [[`keypair_from_id()`](#keypair_from_id)] - [[`keypair_from_pk_and_sk()`](#keypair_from_pk_and_sk)] - [[`keypair_export()`](#keypair_export)] - [[`keypair_publickey()`](#keypair_publickey)] - [[`keypair_secretkey()`](#keypair_secretkey)] - [[`keypair_close()`](#keypair_close)] - [[`publickey_import()`](#publickey_import)] - [[`publickey_export()`](#publickey_export)] - [[`publickey_verify()`](#publickey_verify)] - [[`publickey_from_secretkey()`](#publickey_from_secretkey)] - [[`publickey_close()`](#publickey_close)] - [[`secretkey_import()`](#secretkey_import)] - [[`secretkey_export()`](#secretkey_export)] - [[`secretkey_close()`](#secretkey_close)]

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

## Functions

### [`keypair_generate()`](#keypair_generate)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`algorithm_type`**: _[`algorithm_type`](#algorithm_type)_
* **`algorithm`**: `string`
* **`options`**: _[`opt_options`](#opt_options)_

#### Output:

* _[`keypair`](#keypair)_ mutable pointer

> Generate a new key pair.
> 
> Internally, a key pair stores the supplied algorithm and optional parameters.
> 
> Trying to use that key pair with different parameters will throw an `invalid_key` error.
> 
> This function may return `$crypto_errno.unsupported_feature` if key generation is not supported by the host for the chosen algorithm.
> 
> The function may also return `unsupported_algorithm` if the algorithm is not supported by the host.
> 
> Finally, if generating that type of key pair is an expensive operation, the function may return `in_progress`.
> In that case, the guest should retry with the same parameters until the function completes.
> 
> Example usage:
> 
> ```rust
> let kp_handle = ctx.keypair_generate(AlgorithmType::Signatures, "RSA_PKCS1_2048_SHA256", None)?;
> ```


---

### [`keypair_import()`](#keypair_import)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`algorithm_type`**: _[`algorithm_type`](#algorithm_type)_
* **`algorithm`**: `string`
* **`encoded`**: `u8` pointer
* **`encoded_len`**: _[`size`](#size)_
* **`encoding`**: _[`keypair_encoding`](#keypair_encoding)_

#### Output:

* _[`keypair`](#keypair)_ mutable pointer

> Import a key pair.
> 
> This function creates a `keypair` object from existing material.
> 
> It may return `unsupported_algorithm` if the encoding scheme is not supported, or `invalid_key` if the key cannot be decoded.
> 
> The function may also return `unsupported_algorithm` if the algorithm is not supported by the host.
> 
> Example usage:
> 
> ```rust
> let kp_handle = ctx.keypair_import(AlgorithmType::Signatures, "RSA_PKCS1_2048_SHA256", KeypairEncoding::PKCS8)?;
> ```


---

### [`keypair_generate_managed()`](#keypair_generate_managed)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`secrets_manager`**: _[`secrets_manager`](#secrets_manager)_
* **`algorithm_type`**: _[`algorithm_type`](#algorithm_type)_
* **`algorithm`**: `string`
* **`options`**: _[`opt_options`](#opt_options)_

#### Output:

* _[`keypair`](#keypair)_ mutable pointer

> __(optional)__
> Generate a new managed key pair.
> 
> The key pair is generated and stored by the secrets management facilities.
> 
> It may be used through its identifier, but the host may not allow it to be exported.
> 
> The function returns the `unsupported_feature` error code if secrets management facilities are not supported by the host,
> or `unsupported_algorithm` if a key cannot be created for the chosen algorithm.
> 
> The function may also return `unsupported_algorithm` if the algorithm is not supported by the host.
> 
> This is also an optional import, meaning that the function may not even exist.


---

### [`keypair_store_managed()`](#keypair_store_managed)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`secrets_manager`**: _[`secrets_manager`](#secrets_manager)_
* **`kp`**: _[`keypair`](#keypair)_
* **`kp_id`**: `u8` mutable pointer
* **`kp_id_max_len`**: _[`size`](#size)_

This function has no output.

> __(optional)__
> Store a key pair into the secrets manager.
> 
> On success, the function stores the key pair identifier into `$kp_id`,
> into which up to `$kp_id_max_len` can be written.
> 
> The function returns `overflow` if the supplied buffer is too small.


---

### [`keypair_replace_managed()`](#keypair_replace_managed)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`secrets_manager`**: _[`secrets_manager`](#secrets_manager)_
* **`kp_old`**: _[`keypair`](#keypair)_
* **`kp_new`**: _[`keypair`](#keypair)_

#### Output:

* _[`version`](#version)_ mutable pointer

> __(optional)__
> Replace a managed key pair.
> 
> This function crates a new version of a managed key pair, by replacing `$kp_old` with `$kp_new`.
> 
> It does several things:
> 
> - The key identifier for `$kp_new` is set to the one of `$kp_old`.
> - A new, unique version identifier is assigned to `$kp_new`. This version will be equivalent to using `$version_latest` until the key is replaced.
> - The `$kp_old` handle is closed.
> 
> Both keys must share the same algorithm and have compatible parameters. If this is not the case, `incompatible_keys` is returned.
> 
> The function may also return the `unsupported_feature` error code if secrets management facilities are not supported by the host,
> or if keys cannot be rotated.
> 
> Finally, `prohibited_operation` can be returned if `$kp_new` wasn't created by the secrets manager, and the secrets manager prohibits imported keys.
> 
> If the operation succeeded, the new version is returned.
> 
> This is an optional import, meaning that the function may not even exist.


---

### [`keypair_id()`](#keypair_id)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`kp`**: _[`keypair`](#keypair)_
* **`kp_id`**: `u8` mutable pointer
* **`kp_id_max_len`**: _[`size`](#size)_

#### Output:

* _[`size`](#size)_ mutable pointer
* _[`version`](#version)_ mutable pointer

> __(optional)__
> Return the key pair identifier and version of a managed key pair.
> 
> If the key pair is not managed, `unsupported_feature` is returned instead.
> 
> This is an optional import, meaning that the function may not even exist.


---

### [`keypair_from_id()`](#keypair_from_id)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`secrets_manager`**: _[`secrets_manager`](#secrets_manager)_
* **`kp_id`**: `u8` pointer
* **`kp_id_len`**: _[`size`](#size)_
* **`kp_version`**: _[`version`](#version)_

#### Output:

* _[`keypair`](#keypair)_ mutable pointer

> __(optional)__
> Return a managed key pair from a key identifier.
> 
> `kp_version` can be set to `version_latest` to retrieve the most recent version of a key pair.
> 
> If no key pair matching the provided information is found, `not_found` is returned instead.
> 
> This is an optional import, meaning that the function may not even exist.
> ```


---

### [`keypair_from_pk_and_sk()`](#keypair_from_pk_and_sk)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`publickey`**: _[`publickey`](#publickey)_
* **`secretkey`**: _[`secretkey`](#secretkey)_

#### Output:

* _[`keypair`](#keypair)_ mutable pointer

> Create a key pair from a public key and a secret key.


---

### [`keypair_export()`](#keypair_export)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`kp`**: _[`keypair`](#keypair)_
* **`encoding`**: _[`keypair_encoding`](#keypair_encoding)_

#### Output:

* _[`array_output`](#array_output)_ mutable pointer

> Export a key pair as the given encoding format.
> 
> May return `prohibited_operation` if this operation is denied or `unsupported_encoding` if the encoding is not supported.


---

### [`keypair_publickey()`](#keypair_publickey)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`kp`**: _[`keypair`](#keypair)_

#### Output:

* _[`publickey`](#publickey)_ mutable pointer

> Get the public key of a key pair.


---

### [`keypair_secretkey()`](#keypair_secretkey)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`kp`**: _[`keypair`](#keypair)_

#### Output:

* _[`secretkey`](#secretkey)_ mutable pointer

> Get the secret key of a key pair.


---

### [`keypair_close()`](#keypair_close)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`kp`**: _[`keypair`](#keypair)_

This function has no output.

> Destroy a key pair.
> 
> The host will automatically wipe traces of the secret key from memory.
> 
> If this is a managed key, the key will not be removed from persistent storage, and can be reconstructed later using the key identifier.


---

### [`publickey_import()`](#publickey_import)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`algorithm_type`**: _[`algorithm_type`](#algorithm_type)_
* **`algorithm`**: `string`
* **`encoded`**: `u8` pointer
* **`encoded_len`**: _[`size`](#size)_
* **`encoding`**: _[`publickey_encoding`](#publickey_encoding)_

#### Output:

* _[`publickey`](#publickey)_ mutable pointer

> Import a public key.
> 
> The function may return `unsupported_encoding` if importing from the given format is not implemented or incompatible with the key type.
> 
> It may also return `invalid_key` if the key doesn't appear to match the supplied algorithm.
> 
> Finally, the function may return `unsupported_algorithm` if the algorithm is not supported by the host.
> 
> Example usage:
> 
> ```rust
> let pk_handle = ctx.publickey_import(AlgorithmType::Signatures, encoded, PublicKeyEncoding::Sec)?;
> ```


---

### [`publickey_export()`](#publickey_export)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`pk`**: _[`publickey`](#publickey)_
* **`encoding`**: _[`publickey_encoding`](#publickey_encoding)_

#### Output:

* _[`array_output`](#array_output)_ mutable pointer

> Export a public key as the given encoding format.
> 
> May return `unsupported_encoding` if the encoding is not supported.


---

### [`publickey_verify()`](#publickey_verify)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`pk`**: _[`publickey`](#publickey)_

This function has no output.

> Check that a public key is valid and in canonical form.
> 
> This function may perform stricter checks than those made during importation at the expense of additional CPU cycles.
> 
> The function returns `invalid_key` if the public key didn't pass the checks.


---

### [`publickey_from_secretkey()`](#publickey_from_secretkey)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`sk`**: _[`secretkey`](#secretkey)_

#### Output:

* _[`publickey`](#publickey)_ mutable pointer

> Compute the public key for a secret key.


---

### [`publickey_close()`](#publickey_close)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`pk`**: _[`publickey`](#publickey)_

This function has no output.

> Destroy a public key.
> 
> Objects are reference counted. It is safe to close an object immediately after the last function needing it is called.


---

### [`secretkey_import()`](#secretkey_import)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`algorithm_type`**: _[`algorithm_type`](#algorithm_type)_
* **`algorithm`**: `string`
* **`encoded`**: `u8` pointer
* **`encoded_len`**: _[`size`](#size)_
* **`encoding`**: _[`secretkey_encoding`](#secretkey_encoding)_

#### Output:

* _[`secretkey`](#secretkey)_ mutable pointer

> Import a secret key.
> 
> The function may return `unsupported_encoding` if importing from the given format is not implemented or incompatible with the key type.
> 
> It may also return `invalid_key` if the key doesn't appear to match the supplied algorithm.
> 
> Finally, the function may return `unsupported_algorithm` if the algorithm is not supported by the host.
> 
> Example usage:
> 
> ```rust
> let pk_handle = ctx.secretkey_import(AlgorithmType::KX, encoded, SecretKeyEncoding::Raw)?;
> ```


---

### [`secretkey_export()`](#secretkey_export)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`sk`**: _[`secretkey`](#secretkey)_
* **`encoding`**: _[`secretkey_encoding`](#secretkey_encoding)_

#### Output:

* _[`array_output`](#array_output)_ mutable pointer

> Export a secret key as the given encoding format.
> 
> May return `unsupported_encoding` if the encoding is not supported.


---

### [`secretkey_close()`](#secretkey_close)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`sk`**: _[`secretkey`](#secretkey)_

This function has no output.

> Destroy a secret key.
> 
> Objects are reference counted. It is safe to close an object immediately after the last function needing it is called.


---

