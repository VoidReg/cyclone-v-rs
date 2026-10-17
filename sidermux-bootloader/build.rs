use std::{env, fs, path::PathBuf};

fn main() {
    let manifest = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // Application layout script: target-agnostic over staged names.
    stage(&manifest, &out, "ld/linkall.x", "linkall.x");

    // Boot profile: the SoC-specific boot image contract (vectors, header
    // slot, entry slot, size limits, trailer reservation). Uniform staged
    // names; contents are per-SoC.
    if env::var_os("CARGO_FEATURE_CYCLONE_V").is_some() {
        stage(
            &manifest,
            &out,
            "ld/profiles/cyclone-v/boot.x",
            "sidermux-boot.x",
        );
        stage(
            &manifest,
            &out,
            "ld/profiles/cyclone-v/boot-pack-reserve.x",
            "sidermux-boot-reserve.x",
        );
    }

    println!("cargo::rustc-link-search={}", out.display());
    println!("cargo::rustc-link-arg=-Tlinkall.x");
}

fn stage(manifest: &PathBuf, out: &PathBuf, source: &str, destination: &str) {
    let source = manifest.join(source);
    let destination = out.join(destination);

    println!("cargo::rerun-if-changed={}", source.display());

    fs::copy(&source, &destination).unwrap_or_else(|error| {
        panic!(
            "failed to copy {} to {}: {error}",
            source.display(),
            destination.display(),
        );
    });
}
