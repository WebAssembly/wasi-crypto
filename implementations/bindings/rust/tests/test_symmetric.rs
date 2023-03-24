use wycheproof::{self, TestResult};
use wasi_crypto_guest::prelude::WasiCryptoError;
use wasi_crypto_guest::symmetric::{
    Aead, AeadKey,
    Auth, AuthKey,
    Hkdf, HkdfKey,
};

fn do_aead_test_vector(alg: &'static str, test: &wycheproof::aead::Test)
    -> Result<(), WasiCryptoError> {

    let key = AeadKey::from_raw(alg, &test.key)?;
    let mut state = Aead::new(
        &key, Some(&test.nonce), Some(&test.aad))?;

    let ciphertext = state.encrypt(&test.pt)?;

    let ctlen = test.ct.len();
    assert_eq!(&ciphertext[..ctlen], test.ct.as_slice());

    if test.result == TestResult::Valid {
        assert_eq!(&ciphertext[ctlen..], test.tag.as_slice());
    } else {
        assert_ne!(&ciphertext[ctlen..], test.tag.as_slice());
    }

    Ok(())
}

#[test]
fn test_wasi_aesgcm() -> Result<(), WasiCryptoError> {
    let test_set = wycheproof::aead::TestSet::load(
        wycheproof::aead::TestName::AesGcm).unwrap();

    // wasi-crypto has some constraints around the supported key sizes and
    // nonce sizes so we only want to use the test vectors for the combinations
    // that are supported.
    let tgiter = test_set.test_groups.iter()
        .filter(|x| x.key_size != 192)
        .filter(|x| x.nonce_size == 96);

    for test_group in tgiter {

        let alg = match test_group.key_size {
            128 => Some("AES-128-GCM"),
            256 => Some("AES-256-GCM"),
            _ => None,
        }.ok_or(WasiCryptoError::UnsupportedAlgorithm)?;

        let test_iter = test_group.tests.iter();
        for test in test_iter {
            do_aead_test_vector(alg, test)?;
        }
    }

    Ok(())
}

#[test]
fn test_wasi_chacha20poly1305() -> Result<(), WasiCryptoError> {
    let test_set = wycheproof::aead::TestSet::load(
        wycheproof::aead::TestName::ChaCha20Poly1305).unwrap();

    let tgiter = test_set.test_groups.iter()
        .filter(|x| x.nonce_size == 96);

    for test_group in tgiter {
        let alg = "CHACHA20-POLY1305";

        let test_iter = test_group.tests.iter();
        for test in test_iter {
            do_aead_test_vector(alg, test)?;
        }
    }

    Ok(())
}

fn do_hmacsha_test_vector(alg: &'static str, test: &wycheproof::mac::Test)
    -> Result<(), WasiCryptoError> {

    let key = AuthKey::from_raw(alg, &test.key)?;
    let tag = Auth::auth(&test.msg, &key)?;
    if test.result == wycheproof::TestResult::Valid {
        assert_eq!(&tag[..test.tag.len()], test.tag.as_slice());
    } else {
        assert_ne!(&tag[..test.tag.len()], test.tag.as_slice());
    }

    Ok(())
}

fn do_hmacsha_testset(alg: &'static str, test_set: &wycheproof::mac::TestSet)
    -> Result<(), WasiCryptoError> {

    let tgiter = test_set.test_groups.iter();

    for test_group in tgiter {
        let test_iter = test_group.tests.iter();
        for test in test_iter {
            assert!(do_hmacsha_test_vector(alg, test).is_ok(),
                "Test ID {} Error", test.tc_id);
        }
    }

    Ok(())
}

#[test]
fn test_wasi_hmacsha256() -> Result<(), WasiCryptoError> {
    let alg = "HMAC/SHA-256";
    let test_set = wycheproof::mac::TestSet::load(
        wycheproof::mac::TestName::HmacSha256).unwrap();
    do_hmacsha_testset(alg, &test_set)?;

    Ok(())
}

#[test]
fn test_wasi_hmacsha512() -> Result<(), WasiCryptoError> {
    let alg = "HMAC/SHA-512";
    let test_set = wycheproof::mac::TestSet::load(
        wycheproof::mac::TestName::HmacSha512).unwrap();
    do_hmacsha_testset(alg, &test_set)?;

    Ok(())
}

fn do_hkdf_test_vector(
    ext_alg: &'static str,
    exp_alg: &'static str,
    test: &wycheproof::hkdf::Test)
    -> Result<(), WasiCryptoError> {

    let key = HkdfKey::from_raw(ext_alg, &test.ikm)?;
    let prk = Hkdf::new(exp_alg, &key, Some(&test.salt))?;
    let okm = prk.expand(&test.info, test.okm.len())?;
    assert_eq!(&okm, test.okm.as_slice());

    Ok(())
}

fn do_hkdf_testset(
    ext_alg: &'static str,
    exp_alg: &'static str,
    test_set: &wycheproof::hkdf::TestSet)
    -> Result<(), WasiCryptoError> {

    let tgiter = test_set.test_groups.iter();

    for test_group in tgiter {
        let test_iter = test_group.tests.iter();
        for test in test_iter {
            assert!(do_hkdf_test_vector(ext_alg, exp_alg, test).is_ok(),
                "Test ID {} Error", test.tc_id);
        }
    }

    Ok(())
}

#[test]
fn test_wasi_hkdf() -> Result<(), WasiCryptoError> {
    let ext_alg = "HKDF-EXTRACT/SHA-256";
    let exp_alg = "HKDF-EXPAND/SHA-256";
    let test_set = wycheproof::hkdf::TestSet::load(
        wycheproof::hkdf::TestName::HkdfSha256).unwrap();
        do_hkdf_testset(ext_alg, exp_alg, &test_set)?;

    let ext_alg = "HKDF-EXTRACT/SHA-512";
    let exp_alg = "HKDF-EXPAND/SHA-512";
    let test_set = wycheproof::hkdf::TestSet::load(
        wycheproof::hkdf::TestName::HkdfSha512).unwrap();
        do_hkdf_testset(ext_alg, exp_alg, &test_set)?;

    Ok(())
}
