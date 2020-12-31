import * as crypto from "./wasi_crypto";
import { error, buf, ptr, fromWasiArray } from "./common";
import { KeyPair, PublicKey } from "./asymmetric_common";

export class Signature {
    handle: crypto.signature;

    constructor(handle: crypto.signature) {
        this.handle = handle;
    }

    protected encoded_as(encoding: crypto.signature_encoding): ArrayBuffer | null {
        if ((error.last = crypto.signature_export(this.handle, encoding, buf))) {
            return null;
        }
        return fromWasiArray(load<crypto.symmetric_tag>(buf));
    }

    raw(): ArrayBuffer | null {
        return this.encoded_as(crypto.signature_encoding.RAW);
    }

    der(): ArrayBuffer | null {
        return this.encoded_as(crypto.signature_encoding.DER);
    }

    protected static decode_from(alg: string, encoded: ArrayBuffer, encoding: crypto.signature_encoding): Signature | null {
        let wasiAlg = new crypto.WasiString(alg);
        if ((error.last = crypto.signature_import(wasiAlg.ptr, wasiAlg.length, changetype<ptr<u8>>(encoded), encoded.byteLength, encoding, buf))) {
            return null;
        }
        return new Signature(load<crypto.handle>(buf));
    }

    static fromRaw(alg: string, encoded: ArrayBuffer): Signature | null {
        return this.decode_from(alg, encoded, crypto.signature_encoding.RAW);
    }

    static fromDer(alg: string, encoded: ArrayBuffer): Signature | null {
        return this.decode_from(alg, encoded, crypto.signature_encoding.DER);
    }
}

export class SignaturePublicKey extends PublicKey {
    signature_verify(msg: ArrayBuffer, signature: Signature): bool {
        if ((error.last = crypto.signature_verification_state_open(this.handle, buf))) {
            return false
        }
        let state = load<crypto.signature_verification_state>(buf);
        if ((error.last = crypto.signature_verification_state_update(state, changetype<ptr<u8>>(msg), msg.byteLength))) {
            return false;
        }
        error.last = crypto.signature_verification_state_verify(state, signature.handle);
        crypto.signature_verification_state_close(state);
        return error.last === 0;
    }

    static fromRaw(alg: string, encoded: ArrayBuffer): SignaturePublicKey | null {
        return changetype<SignaturePublicKey | null>(super._fromRaw(crypto.algorithm_type.SIGNATURES, alg, encoded))
    }

    static fromPkcs8(alg: string, encoded: ArrayBuffer): SignaturePublicKey | null {
        return changetype<SignaturePublicKey | null>(super._fromPkcs8(crypto.algorithm_type.SIGNATURES, alg, encoded))
    }

    static fromPem(alg: string, encoded: ArrayBuffer): SignaturePublicKey | null {
        return changetype<SignaturePublicKey | null>(super._fromPem(crypto.algorithm_type.SIGNATURES, alg, encoded))
    }

    static fromSec(alg: string, encoded: ArrayBuffer): SignaturePublicKey | null {
        return changetype<SignaturePublicKey | null>(super._fromSec(crypto.algorithm_type.SIGNATURES, alg, encoded))
    }

    static fromCompressedSec(alg: string, encoded: ArrayBuffer): SignaturePublicKey | null {
        return changetype<SignaturePublicKey | null>(super._fromCompressedSec(crypto.algorithm_type.SIGNATURES, alg, encoded))
    }

    static fromLocal(alg: string, encoded: ArrayBuffer): SignaturePublicKey | null {
        return changetype<SignaturePublicKey | null>(super._fromLocal(crypto.algorithm_type.SIGNATURES, alg, encoded))
    }
}

export class SignatureKeyPair extends KeyPair {
    static generate(alg: string): SignatureKeyPair | null {
        return changetype<SignatureKeyPair | null>(KeyPair._generate(crypto.algorithm_type.SIGNATURES, alg));
    }

    publicKey(): SignaturePublicKey | null {
        return changetype<SignaturePublicKey | null>(super.publicKey())
    }

    static fromRaw(alg: string, encoded: ArrayBuffer): SignatureKeyPair | null {
        return changetype<SignatureKeyPair | null>(super._fromRaw(crypto.algorithm_type.SIGNATURES, alg, encoded))
    }

    static fromPkcs8(alg: string, encoded: ArrayBuffer): SignatureKeyPair | null {
        return changetype<SignatureKeyPair | null>(super._fromPkcs8(crypto.algorithm_type.SIGNATURES, alg, encoded))
    }

    static fromPem(alg: string, encoded: ArrayBuffer): SignatureKeyPair | null {
        return changetype<SignatureKeyPair | null>(super._fromPem(crypto.algorithm_type.SIGNATURES, alg, encoded))
    }

    static fromLocal(alg: string, encoded: ArrayBuffer): SignatureKeyPair | null {
        return changetype<SignatureKeyPair | null>(super._fromLocal(crypto.algorithm_type.SIGNATURES, alg, encoded))
    }

    sign(msg: ArrayBuffer): Signature | null {
        if ((error.last = crypto.signature_state_open(this.handle, buf))) {
            return null
        }
        let state = load<crypto.signature_state>(buf);
        if ((error.last = crypto.signature_state_update(state, changetype<ptr<u8>>(msg), msg.byteLength))) {
            return null;
        }
        if ((error.last = crypto.signature_state_sign(state, buf))) {
            return null;
        }
        crypto.signature_state_close(state);
        return new Signature(load<crypto.signature>(buf));
    }
}
