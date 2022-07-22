import * as crypto from "./wasi_crypto";
import {error, buf, ptr, fromWasiArray} from "./common";
import {KeyPair, PublicKey} from "./asymmetric_common";

export class Signature {
    handle: crypto.Signature;

    constructor(handle: crypto.Signature) {
        this.handle = handle;
    }

    protected encoded_as(encoding: crypto.SignatureEncoding): ArrayBuffer | null {
        if ((error.last = crypto.signatureExport(this.handle, encoding, buf))) {
            return null;
        }
        return fromWasiArray(load<crypto.SymmetricTag>(buf));
    }

    raw(): ArrayBuffer | null {
        return this.encoded_as(crypto.SignatureEncoding.RAW);
    }

    der(): ArrayBuffer | null {
        return this.encoded_as(crypto.SignatureEncoding.DER);
    }

    protected static decode_from(alg: string, encoded: ArrayBuffer, encoding: crypto.SignatureEncoding): Signature | null {
        let wasiAlg = new crypto.WasiString(alg);
        if ((error.last = crypto.signatureImport(wasiAlg.ptr, wasiAlg.length, changetype<ptr<u8>>(encoded), encoded.byteLength, encoding, buf))) {
            return null;
        }
        return new Signature(load<crypto.WasiHandle>(buf));
    }

    static fromRaw(alg: string, encoded: ArrayBuffer): Signature | null {
        return this.decode_from(alg, encoded, crypto.SignatureEncoding.RAW);
    }

    static fromDer(alg: string, encoded: ArrayBuffer): Signature | null {
        return this.decode_from(alg, encoded, crypto.SignatureEncoding.DER);
    }
}

export class SignaturePublicKey extends PublicKey {
    signature_verify(msg: ArrayBuffer, signature: Signature): bool {
        if ((error.last = crypto.signatureVerificationStateOpen(this.handle, buf))) {
            return false;
        }
        let state = load<crypto.SignatureVerificationState>(buf);
        if ((error.last = crypto.signatureVerificationStateUpdate(state, changetype<ptr<u8>>(msg), msg.byteLength))) {
            return false;
        }
        error.last = crypto.signatureVerificationStateVerify(state, signature.handle);
        crypto.signatureVerificationStateClose(state);
        return error.last === 0;
    }

    static fromRaw(alg: string, encoded: ArrayBuffer): SignaturePublicKey | null {
        return changetype<SignaturePublicKey | null>(super._fromRaw(crypto.AlgorithmType.SIGNATURES, alg, encoded));
    }

    static fromPkcs8(alg: string, encoded: ArrayBuffer): SignaturePublicKey | null {
        return changetype<SignaturePublicKey | null>(super._fromPkcs8(crypto.AlgorithmType.SIGNATURES, alg, encoded));
    }

    static fromPem(alg: string, encoded: ArrayBuffer): SignaturePublicKey | null {
        return changetype<SignaturePublicKey | null>(super._fromPem(crypto.AlgorithmType.SIGNATURES, alg, encoded));
    }

    static fromSec(alg: string, encoded: ArrayBuffer): SignaturePublicKey | null {
        return changetype<SignaturePublicKey | null>(super._fromSec(crypto.AlgorithmType.SIGNATURES, alg, encoded));
    }

    static fromLocal(alg: string, encoded: ArrayBuffer): SignaturePublicKey | null {
        return changetype<SignaturePublicKey | null>(super._fromLocal(crypto.AlgorithmType.SIGNATURES, alg, encoded));
    }
}

export class SignatureKeyPair extends KeyPair {
    static generate(alg: string): SignatureKeyPair | null {
        return changetype<SignatureKeyPair | null>(KeyPair._generate(crypto.AlgorithmType.SIGNATURES, alg));
    }

    publicKey(): SignaturePublicKey | null {
        return changetype<SignaturePublicKey | null>(super.publicKey());
    }

    static fromRaw(alg: string, encoded: ArrayBuffer): SignatureKeyPair | null {
        return changetype<SignatureKeyPair | null>(super._fromRaw(crypto.AlgorithmType.SIGNATURES, alg, encoded));
    }

    static fromPkcs8(alg: string, encoded: ArrayBuffer): SignatureKeyPair | null {
        return changetype<SignatureKeyPair | null>(super._fromPkcs8(crypto.AlgorithmType.SIGNATURES, alg, encoded));
    }

    static fromPem(alg: string, encoded: ArrayBuffer): SignatureKeyPair | null {
        return changetype<SignatureKeyPair | null>(super._fromPem(crypto.AlgorithmType.SIGNATURES, alg, encoded));
    }

    static fromLocal(alg: string, encoded: ArrayBuffer): SignatureKeyPair | null {
        return changetype<SignatureKeyPair | null>(super._fromLocal(crypto.AlgorithmType.SIGNATURES, alg, encoded));
    }

    sign(msg: ArrayBuffer): Signature | null {
        if ((error.last = crypto.signatureStateOpen(this.handle, buf))) {
            return null;
        }
        let state = load<crypto.SignatureState>(buf);
        if ((error.last = crypto.signatureStateUpdate(state, changetype<ptr<u8>>(msg), msg.byteLength))) {
            return null;
        }
        if ((error.last = crypto.signatureStateSign(state, buf))) {
            return null;
        }
        crypto.signatureStateClose(state);
        return new Signature(load<crypto.Signature>(buf));
    }
}
