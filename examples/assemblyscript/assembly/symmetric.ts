import * as crypto from "./wasi_crypto";
import { error, buf, ptr, fromWasiArray } from "./common";

export class SymmetricKey {
    handle: crypto.symmetric_key;
    alg: string;

    constructor(handle: crypto.symmetric_key, alg: string) {
        this.handle = handle;
        this.alg = alg;
    }

    static generate(alg: string): SymmetricKey | null {
        let wasiAlg = new crypto.WasiString(alg);
        if ((error.last = crypto.symmetric_key_generate(wasiAlg.ptr, wasiAlg.length, crypto.opt_options.none(), buf))) {
            return null;
        }
        return new SymmetricKey(load<crypto.symmetric_key>(buf), alg);
    }

    static fromRaw(alg: string, raw: ArrayBuffer): SymmetricKey | null {
        let wasiAlg = new crypto.WasiString(alg);
        if ((error.last = crypto.symmetric_key_import(wasiAlg.ptr, wasiAlg.length, changetype<ptr<u8>>(raw), raw.byteLength, buf))) {
            return null;
        }
        return new SymmetricKey(load<crypto.handle>(buf), alg);
    }

    raw(): ArrayBuffer | null {
        if ((error.last = crypto.symmetric_key_export(this.handle, buf))) {
            return null;
        }
        return fromWasiArray(load<crypto.symmetric_tag>(buf));
    }
}

export class Hash {
    state: crypto.symmetric_state;

    protected constructor(state: crypto.symmetric_state) {
        this.state = state;
    }

    protected static new(alg: string, key: SymmetricKey | null = null): Hash | null {
        let wasiAlg = new crypto.WasiString(alg);
        let optKey = key ? crypto.opt_symmetric_key.some(key.handle) : crypto.opt_symmetric_key.none();
        if ((error.last = crypto.symmetric_state_open(wasiAlg.ptr, wasiAlg.length, optKey, crypto.opt_options.none(), buf))) {
            return null;
        }
        let state = load<crypto.symmetric_state>(buf);
        return new Hash(state);
    }

    static keyed(key: SymmetricKey): Hash | null {
        return Hash.new(key.alg, key);
    }

    static unkeyed(alg: string): Hash | null {
        return Hash.new(alg);
    }

    absorb(msg: ArrayBuffer): bool {
        if ((error.last = crypto.symmetric_state_absorb(this.state, changetype<usize>(msg), msg.byteLength))) {
            return false;
        }
        return true;
    }

    squeeze(outLen: usize): ArrayBuffer | null {
        let out = new ArrayBuffer(outLen as i32);
        if ((error.last = crypto.symmetric_state_squeeze(this.state, changetype<usize>(out), outLen))) {
            return null;
        }
        return out;
    }

    static hash(alg: string, msg: ArrayBuffer, outLen: usize, key: SymmetricKey | null = null): ArrayBuffer | null {
        let st = Hash.new(alg, key);
        if (!st) {
            return null;
        }
        if (!st.absorb(msg)) {
            return null;
        }
        return st.squeeze(outLen);
    }
}

export class CiphertextAndTag {
    ciphertext: ArrayBuffer;
    rawTag: ArrayBuffer;

    constructor(ciphertext: ArrayBuffer, rawTag: ArrayBuffer) {
        this.ciphertext = ciphertext;
        this.rawTag = rawTag;
    }
}

export class Aead {
    state: crypto.symmetric_state;

    protected constructor(state: crypto.symmetric_state) {
        this.state = state;
    }

