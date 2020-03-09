# Types
## <a href="#size" name="size"></a> `size`: `usize`

## <a href="#errno" name="errno"></a> `errno`: Enum(`u16`)
Error codes returned by functions.
Not all of these error codes are returned by the functions provided by this
API; some are used in higher-level library layers, and others are provided
merely for alignment with POSIX.

### Variants
- <a href="#errno.success" name="errno.success"></a> `success`
No error occurred. System call completed successfully.

- <a href="#errno.2big" name="errno.2big"></a> `2big`
Argument list too long.

- <a href="#errno.access" name="errno.access"></a> `access`
Permission denied.

- <a href="#errno.addrinuse" name="errno.addrinuse"></a> `addrinuse`
Address in use.

- <a href="#errno.addrnotavail" name="errno.addrnotavail"></a> `addrnotavail`
Address not available.

- <a href="#errno.afnosupport" name="errno.afnosupport"></a> `afnosupport`
Address family not supported.

- <a href="#errno.again" name="errno.again"></a> `again`
Resource unavailable, or operation would block.

- <a href="#errno.already" name="errno.already"></a> `already`
Connection already in progress.

- <a href="#errno.badf" name="errno.badf"></a> `badf`
Bad file descriptor.

- <a href="#errno.badmsg" name="errno.badmsg"></a> `badmsg`
Bad message.

- <a href="#errno.busy" name="errno.busy"></a> `busy`
Device or resource busy.

- <a href="#errno.canceled" name="errno.canceled"></a> `canceled`
Operation canceled.

- <a href="#errno.child" name="errno.child"></a> `child`
No child processes.

- <a href="#errno.connaborted" name="errno.connaborted"></a> `connaborted`
Connection aborted.

- <a href="#errno.connrefused" name="errno.connrefused"></a> `connrefused`
Connection refused.

- <a href="#errno.connreset" name="errno.connreset"></a> `connreset`
Connection reset.

- <a href="#errno.deadlk" name="errno.deadlk"></a> `deadlk`
Resource deadlock would occur.

- <a href="#errno.destaddrreq" name="errno.destaddrreq"></a> `destaddrreq`
Destination address required.

- <a href="#errno.dom" name="errno.dom"></a> `dom`
Mathematics argument out of domain of function.

- <a href="#errno.dquot" name="errno.dquot"></a> `dquot`
Reserved.

- <a href="#errno.exist" name="errno.exist"></a> `exist`
File exists.

- <a href="#errno.fault" name="errno.fault"></a> `fault`
Bad address.

- <a href="#errno.fbig" name="errno.fbig"></a> `fbig`
File too large.

- <a href="#errno.hostunreach" name="errno.hostunreach"></a> `hostunreach`
Host is unreachable.

- <a href="#errno.idrm" name="errno.idrm"></a> `idrm`
Identifier removed.

- <a href="#errno.ilseq" name="errno.ilseq"></a> `ilseq`
Illegal byte sequence.

- <a href="#errno.inprogress" name="errno.inprogress"></a> `inprogress`
Operation in progress.

- <a href="#errno.intr" name="errno.intr"></a> `intr`
Interrupted function.

- <a href="#errno.inval" name="errno.inval"></a> `inval`
Invalid argument.

- <a href="#errno.io" name="errno.io"></a> `io`
I/O error.

- <a href="#errno.isconn" name="errno.isconn"></a> `isconn`
Socket is connected.

- <a href="#errno.isdir" name="errno.isdir"></a> `isdir`
Is a directory.

- <a href="#errno.loop" name="errno.loop"></a> `loop`
Too many levels of symbolic links.

- <a href="#errno.mfile" name="errno.mfile"></a> `mfile`
File descriptor value too large.

- <a href="#errno.mlink" name="errno.mlink"></a> `mlink`
Too many links.

- <a href="#errno.msgsize" name="errno.msgsize"></a> `msgsize`
Message too large.

- <a href="#errno.multihop" name="errno.multihop"></a> `multihop`
Reserved.

- <a href="#errno.nametoolong" name="errno.nametoolong"></a> `nametoolong`
Filename too long.

- <a href="#errno.netdown" name="errno.netdown"></a> `netdown`
Network is down.

- <a href="#errno.netreset" name="errno.netreset"></a> `netreset`
Connection aborted by network.

- <a href="#errno.netunreach" name="errno.netunreach"></a> `netunreach`
Network unreachable.

