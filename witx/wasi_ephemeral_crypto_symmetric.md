
# Module: wasi_ephemeral_crypto_symmetric

## Table of contents

### Types list:

[**[All](#types)**] - [_[`crypto_errno`](#crypto_errno)_] - [_[`keypair_encoding`](#keypair_encoding)_] - [_[`publickey_encoding`](#publickey_encoding)_] - [_[`secretkey_encoding`](#secretkey_encoding)_] - [_[`signature_encoding`](#signature_encoding)_] - [_[`algorithm_type`](#algorithm_type)_] - [_[`version`](#version)_] - [_[`size`](#size)_] - [_[`timestamp`](#timestamp)_] - [_[`u64`](#u64)_] - [_[`array_output`](#array_output)_] - [_[`options`](#options)_] - [_[`secrets_manager`](#secrets_manager)_] - [_[`keypair`](#keypair)_] - [_[`signature_state`](#signature_state)_] - [_[`signature`](#signature)_] - [_[`publickey`](#publickey)_] - [_[`secretkey`](#secretkey)_] - [_[`signature_verification_state`](#signature_verification_state)_] - [_[`symmetric_state`](#symmetric_state)_] - [_[`symmetric_key`](#symmetric_key)_] - [_[`symmetric_tag`](#symmetric_tag)_] - [_[`opt_options_u`](#opt_options_u)_] - [_[`opt_options`](#opt_options)_] - [_[`opt_symmetric_key_u`](#opt_symmetric_key_u)_] - [_[`opt_symmetric_key`](#opt_symmetric_key)_]

### Functions list:

[**[All](#functions)**] - [[`symmetric_key_generate()`](#symmetric_key_generate)] - [[`symmetric_key_import()`](#symmetric_key_import)] - [[`symmetric_key_export()`](#symmetric_key_export)] - [[`symmetric_key_close()`](#symmetric_key_close)] - [[`symmetric_key_generate_managed()`](#symmetric_key_generate_managed)] - [[`symmetric_key_store_managed()`](#symmetric_key_store_managed)] - [[`symmetric_key_replace_managed()`](#symmetric_key_replace_managed)] - [[`symmetric_key_id()`](#symmetric_key_id)] - [[`symmetric_key_from_id()`](#symmetric_key_from_id)] - [[`symmetric_state_open()`](#symmetric_state_open)] - [[`symmetric_state_options_get()`](#symmetric_state_options_get)] - [[`symmetric_state_options_get_u64()`](#symmetric_state_options_get_u64)] - [[`symmetric_state_close()`](#symmetric_state_close)] - [[`symmetric_state_absorb()`](#symmetric_state_absorb)] - [[`symmetric_state_squeeze()`](#symmetric_state_squeeze)] - [[`symmetric_state_squeeze_tag()`](#symmetric_state_squeeze_tag)] - [[`symmetric_state_squeeze_key()`](#symmetric_state_squeeze_key)] - [[`symmetric_state_max_tag_len()`](#symmetric_state_max_tag_len)] - [[`symmetric_state_encrypt()`](#symmetric_state_encrypt)] - [[`symmetric_state_encrypt_detached()`](#symmetric_state_encrypt_detached)] - [[`symmetric_state_decrypt()`](#symmetric_state_decrypt)] - [[`symmetric_state_decrypt_detached()`](#symmetric_state_decrypt_detached)] - [[`symmetric_state_ratchet()`](#symmetric_state_ratchet)] - [[`symmetric_tag_len()`](#symmetric_tag_len)] - [[`symmetric_tag_pull()`](#symmetric_tag_pull)] - [[`symmetric_tag_verify()`](#symmetric_tag_verify)] - [[`symmetric_tag_close()`](#symmetric_tag_close)]

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

