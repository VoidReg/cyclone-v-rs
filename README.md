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

## LICENSE

MIT or Apache 2.0 - Copyright © 2026 sidermux.