- <a href="#errno.nfile" name="errno.nfile"></a> `nfile`
Too many files open in system.

- <a href="#errno.nobufs" name="errno.nobufs"></a> `nobufs`
No buffer space available.

- <a href="#errno.nodev" name="errno.nodev"></a> `nodev`
No such device.

- <a href="#errno.noent" name="errno.noent"></a> `noent`
No such file or directory.

- <a href="#errno.noexec" name="errno.noexec"></a> `noexec`
Executable file format error.

- <a href="#errno.nolck" name="errno.nolck"></a> `nolck`
No locks available.

- <a href="#errno.nolink" name="errno.nolink"></a> `nolink`
Reserved.

- <a href="#errno.nomem" name="errno.nomem"></a> `nomem`
Not enough space.

- <a href="#errno.nomsg" name="errno.nomsg"></a> `nomsg`
No message of the desired type.

- <a href="#errno.noprotoopt" name="errno.noprotoopt"></a> `noprotoopt`
Protocol not available.

- <a href="#errno.nospc" name="errno.nospc"></a> `nospc`
No space left on device.

- <a href="#errno.nosys" name="errno.nosys"></a> `nosys`
Function not supported.

- <a href="#errno.notconn" name="errno.notconn"></a> `notconn`
The socket is not connected.

- <a href="#errno.notdir" name="errno.notdir"></a> `notdir`
Not a directory or a symbolic link to a directory.

- <a href="#errno.notempty" name="errno.notempty"></a> `notempty`
Directory not empty.

- <a href="#errno.notrecoverable" name="errno.notrecoverable"></a> `notrecoverable`
State not recoverable.

- <a href="#errno.notsock" name="errno.notsock"></a> `notsock`
Not a socket.

- <a href="#errno.notsup" name="errno.notsup"></a> `notsup`
Not supported, or operation not supported on socket.

- <a href="#errno.notty" name="errno.notty"></a> `notty`
Inappropriate I/O control operation.

- <a href="#errno.nxio" name="errno.nxio"></a> `nxio`
No such device or address.

- <a href="#errno.overflow" name="errno.overflow"></a> `overflow`
Value too large to be stored in data type.

- <a href="#errno.ownerdead" name="errno.ownerdead"></a> `ownerdead`
Previous owner died.

- <a href="#errno.perm" name="errno.perm"></a> `perm`
Operation not permitted.

- <a href="#errno.pipe" name="errno.pipe"></a> `pipe`
Broken pipe.

- <a href="#errno.proto" name="errno.proto"></a> `proto`
Protocol error.

- <a href="#errno.protonosupport" name="errno.protonosupport"></a> `protonosupport`
Protocol not supported.

- <a href="#errno.prototype" name="errno.prototype"></a> `prototype`
Protocol wrong type for socket.

- <a href="#errno.range" name="errno.range"></a> `range`
Result too large.

- <a href="#errno.rofs" name="errno.rofs"></a> `rofs`
Read-only file system.

- <a href="#errno.spipe" name="errno.spipe"></a> `spipe`
Invalid seek.

- <a href="#errno.srch" name="errno.srch"></a> `srch`
No such process.

- <a href="#errno.stale" name="errno.stale"></a> `stale`
Reserved.

- <a href="#errno.timedout" name="errno.timedout"></a> `timedout`
Connection timed out.

- <a href="#errno.txtbsy" name="errno.txtbsy"></a> `txtbsy`
Text file busy.

- <a href="#errno.xdev" name="errno.xdev"></a> `xdev`
Cross-device link.

- <a href="#errno.notcapable" name="errno.notcapable"></a> `notcapable`
Extension: Capabilities insufficient.

## <a href="#cipher_state_initial" name="cipher_state_initial"></a> `cipher_state_initial`
Handles for encryption and decryption operations.

### Supertypes
## <a href="#cipher_state_encrypting" name="cipher_state_encrypting"></a> `cipher_state_encrypting`

### Supertypes
## <a href="#cipher_state_encrypted" name="cipher_state_encrypted"></a> `cipher_state_encrypted`

### Supertypes
## <a href="#cipher_state_decrypting" name="cipher_state_decrypting"></a> `cipher_state_decrypting`

### Supertypes
## <a href="#cipher_state_decrypted" name="cipher_state_decrypted"></a> `cipher_state_decrypted`

### Supertypes
## <a href="#cipher_state_final" name="cipher_state_final"></a> `cipher_state_final`

