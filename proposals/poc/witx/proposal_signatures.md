# Types
## <a href="#crypto_errno" name="crypto_errno"></a> `crypto_errno`: Enum(`u16`)

### Variants
- <a href="#crypto_errno.success" name="crypto_errno.success"></a> `success`

- <a href="#crypto_errno.not_implemented" name="crypto_errno.not_implemented"></a> `not_implemented`

- <a href="#crypto_errno.unsupported_feature" name="crypto_errno.unsupported_feature"></a> `unsupported_feature`

- <a href="#crypto_errno.prohibited_operation" name="crypto_errno.prohibited_operation"></a> `prohibited_operation`

- <a href="#crypto_errno.unsupported_encoding" name="crypto_errno.unsupported_encoding"></a> `unsupported_encoding`

- <a href="#crypto_errno.unsupported_algorithm" name="crypto_errno.unsupported_algorithm"></a> `unsupported_algorithm`

- <a href="#crypto_errno.invalid_key" name="crypto_errno.invalid_key"></a> `invalid_key`

- <a href="#crypto_errno.verification_failed" name="crypto_errno.verification_failed"></a> `verification_failed`

- <a href="#crypto_errno.rng_error" name="crypto_errno.rng_error"></a> `rng_error`

- <a href="#crypto_errno.algorithm_failure" name="crypto_errno.algorithm_failure"></a> `algorithm_failure`

- <a href="#crypto_errno.invalid_signature" name="crypto_errno.invalid_signature"></a> `invalid_signature`

- <a href="#crypto_errno.closed" name="crypto_errno.closed"></a> `closed`

- <a href="#crypto_errno.invalid_handle" name="crypto_errno.invalid_handle"></a> `invalid_handle`

- <a href="#crypto_errno.overflow" name="crypto_errno.overflow"></a> `overflow`

- <a href="#crypto_errno.internal_error" name="crypto_errno.internal_error"></a> `internal_error`

- <a href="#crypto_errno.too_many_handles" name="crypto_errno.too_many_handles"></a> `too_many_handles`

## <a href="#keypair_encoding" name="keypair_encoding"></a> `keypair_encoding`: Enum(`u16`)

### Variants
- <a href="#keypair_encoding.raw" name="keypair_encoding.raw"></a> `raw`

- <a href="#keypair_encoding.pkcs8" name="keypair_encoding.pkcs8"></a> `pkcs8`

- <a href="#keypair_encoding.der" name="keypair_encoding.der"></a> `der`

- <a href="#keypair_encoding.pem" name="keypair_encoding.pem"></a> `pem`

## <a href="#publickey_encoding" name="publickey_encoding"></a> `publickey_encoding`: Enum(`u16`)

### Variants
- <a href="#publickey_encoding.raw" name="publickey_encoding.raw"></a> `raw`

- <a href="#publickey_encoding.hex" name="publickey_encoding.hex"></a> `hex`

- <a href="#publickey_encoding.base64_original" name="publickey_encoding.base64_original"></a> `base64_original`

- <a href="#publickey_encoding.base64_original_nopadding" name="publickey_encoding.base64_original_nopadding"></a> `base64_original_nopadding`

- <a href="#publickey_encoding.base64_urlsafe" name="publickey_encoding.base64_urlsafe"></a> `base64_urlsafe`

- <a href="#publickey_encoding.base64_urlsafe_nopadding" name="publickey_encoding.base64_urlsafe_nopadding"></a> `base64_urlsafe_nopadding`

## <a href="#signature_encoding" name="signature_encoding"></a> `signature_encoding`: Enum(`u16`)

### Variants
- <a href="#signature_encoding.raw" name="signature_encoding.raw"></a> `raw`

- <a href="#signature_encoding.hex" name="signature_encoding.hex"></a> `hex`

- <a href="#signature_encoding.base64_original" name="signature_encoding.base64_original"></a> `base64_original`

- <a href="#signature_encoding.base64_original_nopadding" name="signature_encoding.base64_original_nopadding"></a> `base64_original_nopadding`

- <a href="#signature_encoding.base64_urlsafe" name="signature_encoding.base64_urlsafe"></a> `base64_urlsafe`

- <a href="#signature_encoding.base64_urlsafe_nopadding" name="signature_encoding.base64_urlsafe_nopadding"></a> `base64_urlsafe_nopadding`

