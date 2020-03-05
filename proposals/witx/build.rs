use std::{fs::File, io::Write, path::PathBuf};
use witx::Documentation;

fn main() {
    const WASI_CRYPTO_WITX: &str =
        "wasi_ephemeral_crypto.witx";
    let witx_path: PathBuf = WASI_CRYPTO_WITX.into();
    let doc = witx::load(&[&witx_path]).unwrap();

    const DOCS_MD: &str = "docs.md";
    let docs_path: PathBuf = DOCS_MD.into();

    let mut file = File::create(&docs_path)
        .expect("create output file");
    file.write_all(doc.to_md().as_bytes())
        .expect("write output file");

    println!("cargo:rerun-if-changed={}", witx_path.display());
    println!(
        "cargo:rerun-if-changed={}",
        witx_path.with_file_name("typenames.witx").display(),
    );
}