### Supertypes
## <a href="#hash_state_initial" name="hash_state_initial"></a> `hash_state_initial`
A handle for secure hashing operation.

### Supertypes
## <a href="#sign_state_initial" name="sign_state_initial"></a> `sign_state_initial`
A handle for signing operation.

### Supertypes
## <a href="#sign_state_signing" name="sign_state_signing"></a> `sign_state_signing`

### Supertypes
## <a href="#sign_state_final" name="sign_state_final"></a> `sign_state_final`

### Supertypes
## <a href="#derive_key_state_initial" name="derive_key_state_initial"></a> `derive_key_state_initial`
A handle for key or secret derivation operation.

### Supertypes
## <a href="#generate_key_state_initial" name="generate_key_state_initial"></a> `generate_key_state_initial`
A handle for private key generation operation.

### Supertypes
## <a href="#private_key" name="private_key"></a> `private_key`
A handle for a private key.

### Supertypes
# Modules
## <a href="#wasi_ephemeral_crypto" name="wasi_ephemeral_crypto"></a> wasi_ephemeral_crypto
### Imports
#### Memory
### Functions

---

#### <a href="#cipher_open" name="cipher_open"></a> `cipher_open(private_key: private_key) -> (errno, cipher_state_initial)`
Open a handle for cipher operations.

