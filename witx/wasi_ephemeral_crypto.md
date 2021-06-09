
# Module: wasi_ephemeral_crypto_common

## Table of contents

### Types list:

[**[All](#types)**] - [_[`crypto_errno`](#crypto_errno)_] - [_[`keypair_encoding`](#keypair_encoding)_] - [_[`publickey_encoding`](#publickey_encoding)_] - [_[`secretkey_encoding`](#secretkey_encoding)_] - [_[`signature_encoding`](#signature_encoding)_] - [_[`algorithm_type`](#algorithm_type)_] - [_[`version`](#version)_] - [_[`size`](#size)_] - [_[`timestamp`](#timestamp)_] - [_[`u64`](#u64)_] - [_[`array_output`](#array_output)_] - [_[`options`](#options)_] - [_[`secrets_manager`](#secrets_manager)_] - [_[`keypair`](#keypair)_] - [_[`signature_state`](#signature_state)_] - [_[`signature`](#signature)_] - [_[`publickey`](#publickey)_] - [_[`secretkey`](#secretkey)_] - [_[`signature_verification_state`](#signature_verification_state)_] - [_[`symmetric_state`](#symmetric_state)_] - [_[`symmetric_key`](#symmetric_key)_] - [_[`symmetric_tag`](#symmetric_tag)_] - [_[`opt_options_u`](#opt_options_u)_] - [_[`opt_options`](#opt_options)_] - [_[`opt_symmetric_key_u`](#opt_symmetric_key_u)_] - [_[`opt_symmetric_key`](#opt_symmetric_key)_]

### Functions list:

[**[All](#functions)**] - [[`options_open()`](#options_open)] - [[`options_close()`](#options_close)] - [[`options_set()`](#options_set)] - [[`options_set_u64()`](#options_set_u64)] - [[`options_set_guest_buffer()`](#options_set_guest_buffer)] - [[`array_output_len()`](#array_output_len)] - [[`array_output_pull()`](#array_output_pull)] - [[`secrets_manager_open()`](#secrets_manager_open)] - [[`secrets_manager_close()`](#secrets_manager_close)] - [[`secrets_manager_invalidate()`](#secrets_manager_invalidate)]

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

Predefined constants for _[`version`](#version)_:

* **`unspecified`** = `0xff00000000000000`
* **`latest`** = `0xff00000000000001`
* **`all`** = `0xff00000000000002`

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

### [`options_open()`](#options_open)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`algorithm_type`**: _[`algorithm_type`](#algorithm_type)_

#### Output:

* _[`options`](#options)_ mutable pointer

> Create a new object to set non-default options.
> 
> Example usage:
> 
> ```rust
> let options_handle = options_open(AlgorithmType::Symmetric)?;
> options_set(options_handle, "context", context)?;
> options_set_u64(options_handle, "threads", 4)?;
> let state = symmetric_state_open("BLAKE3", None, Some(options_handle))?;
> options_close(options_handle)?;
> ```


---

### [`options_close()`](#options_close)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`handle`**: _[`options`](#options)_

This function has no output.

> Destroy an options object.
> 
> Objects are reference counted. It is safe to close an object immediately after the last function needing it is called.


---

### [`options_set()`](#options_set)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`handle`**: _[`options`](#options)_
* **`name`**: `string`
* **`value`**: `u8` pointer
* **`value_len`**: _[`size`](#size)_

This function has no output.

> Set or update an option.
> 
> This is used to set algorithm-specific parameters, but also to provide credentials for the secrets management facilities, if required.
> 
> This function may return `unsupported_option` if an option that doesn't exist for any implemented algorithms is specified.


---

### [`options_set_u64()`](#options_set_u64)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`handle`**: _[`options`](#options)_
* **`name`**: `string`
* **`value`**: `u64`

This function has no output.

> Set or update an integer option.
> 
> This is used to set algorithm-specific parameters.
> 
> This function may return `unsupported_option` if an option that doesn't exist for any implemented algorithms is specified.


---

### [`options_set_guest_buffer()`](#options_set_guest_buffer)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`handle`**: _[`options`](#options)_
* **`name`**: `string`
* **`buffer`**: `u8` mutable pointer
* **`buffer_len`**: _[`size`](#size)_

This function has no output.

> Set or update a guest-allocated memory that the host can use or return data into.
> 
> This is for example used to set the scratch buffer required by memory-hard functions.
> 
> This function may return `unsupported_option` if an option that doesn't exist for any implemented algorithms is specified.


---

### [`array_output_len()`](#array_output_len)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`array_output`**: _[`array_output`](#array_output)_

#### Output:

* _[`size`](#size)_ mutable pointer

> Return the length of an `array_output` object.
> 
> This allows a guest to allocate a buffer of the correct size in order to copy the output of a function returning this object type.


---

### [`array_output_pull()`](#array_output_pull)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`array_output`**: _[`array_output`](#array_output)_
* **`buf`**: `u8` mutable pointer
* **`buf_len`**: _[`size`](#size)_

#### Output:

* _[`size`](#size)_ mutable pointer

> Copy the content of an `array_output` object into an application-allocated buffer.
> 
> Multiple calls to that function can be made in order to consume the data in a streaming fashion, if necessary.
> 
> The function returns the number of bytes that were actually copied. `0` means that the end of the stream has been reached. The total size always matches the output of `array_output_len()`.
> 
> The handle is automatically closed after all the data has been consumed.
> 
> Example usage:
> 
> ```rust
> let len = array_output_len(output_handle)?;
> let mut out = vec![0u8; len];
> array_output_pull(output_handle, &mut out)?;
> ```


---

### [`secrets_manager_open()`](#secrets_manager_open)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`options`**: _[`opt_options`](#opt_options)_

#### Output:

* _[`secrets_manager`](#secrets_manager)_ mutable pointer

> __(optional)__
> Create a context to use a secrets manager.
> 
> The set of required and supported options is defined by the host.
> 
> The function returns the `unsupported_feature` error code if secrets management facilities are not supported by the host.
> This is also an optional import, meaning that the function may not even exist.


---

### [`secrets_manager_close()`](#secrets_manager_close)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`secrets_manager`**: _[`secrets_manager`](#secrets_manager)_

This function has no output.

> __(optional)__
> Destroy a secrets manager context.
> 
> The function returns the `unsupported_feature` error code if secrets management facilities are not supported by the host.
> This is also an optional import, meaning that the function may not even exist.


---

### [`secrets_manager_invalidate()`](#secrets_manager_invalidate)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`secrets_manager`**: _[`secrets_manager`](#secrets_manager)_
* **`key_id`**: `u8` pointer
* **`key_id_len`**: _[`size`](#size)_
* **`key_version`**: _[`version`](#version)_

This function has no output.

> __(optional)__
> Invalidate a managed key or key pair given an identifier and a version.
> 
> This asks the secrets manager to delete or revoke a stored key, a specific version of a key.
> 
> `key_version` can be set to a version number, to `version.latest` to invalidate the current version, or to `version.all` to invalidate all versions of a key.
> 
> The function returns `unsupported_feature` if this operation is not supported by the host, and `not_found` if the identifier and version don't match any existing key.
> 
> This is an optional import, meaning that the function may not even exist.


---


# Module: wasi_ephemeral_crypto_asymmetric_common

## Table of contents

### Types list:

[**[All](#types)**]

### Functions list:

[**[All](#functions)**] - [[`keypair_generate()`](#keypair_generate)] - [[`keypair_import()`](#keypair_import)] - [[`keypair_generate_managed()`](#keypair_generate_managed)] - [[`keypair_store_managed()`](#keypair_store_managed)] - [[`keypair_replace_managed()`](#keypair_replace_managed)] - [[`keypair_id()`](#keypair_id)] - [[`keypair_from_id()`](#keypair_from_id)] - [[`keypair_from_pk_and_sk()`](#keypair_from_pk_and_sk)] - [[`keypair_export()`](#keypair_export)] - [[`keypair_publickey()`](#keypair_publickey)] - [[`keypair_secretkey()`](#keypair_secretkey)] - [[`keypair_close()`](#keypair_close)] - [[`publickey_import()`](#publickey_import)] - [[`publickey_export()`](#publickey_export)] - [[`publickey_verify()`](#publickey_verify)] - [[`publickey_from_secretkey()`](#publickey_from_secretkey)] - [[`publickey_close()`](#publickey_close)] - [[`secretkey_import()`](#secretkey_import)] - [[`secretkey_export()`](#secretkey_export)] - [[`secretkey_close()`](#secretkey_close)]

## Types

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


# Module: wasi_ephemeral_crypto_symmetric

## Table of contents

### Types list:

[**[All](#types)**]

### Functions list:

[**[All](#functions)**] - [[`symmetric_key_generate()`](#symmetric_key_generate)] - [[`symmetric_key_import()`](#symmetric_key_import)] - [[`symmetric_key_export()`](#symmetric_key_export)] - [[`symmetric_key_close()`](#symmetric_key_close)] - [[`symmetric_key_generate_managed()`](#symmetric_key_generate_managed)] - [[`symmetric_key_store_managed()`](#symmetric_key_store_managed)] - [[`symmetric_key_replace_managed()`](#symmetric_key_replace_managed)] - [[`symmetric_key_id()`](#symmetric_key_id)] - [[`symmetric_key_from_id()`](#symmetric_key_from_id)] - [[`symmetric_state_open()`](#symmetric_state_open)] - [[`symmetric_state_options_get()`](#symmetric_state_options_get)] - [[`symmetric_state_options_get_u64()`](#symmetric_state_options_get_u64)] - [[`symmetric_state_close()`](#symmetric_state_close)] - [[`symmetric_state_absorb()`](#symmetric_state_absorb)] - [[`symmetric_state_squeeze()`](#symmetric_state_squeeze)] - [[`symmetric_state_squeeze_tag()`](#symmetric_state_squeeze_tag)] - [[`symmetric_state_squeeze_key()`](#symmetric_state_squeeze_key)] - [[`symmetric_state_max_tag_len()`](#symmetric_state_max_tag_len)] - [[`symmetric_state_encrypt()`](#symmetric_state_encrypt)] - [[`symmetric_state_encrypt_detached()`](#symmetric_state_encrypt_detached)] - [[`symmetric_state_decrypt()`](#symmetric_state_decrypt)] - [[`symmetric_state_decrypt_detached()`](#symmetric_state_decrypt_detached)] - [[`symmetric_state_ratchet()`](#symmetric_state_ratchet)] - [[`symmetric_tag_len()`](#symmetric_tag_len)] - [[`symmetric_tag_pull()`](#symmetric_tag_pull)] - [[`symmetric_tag_verify()`](#symmetric_tag_verify)] - [[`symmetric_tag_close()`](#symmetric_tag_close)]

## Types

## Functions

### [`symmetric_key_generate()`](#symmetric_key_generate)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`algorithm`**: `string`
* **`options`**: _[`opt_options`](#opt_options)_

#### Output:

* _[`symmetric_key`](#symmetric_key)_ mutable pointer

> Generate a new symmetric key for a given algorithm.
> 
> `options` can be `None` to use the default parameters, or an algoritm-specific set of parameters to override.
> 
> This function may return `unsupported_feature` if key generation is not supported by the host for the chosen algorithm, or `unsupported_algorithm` if the algorithm is not supported by the host.


---

### [`symmetric_key_import()`](#symmetric_key_import)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`algorithm`**: `string`
* **`raw`**: `u8` pointer
* **`raw_len`**: _[`size`](#size)_

#### Output:

* _[`symmetric_key`](#symmetric_key)_ mutable pointer

> Create a symmetric key from raw material.
> 
> The algorithm is internally stored along with the key, and trying to use the key with an operation expecting a different algorithm will return `invalid_key`.
> 
> The function may also return `unsupported_algorithm` if the algorithm is not supported by the host.


---

### [`symmetric_key_export()`](#symmetric_key_export)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`symmetric_key`**: _[`symmetric_key`](#symmetric_key)_

#### Output:

* _[`array_output`](#array_output)_ mutable pointer

> Export a symmetric key as raw material.
> 
> This is mainly useful to export a managed key.
> 
> May return `prohibited_operation` if this operation is denied.


---

### [`symmetric_key_close()`](#symmetric_key_close)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`symmetric_key`**: _[`symmetric_key`](#symmetric_key)_

This function has no output.

> Destroy a symmetric key.
> 
> Objects are reference counted. It is safe to close an object immediately after the last function needing it is called.


---

### [`symmetric_key_generate_managed()`](#symmetric_key_generate_managed)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`secrets_manager`**: _[`secrets_manager`](#secrets_manager)_
* **`algorithm`**: `string`
* **`options`**: _[`opt_options`](#opt_options)_

#### Output:

* _[`symmetric_key`](#symmetric_key)_ mutable pointer

> __(optional)__
> Generate a new managed symmetric key.
> 
> The key is generated and stored by the secrets management facilities.
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

### [`symmetric_key_store_managed()`](#symmetric_key_store_managed)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`secrets_manager`**: _[`secrets_manager`](#secrets_manager)_
* **`symmetric_key`**: _[`symmetric_key`](#symmetric_key)_
* **`symmetric_key_id`**: `u8` mutable pointer
* **`symmetric_key_id_max_len`**: _[`size`](#size)_

This function has no output.

> __(optional)__
> Store a symmetric key into the secrets manager.
> 
> On success, the function stores the key identifier into `$symmetric_key_id`,
> into which up to `$symmetric_key_id_max_len` can be written.
> 
> The function returns `overflow` if the supplied buffer is too small.


---

### [`symmetric_key_replace_managed()`](#symmetric_key_replace_managed)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`secrets_manager`**: _[`secrets_manager`](#secrets_manager)_
* **`symmetric_key_old`**: _[`symmetric_key`](#symmetric_key)_
* **`symmetric_key_new`**: _[`symmetric_key`](#symmetric_key)_

#### Output:

* _[`version`](#version)_ mutable pointer

> __(optional)__
> Replace a managed symmetric key.
> 
> This function crates a new version of a managed symmetric key, by replacing `$kp_old` with `$kp_new`.
> 
> It does several things:
> 
> - The key identifier for `$symmetric_key_new` is set to the one of `$symmetric_key_old`.
> - A new, unique version identifier is assigned to `$kp_new`. This version will be equivalent to using `$version_latest` until the key is replaced.
> - The `$symmetric_key_old` handle is closed.
> 
> Both keys must share the same algorithm and have compatible parameters. If this is not the case, `incompatible_keys` is returned.
> 
> The function may also return the `unsupported_feature` error code if secrets management facilities are not supported by the host,
> or if keys cannot be rotated.
> 
> Finally, `prohibited_operation` can be returned if `$symmetric_key_new` wasn't created by the secrets manager, and the secrets manager prohibits imported keys.
> 
> If the operation succeeded, the new version is returned.
> 
> This is an optional import, meaning that the function may not even exist.


---

### [`symmetric_key_id()`](#symmetric_key_id)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`symmetric_key`**: _[`symmetric_key`](#symmetric_key)_
* **`symmetric_key_id`**: `u8` mutable pointer
* **`symmetric_key_id_max_len`**: _[`size`](#size)_

#### Output:

* _[`size`](#size)_ mutable pointer
* _[`version`](#version)_ mutable pointer

> __(optional)__
> Return the key identifier and version of a managed symmetric key.
> 
> If the key is not managed, `unsupported_feature` is returned instead.
> 
> This is an optional import, meaning that the function may not even exist.


---

### [`symmetric_key_from_id()`](#symmetric_key_from_id)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`secrets_manager`**: _[`secrets_manager`](#secrets_manager)_
* **`symmetric_key_id`**: `u8` pointer
* **`symmetric_key_id_len`**: _[`size`](#size)_
* **`symmetric_key_version`**: _[`version`](#version)_

#### Output:

* _[`symmetric_key`](#symmetric_key)_ mutable pointer

> __(optional)__
> Return a managed symmetric key from a key identifier.
> 
> `kp_version` can be set to `version_latest` to retrieve the most recent version of a symmetric key.
> 
> If no key matching the provided information is found, `not_found` is returned instead.
> 
> This is an optional import, meaning that the function may not even exist.


---

### [`symmetric_state_open()`](#symmetric_state_open)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`algorithm`**: `string`
* **`key`**: _[`opt_symmetric_key`](#opt_symmetric_key)_
* **`options`**: _[`opt_options`](#opt_options)_

#### Output:

* _[`symmetric_state`](#symmetric_state)_ mutable pointer

> Create a new state to aborb and produce data using symmetric operations.
> 
> The state remains valid after every operation in order to support incremental updates.
> 
> The function has two optional parameters: a key and an options set.
> 
> It will fail with a `key_not_supported` error code if a key was provided but the chosen algorithm doesn't natively support keying.
> 
> On the other hand, if a key is required, but was not provided, a `key_required` error will be thrown.
> 
> Some algorithms may require additional parameters. They have to be supplied as an options set:
> 
> ```rust
> let options_handle = ctx.options_open()?;
> ctx.options_set("context", b"My application")?;
> ctx.options_set_u64("fanout", 16)?;
> let state_handle = ctx.symmetric_state_open("BLAKE2b-512", None, Some(options_handle))?;
> ```
> 
> If some parameters are mandatory but were not set, the `parameters_missing` error code will be returned.
> 
> A notable exception is the `nonce` parameter, that is common to most AEAD constructions.
> 
> If a nonce is required but was not supplied:
> 
> - If it is safe to do so, the host will automatically generate a nonce. This is true for nonces that are large enough to be randomly generated, or if the host is able to maintain a global counter.
> - If not, the function will fail and return the dedicated `nonce_required` error code.
> 
> A nonce that was automatically generated can be retrieved after the function returns with `symmetric_state_get(state_handle, "nonce")`.
> 
> **Sample usage patterns:**
> 
> - **Hashing**
> 
> ```rust
> let mut out = [0u8; 64];
> let state_handle = ctx.symmetric_state_open("SHAKE-128", None, None)?;
> ctx.symmetric_state_absorb(state_handle, b"data")?;
> ctx.symmetric_state_absorb(state_handle, b"more_data")?;
> ctx.symmetric_state_squeeze(state_handle, &mut out)?;
> ```
> 
> - **MAC**
> 
> ```rust
> let mut raw_tag = [0u8; 64];
> let key_handle = ctx.symmetric_key_import("HMAC/SHA-512", b"key")?;
> let state_handle = ctx.symmetric_state_open("HMAC/SHA-512", Some(key_handle), None)?;
> ctx.symmetric_state_absorb(state_handle, b"data")?;
> ctx.symmetric_state_absorb(state_handle, b"more_data")?;
> let computed_tag_handle = ctx.symmetric_state_squeeze_tag(state_handle)?;
> ctx.symmetric_tag_pull(computed_tag_handle, &mut raw_tag)?;
> ```
> 
> Verification:
> 
> ```rust
> let state_handle = ctx.symmetric_state_open("HMAC/SHA-512", Some(key_handle), None)?;
> ctx.symmetric_state_absorb(state_handle, b"data")?;
> ctx.symmetric_state_absorb(state_handle, b"more_data")?;
> let computed_tag_handle = ctx.symmetric_state_squeeze_tag(state_handle)?;
> ctx.symmetric_tag_verify(computed_tag_handle, expected_raw_tag)?;
> ```
> 
> - **Tuple hashing**
> 
> ```rust
> let mut out = [0u8; 64];
> let state_handle = ctx.symmetric_state_open("TupleHashXOF256", None, None)?;
> ctx.symmetric_state_absorb(state_handle, b"value 1")?;
> ctx.symmetric_state_absorb(state_handle, b"value 2")?;
> ctx.symmetric_state_absorb(state_handle, b"value 3")?;
> ctx.symmetric_state_squeeze(state_handle, &mut out)?;
> ```
> Unlike MACs and regular hash functions, inputs are domain separated instead of being concatenated.
> 
> - **Key derivation using extract-and-expand**
> 
> Extract:
> 
> ```rust
> let mut prk = vec![0u8; 64];
> let key_handle = ctx.symmetric_key_import("HKDF-EXTRACT/SHA-512", b"key")?;
> let state_handle = ctx.symmetric_state_open("HKDF-EXTRACT/SHA-512", Some(key_handle), None)?;
> ctx.symmetric_state_absorb(state_handle, b"salt")?;
> let prk_handle = ctx.symmetric_state_squeeze_key(state_handle, "HKDF-EXPAND/SHA-512")?;
> ```
> 
> Expand:
> 
> ```rust
> let mut subkey = vec![0u8; 32];
> let state_handle = ctx.symmetric_state_open("HKDF-EXPAND/SHA-512", Some(prk_handle), None)?;
> ctx.symmetric_state_absorb(state_handle, b"info")?;
> ctx.symmetric_state_squeeze(state_handle, &mut subkey)?;
> ```
> 
> - **Key derivation using a XOF**
> 
> ```rust
> let mut subkey1 = vec![0u8; 32];
> let mut subkey2 = vec![0u8; 32];
> let key_handle = ctx.symmetric_key_import("BLAKE3", b"key")?;
> let state_handle = ctx.symmetric_state_open("BLAKE3", Some(key_handle), None)?;
> ctx.symmetric_absorb(state_handle, b"context")?;
> ctx.squeeze(state_handle, &mut subkey1)?;
> ctx.squeeze(state_handle, &mut subkey2)?;
> ```
> 
> - **Password hashing**
> 
> ```rust
> let mut memory = vec![0u8; 1_000_000_000];
> let options_handle = ctx.symmetric_options_open()?;
> ctx.symmetric_options_set_guest_buffer(options_handle, "memory", &mut memory)?;
> ctx.symmetric_options_set_u64(options_handle, "opslimit", 5)?;
> ctx.symmetric_options_set_u64(options_handle, "parallelism", 8)?;
> 
> let state_handle = ctx.symmetric_state_open("ARGON2-ID-13", None, Some(options))?;
> ctx.symmtric_state_absorb(state_handle, b"password")?;
> 
> let pw_str_handle = ctx.symmetric_state_squeeze_tag(state_handle)?;
> let mut pw_str = vec![0u8; ctx.symmetric_tag_len(pw_str_handle)?];
> ctx.symmetric_tag_pull(pw_str_handle, &mut pw_str)?;
> ```
> 
> - **AEAD encryption with an explicit nonce**
> 
> ```rust
> let key_handle = ctx.symmetric_key_generate("AES-256-GCM", None)?;
> let message = b"test";
> 
> let options_handle = ctx.symmetric_options_open()?;
> ctx.symmetric_options_set(options_handle, "nonce", nonce)?;
> 
> let state_handle = ctx.symmetric_state_open("AES-256-GCM", Some(key_handle), Some(options_handle))?;
> let mut ciphertext = vec![0u8; message.len() + ctx.symmetric_state_max_tag_len(state_handle)?];
> ctx.symmetric_state_absorb(state_handle, "additional data")?;
> ctx.symmetric_state_encrypt(state_handle, &mut ciphertext, message)?;
> ```
> 
> - **AEAD encryption with automatic nonce generation**
> 
> ```rust
> let key_handle = ctx.symmetric_key_generate("AES-256-GCM-SIV", None)?;
> let message = b"test";
> let mut nonce = [0u8; 24];
> 
> let state_handle = ctx.symmetric_state_open("AES-256-GCM-SIV", Some(key_handle), None)?;
> 
> let nonce_handle = ctx.symmetric_state_options_get(state_handle, "nonce")?;
> ctx.array_output_pull(nonce_handle, &mut nonce)?;
> 
> let mut ciphertext = vec![0u8; message.len() + ctx.symmetric_state_max_tag_len(state_handle)?];
> ctx.symmetric_state_absorb(state_handle, "additional data")?;
> ctx.symmetric_state_encrypt(state_handle, &mut ciphertext, message)?;
> ```
> 
> - **Session authenticated modes**
> 
> ```rust
> let mut out = [0u8; 16];
> let mut out2 = [0u8; 16];
> let mut ciphertext = [0u8; 20];
> let key_handle = ctx.symmetric_key_generate("Xoodyak-128", None)?;
> let state_handle = ctx.symmetric_state_open("Xoodyak-128", Some(key_handle), None)?;
> ctx.symmetric_state_absorb(state_handle, b"data")?;
> ctx.symmetric_state_encrypt(state_handle, &mut ciphertext, b"abcd")?;
> ctx.symmetric_state_absorb(state_handle, b"more data")?;
> ctx.symmetric_state_squeeze(state_handle, &mut out)?;
> ctx.symmetric_state_squeeze(state_handle, &mut out2)?;
> ctx.symmetric_state_ratchet(state_handle)?;
> ctx.symmetric_state_absorb(state_handle, b"more data")?;
> let next_key_handle = ctx.symmetric_state_squeeze_key(state_handle, "Xoodyak-128")?;
> // ...
> ```


---

### [`symmetric_state_options_get()`](#symmetric_state_options_get)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`handle`**: _[`symmetric_state`](#symmetric_state)_
* **`name`**: `string`
* **`value`**: `u8` mutable pointer
* **`value_max_len`**: _[`size`](#size)_

#### Output:

* _[`size`](#size)_ mutable pointer

> Retrieve a parameter from the current state.
> 
> In particular, `symmetric_state_options_get("nonce")` can be used to get a nonce that as automatically generated.
> 
> The function may return `options_not_set` if an option was not set, which is different from an empty value.
> 
> It may also return `unsupported_option` if the option doesn't exist for the chosen algorithm.


---

### [`symmetric_state_options_get_u64()`](#symmetric_state_options_get_u64)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`handle`**: _[`symmetric_state`](#symmetric_state)_
* **`name`**: `string`

#### Output:

* _[`u64`](#u64)_ mutable pointer

> Retrieve an integer parameter from the current state.
> 
> In particular, `symmetric_state_options_get("nonce")` can be used to get a nonce that as automatically generated.
> 
> The function may return `options_not_set` if an option was not set.
> 
> It may also return `unsupported_option` if the option doesn't exist for the chosen algorithm.


---

### [`symmetric_state_close()`](#symmetric_state_close)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`handle`**: _[`symmetric_state`](#symmetric_state)_

This function has no output.

> Destroy a symmetric state.
> 
> Objects are reference counted. It is safe to close an object immediately after the last function needing it is called.


---

### [`symmetric_state_absorb()`](#symmetric_state_absorb)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`handle`**: _[`symmetric_state`](#symmetric_state)_
* **`data`**: `u8` pointer
* **`data_len`**: _[`size`](#size)_

This function has no output.

> Absorb data into the state.
> 
> - **Hash functions:** adds data to be hashed.
> - **MAC functions:** adds data to be authenticated.
> - **Tuplehash-like constructions:** adds a new tuple to the state.
> - **Key derivation functions:** adds to the IKM or to the subkey information.
> - **AEAD constructions:** adds additional data to be authenticated.
> - **Stateful hash objects, permutation-based constructions:** absorbs.
> 
> If the chosen algorithm doesn't accept input data, the `invalid_operation` error code is returned.
> 
> If too much data has been fed for the algorithm, `overflow` may be thrown.


---

### [`symmetric_state_squeeze()`](#symmetric_state_squeeze)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`handle`**: _[`symmetric_state`](#symmetric_state)_
* **`out`**: `u8` mutable pointer
* **`out_len`**: _[`size`](#size)_

This function has no output.

> Squeeze bytes from the state.
> 
> - **Hash functions:** this tries to output an `out_len` bytes digest from the absorbed data. The hash function output will be truncated if necessary. If the requested size is too large, the `invalid_len` error code is returned.
> - **Key derivation functions:** : outputs an arbitrary-long derived key.
> - **RNGs, DRBGs, stream ciphers:**: outputs arbitrary-long data.
> - **Stateful hash objects, permutation-based constructions:** squeeze.
> 
> Other kinds of algorithms may return `invalid_operation` instead.
> 
> For password-stretching functions, the function may return `in_progress`.
> In that case, the guest should retry with the same parameters until the function completes.


---

### [`symmetric_state_squeeze_tag()`](#symmetric_state_squeeze_tag)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`handle`**: _[`symmetric_state`](#symmetric_state)_

#### Output:

* _[`symmetric_tag`](#symmetric_tag)_ mutable pointer

> Compute and return a tag for all the data injected into the state so far.
> 
> - **MAC functions**: returns a tag authenticating the absorbed data.
> - **Tuplehash-like constructions:** returns a tag authenticating all the absorbed tuples.
> - **Password-hashing functions:** returns a standard string containing all the required parameters for password verification.
> 
> Other kinds of algorithms may return `invalid_operation` instead.
> 
> For password-stretching functions, the function may return `in_progress`.
> In that case, the guest should retry with the same parameters until the function completes.


---

### [`symmetric_state_squeeze_key()`](#symmetric_state_squeeze_key)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`handle`**: _[`symmetric_state`](#symmetric_state)_
* **`alg_str`**: `string`

#### Output:

* _[`symmetric_key`](#symmetric_key)_ mutable pointer

> Use the current state to produce a key for a target algorithm.
> 
> For extract-then-expand constructions, this returns the PRK.
> For session-base authentication encryption, this returns a key that can be used to resume a session without storing a nonce.
> 
> `invalid_operation` is returned for algorithms not supporting this operation.


---

### [`symmetric_state_max_tag_len()`](#symmetric_state_max_tag_len)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`handle`**: _[`symmetric_state`](#symmetric_state)_

#### Output:

* _[`size`](#size)_ mutable pointer

> Return the maximum length of an authentication tag for the current algorithm.
> 
> This allows guests to compute the size required to store a ciphertext along with its authentication tag.
> 
> The returned length may include the encryption mode's padding requirements in addition to the actual tag.
> 
> For an encryption operation, the size of the output buffer should be `input_len + symmetric_state_max_tag_len()`.
> 
> For a decryption operation, the size of the buffer that will store the decrypted data must be `ciphertext_len - symmetric_state_max_tag_len()`.


---

### [`symmetric_state_encrypt()`](#symmetric_state_encrypt)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`handle`**: _[`symmetric_state`](#symmetric_state)_
* **`out`**: `u8` mutable pointer
* **`out_len`**: _[`size`](#size)_
* **`data`**: `u8` pointer
* **`data_len`**: _[`size`](#size)_

#### Output:

* _[`size`](#size)_ mutable pointer

> Encrypt data with an attached tag.
> 
> - **Stream cipher:** adds the input to the stream cipher output. `out_len` and `data_len` can be equal, as no authentication tags will be added.
> - **AEAD:** encrypts `data` into `out`, including the authentication tag to the output. Additional data must have been previously absorbed using `symmetric_state_absorb()`. The `symmetric_state_max_tag_len()` function can be used to retrieve the overhead of adding the tag, as well as padding if necessary.
> - **SHOE, Xoodyak, Strobe:** encrypts data, squeezes a tag and appends it to the output.
> 
> If `out` and `data` are the same address, encryption may happen in-place.
> 
> The function returns the actual size of the ciphertext along with the tag.
> 
> `invalid_operation` is returned for algorithms not supporting encryption.


---

### [`symmetric_state_encrypt_detached()`](#symmetric_state_encrypt_detached)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`handle`**: _[`symmetric_state`](#symmetric_state)_
* **`out`**: `u8` mutable pointer
* **`out_len`**: _[`size`](#size)_
* **`data`**: `u8` pointer
* **`data_len`**: _[`size`](#size)_

#### Output:

* _[`symmetric_tag`](#symmetric_tag)_ mutable pointer

> Encrypt data, with a detached tag.
> 
> - **Stream cipher:** returns `invalid_operation` since stream ciphers do not include authentication tags.
> - **AEAD:** encrypts `data` into `out` and returns the tag separately. Additional data must have been previously absorbed using `symmetric_state_absorb()`. The output and input buffers must be of the same length.
> - **SHOE, Xoodyak, Strobe:** encrypts data and squeezes a tag.
> 
> If `out` and `data` are the same address, encryption may happen in-place.
> 
> The function returns the tag.
> 
> `invalid_operation` is returned for algorithms not supporting encryption.


---

### [`symmetric_state_decrypt()`](#symmetric_state_decrypt)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`handle`**: _[`symmetric_state`](#symmetric_state)_
* **`out`**: `u8` mutable pointer
* **`out_len`**: _[`size`](#size)_
* **`data`**: `u8` pointer
* **`data_len`**: _[`size`](#size)_

#### Output:

* _[`size`](#size)_ mutable pointer

> - **Stream cipher:** adds the input to the stream cipher output. `out_len` and `data_len` can be equal, as no authentication tags will be added.
> - **AEAD:** decrypts `data` into `out`. Additional data must have been previously absorbed using `symmetric_state_absorb()`.
> - **SHOE, Xoodyak, Strobe:** decrypts data, squeezes a tag and verify that it matches the one that was appended to the ciphertext.
> 
> If `out` and `data` are the same address, decryption may happen in-place.
> 
> `out_len` must be exactly `data_len` + `max_tag_len` bytes.
> 
> The function returns the actual size of the decrypted message, which can be smaller than `out_len` for modes that requires padding.
> 
> `invalid_tag` is returned if the tag didn't verify.
> 
> `invalid_operation` is returned for algorithms not supporting encryption.


---

### [`symmetric_state_decrypt_detached()`](#symmetric_state_decrypt_detached)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`handle`**: _[`symmetric_state`](#symmetric_state)_
* **`out`**: `u8` mutable pointer
* **`out_len`**: _[`size`](#size)_
* **`data`**: `u8` pointer
* **`data_len`**: _[`size`](#size)_
* **`raw_tag`**: `u8` pointer
* **`raw_tag_len`**: _[`size`](#size)_

#### Output:

* _[`size`](#size)_ mutable pointer

> - **Stream cipher:** returns `invalid_operation` since stream ciphers do not include authentication tags.
> - **AEAD:** decrypts `data` into `out`. Additional data must have been previously absorbed using `symmetric_state_absorb()`.
> - **SHOE, Xoodyak, Strobe:** decrypts data, squeezes a tag and verify that it matches the expected one.
> 
> `raw_tag` is the expected tag, as raw bytes.
> 
> `out` and `data` be must have the same length.
> If they also share the same address, decryption may happen in-place.
> 
> The function returns the actual size of the decrypted message.
> 
> `invalid_tag` is returned if the tag verification failed.
> 
> `invalid_operation` is returned for algorithms not supporting encryption.


---

### [`symmetric_state_ratchet()`](#symmetric_state_ratchet)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`handle`**: _[`symmetric_state`](#symmetric_state)_

This function has no output.

> Make it impossible to recover the previous state.
> 
> This operation is supported by some systems keeping a rolling state over an entire session, for forward security.
> 
> `invalid_operation` is returned for algorithms not supporting ratcheting.


---

### [`symmetric_tag_len()`](#symmetric_tag_len)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`symmetric_tag`**: _[`symmetric_tag`](#symmetric_tag)_

#### Output:

* _[`size`](#size)_ mutable pointer

> Return the length of an authentication tag.
> 
> This function can be used by a guest to allocate the correct buffer size to copy a computed authentication tag.


---

### [`symmetric_tag_pull()`](#symmetric_tag_pull)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`symmetric_tag`**: _[`symmetric_tag`](#symmetric_tag)_
* **`buf`**: `u8` mutable pointer
* **`buf_len`**: _[`size`](#size)_

#### Output:

* _[`size`](#size)_ mutable pointer

> Copy an authentication tag into a guest-allocated buffer.
> 
> The handle automatically becomes invalid after this operation. Manually closing it is not required.
> 
> Example usage:
> 
> ```rust
> let mut raw_tag = [0u8; 16];
> ctx.symmetric_tag_pull(raw_tag_handle, &mut raw_tag)?;
> ```
> 
> The function returns `overflow` if the supplied buffer is too small to copy the tag.
> 
> Otherwise, it returns the number of bytes that have been copied.


---

### [`symmetric_tag_verify()`](#symmetric_tag_verify)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`symmetric_tag`**: _[`symmetric_tag`](#symmetric_tag)_
* **`expected_raw_tag_ptr`**: `u8` pointer
* **`expected_raw_tag_len`**: _[`size`](#size)_

This function has no output.

> Verify that a computed authentication tag matches the expected value, in constant-time.
> 
> The expected tag must be provided as a raw byte string.
> 
> The function returns `invalid_tag` if the tags don't match.
> 
> Example usage:
> 
> ```rust
> let key_handle = ctx.symmetric_key_import("HMAC/SHA-256", b"key")?;
> let state_handle = ctx.symmetric_state_open("HMAC/SHA-256", Some(key_handle), None)?;
> ctx.symmetric_state_absorb(state_handle, b"data")?;
> let computed_tag_handle = ctx.symmetric_state_squeeze_tag(state_handle)?;
> ctx.symmetric_tag_verify(computed_tag_handle, expected_raw_tag)?;
> ```


---

### [`symmetric_tag_close()`](#symmetric_tag_close)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`symmetric_tag`**: _[`symmetric_tag`](#symmetric_tag)_

This function has no output.

> Explicitly destroy an unused authentication tag.
> 
> This is usually not necessary, as `symmetric_tag_pull()` automatically closes a tag after it has been copied.
> 
> Objects are reference counted. It is safe to close an object immediately after the last function needing it is called.


---


# Module: wasi_ephemeral_crypto_signatures

## Table of contents

### Types list:

[**[All](#types)**] - [_[`signature_keypair`](#signature_keypair)_] - [_[`signature_publickey`](#signature_publickey)_] - [_[`signature_secretkey`](#signature_secretkey)_]

### Functions list:

[**[All](#functions)**] - [[`signature_export()`](#signature_export)] - [[`signature_import()`](#signature_import)] - [[`signature_state_open()`](#signature_state_open)] - [[`signature_state_update()`](#signature_state_update)] - [[`signature_state_sign()`](#signature_state_sign)] - [[`signature_state_close()`](#signature_state_close)] - [[`signature_verification_state_open()`](#signature_verification_state_open)] - [[`signature_verification_state_update()`](#signature_verification_state_update)] - [[`signature_verification_state_verify()`](#signature_verification_state_verify)] - [[`signature_verification_state_close()`](#signature_verification_state_close)] - [[`signature_close()`](#signature_close)]

## Types

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


# Module: wasi_ephemeral_crypto_kx

## Table of contents

### Types list:

[**[All](#types)**] - [_[`kx_keypair`](#kx_keypair)_] - [_[`kx_publickey`](#kx_publickey)_] - [_[`kx_secretkey`](#kx_secretkey)_]

### Functions list:

[**[All](#functions)**] - [[`kx_dh()`](#kx_dh)] - [[`kx_encapsulate()`](#kx_encapsulate)] - [[`kx_decapsulate()`](#kx_decapsulate)]

## Types

### _[`kx_keypair`](#kx_keypair)_

Alias for `handle`.


> `$kx_keypair` is just an alias for `$keypair`
> 
> However, bindings may want to define a specialized type `kx_keypair` as a super class of `keypair`.


---

### _[`kx_publickey`](#kx_publickey)_

Alias for `handle`.


> `$kx_publickey` is just an alias for `$publickey`
> 
> However, bindings may want to define a specialized type `kx_publickey` as a super class of `publickey`, with additional methods such as `dh`.


---

### _[`kx_secretkey`](#kx_secretkey)_

Alias for `handle`.


> `$kx_secretkey` is just an alias for `$secretkey`
> 
> However, bindings may want to define a specialized type `kx_secretkey` as a super class of `secretkeykey`, with additional methods such as `dh`.


---

## Functions

### [`kx_dh()`](#kx_dh)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`pk`**: _[`publickey`](#publickey)_
* **`sk`**: _[`secretkey`](#secretkey)_

#### Output:

* _[`array_output`](#array_output)_ mutable pointer

> Perform a simple Diffie-Hellman key exchange.
> 
> Both keys must be of the same type, or else the `$crypto_errno.incompatible_keys` error is returned.
> The algorithm also has to support this kind of key exchange. If this is not the case, the `$crypto_errno.invalid_operation` error is returned.
> 
> Otherwide, a raw shared key is returned, and can be imported as a symmetric key.
> ```


---

### [`kx_encapsulate()`](#kx_encapsulate)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`pk`**: _[`publickey`](#publickey)_

#### Output:

* _[`array_output`](#array_output)_ mutable pointer
* _[`array_output`](#array_output)_ mutable pointer

> Create a shared secret and encrypt it for the given public key.
> 
> This operation is only compatible with specific algorithms.
> If a selected algorithm doesn't support it, `$crypto_errno.invalid_operation` is returned.
> 
> On success, both the shared secret and its encrypted version are returned.


---

### [`kx_decapsulate()`](#kx_decapsulate)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`sk`**: _[`secretkey`](#secretkey)_
* **`encapsulated_secret`**: `u8` pointer
* **`encapsulated_secret_len`**: _[`size`](#size)_

#### Output:

* _[`array_output`](#array_output)_ mutable pointer

> Decapsulate an encapsulated secret crated with `kx_encapsulate`
> 
> Return the secret, or `$crypto_errno.verification_failed` on error.


---


# Module: wasi_ephemeral_crypto_external_secrets

## Table of contents

### Types list:

[**[All](#types)**]

### Functions list:

[**[All](#functions)**] - [[`external_secret_store()`](#external_secret_store)] - [[`external_secret_replace()`](#external_secret_replace)] - [[`external_secret_from_id()`](#external_secret_from_id)] - [[`external_secret_invalidate()`](#external_secret_invalidate)] - [[`external_secret_encapsulate()`](#external_secret_encapsulate)] - [[`external_secret_decapsulate()`](#external_secret_decapsulate)]

## Types

## Functions

### [`external_secret_store()`](#external_secret_store)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`secrets_manager`**: _[`secrets_manager`](#secrets_manager)_
* **`secret`**: `u8` pointer
* **`secret_len`**: _[`size`](#size)_
* **`expiration`**: _[`timestamp`](#timestamp)_
* **`secret_id`**: `u8` mutable pointer
* **`secret_id_max_len`**: _[`size`](#size)_

This function has no output.

> Store an external secret into the secrets manager.
> 
> `$expiration` is the expiration date of the secret as a UNIX timestamp, in seconds.
> An expiration date is mandatory.
> 
> On success, the secret identifier is put into `$secret_id` if it fits into `$secret_id_max_len` bytes.
> If the supplied ouptut buffer is too small, `$overflow` is returned.
> 
> If this function is not supported by the host the `$unsupported_feature` error is returned.


---

### [`external_secret_replace()`](#external_secret_replace)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`secrets_manager`**: _[`secrets_manager`](#secrets_manager)_
* **`secret`**: `u8` pointer
* **`secret_len`**: _[`size`](#size)_
* **`expiration`**: _[`timestamp`](#timestamp)_
* **`secret_id`**: `u8` pointer
* **`secret_id_len`**: _[`size`](#size)_

#### Output:

* _[`version`](#version)_ mutable pointer

> Replace a managed external with a new version.
> 
> `$expiration` is the expiration date of the secret as a UNIX timestamp, in seconds.
> An expiration date is mandatory.
> 
> On success, a new version is created and returned.
> 
> If this function is not supported by the host the `$unsupported_feature` error is returned.


---

### [`external_secret_from_id()`](#external_secret_from_id)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`secrets_manager`**: _[`secrets_manager`](#secrets_manager)_
* **`secret_id`**: `u8` pointer
* **`secret_id_len`**: _[`size`](#size)_
* **`secret_version`**: _[`version`](#version)_

#### Output:

* _[`array_output`](#array_output)_ mutable pointer

> Get a copy of an external secret given an identifier and version.
> 
> `secret_version` can be set to a version number, or to `version.latest` to retrieve the most recent version of a secret.
> 
> On success, a copy of the secret is returned.
> 
> The function returns `$unsupported_feature` if this operation is not supported by the host, and `not_found` if the identifier and version don't match any existing secret.


---

### [`external_secret_invalidate()`](#external_secret_invalidate)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`secrets_manager`**: _[`secrets_manager`](#secrets_manager)_
* **`secret_id`**: `u8` pointer
* **`secret_id_len`**: _[`size`](#size)_
* **`secret_version`**: _[`version`](#version)_

This function has no output.

> Invalidate an external secret given an identifier and a version.
> 
> This asks the secrets manager to delete or revoke a stored secret, a specific version of a secret.
> 
> `secret_version` can be set to a version number, or to `version.latest` to invalidate the current version, or to `version.all` to invalidate all versions of a secret.
> 
> The function returns `$unsupported_feature` if this operation is not supported by the host, and `not_found` if the identifier and version don't match any existing secret.


---

### [`external_secret_encapsulate()`](#external_secret_encapsulate)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`secrets_manager`**: _[`secrets_manager`](#secrets_manager)_
* **`secret`**: `u8` pointer
* **`secret_len`**: _[`size`](#size)_
* **`expiration`**: _[`timestamp`](#timestamp)_

#### Output:

* _[`array_output`](#array_output)_ mutable pointer

> Encrypt an external secret.
> 
> Applications don't have access to the encryption key, and the secrets manager is free to choose any suitable algorithm.
> 
> However, the returned ciphertext must include and authenticate both the secret and the expiration date.
> 
> On success, the ciphertext is returned.


---

### [`external_secret_decapsulate()`](#external_secret_decapsulate)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`secrets_manager`**: _[`secrets_manager`](#secrets_manager)_
* **`encrypted_secret`**: `u8` pointer
* **`encrypted_secret_len`**: _[`size`](#size)_

#### Output:

* _[`array_output`](#array_output)_ mutable pointer

> Decrypt an external secret previously encrypted by the secrets manager.
> 
> Returns the original secret if the ciphertext is valid.
> Returns `$expired` if the current date is past the stored expiration date.
> Returns `$verification_failed` if the ciphertext format is invalid or if its authentication tag couldn't be verified.


---