- <a href="#signature_encoding.der" name="signature_encoding.der"></a> `der`

## <a href="#version" name="version"></a> `version`: Int(`u64`)

### Consts
- <a href="#version.unspecified" name="version.unspecified"></a> `unspecified`
Key doesn't support versioning.

- <a href="#version.latest" name="version.latest"></a> `latest`
Retrieve the latest version of a key.

- <a href="#version.all" name="version.all"></a> `all`
Perform an operation over all versions of a key.

## <a href="#size" name="size"></a> `size`: `usize`

## <a href="#array_output" name="array_output"></a> `array_output`

### Supertypes
## <a href="#signature_op" name="signature_op"></a> `signature_op`

### Supertypes
## <a href="#signature_keypair_builder" name="signature_keypair_builder"></a> `signature_keypair_builder`

### Supertypes
## <a href="#signature_keypair" name="signature_keypair"></a> `signature_keypair`

### Supertypes
## <a href="#signature_state" name="signature_state"></a> `signature_state`

### Supertypes
## <a href="#signature" name="signature"></a> `signature`

### Supertypes
## <a href="#signature_publickey" name="signature_publickey"></a> `signature_publickey`

### Supertypes
## <a href="#signature_verification_state" name="signature_verification_state"></a> `signature_verification_state`

### Supertypes
# Modules
## <a href="#wasi_ephemeral_crypto" name="wasi_ephemeral_crypto"></a> wasi_ephemeral_crypto
### Imports
#### Memory
### Functions

---

#### <a href="#array_output_len" name="array_output_len"></a> `array_output_len(array_output: array_output) -> size`
Return the length of an array_output object.

