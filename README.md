# sidermux-hal

Bare-metal HAL for SoC FPGAs — one interface across supported architectures,
SoCs, and boards. Adding a board or architecture goes through a unified
surface instead of a new project.

## Crates

| Crate | Role |
|---|---|
| `sidermux-hal` | Unified HAL facade: SoC, architecture, and board modules |
| `sidermux-rt` | Runtime startup, selected per architecture |
| `sidermux-bootloader` | Bootable image flow; SoC boot profiles live here |

## Targets

- **Cyclone V** (Intel/Altera SoC FPGA, dual Cortex-A9, `armv7a-none-eabi`)

## Target selection

SoC features live on `sidermux-hal` and imply their architecture. The
bootloader forwards its own SoC feature to both the HAL and the runtime.

| Feature | Effect |
|---|---|
| `sidermux-hal/cyclone-v` | Cyclone V SoC support (implies `arch-armv7a`) |
| `sidermux-hal/arch-armv7a` | ARMv7-A CPU support, SoC-independent |
| `sidermux-bootloader/cyclone-v` | Builds the Cyclone V boot image (forwards both) |

`sidermux-hal` builds target-independent with no SoC selected;
`sidermux-bootloader` requires exactly one SoC feature.

## LICENSE

MIT or Apache 2.0 - Copyright © 2026 sidermux.
