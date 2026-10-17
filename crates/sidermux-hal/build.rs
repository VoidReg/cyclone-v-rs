use std::{env, fs, path::PathBuf};

fn main() {
    let manifest = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let scripts = [
        ("ld/memory.x", "sidermux-target-memory.x"),
        ("ld/sections/text.x", "sidermux-text.x"),
        ("ld/sections/rodata.x", "sidermux-rodata.x"),
        ("ld/sections/data.x", "sidermux-data.x"),
        ("ld/sections/bss.x", "sidermux-bss.x"),
        ("ld/sections/unwind.x", "sidermux-unwind.x"),
        ("ld/sections/stack.x", "sidermux-stack.x"),
    ];

    for (source, destination) in scripts {
        let source = manifest.join(source);

        println!("cargo::rerun-if-changed={}", source.display());
        fs::copy(&source, out.join(destination))
            .expect("failed to stage HAL linker script");
    }

    println!("cargo::rustc-link-search={}", out.display());
}