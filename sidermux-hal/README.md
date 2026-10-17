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
    <strong>Unified bare-metal HAL surface for SoC FPGAs: architecture, SoC, and board support in one crate.</strong>
  </p>

  <p>
    <a href="https://docs.rs/sidermux-hal">Documentation</a>
  </p>
</div>

> [!WARNING]
> **Beta software.** This project is pre-release and breaking changes may
> happen frequently between versions. Pin the exact version you depend on.

---

Cyclone V is the first supported SoC. SoC, architecture, and board support
are feature-gated modules behind one unified surface; no additional
hardware families are claimed as supported.

| Feature | Effect |
|---|---|
| `cyclone-v` | Cyclone V SoC support (implies `arch-armv7a`) |
| `arch-armv7a` | ARMv7-A CPU support, SoC-independent |

The crate builds target-independent with no SoC selected.

## Module layout

```text
src/
├── arch/            # CPU-level operations, shared by SoCs on the same core
├── boards/          # installed DDR geometry and board hardware settings
└── soc/             # physical memory maps and Boot ROM contracts
    └── cyclone_v/   # OCRAM window, handoff contract, physical memory.x
```

## Responsibility split

| Concern | Owner |
|---|---|
| Physical addresses and capacities | `soc` |
| CPU-level operations | `arch` |
| Installed DDR geometry, board hardware | `boards` |
| Boot arena and reserved ranges | boot profile (`sidermux-bootloader`) |
| Application section destinations | application linker script |

## Usage

```toml
[dependencies]
sidermux-hal = { version = "0.1", features = ["cyclone-v"] }
```

Part of the [sidermux-hal](https://github.com/sidermux/sidermux-hal)
workspace, with [`sidermux-rt`](https://docs.rs/sidermux-rt) and
[`sidermux-bootloader`](https://docs.rs/sidermux-bootloader).

## License

MIT or Apache 2.0. Copyright © 2026 sidermux.
