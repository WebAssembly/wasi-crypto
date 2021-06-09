
# Module: wasi_ephemeral_crypto_kx

## Table of contents

### Types list:

[**[All](#types)**] - [_[`crypto_errno`](#crypto_errno)_] - [_[`keypair_encoding`](#keypair_encoding)_] - [_[`publickey_encoding`](#publickey_encoding)_] - [_[`secretkey_encoding`](#secretkey_encoding)_] - [_[`signature_encoding`](#signature_encoding)_] - [_[`algorithm_type`](#algorithm_type)_] - [_[`version`](#version)_] - [_[`size`](#size)_] - [_[`timestamp`](#timestamp)_] - [_[`u64`](#u64)_] - [_[`array_output`](#array_output)_] - [_[`options`](#options)_] - [_[`secrets_manager`](#secrets_manager)_] - [_[`keypair`](#keypair)_] - [_[`signature_state`](#signature_state)_] - [_[`signature`](#signature)_] - [_[`publickey`](#publickey)_] - [_[`secretkey`](#secretkey)_] - [_[`signature_verification_state`](#signature_verification_state)_] - [_[`symmetric_state`](#symmetric_state)_] - [_[`symmetric_key`](#symmetric_key)_] - [_[`symmetric_tag`](#symmetric_tag)_] - [_[`opt_options_u`](#opt_options_u)_] - [_[`opt_options`](#opt_options)_] - [_[`opt_symmetric_key_u`](#opt_symmetric_key_u)_] - [_[`opt_symmetric_key`](#opt_symmetric_key)_] - [_[`kx_keypair`](#kx_keypair)_] - [_[`kx_publickey`](#kx_publickey)_] - [_[`kx_secretkey`](#kx_secretkey)_]

### Functions list:

[**[All](#functions)**] - [[`kx_dh()`](#kx_dh)] - [[`kx_encapsulate()`](#kx_encapsulate)] - [[`kx_decapsulate()`](#kx_decapsulate)]

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

