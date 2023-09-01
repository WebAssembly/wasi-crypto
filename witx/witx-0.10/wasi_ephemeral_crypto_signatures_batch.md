
# Module: wasi_ephemeral_crypto_signatures_batch

## Table of contents

### Types list:

[**[All](#types)**] - [_[`crypto_errno`](#crypto_errno)_] - [_[`array_output`](#array_output)_] - [_[`signature`](#signature)_] - [_[`signature_state`](#signature_state)_] - [_[`signature_verification_state`](#signature_verification_state)_] - [_[`signature_sign_result`](#signature_sign_result)_] - [_[`signature_results`](#signature_results)_] - [_[`signature_verification_input`](#signature_verification_input)_] - [_[`signature_verification_results`](#signature_verification_results)_]

### Functions list:

[**[All](#functions)**] - [[`batch_signature_state_sign()`](#batch_signature_state_sign)] - [[`batch_signature_state_verify()`](#batch_signature_state_verify)]

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

### _[`signature`](#signature)_
Alias for `handle`.


> A signature.


---

### _[`signature_state`](#signature_state)_
Alias for `handle`.


> A state to absorb data to be signed.
> 
> After a signature has been computed or verified, the state remains valid for further operations.
> 
> A subsequent signature would sign all the data accumulated since the creation of the state object.


---

### _[`signature_verification_state`](#signature_verification_state)_
Alias for `handle`.


> A state to absorb signed data to be verified.


---

### _[`signature_sign_result`](#signature_sign_result)_
Tuple, representing (_[`array_output`](#array_output)_, _[`crypto_errno`](#crypto_errno)_).


> The result pf a signature sign operation. A pair of the signature and an error code.


---

### _[`signature_results`](#signature_results)_
Alias for _[`signature_sign_result`](#signature_sign_result)_ mutable slice.


> A list of signature_sign results.


---

### _[`signature_verification_input`](#signature_verification_input)_
Tuple, representing (_[`signature_verification_state`](#signature_verification_state)_, _[`signature`](#signature)_).


> A tuple of a signature verification state and the signature to verify.
> 
> Used for grouping signature verification state to be verified with the signature to verify.
> Used with batch_signature_state_verify().


---

### _[`signature_verification_results`](#signature_verification_results)_
Alias for _[`crypto_errno`](#crypto_errno)_ mutable slice.


---

## Functions

### [`batch_signature_state_sign()`](#batch_signature_state_sign)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`states`**: _[`signature_state`](#signature_state)_ mutable slice

#### Output:

* _[`signature_results`](#signature_results)_ mutable pointer

> Compute a batch of signatures.
> 
> This is a batch version of the signature_state_sign operation and is an extension of the wasi_empemeral_crypto_signatures module.
> 
> If the entire batch could not be processed an error code of type
> $crypto_errno is returned. If part of the batch was successfully
> processed and part (one or more) resulted in a failure, each result
> is a pair of an error code and a signature. The signature is only valid
> if the error code indicates success.
> 
> Example usage:
> 
> ```rust
> let kp_handle = keypair_import(AlgorithmType::Signatures, "Ed25519", encoded, KeypairEncoding::Raw)?;
> 
> let mut state_handles = Vec::new();
> 
> let state_handle = signature_state_open(kp_handle)?;
> signature_state_update(state_handle, b"message part 1")?;
> signature_state_update(state_handle, b"message part 2")?;
> state_handles.push(state_handle);
> 
> let state_handle = signature_state_open(kp_handle)?;
> signature_state_update(state_handle, b"message part 1")?;
> signature_state_update(state_handle, b"message part 2")?;
> state_handles.push(state_handle);
> 
> let sig_handles = batch_signature_state_sign(state_handles)?;
> 
> let raw_sig1 = signature_export(sig_handle[0], SignatureEncoding::Raw)?;
> let raw_sig2 = signature_export(sig_handle[1], SignatureEncoding::Raw)?;
> ```


---

### [`batch_signature_state_verify()`](#batch_signature_state_verify)
Returned error type: _[`crypto_errno`](#crypto_errno)_

#### Input:

* **`states`**: _[`signature_verification_input`](#signature_verification_input)_ mutable slice

#### Output:

* _[`signature_verification_results`](#signature_verification_results)_ mutable pointer

> Verify a batch of signatures.
> 
> This is a batch version of the signature_state_verify operation and is
> an extension of the wasi_empemeral_crypto_signatures module.
> 
> Each entry in the input list has a corresponding error state returned
> to indicate if the verification succeeded or encountered and error.
> If the batch could not be processed an error code is returned,
> otherwise a list of verification results is produced.
> Each entry in the list is an error code that indicates the verification
> result for the corresponding entry in the verification input list.
> 
> Example usage:
> 
> ```rust
> let kp_handle = keypair_import(AlgorithmType::Signatures, "Ed25519", encoded, KeypairEncoding::Raw)?;
> 
> let mut batch = Vec::new();
> 
> let state_handle = signature_verification_state_open(kp_handle)?;
> signature_verification_state_update(state_handle, b"message part 1")?;
> signature_verification_state_update(state_handle, b"message part 2")?;
> state_handles.push((state_handle, signature1));
> 
> let state_handle = signature_state_open(kp_handle)?;
> signature_state_update(state_handle, b"message part 1")?;
> signature_state_update(state_handle, b"message part 2")?;
> state_handles.push((state_handle, signature2));
> 
> let results = batch_signature_state_verify(state_handles)?;
> ```


---

