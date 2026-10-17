use std::{env, fs, path::PathBuf};

fn main() {
    let manifest = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let scripts = [
        ("ld/linkall.x", "linkall.x"),
        ("ld/sections/boot.x", "boot.x"),
        ("ld/sections/boot-pack-reserve.x", "boot-pack-reserve.x"),
    ];

    for (source, destination) in scripts {
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

    println!("cargo::rustc-link-search={}", out.display());
    println!("cargo::rustc-link-arg=-Tlinkall.x");
}