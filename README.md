<div align="center">
  <picture>
    <source
      media="(prefers-color-scheme: dark)"
      srcset="https://raw.githubusercontent.com/sidermux/sidermux-assets/main/logos/logo-horizontal.svg"
    >
    <img
      alt="sidermux-hal"
      src="https://raw.githubusercontent.com/sidermux/sidermux-assets/main/logos/logo-horizontal-light.svg"
      width="420"
    >
  </picture>

  <p>
    <strong>Bare-metal HAL for SoC FPGAs: one interface across architectures, SoCs, and boards.</strong>
  </p>

  <p>
    <a href="https://docs.rs/sidermux-hal">Documentation</a>
  </p>
</div>

> [!WARNING]
> **Beta software.** This project is pre-release and breaking changes may
> happen frequently between versions. Pin the exact version you depend on.

---

Cyclone V is the first supported SoC (`armv7a-none-eabi`, dual Cortex-A9).
SoC, architecture, and board support are feature-gated modules behind one
unified surface; no additional hardware families are claimed as supported.

| Feature | Effect |
|---|---|
| `sidermux-hal/cyclone-v` | Cyclone V SoC support (implies `arch-armv7a`) |
| `sidermux-hal/arch-armv7a` | ARMv7-A CPU support, SoC-independent |
| `sidermux-bootloader/cyclone-v` | Builds the Cyclone V boot image (forwards both) |

`sidermux-hal` builds target-independent with no SoC selected;
`sidermux-bootloader` requires exactly one SoC feature.

## Quick start

Build the bootable image:

```sh
cargo build -p sidermux-bootloader --target armv7a-none-eabi --features cyclone-v
```

In VS Code, the default build task (`Ctrl+Shift+B`) prompts for the SoC
target; rust-analyzer is preconfigured to match.

## Workspace

```text
sidermux-hal/
├── sidermux-hal/          # unified HAL: SoC, architecture, and board modules
├── sidermux-rt/           # runtime startup, selected per architecture
└── sidermux-bootloader/   # bootable image; SoC boot profiles live here
```

## Documentation

The full documentation, covering target selection, boot flow layering, and
extension points, lives in the Rust API docs:

- [`sidermux-hal`](https://docs.rs/sidermux-hal): unified HAL surface and
  responsibility split
- [`sidermux-rt`](https://docs.rs/sidermux-rt): runtime startup contracts
- [`sidermux-bootloader`](https://docs.rs/sidermux-bootloader): bootable
  image and boot profiles

Or read them locally:

```sh
cargo doc --workspace --no-deps --features sidermux-bootloader/cyclone-v --open
```

## License

MIT or Apache 2.0. Copyright © 2026 sidermux.
