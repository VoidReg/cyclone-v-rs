use std::{env, fs, path::PathBuf};

fn main() {
    let manifest = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let scripts = [
        ("ld/memory.x", "cyclone-v-memory.x"),
        ("ld/sections/text.x", "cyclone-v-text.x"),
        ("ld/sections/rodata.x", "cyclone-v-rodata.x"),
        ("ld/sections/data.x", "cyclone-v-data.x"),
        ("ld/sections/bss.x", "cyclone-v-bss.x"),
    ];

    for (source, destination) in scripts {
        let source = manifest.join(source);

        println!("cargo::rerun-if-changed={}", source.display());
        fs::copy(&source, out.join(destination))
            .expect("failed to stage HAL linker script");
    }

    println!("cargo::rustc-link-search={}", out.display());
}