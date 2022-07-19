import * as crypto from "./wasi_crypto";

// @ts-ignore: decorator
@lazy const mem64: u64[] = [0];
// @ts-ignore: cast
export let buf: usize = changetype<ArrayBufferView>(mem64).dataStart;

export type ptr<T> = crypto.WasiPtr<T>;

export namespace error {
    // @ts-ignore: decorator
    export let last: crypto.CryptoErrno = 0;

    function reset(): void {
        last = 0;
    }
}

export function fromWasiArray(arrayOutput: crypto.ArrayOutput): ArrayBuffer | null {
    if ((error.last = crypto.arrayOutputLen(arrayOutput, buf))) {
        return null;
    }
    let out = new ArrayBuffer(load<usize>(buf) as i32);
    crypto.arrayOutputPull(arrayOutput, changetype<ptr<u8>>(out), out.byteLength, buf);
    return out;
}

