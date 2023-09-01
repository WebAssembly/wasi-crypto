
# Module: wasi_ephemeral_crypto_symmetric_batch

## Table of contents

### Types list:

[**[All](#types)**] - [_[`crypto_errno`](#crypto_errno)_] - [_[`keypair_encoding`](#keypair_encoding)_] - [_[`publickey_encoding`](#publickey_encoding)_] - [_[`secretkey_encoding`](#secretkey_encoding)_] - [_[`signature_encoding`](#signature_encoding)_] - [_[`algorithm_type`](#algorithm_type)_] - [_[`version`](#version)_] - [_[`size`](#size)_] - [_[`timestamp`](#timestamp)_] - [_[`u64`](#u64)_] - [_[`array_output`](#array_output)_] - [_[`options`](#options)_] - [_[`secrets_manager`](#secrets_manager)_] - [_[`keypair`](#keypair)_] - [_[`signature_state`](#signature_state)_] - [_[`signature`](#signature)_] - [_[`publickey`](#publickey)_] - [_[`secretkey`](#secretkey)_] - [_[`signature_verification_state`](#signature_verification_state)_] - [_[`symmetric_state`](#symmetric_state)_] - [_[`symmetric_key`](#symmetric_key)_] - [_[`symmetric_tag`](#symmetric_tag)_] - [_[`opt_options_u`](#opt_options_u)_] - [_[`opt_options`](#opt_options)_] - [_[`opt_symmetric_key_u`](#opt_symmetric_key_u)_] - [_[`opt_symmetric_key`](#opt_symmetric_key)_] - [_[`output`](#output)_] - [_[`output_len`](#output_len)_] - [_[`data`](#data)_] - [_[`data_len`](#data_len)_] - [_[`raw_tag`](#raw_tag)_] - [_[`raw_tag_len`](#raw_tag_len)_] - [_[`encrypt_params`](#encrypt_params)_] - [_[`decrypt_detached_params`](#decrypt_detached_params)_] - [_[`encrypt_result`](#encrypt_result)_] - [_[`batch_encrypt_results`](#batch_encrypt_results)_] - [_[`encrypt_detached_result`](#encrypt_detached_result)_] - [_[`batch_encrypt_detached_results`](#batch_encrypt_detached_results)_] - [_[`squeeze_params`](#squeeze_params)_] - [_[`squeeze_results`](#squeeze_results)_]

### Functions list:

[**[All](#functions)**] - [[`batch_symmetric_state_squeeze()`](#batch_symmetric_state_squeeze)] - [[`batch_symmetric_state_squeeze_tag()`](#batch_symmetric_state_squeeze_tag)] - [[`batch_symmetric_state_encrypt()`](#batch_symmetric_state_encrypt)] - [[`batch_symmetric_state_encrypt_detached()`](#batch_symmetric_state_encrypt_detached)] - [[`batch_symmetric_state_decrypt()`](#batch_symmetric_state_decrypt)] - [[`batch_symmetric_state_decrypt_detached()`](#batch_symmetric_state_decrypt_detached)]

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

### _[`output`](#output)_
Alias for `u8` mutable slice.


> An output buffer


---

### _[`output_len`](#output_len)_

Alias for `usize`.


---

### _[`data`](#data)_
Alias for `u8` slice.


> A non-mutable data buffer


---

### _[`data_len`](#data_len)_

Alias for `usize`.


---

### _[`raw_tag`](#raw_tag)_
Alias for `u8` slice.


> An raw tag buffer


---

### _[`raw_tag_len`](#raw_tag_len)_

Alias for `usize`.


---

### _[`encrypt_params`](#encrypt_params)_
Tuple, representing (_[`symmetric_state`](#symmetric_state)_, _[`output`](#output)_, _[`output_len`](#output_len)_, _[`data`](#data)_, _[`data_len`](#data_len)_).


> A tuple of parameters for an encryption operation.


---

### _[`decrypt_detached_params`](#decrypt_detached_params)_
Tuple, representing (_[`symmetric_state`](#symmetric_state)_, _[`output`](#output)_, _[`output_len`](#output_len)_, _[`data`](#data)_, _[`data_len`](#data_len)_, _[`raw_tag`](#raw_tag)_, _[`raw_tag_len`](#raw_tag_len)_).


> A tuple of parameters for a detached decryption operation.


---

### _[`encrypt_result`](#encrypt_result)_
Tuple, representing (_[`size`](#size)_, _[`crypto_errno`](#crypto_errno)_).


---

### _[`batch_encrypt_results`](#batch_encrypt_results)_
Alias for _[`encrypt_result`](#encrypt_result)_ mutable slice.


---

### _[`encrypt_detached_result`](#encrypt_detached_result)_
Tuple, representing (_[`symmetric_tag`](#symmetric_tag)_, _[`crypto_errno`](#crypto_errno)_).


---

### _[`batch_encrypt_detached_results`](#batch_encrypt_detached_results)_
Alias for _[`encrypt_detached_result`](#encrypt_detached_result)_ mutable slice.


---

### _[`squeeze_params`](#squeeze_params)_
Tuple, representing (_[`symmetric_state`](#symmetric_state)_, _[`data`](#data)_, _[`data_len`](#data_len)_).


---

### _[`squeeze_results`](#squeeze_results)_
Alias for _[`crypto_errno`](#crypto_errno)_ mutable slice.


---

## Functions

### [`batch_symmetric_state_squeeze()`](#batch_symmetric_state_squeeze)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`params`**: _[`squeeze_params`](#squeeze_params)_ mutable slice

#### Output:

* _[`squeeze_results`](#squeeze_results)_ mutable pointer

> Batch of operations to squeeze bytes from the state.
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

### [`batch_symmetric_state_squeeze_tag()`](#batch_symmetric_state_squeeze_tag)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`states`**: _[`symmetric_state`](#symmetric_state)_ mutable slice

#### Output:

* _[`batch_encrypt_detached_results`](#batch_encrypt_detached_results)_ mutable pointer

> Batch of operations to compute and return a tag for all the data
> injected into the state so far.
> 
> - **MAC functions**: returns a tag authenticating the absorbed data.
> - **Tuplehash-like constructions:** returns a tag authenticating all the absorbed tuples.
> - **Password-hashing functions:** returns a standard string containing all the required parameters for password verification.
> 
> Other kinds of algorithms may return `invalid_operation` instead.
> 
> For password-stretching functions, the function may return `in_progress`.
> In that case, the guest should retry with the same parameters until the function completes.
> 
> TODO: Refactor return type to share a more generic common type with encrypt detached results


---

### [`batch_symmetric_state_encrypt()`](#batch_symmetric_state_encrypt)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`batch`**: (_[`symmetric_state`](#symmetric_state)_, _[`output`](#output)_, _[`output_len`](#output_len)_, _[`data`](#data)_, _[`data_len`](#data_len)_) mutable slice

#### Output:

* _[`batch_encrypt_results`](#batch_encrypt_results)_ mutable pointer

> Perform a batch of symmetric encrypt operations.
> 
> This is a batch version of the symmetric_state_encrypto operation and
> is an extension of the wasi_empemeral_crypto_symmetric module.
> 
> Each entry in the batch corresponds to an individual encrypt operation.
> The parameters associated with each encrypt operation are grouped into a
> tuple.
> 
> If the entire batch could not be processed an error code of type
> $crypto_errno is returned. If part of the batch was successfully
> processed and part (one or more) resulted in a failure, each result
> is a pair of an error code and a size. The size is only valid
> if the error code indicates success.
> 
> The size corresponds to the actual size of the ciphertext and the tag
> in the ouput buffer.
> 
> Example usage:
> 
> ```rust
> let mut batch = Vec::new();
> 
> let state_handle = ctx.symmetric_state_open("AES-256-GCM", Some(key_handle1), Some(options_handle1))?;
> let mut ciphertext = vec![0u8; message.len() + ctx.symmetric_state_max_tag_len(state_handle)?];
> batch.push((batch, state_handle, ciphertext, ciphertext.len(), message, message.len()));
> 
> let state_handle = ctx.symmetric_state_open("AES-256-GCM", Some(key_handle2), Some(options_handle2))?;
> let mut ciphertext = vec![0u8; message2.len() + ctx.symmetric_state_max_tag_len(state_handle)?];
> batch.push((batch, state_handle, ciphertext, ciphertext.len(), message2, message2.len()));
> 
> let results = ctx.batch_symmetric_state_encrypt(batch)?;
> ```


---

### [`batch_symmetric_state_encrypt_detached()`](#batch_symmetric_state_encrypt_detached)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`batch`**: (_[`symmetric_state`](#symmetric_state)_, _[`output`](#output)_, _[`output_len`](#output_len)_, _[`data`](#data)_, _[`data_len`](#data_len)_) mutable slice

#### Output:

* _[`batch_encrypt_detached_results`](#batch_encrypt_detached_results)_ mutable pointer

> Perform a batch of symmetric encrypt operations with detached tags.


---

### [`batch_symmetric_state_decrypt()`](#batch_symmetric_state_decrypt)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`batch`**: (_[`symmetric_state`](#symmetric_state)_, _[`output`](#output)_, _[`output_len`](#output_len)_, _[`data`](#data)_, _[`data_len`](#data_len)_) mutable slice

#### Output:

* _[`batch_encrypt_results`](#batch_encrypt_results)_ mutable pointer

> Perform a batch of symmetric decrypt operations.


---

### [`batch_symmetric_state_decrypt_detached()`](#batch_symmetric_state_decrypt_detached)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`batch`**: (_[`symmetric_state`](#symmetric_state)_, _[`output`](#output)_, _[`output_len`](#output_len)_, _[`data`](#data)_, _[`data_len`](#data_len)_, _[`raw_tag`](#raw_tag)_, _[`raw_tag_len`](#raw_tag_len)_) mutable slice

#### Output:

* _[`batch_encrypt_results`](#batch_encrypt_results)_ mutable pointer

> Perform a batch of symmetric decrypt operations with detached tags.
> 
> TODO: Replace the encrypt_params type with something more generic that
> that works for both encrypt and decrypt.


---