    static new(key: SymmetricKey, nonce: ArrayBuffer | null, ad: ArrayBuffer | null): Aead | null {
        let wasiAlg = new crypto.WasiString(key.alg);
        if (crypto.options_open(crypto.algorithm_type.SYMMETRIC, buf)) {
            return null;
        }
        let options = load<crypto.options>(buf);
        if (nonce) {
            let wasiOptionStr = new crypto.WasiString("nonce");
            crypto.options_set(options, wasiOptionStr.ptr, wasiOptionStr.length, changetype<ptr<u8>>(nonce), nonce.byteLength);
        }
        if ((error.last = crypto.symmetric_state_open(wasiAlg.ptr, wasiAlg.length, crypto.opt_symmetric_key.some(key.handle), crypto.opt_options.some(options), buf))) {
            return null;
        }
        let state = load<crypto.symmetric_state>(buf);
        let aead = new Aead(state);
        if (ad !== null) {
            if ((error.last = crypto.symmetric_state_absorb(aead.state, changetype<usize>(ad), ad.byteLength)) != 0) {
                return null;
            }
        }
        return aead;
    }

    maxTagLen(): usize {
        if ((error.last = crypto.symmetric_state_max_tag_len(this.state, buf))) {
            return 0;
        }
        let maxTagLen = load<usize>(buf);
        return maxTagLen;
    }

    encrypt(msg: ArrayBuffer): ArrayBuffer | null {
        let maxTagLen = this.maxTagLen();
        let out = new ArrayBuffer(msg.byteLength + (maxTagLen as i32));
        if ((error.last = crypto.symmetric_state_encrypt(this.state, changetype<ptr<u8>>(out), msg.byteLength as usize + maxTagLen, changetype<ptr<u8>>(msg), msg.byteLength, buf))) {
            return null;
        }
        return out.slice(0, load<usize>(buf) as i32);
    }

    decrypt(ciphertext: ArrayBuffer): ArrayBuffer | null {
        let maxTagLen = this.maxTagLen();
        if (ciphertext.byteLength < (maxTagLen as i32)) {
            return null;
        }
        let out = new ArrayBuffer(ciphertext.byteLength - (maxTagLen as i32));
        if ((crypto.symmetric_state_decrypt(this.state, changetype<ptr<u8>>(out), out.byteLength, changetype<ptr<u8>>(ciphertext), ciphertext.byteLength, buf))) {
            return null;
        }
        return out.slice(0, load<usize>(buf) as i32);
    }

    encryptDetached(msg: ArrayBuffer): CiphertextAndTag | null {
        let ciphertext = new ArrayBuffer(msg.byteLength);
        if ((crypto.symmetric_state_encrypt_detached(this.state, changetype<ptr<u8>>(ciphertext), ciphertext.byteLength, changetype<ptr<u8>>(msg), msg.byteLength, buf))) {
            return null;
        }
        let tag = load<crypto.symmetric_tag>(buf);
        crypto.symmetric_tag_len(tag, buf);
        let rawTag = new ArrayBuffer(load<usize>(buf) as i32);
        crypto.symmetric_tag_pull(tag, changetype<ptr<u8>>(rawTag), rawTag.byteLength, buf);
        return new CiphertextAndTag(ciphertext, rawTag);
    }

    decryptDetached(ciphertextAndTag: CiphertextAndTag): ArrayBuffer | null {
        let msg = new ArrayBuffer(ciphertextAndTag.ciphertext.byteLength);
        if ((crypto.symmetric_state_decrypt_detached(this.state, changetype<ptr<u8>>(msg), msg.byteLength, changetype<ptr<u8>>(ciphertextAndTag.ciphertext), ciphertextAndTag.ciphertext.byteLength, changetype<ptr<u8>>(ciphertextAndTag.rawTag), ciphertextAndTag.rawTag.byteLength, buf))) {
            return null;
        }
        return msg.slice(0, load<usize>(buf) as i32);
    }
}

export class Auth {
    state: crypto.symmetric_state;

    protected constructor(state: crypto.symmetric_state) {
        this.state = state;
    }

