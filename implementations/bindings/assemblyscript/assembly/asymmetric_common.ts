import * as crypto from "./wasi_crypto";
import {error, buf, ptr, fromWasiArray} from "./common";

export class PublicKey {
    handle: crypto.Publickey;

    constructor(handle: crypto.Publickey) {
        this.handle = handle;
    }

    protected encode_as(encoding: crypto.PublickeyEncoding): ArrayBuffer | null {
        if ((error.last = crypto.publickeyExport(this.handle, encoding, buf))) {
            return null;
        }
        return fromWasiArray(load<crypto.SymmetricTag>(buf));
    }

    raw(): ArrayBuffer | null {
        return this.encode_as(crypto.PublickeyEncoding.RAW);
    }

    pkcs8(): ArrayBuffer | null {
        return this.encode_as(crypto.PublickeyEncoding.PKCS_8);
    }

    pem(): ArrayBuffer | null {
        return this.encode_as(crypto.PublickeyEncoding.PEM);
    }

    sec(): ArrayBuffer | null {
        return this.encode_as(crypto.PublickeyEncoding.SEC);
    }

    local(): ArrayBuffer | null {
        return this.encode_as(crypto.PublickeyEncoding.LOCAL);
    }

    private static decode_from(algType: crypto.AlgorithmType, alg: string, encoded: ArrayBuffer, encoding: crypto.PublickeyEncoding): PublicKey | null {
        let wasiAlg = new crypto.WasiString(alg);
        if ((error.last = crypto.publickeyImport(algType, wasiAlg.ptr, wasiAlg.length, changetype<ptr<u8>>(encoded), encoded.byteLength, encoding, buf))) {
            return null;
        }
        return new PublicKey(load<crypto.WasiHandle>(buf));
    }

    protected static _fromRaw(algType: crypto.AlgorithmType, alg: string, encoded: ArrayBuffer): PublicKey | null {
        return this.decode_from(algType, alg, encoded, crypto.PublickeyEncoding.RAW);
    }

    protected static _fromPkcs8(algType: crypto.AlgorithmType, alg: string, encoded: ArrayBuffer): PublicKey | null {
        return this.decode_from(algType, alg, encoded, crypto.PublickeyEncoding.PKCS_8);
    }

    protected static _fromPem(algType: crypto.AlgorithmType, alg: string, encoded: ArrayBuffer): PublicKey | null {
        return this.decode_from(algType, alg, encoded, crypto.PublickeyEncoding.PEM);
    }

    protected static _fromSec(algType: crypto.AlgorithmType, alg: string, encoded: ArrayBuffer): PublicKey | null {
        return this.decode_from(algType, alg, encoded, crypto.PublickeyEncoding.SEC);
    }

    protected static _fromLocal(algType: crypto.AlgorithmType, alg: string, encoded: ArrayBuffer): PublicKey | null {
        return this.decode_from(algType, alg, encoded, crypto.PublickeyEncoding.LOCAL);
    }
}

export class KeyPair {
    handle: crypto.Keypair;
    alg: string;

    constructor(handle: crypto.Keypair, alg: string) {
        this.handle = handle;
        this.alg = alg;
    }

    protected static _generate(algType: crypto.AlgorithmType, alg: string): KeyPair | null {
        let wasiAlg = new crypto.WasiString(alg);
        if ((error.last = crypto.keypairGenerate(algType, wasiAlg.ptr, wasiAlg.length, crypto.OptOptions.none(), buf))) {
            return null;
        }
        return new KeyPair(load<crypto.Keypair>(buf), alg);
    }

    publicKey(): PublicKey | null {
        if ((error.last = crypto.keypairPublickey(this.handle, buf))) {
            return null;
        }
        return new PublicKey(load<crypto.Publickey>(buf));
    }

    protected encode_as(encoding: crypto.KeypairEncoding): ArrayBuffer | null {
        if ((error.last = crypto.keypairExport(this.handle, encoding, buf))) {
            return null;
        }
        return fromWasiArray(load<crypto.SymmetricTag>(buf));
    }

    raw(): ArrayBuffer | null {
        return this.encode_as(crypto.KeypairEncoding.RAW);
    }

    pkcs8(): ArrayBuffer | null {
        return this.encode_as(crypto.KeypairEncoding.PKCS_8);
    }

    pem(): ArrayBuffer | null {
        return this.encode_as(crypto.KeypairEncoding.PEM);
    }

    local(): ArrayBuffer | null {
        return this.encode_as(crypto.KeypairEncoding.LOCAL);
    }

    private static decode_from(algType: crypto.AlgorithmType, alg: string, encoded: ArrayBuffer, encoding: crypto.KeypairEncoding): KeyPair | null {
        let wasiAlg = new crypto.WasiString(alg);
        if ((error.last = crypto.keypairImport(algType, wasiAlg.ptr, wasiAlg.length, changetype<ptr<u8>>(encoded), encoded.byteLength, encoding, buf))) {
            return null;
        }
        return new KeyPair(load<crypto.WasiHandle>(buf), alg);
    }

    protected static _fromRaw(algType: crypto.AlgorithmType, alg: string, encoded: ArrayBuffer): KeyPair | null {
        return this.decode_from(algType, alg, encoded, crypto.KeypairEncoding.RAW);
    }

    protected static _fromPkcs8(algType: crypto.AlgorithmType, alg: string, encoded: ArrayBuffer): KeyPair | null {
        return this.decode_from(algType, alg, encoded, crypto.KeypairEncoding.PKCS_8);
    }

    protected static _fromPem(algType: crypto.AlgorithmType, alg: string, encoded: ArrayBuffer): KeyPair | null {
        return this.decode_from(algType, alg, encoded, crypto.KeypairEncoding.PEM);
    }

    protected static _fromLocal(algType: crypto.AlgorithmType, alg: string, encoded: ArrayBuffer): KeyPair | null {
        return this.decode_from(algType, alg, encoded, crypto.KeypairEncoding.LOCAL);
    }
}
