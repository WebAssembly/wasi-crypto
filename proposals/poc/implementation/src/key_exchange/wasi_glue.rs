use crate::types as guest_types;
use crate::WasiCryptoCtx;

impl crate::wasi_ephemeral_crypto_kx::WasiEphemeralCryptoKx for WasiCryptoCtx {
    // --- key exchange

    fn kx_dh(
        &self,
        pk_handle: guest_types::Publickey,
        sk_handle: guest_types::Secretkey,
    ) -> Result<guest_types::ArrayOutput, guest_types::CryptoErrno> {
        Ok(self.ctx.kx_dh(pk_handle.into(), sk_handle.into())?.into())
    }
}