##### Params
- <a href="#cipher_open.private_key" name="cipher_open.private_key"></a> `private_key`: [`private_key`](#private_key)
The key for this operation.

##### Results
- <a href="#cipher_open.error" name="cipher_open.error"></a> `error`: [`errno`](#errno)

- <a href="#cipher_open.new_state" name="cipher_open.new_state"></a> `new_state`: [`cipher_state_initial`](#cipher_state_initial)
The cipher handle that has been opened.


---

#### <a href="#cipher_set_nonce" name="cipher_set_nonce"></a> `cipher_set_nonce(state: cipher_state_initial, nonce: ConstPointer<u8>, nonce_len: size) -> errno`
Set the nonce or initialization vector (IV).

This affects both encryption and decryption.

##### Params
- <a href="#cipher_set_nonce.state" name="cipher_set_nonce.state"></a> `state`: [`cipher_state_initial`](#cipher_state_initial)

- <a href="#cipher_set_nonce.nonce" name="cipher_set_nonce.nonce"></a> `nonce`: `ConstPointer<u8>`
The nonce to set for this message.

- <a href="#cipher_set_nonce.nonce_len" name="cipher_set_nonce.nonce_len"></a> `nonce_len`: [`size`](#size)

##### Results
- <a href="#cipher_set_nonce.error" name="cipher_set_nonce.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#cipher_set_auth_data" name="cipher_set_auth_data"></a> `cipher_set_auth_data(state: cipher_state_initial, auth: ConstPointer<u8>, auth_len: size) -> errno`
Set the additional authentication data (AAD). This works only if
the cipher algorithm is AEAD (Authenticated Encryption with
Additional Data); otherwise [`errno::inval`](#errno.inval)is returned.

##### Params
- <a href="#cipher_set_auth_data.state" name="cipher_set_auth_data.state"></a> `state`: [`cipher_state_initial`](#cipher_state_initial)

- <a href="#cipher_set_auth_data.auth" name="cipher_set_auth_data.auth"></a> `auth`: `ConstPointer<u8>`
The additional data to be authenticated.

- <a href="#cipher_set_auth_data.auth_len" name="cipher_set_auth_data.auth_len"></a> `auth_len`: [`size`](#size)

##### Results
- <a href="#cipher_set_auth_data.error" name="cipher_set_auth_data.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#cipher_get_auth_tag" name="cipher_get_auth_tag"></a> `cipher_get_auth_tag(state: cipher_state_initial, tag: Pointer<u8>, tag_len: size) -> errno`
Get the authentication tag generated by the last encryption
operation.  This only works if the cipher algorithm is AEAD
(Authenticated Encryption with Additional Data); otherwise
[`errno::inval`](#errno.inval)is returned.

##### Params
- <a href="#cipher_get_auth_tag.state" name="cipher_get_auth_tag.state"></a> `state`: [`cipher_state_initial`](#cipher_state_initial)

- <a href="#cipher_get_auth_tag.tag" name="cipher_get_auth_tag.tag"></a> `tag`: `Pointer<u8>`
The buffer where the authentication tag is stored. The length
must be equal to or shorter than the tag length specified for
the algorithm.  If it is shorter, only the first `tag_len`
bytes are written to `tag`.

- <a href="#cipher_get_auth_tag.tag_len" name="cipher_get_auth_tag.tag_len"></a> `tag_len`: [`size`](#size)

##### Results
- <a href="#cipher_get_auth_tag.error" name="cipher_get_auth_tag.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#cipher_set_auth_tag" name="cipher_set_auth_tag"></a> `cipher_set_auth_tag(state: cipher_state_initial, tag: ConstPointer<u8>, tag_len: size) -> errno`
Set the authentication tag for plaintext authentication. This
only works if the cipher algorithm is AEAD (Authenticated
Encryption with Additional Data); otherwise [`errno::inval`](#errno.inval) is
returned.

##### Params
- <a href="#cipher_set_auth_tag.state" name="cipher_set_auth_tag.state"></a> `state`: [`cipher_state_initial`](#cipher_state_initial)

- <a href="#cipher_set_auth_tag.tag" name="cipher_set_auth_tag.tag"></a> `tag`: `ConstPointer<u8>`
The authentication tag.

- <a href="#cipher_set_auth_tag.tag_len" name="cipher_set_auth_tag.tag_len"></a> `tag_len`: [`size`](#size)

##### Results
- <a href="#cipher_set_auth_tag.error" name="cipher_set_auth_tag.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#cipher_encrypt_begin" name="cipher_encrypt_begin"></a> `cipher_encrypt_begin(state: cipher_state_initial) -> (errno, cipher_state_encrypting)`
Initiate a series of encryption operations.

##### Params
- <a href="#cipher_encrypt_begin.state" name="cipher_encrypt_begin.state"></a> `state`: [`cipher_state_initial`](#cipher_state_initial)

##### Results
- <a href="#cipher_encrypt_begin.error" name="cipher_encrypt_begin.error"></a> `error`: [`errno`](#errno)

- <a href="#cipher_encrypt_begin.new_state" name="cipher_encrypt_begin.new_state"></a> `new_state`: [`cipher_state_encrypting`](#cipher_state_encrypting)


---

#### <a href="#cipher_encrypt" name="cipher_encrypt"></a> `cipher_encrypt(state: cipher_state_encrypting, plaintext: ConstPointer<u8>, plaintext_len: size, ciphertext: Pointer<u8>, ciphertext_len: size) -> errno`
Encrypt a single message.

##### Params
- <a href="#cipher_encrypt.state" name="cipher_encrypt.state"></a> `state`: [`cipher_state_encrypting`](#cipher_state_encrypting)

- <a href="#cipher_encrypt.plaintext" name="cipher_encrypt.plaintext"></a> `plaintext`: `ConstPointer<u8>`
The plaintext to be encrypted.

- <a href="#cipher_encrypt.plaintext_len" name="cipher_encrypt.plaintext_len"></a> `plaintext_len`: [`size`](#size)

- <a href="#cipher_encrypt.ciphertext" name="cipher_encrypt.ciphertext"></a> `ciphertext`: `Pointer<u8>`
The buffer where the resulting ciphertext is stored. This can
overlap with `plaintext`.

- <a href="#cipher_encrypt.ciphertext_len" name="cipher_encrypt.ciphertext_len"></a> `ciphertext_len`: [`size`](#size)

##### Results
- <a href="#cipher_encrypt.error" name="cipher_encrypt.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#cipher_encrypt_end" name="cipher_encrypt_end"></a> `cipher_encrypt_end(state: cipher_state_encrypting) -> (errno, cipher_state_initial)`
Finalize a series of encryption operations.

##### Params
- <a href="#cipher_encrypt_end.state" name="cipher_encrypt_end.state"></a> `state`: [`cipher_state_encrypting`](#cipher_state_encrypting)

##### Results
- <a href="#cipher_encrypt_end.error" name="cipher_encrypt_end.error"></a> `error`: [`errno`](#errno)

- <a href="#cipher_encrypt_end.new_state" name="cipher_encrypt_end.new_state"></a> `new_state`: [`cipher_state_initial`](#cipher_state_initial)


---

#### <a href="#cipher_decrypt_begin" name="cipher_decrypt_begin"></a> `cipher_decrypt_begin(state: cipher_state_initial) -> (errno, cipher_state_decrypting)`
Initiate a series of decryption operations.

##### Params
- <a href="#cipher_decrypt_begin.state" name="cipher_decrypt_begin.state"></a> `state`: [`cipher_state_initial`](#cipher_state_initial)

##### Results
- <a href="#cipher_decrypt_begin.error" name="cipher_decrypt_begin.error"></a> `error`: [`errno`](#errno)

- <a href="#cipher_decrypt_begin.new_state" name="cipher_decrypt_begin.new_state"></a> `new_state`: [`cipher_state_decrypting`](#cipher_state_decrypting)


---

#### <a href="#cipher_decrypt" name="cipher_decrypt"></a> `cipher_decrypt(state: cipher_state_decrypting, ciphertext: ConstPointer<u8>, ciphertext_len: size, plaintext: Pointer<u8>, plaintext_len: size) -> errno`
Decryption a single message.

##### Params
- <a href="#cipher_decrypt.state" name="cipher_decrypt.state"></a> `state`: [`cipher_state_decrypting`](#cipher_state_decrypting)

- <a href="#cipher_decrypt.ciphertext" name="cipher_decrypt.ciphertext"></a> `ciphertext`: `ConstPointer<u8>`
The ciphertext to be decrypted.

- <a href="#cipher_decrypt.ciphertext_len" name="cipher_decrypt.ciphertext_len"></a> `ciphertext_len`: [`size`](#size)

- <a href="#cipher_decrypt.plaintext" name="cipher_decrypt.plaintext"></a> `plaintext`: `Pointer<u8>`
The buffer where the resulting plaintext is stored. This can
overlap with `plaintext`.

- <a href="#cipher_decrypt.plaintext_len" name="cipher_decrypt.plaintext_len"></a> `plaintext_len`: [`size`](#size)

##### Results
- <a href="#cipher_decrypt.error" name="cipher_decrypt.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#cipher_decrypt_end" name="cipher_decrypt_end"></a> `cipher_decrypt_end(state: cipher_state_decrypted) -> (errno, cipher_state_initial)`
Finalize a series of decryption operations.

##### Params
- <a href="#cipher_decrypt_end.state" name="cipher_decrypt_end.state"></a> `state`: [`cipher_state_decrypted`](#cipher_state_decrypted)

##### Results
- <a href="#cipher_decrypt_end.error" name="cipher_decrypt_end.error"></a> `error`: [`errno`](#errno)

- <a href="#cipher_decrypt_end.new_state" name="cipher_decrypt_end.new_state"></a> `new_state`: [`cipher_state_initial`](#cipher_state_initial)


---

#### <a href="#cipher_close" name="cipher_close"></a> `cipher_close(state: cipher_state_initial) -> errno`
Close a handle for cipher operations. The closed handle cannot
be used afterwards.

##### Params
- <a href="#cipher_close.state" name="cipher_close.state"></a> `state`: [`cipher_state_initial`](#cipher_state_initial)

##### Results
- <a href="#cipher_close.error" name="cipher_close.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#hash_open" name="hash_open"></a> `hash_open(algorithm: string) -> (errno, hash_state_initial)`
Create a handle for secure hashing operations.

##### Params
- <a href="#hash_open.algorithm" name="hash_open.algorithm"></a> `algorithm`: `string`
The name of the hash algorithm, either predefined or defined
by the implementation. The currently defined value are
`SHA-1`, `SHA-224`, `SHA-256`, `SHA-384`, `SHA-512`,
`SHA3-224`, `SHA3-256`, `SHA3-384`, and `SHA3-512`.

##### Results
- <a href="#hash_open.error" name="hash_open.error"></a> `error`: [`errno`](#errno)

- <a href="#hash_open.new_state" name="hash_open.new_state"></a> `new_state`: [`hash_state_initial`](#hash_state_initial)


---

#### <a href="#hash_update" name="hash_update"></a> `hash_update(state: hash_state_initial, data: ConstPointer<u8>, data_len: size) -> errno`
Feed some data for hashing. This function can be called as many
times while the hash handle remains open.

##### Params
- <a href="#hash_update.state" name="hash_update.state"></a> `state`: [`hash_state_initial`](#hash_state_initial)

- <a href="#hash_update.data" name="hash_update.data"></a> `data`: `ConstPointer<u8>`
The data to be hashed.

- <a href="#hash_update.data_len" name="hash_update.data_len"></a> `data_len`: [`size`](#size)

##### Results
- <a href="#hash_update.error" name="hash_update.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#hash_digest" name="hash_digest"></a> `hash_digest(state: hash_state_initial, digest: Pointer<u8>, digest_len: size) -> errno`
Extract the hash value. This function can be called as many
times while the hash handle remains open and it is safe to call
[`hash_update`](#hash_update) subsequently.

##### Params
- <a href="#hash_digest.state" name="hash_digest.state"></a> `state`: [`hash_state_initial`](#hash_state_initial)

- <a href="#hash_digest.digest" name="hash_digest.digest"></a> `digest`: `Pointer<u8>`
The buffer where the digest value is stored. For XOF
(Extensible Output Function) such as SHAKE, the length can be
arbitrary as long as it is smaller than the size allocated for
`digest`. Otherwise, the length must be equal to or shorter
than the tag length specified for the algorithm.  If it is
shorter, only the first `digest_len` bytes are written to
`digest`.

- <a href="#hash_digest.digest_len" name="hash_digest.digest_len"></a> `digest_len`: [`size`](#size)

##### Results
- <a href="#hash_digest.error" name="hash_digest.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#hash_close" name="hash_close"></a> `hash_close(state: hash_state_initial) -> errno`
Close a handle for secure hashing operations. The closed handle
cannot be used afterwards.

##### Params
- <a href="#hash_close.state" name="hash_close.state"></a> `state`: [`hash_state_initial`](#hash_state_initial)

##### Results
- <a href="#hash_close.error" name="hash_close.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#sign_open" name="sign_open"></a> `sign_open(algorithm: string, private_key: private_key) -> (errno, sign_state_initial)`
Create a handle of signing operation. The operation can be
either a public key signing or a MAC (Message Authentication
Code) algorithm.

##### Params
- <a href="#sign_open.algorithm" name="sign_open.algorithm"></a> `algorithm`: `string`
The name of the signing algorithm. This is either from the
IANA assignments of JOSE (JSON Object Signing and Encryption)
or defined by the host implementation.

- <a href="#sign_open.private_key" name="sign_open.private_key"></a> `private_key`: [`private_key`](#private_key)
The key for this operation.

##### Results
- <a href="#sign_open.error" name="sign_open.error"></a> `error`: [`errno`](#errno)

- <a href="#sign_open.new_state" name="sign_open.new_state"></a> `new_state`: [`sign_state_initial`](#sign_state_initial)


---

#### <a href="#sign_set_nonce" name="sign_set_nonce"></a> `sign_set_nonce(state: sign_state_initial, nonce: ConstPointer<u8>, nonce_len: size) -> errno`
Set the nonce for the subsequent signing operation. This
function must be called before [`sign_update`](#sign_update) or `sign_digest` is
called.

##### Params
- <a href="#sign_set_nonce.state" name="sign_set_nonce.state"></a> `state`: [`sign_state_initial`](#sign_state_initial)

- <a href="#sign_set_nonce.nonce" name="sign_set_nonce.nonce"></a> `nonce`: `ConstPointer<u8>`
The nonce for this operation.

- <a href="#sign_set_nonce.nonce_len" name="sign_set_nonce.nonce_len"></a> `nonce_len`: [`size`](#size)

##### Results
- <a href="#sign_set_nonce.error" name="sign_set_nonce.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#sign_begin" name="sign_begin"></a> `sign_begin(state: sign_state_initial) -> (errno, sign_state_signing)`
Initiate a series of signing operations.

##### Params
- <a href="#sign_begin.state" name="sign_begin.state"></a> `state`: [`sign_state_initial`](#sign_state_initial)

##### Results
- <a href="#sign_begin.error" name="sign_begin.error"></a> `error`: [`errno`](#errno)

- <a href="#sign_begin.new_state" name="sign_begin.new_state"></a> `new_state`: [`sign_state_signing`](#sign_state_signing)


---

#### <a href="#sign_update" name="sign_update"></a> `sign_update(state: sign_state_signing, data: ConstPointer<u8>, data_len: size) -> errno`
Feed some data to be signed. This function can be called as many
times while the sign handle is open.

##### Params
- <a href="#sign_update.state" name="sign_update.state"></a> `state`: [`sign_state_signing`](#sign_state_signing)

- <a href="#sign_update.data" name="sign_update.data"></a> `data`: `ConstPointer<u8>`
The data to be signed.

- <a href="#sign_update.data_len" name="sign_update.data_len"></a> `data_len`: [`size`](#size)

##### Results
- <a href="#sign_update.error" name="sign_update.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#sign" name="sign"></a> `sign(state: sign_state_signing, digest: Pointer<u8>, digest_len: size) -> errno`
Extract the signature. This function can be called as many times
while the sign handle is open and it is safe to call
[`sign_update`](#sign_update) subsequently. That is particularly useful when the
algorithm is MAC (Message Authentication Code).

##### Params
- <a href="#sign.state" name="sign.state"></a> `state`: [`sign_state_signing`](#sign_state_signing)

- <a href="#sign.digest" name="sign.digest"></a> `digest`: `Pointer<u8>`
The buffer where the generated signature is stored. The length
must be equal to or shorter than the output length specified
for the algorithm.  If it is shorter, only the first
`signature_len` bytes are written to `signature`.

- <a href="#sign.digest_len" name="sign.digest_len"></a> `digest_len`: [`size`](#size)

##### Results
- <a href="#sign.error" name="sign.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#sign_end" name="sign_end"></a> `sign_end(state: sign_state_signing) -> (errno, sign_state_initial)`
Complete a series of signing operations.

##### Params
- <a href="#sign_end.state" name="sign_end.state"></a> `state`: [`sign_state_signing`](#sign_state_signing)

##### Results
- <a href="#sign_end.error" name="sign_end.error"></a> `error`: [`errno`](#errno)

- <a href="#sign_end.new_state" name="sign_end.new_state"></a> `new_state`: [`sign_state_initial`](#sign_state_initial)


---

#### <a href="#sign_close" name="sign_close"></a> `sign_close(state: sign_state_initial) -> errno`
Close a handle for signing operations.

##### Params
- <a href="#sign_close.state" name="sign_close.state"></a> `state`: [`sign_state_initial`](#sign_state_initial)

##### Results
- <a href="#sign_close.error" name="sign_close.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#generate_key_prepare" name="generate_key_prepare"></a> `generate_key_prepare(algorithm: string) -> (errno, generate_key_state_initial)`
Prepare a handle for private key generation for an
algorithm. The algorithm can be a public key encryption and
signing algorithms, a symmetric key encryption, MAC, or key
derivation.

##### Params
- <a href="#generate_key_prepare.algorithm" name="generate_key_prepare.algorithm"></a> `algorithm`: `string`
The name of the algorithm which the resultant private key is
used with. This is either from the IANA assignments of JSON
Object Signing and Encryption (JOSE) or defined by the host
implementation.

##### Results
- <a href="#generate_key_prepare.error" name="generate_key_prepare.error"></a> `error`: [`errno`](#errno)

- <a href="#generate_key_prepare.new_state" name="generate_key_prepare.new_state"></a> `new_state`: [`generate_key_state_initial`](#generate_key_state_initial)


---

#### <a href="#generate_key_set_nonce" name="generate_key_set_nonce"></a> `generate_key_set_nonce(state: generate_key_state_initial, nonce: ConstPointer<u8>, nonce_len: size) -> errno`
Set the nonce needed for some algorithms. For example, to
generate a pseudo-random key with HKDF-Extract, it is an
optional salt value.

##### Params
- <a href="#generate_key_set_nonce.state" name="generate_key_set_nonce.state"></a> `state`: [`generate_key_state_initial`](#generate_key_state_initial)

- <a href="#generate_key_set_nonce.nonce" name="generate_key_set_nonce.nonce"></a> `nonce`: `ConstPointer<u8>`

- <a href="#generate_key_set_nonce.nonce_len" name="generate_key_set_nonce.nonce_len"></a> `nonce_len`: [`size`](#size)

##### Results
- <a href="#generate_key_set_nonce.error" name="generate_key_set_nonce.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#generate_key_set_input" name="generate_key_set_input"></a> `generate_key_set_input(state: generate_key_state_initial, input: ConstPointer<u8>, input_len: size) -> errno`
Set the input keying material shall be used as a base of the
generated private key for some algorithms.

##### Params
- <a href="#generate_key_set_input.state" name="generate_key_set_input.state"></a> `state`: [`generate_key_state_initial`](#generate_key_state_initial)

- <a href="#generate_key_set_input.input" name="generate_key_set_input.input"></a> `input`: `ConstPointer<u8>`

- <a href="#generate_key_set_input.input_len" name="generate_key_set_input.input_len"></a> `input_len`: [`size`](#size)

##### Results
- <a href="#generate_key_set_input.error" name="generate_key_set_input.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#generate_key_set_hash" name="generate_key_set_hash"></a> `generate_key_set_hash(state: generate_key_state_initial, hash_state: hash_state_initial) -> errno`
Set the hash algorithm used for the private key generation.

##### Params
- <a href="#generate_key_set_hash.state" name="generate_key_set_hash.state"></a> `state`: [`generate_key_state_initial`](#generate_key_state_initial)

- <a href="#generate_key_set_hash.hash_state" name="generate_key_set_hash.hash_state"></a> `hash_state`: [`hash_state_initial`](#hash_state_initial)

##### Results
- <a href="#generate_key_set_hash.error" name="generate_key_set_hash.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#generate_key" name="generate_key"></a> `generate_key(state: generate_key_state_initial) -> (errno, private_key)`
Generate a private key and close the handle for the private key
generation operation. The closed handle cannot be used
afterwards.

##### Params
- <a href="#generate_key.state" name="generate_key.state"></a> `state`: [`generate_key_state_initial`](#generate_key_state_initial)

##### Results
- <a href="#generate_key.error" name="generate_key.error"></a> `error`: [`errno`](#errno)

- <a href="#generate_key.private_key" name="generate_key.private_key"></a> `private_key`: [`private_key`](#private_key)


---

#### <a href="#derive_key_prepare" name="derive_key_prepare"></a> `derive_key_prepare(algorithm: string, private_key: private_key) -> (errno, derive_key_state_initial)`
Prepare a handle for key derivation for an algorithm. The
algorithm can be either a KDF (Key Derivation Function)
algorithm or a shared secret derivation algorithm.

##### Params
- <a href="#derive_key_prepare.algorithm" name="derive_key_prepare.algorithm"></a> `algorithm`: `string`

- <a href="#derive_key_prepare.private_key" name="derive_key_prepare.private_key"></a> `private_key`: [`private_key`](#private_key)

##### Results
- <a href="#derive_key_prepare.error" name="derive_key_prepare.error"></a> `error`: [`errno`](#errno)

- <a href="#derive_key_prepare.new_state" name="derive_key_prepare.new_state"></a> `new_state`: [`derive_key_state_initial`](#derive_key_state_initial)


---

#### <a href="#derive_key_set_nonce" name="derive_key_set_nonce"></a> `derive_key_set_nonce(state: derive_key_state_initial, nonce: ConstPointer<u8>, nonce_len: size) -> errno`
Set the nonce for the key derivation operation for some
algorithms. For example, to derive an output keying material
with HKDF-Expand, it is an additional "info" value.

##### Params
- <a href="#derive_key_set_nonce.state" name="derive_key_set_nonce.state"></a> `state`: [`derive_key_state_initial`](#derive_key_state_initial)

- <a href="#derive_key_set_nonce.nonce" name="derive_key_set_nonce.nonce"></a> `nonce`: `ConstPointer<u8>`
The nonce value (a non-secret random value).

- <a href="#derive_key_set_nonce.nonce_len" name="derive_key_set_nonce.nonce_len"></a> `nonce_len`: [`size`](#size)

##### Results
- <a href="#derive_key_set_nonce.error" name="derive_key_set_nonce.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#derive_key_set_hash" name="derive_key_set_hash"></a> `derive_key_set_hash(state: derive_key_state_initial, hash_state: hash_state_initial) -> errno`
Set hash algorithm for the key derivation operation.

##### Params
- <a href="#derive_key_set_hash.state" name="derive_key_set_hash.state"></a> `state`: [`derive_key_state_initial`](#derive_key_state_initial)

- <a href="#derive_key_set_hash.hash_state" name="derive_key_set_hash.hash_state"></a> `hash_state`: [`hash_state_initial`](#hash_state_initial)

##### Results
- <a href="#derive_key_set_hash.error" name="derive_key_set_hash.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#derive_key" name="derive_key"></a> `derive_key(state: derive_key_state_initial, output: Pointer<u8>, output_len: size) -> errno`
Derive a secret and close the handle for the key derivation
operation. The closed handle cannot be used afterwards.

##### Params
- <a href="#derive_key.state" name="derive_key.state"></a> `state`: [`derive_key_state_initial`](#derive_key_state_initial)

- <a href="#derive_key.output" name="derive_key.output"></a> `output`: `Pointer<u8>`
The buffer where the generated key or secret is stored. The
length must be equal to or shorter than the tag length
specified for the algorithm.  If it is shorter, only the first
`output_len` bytes are written to `output`.

- <a href="#derive_key.output_len" name="derive_key.output_len"></a> `output_len`: [`size`](#size)

##### Results
- <a href="#derive_key.error" name="derive_key.error"></a> `error`: [`errno`](#errno)