##### Params
- <a href="#array_output_len.array_output" name="array_output_len.array_output"></a> `array_output`: [`array_output`](#array_output)

##### Results
- <a href="#array_output_len.len" name="array_output_len.len"></a> `len`: [`size`](#size)


---

#### <a href="#array_output_pull" name="array_output_pull"></a> `array_output_pull(array_output: array_output, buf: Pointer<u8>, buf_len: size) -> crypto_errno`
Copy an array_output into an application-allocated buffer.
The array_output handle becomes invalid after this operation.

##### Params
- <a href="#array_output_pull.array_output" name="array_output_pull.array_output"></a> `array_output`: [`array_output`](#array_output)

- <a href="#array_output_pull.buf" name="array_output_pull.buf"></a> `buf`: `Pointer<u8>`

- <a href="#array_output_pull.buf_len" name="array_output_pull.buf_len"></a> `buf_len`: [`size`](#size)

##### Results
- <a href="#array_output_pull.error" name="array_output_pull.error"></a> `error`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#signature_op_open" name="signature_op_open"></a> `signature_op_open(op_str: string) -> (crypto_errno, signature_op)`
Create a context for a signature-related operation.

##### Params
- <a href="#signature_op_open.op_str" name="signature_op_open.op_str"></a> `op_str`: `string`

##### Results
- <a href="#signature_op_open.error" name="signature_op_open.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#signature_op_open.signature_op" name="signature_op_open.signature_op"></a> `signature_op`: [`signature_op`](#signature_op)


---

#### <a href="#signature_op_close" name="signature_op_close"></a> `signature_op_close(signature_op: signature_op) -> crypto_errno`
Destroy the signature context.

##### Params
- <a href="#signature_op_close.signature_op" name="signature_op_close.signature_op"></a> `signature_op`: [`signature_op`](#signature_op)

##### Results
- <a href="#signature_op_close.error" name="signature_op_close.error"></a> `error`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#signature_keypair_builder_open" name="signature_keypair_builder_open"></a> `signature_keypair_builder_open(signature_op: signature_op) -> (crypto_errno, signature_keypair_builder)`
Create a context to construct a key pair.

##### Params
- <a href="#signature_keypair_builder_open.signature_op" name="signature_keypair_builder_open.signature_op"></a> `signature_op`: [`signature_op`](#signature_op)

##### Results
- <a href="#signature_keypair_builder_open.error" name="signature_keypair_builder_open.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#signature_keypair_builder_open.handle" name="signature_keypair_builder_open.handle"></a> `handle`: [`signature_keypair_builder`](#signature_keypair_builder)


---

#### <a href="#signature_keypair_builder_close" name="signature_keypair_builder_close"></a> `signature_keypair_builder_close(kp_builder: signature_keypair_builder) -> crypto_errno`
Destroy a key pair builder.

##### Params
- <a href="#signature_keypair_builder_close.kp_builder" name="signature_keypair_builder_close.kp_builder"></a> `kp_builder`: [`signature_keypair_builder`](#signature_keypair_builder)

##### Results
- <a href="#signature_keypair_builder_close.error" name="signature_keypair_builder_close.error"></a> `error`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#signature_keypair_generate" name="signature_keypair_generate"></a> `signature_keypair_generate(kp_builder: signature_keypair_builder) -> (crypto_errno, signature_keypair)`
Generate a new key pair.
This function may return crypto_errno.unsupported_feature if key
generation is not supported by the host for the chosen algorithm.

##### Params
- <a href="#signature_keypair_generate.kp_builder" name="signature_keypair_generate.kp_builder"></a> `kp_builder`: [`signature_keypair_builder`](#signature_keypair_builder)

##### Results
- <a href="#signature_keypair_generate.error" name="signature_keypair_generate.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#signature_keypair_generate.handle" name="signature_keypair_generate.handle"></a> `handle`: [`signature_keypair`](#signature_keypair)


---

#### <a href="#signature_keypair_import" name="signature_keypair_import"></a> `signature_keypair_import(kp_builder: signature_keypair_builder, encoded: ConstPointer<u8>, encoded_len: size, encoding: keypair_encoding) -> (crypto_errno, signature_keypair)`
Import a key pair.
This function may return crypto_errno.unsupported_algorithm if the
encoding scheme is not supported, or crypto_errno.invalid_key if the key
cannot be decoded.

##### Params
- <a href="#signature_keypair_import.kp_builder" name="signature_keypair_import.kp_builder"></a> `kp_builder`: [`signature_keypair_builder`](#signature_keypair_builder)

- <a href="#signature_keypair_import.encoded" name="signature_keypair_import.encoded"></a> `encoded`: `ConstPointer<u8>`

- <a href="#signature_keypair_import.encoded_len" name="signature_keypair_import.encoded_len"></a> `encoded_len`: [`size`](#size)

- <a href="#signature_keypair_import.encoding" name="signature_keypair_import.encoding"></a> `encoding`: [`keypair_encoding`](#keypair_encoding)

##### Results
- <a href="#signature_keypair_import.error" name="signature_keypair_import.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#signature_keypair_import.handle" name="signature_keypair_import.handle"></a> `handle`: [`signature_keypair`](#signature_keypair)


---

#### <a href="#signature_keypair_id" name="signature_keypair_id"></a> `signature_keypair_id(kp: signature_keypair, kp_id: ConstPointer<u8>, kp_id_max_len: size) -> (crypto_errno, size, version)`
[OPTIONAL IMPORT]
Return the key identifier and version, if these are available
or $crypto_errno.unsupported_feature if not.

##### Params
- <a href="#signature_keypair_id.kp" name="signature_keypair_id.kp"></a> `kp`: [`signature_keypair`](#signature_keypair)

- <a href="#signature_keypair_id.kp_id" name="signature_keypair_id.kp_id"></a> `kp_id`: `ConstPointer<u8>`

- <a href="#signature_keypair_id.kp_id_max_len" name="signature_keypair_id.kp_id_max_len"></a> `kp_id_max_len`: [`size`](#size)

##### Results
- <a href="#signature_keypair_id.error" name="signature_keypair_id.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#signature_keypair_id.kp_id_len" name="signature_keypair_id.kp_id_len"></a> `kp_id_len`: [`size`](#size)

- <a href="#signature_keypair_id.version" name="signature_keypair_id.version"></a> `version`: [`version`](#version)


---

#### <a href="#signature_keypair_from_id" name="signature_keypair_from_id"></a> `signature_keypair_from_id(kp_builder: signature_keypair_builder, kp_id: ConstPointer<u8>, kp_id_len: size, kp_version: version) -> (crypto_errno, signature_keypair)`
[OPTIONAL IMPORT]
Create a key pair using an opaque key identifier.
Return crypto_errno.unsupported_feature if this operation is not
supported by the host, and crypto_errno.invalid_key if the identifier
is invalid.
The version can be an actual version number or $version.latest .

##### Params
- <a href="#signature_keypair_from_id.kp_builder" name="signature_keypair_from_id.kp_builder"></a> `kp_builder`: [`signature_keypair_builder`](#signature_keypair_builder)

- <a href="#signature_keypair_from_id.kp_id" name="signature_keypair_from_id.kp_id"></a> `kp_id`: `ConstPointer<u8>`

- <a href="#signature_keypair_from_id.kp_id_len" name="signature_keypair_from_id.kp_id_len"></a> `kp_id_len`: [`size`](#size)

- <a href="#signature_keypair_from_id.kp_version" name="signature_keypair_from_id.kp_version"></a> `kp_version`: [`version`](#version)

##### Results
- <a href="#signature_keypair_from_id.error" name="signature_keypair_from_id.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#signature_keypair_from_id.handle" name="signature_keypair_from_id.handle"></a> `handle`: [`signature_keypair`](#signature_keypair)


---

#### <a href="#signature_keypair_invalidate" name="signature_keypair_invalidate"></a> `signature_keypair_invalidate(kp_builder: signature_keypair_builder, kp_id: ConstPointer<u8>, kp_id_len: size, kp_version: version) -> crypto_errno`
[OPTIONAL IMPORT]
Invalidate a key pair given a key identifier and a version.
Return crypto_errno.unsupported_feature if this operation is not
supported by the host, and crypto_errno.invalid_key if the identifier
is invalid.
The version can be a actual version number, as well as
$version.latest or $version.all .

##### Params
- <a href="#signature_keypair_invalidate.kp_builder" name="signature_keypair_invalidate.kp_builder"></a> `kp_builder`: [`signature_keypair_builder`](#signature_keypair_builder)

- <a href="#signature_keypair_invalidate.kp_id" name="signature_keypair_invalidate.kp_id"></a> `kp_id`: `ConstPointer<u8>`

- <a href="#signature_keypair_invalidate.kp_id_len" name="signature_keypair_invalidate.kp_id_len"></a> `kp_id_len`: [`size`](#size)

- <a href="#signature_keypair_invalidate.kp_version" name="signature_keypair_invalidate.kp_version"></a> `kp_version`: [`version`](#version)

##### Results
- <a href="#signature_keypair_invalidate.error" name="signature_keypair_invalidate.error"></a> `error`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#signature_keypair_export" name="signature_keypair_export"></a> `signature_keypair_export(kp: signature_keypair, encoding: keypair_encoding) -> (crypto_errno, array_output)`
[OPTIONAL IMPORT]
Export the key pair as the given encoding format.
May return crypto_errno.prohibited_operation if this operation is
not available or crypto_errno.unsupported_encoding if the encoding
is not supported.

##### Params
- <a href="#signature_keypair_export.kp" name="signature_keypair_export.kp"></a> `kp`: [`signature_keypair`](#signature_keypair)

- <a href="#signature_keypair_export.encoding" name="signature_keypair_export.encoding"></a> `encoding`: [`keypair_encoding`](#keypair_encoding)

##### Results
- <a href="#signature_keypair_export.error" name="signature_keypair_export.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#signature_keypair_export.encoded" name="signature_keypair_export.encoded"></a> `encoded`: [`array_output`](#array_output)


---

#### <a href="#signature_keypair_publickey" name="signature_keypair_publickey"></a> `signature_keypair_publickey(kp: signature_keypair) -> (crypto_errno, signature_publickey)`
Create a public key object from the key pair.

##### Params
- <a href="#signature_keypair_publickey.kp" name="signature_keypair_publickey.kp"></a> `kp`: [`signature_keypair`](#signature_keypair)

##### Results
- <a href="#signature_keypair_publickey.error" name="signature_keypair_publickey.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#signature_keypair_publickey.pk" name="signature_keypair_publickey.pk"></a> `pk`: [`signature_publickey`](#signature_publickey)


---

#### <a href="#signature_keypair_close" name="signature_keypair_close"></a> `signature_keypair_close(kp: signature_keypair) -> crypto_errno`
Destroys a key pair and wipe memory accordingly.

##### Params
- <a href="#signature_keypair_close.kp" name="signature_keypair_close.kp"></a> `kp`: [`signature_keypair`](#signature_keypair)

##### Results
- <a href="#signature_keypair_close.error" name="signature_keypair_close.error"></a> `error`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#signature_publickey_import" name="signature_publickey_import"></a> `signature_publickey_import(signature_op: signature_op, encoded: ConstPointer<u8>, encoded_len: size, encoding: publickey_encoding) -> (crypto_errno, signature_publickey)`
Import a public key encoded.
Return crypto_errno.unsupported_encoding if exporting
to the given format is not implemented or if the format is
incompatible with the key type.

##### Params
- <a href="#signature_publickey_import.signature_op" name="signature_publickey_import.signature_op"></a> `signature_op`: [`signature_op`](#signature_op)

- <a href="#signature_publickey_import.encoded" name="signature_publickey_import.encoded"></a> `encoded`: `ConstPointer<u8>`

- <a href="#signature_publickey_import.encoded_len" name="signature_publickey_import.encoded_len"></a> `encoded_len`: [`size`](#size)

- <a href="#signature_publickey_import.encoding" name="signature_publickey_import.encoding"></a> `encoding`: [`publickey_encoding`](#publickey_encoding)

##### Results
- <a href="#signature_publickey_import.error" name="signature_publickey_import.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#signature_publickey_import.pk" name="signature_publickey_import.pk"></a> `pk`: [`signature_publickey`](#signature_publickey)


---

#### <a href="#signature_publickey_verify" name="signature_publickey_verify"></a> `signature_publickey_verify(pk: signature_publickey) -> crypto_errno`
Check that a public key is valid and in canonical form.
Return crypto_errno.invalid_key is verification fails.

##### Params
- <a href="#signature_publickey_verify.pk" name="signature_publickey_verify.pk"></a> `pk`: [`signature_publickey`](#signature_publickey)

##### Results
- <a href="#signature_publickey_verify.error" name="signature_publickey_verify.error"></a> `error`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#signature_publickey_close" name="signature_publickey_close"></a> `signature_publickey_close(pk: signature_publickey) -> crypto_errno`
Destroys a public key.

##### Params
- <a href="#signature_publickey_close.pk" name="signature_publickey_close.pk"></a> `pk`: [`signature_publickey`](#signature_publickey)

##### Results
- <a href="#signature_publickey_close.error" name="signature_publickey_close.error"></a> `error`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#signature_export" name="signature_export"></a> `signature_export(signature: signature, encoding: signature_encoding) -> (crypto_errno, array_output)`
Export a signature in the given format.

##### Params
- <a href="#signature_export.signature" name="signature_export.signature"></a> `signature`: [`signature`](#signature)

- <a href="#signature_export.encoding" name="signature_export.encoding"></a> `encoding`: [`signature_encoding`](#signature_encoding)

##### Results
- <a href="#signature_export.error" name="signature_export.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#signature_export.encoded" name="signature_export.encoded"></a> `encoded`: [`array_output`](#array_output)


---

#### <a href="#signature_import" name="signature_import"></a> `signature_import(signature_op: signature_op, encoding: signature_encoding, encoded: ConstPointer<u8>, encoded_len: size) -> (crypto_errno, signature)`
Create a signature object by importing a signature encoded
in a given format.
Return crypto_errno.invalid_signature if the signature is incompatible
with the current content.

##### Params
- <a href="#signature_import.signature_op" name="signature_import.signature_op"></a> `signature_op`: [`signature_op`](#signature_op)

- <a href="#signature_import.encoding" name="signature_import.encoding"></a> `encoding`: [`signature_encoding`](#signature_encoding)

- <a href="#signature_import.encoded" name="signature_import.encoded"></a> `encoded`: `ConstPointer<u8>`

- <a href="#signature_import.encoded_len" name="signature_import.encoded_len"></a> `encoded_len`: [`size`](#size)

##### Results
- <a href="#signature_import.error" name="signature_import.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#signature_import.signature" name="signature_import.signature"></a> `signature`: [`signature`](#signature)


---

#### <a href="#signature_state_open" name="signature_state_open"></a> `signature_state_open(kp: signature_keypair) -> (crypto_errno, signature_state)`
Create a new state to collect data to compute a signature on.

##### Params
- <a href="#signature_state_open.kp" name="signature_state_open.kp"></a> `kp`: [`signature_keypair`](#signature_keypair)

##### Results
- <a href="#signature_state_open.error" name="signature_state_open.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#signature_state_open.state" name="signature_state_open.state"></a> `state`: [`signature_state`](#signature_state)


---

#### <a href="#signature_state_update" name="signature_state_update"></a> `signature_state_update(state: signature_state, input: ConstPointer<u8>, input_len: size) -> crypto_errno`
Inject data into the state.

##### Params
- <a href="#signature_state_update.state" name="signature_state_update.state"></a> `state`: [`signature_state`](#signature_state)

- <a href="#signature_state_update.input" name="signature_state_update.input"></a> `input`: `ConstPointer<u8>`

- <a href="#signature_state_update.input_len" name="signature_state_update.input_len"></a> `input_len`: [`size`](#size)

##### Results
- <a href="#signature_state_update.error" name="signature_state_update.error"></a> `error`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#signature_state_sign" name="signature_state_sign"></a> `signature_state_sign(state: signature_state) -> (crypto_errno, array_output)`
Compute a signature for all the data collected until tht point.
The function can be called multiple times for incremental signing.
May return crypto_errno.overflow is too much data has been processed
for the chosen algorithm or if system resources have been
exceeded.

##### Params
- <a href="#signature_state_sign.state" name="signature_state_sign.state"></a> `state`: [`signature_state`](#signature_state)

##### Results
- <a href="#signature_state_sign.error" name="signature_state_sign.error"></a> `error`: [`crypto_errno`](#crypto_errno)

- <a href="#signature_state_sign.signature" name="signature_state_sign.signature"></a> `signature`: [`array_output`](#array_output)


---

#### <a href="#signature_state_close" name="signature_state_close"></a> `signature_state_close(state: signature_state) -> crypto_errno`
Destroy a signature state.

##### Params
- <a href="#signature_state_close.state" name="signature_state_close.state"></a> `state`: [`signature_state`](#signature_state)

##### Results
- <a href="#signature_state_close.error" name="signature_state_close.error"></a> `error`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#signature_verification_state_update" name="signature_verification_state_update"></a> `signature_verification_state_update(state: signature_verification_state, input: ConstPointer<u8>, input_len: size) -> crypto_errno`
Create a new state to collect data to verify a signature on.

##### Params
- <a href="#signature_verification_state_update.state" name="signature_verification_state_update.state"></a> `state`: [`signature_verification_state`](#signature_verification_state)

- <a href="#signature_verification_state_update.input" name="signature_verification_state_update.input"></a> `input`: `ConstPointer<u8>`

- <a href="#signature_verification_state_update.input_len" name="signature_verification_state_update.input_len"></a> `input_len`: [`size`](#size)

##### Results
- <a href="#signature_verification_state_update.error" name="signature_verification_state_update.error"></a> `error`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#signature_verification_state_verify" name="signature_verification_state_verify"></a> `signature_verification_state_verify(state: signature_verification_state, signature: signature) -> crypto_errno`
Verify that the given signature is valid for the data collected
up to this point.

##### Params
- <a href="#signature_verification_state_verify.state" name="signature_verification_state_verify.state"></a> `state`: [`signature_verification_state`](#signature_verification_state)

- <a href="#signature_verification_state_verify.signature" name="signature_verification_state_verify.signature"></a> `signature`: [`signature`](#signature)

##### Results
- <a href="#signature_verification_state_verify.error" name="signature_verification_state_verify.error"></a> `error`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#signature_verification_state_close" name="signature_verification_state_close"></a> `signature_verification_state_close(state: signature_verification_state) -> crypto_errno`
Destroy a signature verification state.

##### Params
- <a href="#signature_verification_state_close.state" name="signature_verification_state_close.state"></a> `state`: [`signature_verification_state`](#signature_verification_state)

##### Results
- <a href="#signature_verification_state_close.error" name="signature_verification_state_close.error"></a> `error`: [`crypto_errno`](#crypto_errno)


---

#### <a href="#signature_close" name="signature_close"></a> `signature_close(signature: signature) -> crypto_errno`
Destroy a signature.

##### Params
- <a href="#signature_close.signature" name="signature_close.signature"></a> `signature`: [`signature`](#signature)

##### Results
- <a href="#signature_close.error" name="signature_close.error"></a> `error`: [`crypto_errno`](#crypto_errno)


