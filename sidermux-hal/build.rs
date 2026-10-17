use std::{env, fs, path::PathBuf};

fn main() {
    let manifest = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // Shared section fragments. Their contents are target-agnostic; where
    // sections land is decided by the selected target memory script and the
    // boot profile of the final image.
    let shared = [
        ("ld/sections/text.x", "sidermux-text.x"),
        ("ld/sections/rodata.x", "sidermux-rodata.x"),
        ("ld/sections/data.x", "sidermux-data.x"),
        ("ld/sections/bss.x", "sidermux-bss.x"),
        ("ld/sections/unwind.x", "sidermux-unwind.x"),
        ("ld/sections/stack.x", "sidermux-stack.x"),
    ];

    for (source, destination) in shared {
        stage(&manifest, &out, source, destination);
    }

    // The selected SoC supplies the physical memory script under a generic
    // staged name. SoC features imply their architecture via cargo feature
    // dependencies; the exactly-one rule for linkable images is enforced by
    // target-dependent binaries. This crate stays buildable on the host
    // without any SoC selected.
    if env::var_os("CARGO_FEATURE_CYCLONE_V").is_some() {
        stage(
            &manifest,
            &out,
            "src/soc/cyclone_v/ld/memory.x",
            "sidermux-target-memory.x",
        );
    }

    println!("cargo::rustc-link-search={}", out.display());
}

fn stage(manifest: &PathBuf, out: &PathBuf, source: &str, destination: &str) {
    let source = manifest.join(source);
    let destination = out.join(destination);

    println!("cargo::rerun-if-changed={}", source.display());

    fs::copy(&source, &destination).expect("failed to stage HAL linker script");
}
