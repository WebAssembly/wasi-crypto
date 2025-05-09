
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
* **`local`**: _[`publickey_encoding`](#publickey_encoding)_

> Encoding to use for importing or exporting a public key.


---

### _[`secretkey_encoding`](#secretkey_encoding)_

Enumeration with tag type: `u16`, and the following members:

* **`raw`**: _[`secretkey_encoding`](#secretkey_encoding)_
* **`pkcs8`**: _[`secretkey_encoding`](#secretkey_encoding)_
* **`pem`**: _[`secretkey_encoding`](#secretkey_encoding)_
* **`sec`**: _[`secretkey_encoding`](#secretkey_encoding)_
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
> A version can be an arbitrary `u64` integer, with the exception of some reserved values.


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
> The host is responsible for securely wiping them from memory on close.


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