    static new(alg: string, key: SymmetricKey): Auth | null {
        let wasiAlg = new crypto.WasiString(alg);
        if ((error.last = crypto.symmetric_state_open(wasiAlg.ptr, wasiAlg.length, crypto.opt_symmetric_key.some(key.handle), crypto.opt_options.none(), buf))) {
            return null;
        }
        let state = load<crypto.symmetric_state>(buf);
        return new Auth(state);
    }

    absorb(msg: ArrayBuffer): bool {
        if ((error.last = crypto.symmetric_state_absorb(this.state, changetype<usize>(msg), msg.byteLength))) {
            return false;
        }
        return true;
    }

    tag(): ArrayBuffer | null {
        if ((error.last = crypto.symmetric_state_squeeze_tag(this.state, buf))) {
            return null;
        }
        let tag = load<crypto.symmetric_tag>(buf);
        crypto.symmetric_tag_len(tag, buf);
        let rawTag = new ArrayBuffer(load<usize>(buf) as i32);
        crypto.symmetric_tag_pull(tag, changetype<ptr<u8>>(rawTag), rawTag.byteLength, buf);
        return rawTag;
    }

    verify(rawTag: ArrayBuffer): bool {
        if ((error.last = crypto.symmetric_state_squeeze_tag(this.state, buf))) {
            return false;
        }
        let tag = load<crypto.symmetric_tag>(buf);
        if ((error.last = crypto.symmetric_tag_verify(tag, changetype<ptr<u8>>(rawTag), rawTag.byteLength))) {
            return false;
        }
        return true;
    }

    static auth(msg: ArrayBuffer, key: SymmetricKey): ArrayBuffer | null {
        let st = Auth.new(key.alg, key);
        if (!st) {
            return null;
        }
        if (!st.absorb(msg)) {
            return null;
        }
        return st.tag();
    }

    static verify(msg: ArrayBuffer, key: SymmetricKey, rawTag: ArrayBuffer): bool {
        let st = Auth.new(key.alg, key);
        if (!st) {
            return false;
        }
        if (!st.absorb(msg)) {
            return false;
        }
        return st.verify(rawTag);
    }
}

export class Hkdf {
    static extract(prkAlg: string, key: SymmetricKey, salt: ArrayBuffer | null = null): SymmetricKey | null {
        let wasiAlg = new crypto.WasiString(key.alg);
        if ((error.last = crypto.symmetric_state_open(wasiAlg.ptr, wasiAlg.length, crypto.opt_symmetric_key.some(key.handle), crypto.opt_options.none(), buf))) {
            return null;
        }
        let state = load<crypto.symmetric_state>(buf);
        if (salt) {
            if ((error.last = crypto.symmetric_state_absorb(state, changetype<ptr<u8>>(salt), salt.byteLength))) {
                return null;
            }
        }
        let wasiPrkAlg = new crypto.WasiString(prkAlg);
        if ((error.last = crypto.symmetric_state_squeeze_key(state, wasiPrkAlg.ptr, wasiPrkAlg.length, buf))) {
            return null;
        }
        crypto.symmetric_state_close(state);
        return new SymmetricKey(load<crypto.symmetric_key>(buf), prkAlg);
    }

    static expand(prk: SymmetricKey, info: ArrayBuffer, outLen: usize): ArrayBuffer | null {
        let wasiAlg = new crypto.WasiString(prk.alg);
        if ((error.last = crypto.symmetric_state_open(wasiAlg.ptr, wasiAlg.length, crypto.opt_symmetric_key.some(prk.handle), crypto.opt_options.none(), buf))) {
            return null;
        }
        let state = load<crypto.symmetric_state>(buf);
        if ((error.last = crypto.symmetric_state_absorb(state, changetype<ptr<u8>>(info), info.byteLength))) {
            return null;
        }
        let out = new ArrayBuffer(outLen as i32);
        if ((error.last = crypto.symmetric_state_squeeze(state, changetype<usize>(out), outLen))) {
            return null;
        }
        crypto.symmetric_state_close(state);
        return out;
    }
}
