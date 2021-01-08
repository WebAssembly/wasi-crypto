import * as crypto from "./wasi_crypto";
import { error, buf, ptr, fromWasiArray } from "./common";

export class PublicKey {
    handle: crypto.publickey

    constructor(handle: crypto.publickey) {
        this.handle = handle;
    }

    protected encode_as(encoding: crypto.publickey_encoding): ArrayBuffer | null {
        if ((error.last = crypto.publickey_export(this.handle, encoding, buf))) {
            return null;
        }
        return fromWasiArray(load<crypto.symmetric_tag>(buf));
    }

    raw(): ArrayBuffer | null {
        return this.encode_as(crypto.publickey_encoding.RAW);
    }

    pkcs8(): ArrayBuffer | null {
        return this.encode_as(crypto.publickey_encoding.PKCS8);
    }

    pem(): ArrayBuffer | null {
        return this.encode_as(crypto.publickey_encoding.PEM);
    }

    sec(): ArrayBuffer | null {
        return this.encode_as(crypto.publickey_encoding.SEC);
    }

    compressedSec(): ArrayBuffer | null {
        return this.encode_as(crypto.publickey_encoding.COMPRESSED_SEC);
    }

    local(): ArrayBuffer | null {
        return this.encode_as(crypto.publickey_encoding.LOCAL);
    }

    private static decode_from(algType: crypto.algorithm_type, alg: string, encoded: ArrayBuffer, encoding: crypto.publickey_encoding): PublicKey | null {
        let wasiAlg = new crypto.WasiString(alg);
        if ((error.last = crypto.publickey_import(algType, wasiAlg.ptr, wasiAlg.length, changetype<ptr<u8>>(encoded), encoded.byteLength, encoding, buf))) {
            return null;
        }
        return new PublicKey(load<crypto.handle>(buf));
    }

    protected static _fromRaw(algType: crypto.algorithm_type, alg: string, encoded: ArrayBuffer): PublicKey | null {
        return this.decode_from(algType, alg, encoded, crypto.publickey_encoding.RAW);
    }

    protected static _fromPkcs8(algType: crypto.algorithm_type, alg: string, encoded: ArrayBuffer): PublicKey | null {
        return this.decode_from(algType, alg, encoded, crypto.publickey_encoding.PKCS8);
    }

    protected static _fromPem(algType: crypto.algorithm_type, alg: string, encoded: ArrayBuffer): PublicKey | null {
        return this.decode_from(algType, alg, encoded, crypto.publickey_encoding.PEM);
    }

    protected static _fromSec(algType: crypto.algorithm_type, alg: string, encoded: ArrayBuffer): PublicKey | null {
        return this.decode_from(algType, alg, encoded, crypto.publickey_encoding.SEC);
    }

    protected static _fromCompressedSec(algType: crypto.algorithm_type, alg: string, encoded: ArrayBuffer): PublicKey | null {
        return this.decode_from(algType, alg, encoded, crypto.publickey_encoding.COMPRESSED_SEC);
    }

    protected static _fromLocal(algType: crypto.algorithm_type, alg: string, encoded: ArrayBuffer): PublicKey | null {
        return this.decode_from(algType, alg, encoded, crypto.publickey_encoding.LOCAL);
    }
}

export class KeyPair {
    handle: crypto.keypair;
    alg: string;

    constructor(handle: crypto.keypair, alg: string) {
        this.handle = handle;
        this.alg = alg;
    }

    protected static _generate(algType: crypto.algorithm_type, alg: string): KeyPair | null {
        let wasiAlg = new crypto.WasiString(alg);
        if ((error.last = crypto.keypair_generate(algType, wasiAlg.ptr, wasiAlg.length, crypto.opt_options.none(), buf))) {
            return null;
        }
        return new KeyPair(load<crypto.keypair>(buf), alg);
    }

    publicKey(): PublicKey | null {
        if ((error.last = crypto.keypair_publickey(this.handle, buf))) {
            return null;
        }
        return new PublicKey(load<crypto.publickey>(buf));
    }

    protected encode_as(encoding: crypto.keypair_encoding): ArrayBuffer | null {
        if ((error.last = crypto.keypair_export(this.handle, encoding, buf))) {
            return null;
        }
        return fromWasiArray(load<crypto.symmetric_tag>(buf));
    }

    raw(): ArrayBuffer | null {
        return this.encode_as(crypto.keypair_encoding.RAW);
    }

    pkcs8(): ArrayBuffer | null {
        return this.encode_as(crypto.keypair_encoding.PKCS8);
    }

    pem(): ArrayBuffer | null {
        return this.encode_as(crypto.keypair_encoding.PEM);
    }

    local(): ArrayBuffer | null {
        return this.encode_as(crypto.keypair_encoding.LOCAL);
    }

    private static decode_from(algType: crypto.algorithm_type, alg: string, encoded: ArrayBuffer, encoding: crypto.keypair_encoding): KeyPair | null {
        let wasiAlg = new crypto.WasiString(alg);
        if ((error.last = crypto.keypair_import(algType, wasiAlg.ptr, wasiAlg.length, changetype<ptr<u8>>(encoded), encoded.byteLength, encoding, buf))) {
            return null;
        }
        return new KeyPair(load<crypto.handle>(buf), alg);
    }

    protected static _fromRaw(algType: crypto.algorithm_type, alg: string, encoded: ArrayBuffer): KeyPair | null {
        return this.decode_from(algType, alg, encoded, crypto.keypair_encoding.RAW);
    }

    protected static _fromPkcs8(algType: crypto.algorithm_type, alg: string, encoded: ArrayBuffer): KeyPair | null {
        return this.decode_from(algType, alg, encoded, crypto.keypair_encoding.PKCS8);
    }

    protected static _fromPem(algType: crypto.algorithm_type, alg: string, encoded: ArrayBuffer): KeyPair | null {
        return this.decode_from(algType, alg, encoded, crypto.keypair_encoding.PEM);
    }

    protected static _fromLocal(algType: crypto.algorithm_type, alg: string, encoded: ArrayBuffer): KeyPair | null {
        return this.decode_from(algType, alg, encoded, crypto.keypair_encoding.LOCAL);
    }
}
