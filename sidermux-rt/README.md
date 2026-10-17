<div align="center">
  <picture>
    <source
      media="(prefers-color-scheme: dark)"
      srcset="https://raw.githubusercontent.com/sidermux/sidermux-assets/main/logos/logo-horizontal.svg"
    >
    <img
      alt="sidermux-rt"
      src="https://raw.githubusercontent.com/sidermux/sidermux-assets/main/logos/logo-horizontal-light.svg"
      width="420"
    >
  </picture>

  <p>
    <strong>Runtime startup for sidermux targets, selected per architecture.</strong>
  </p>

  <p>
    <a href="https://docs.rs/sidermux-rt">Documentation</a>
  </p>
</div>

> [!WARNING]
> **Beta software.** This project is pre-release and breaking changes may
> happen frequently between versions. Pin the exact version you depend on.

---

`_start` is the entry symbol handed control by the SoC's boot ROM. Its
contract is SoC-scoped, not generic: the armv7a implementation is the
ARMv7-A startup for the supported Cyclone V Boot ROM handoff.

## Startup selection

| Feature | Effect |
|---|---|
| `armv7a` | ARMv7-A startup for the supported Cyclone V Boot ROM handoff |

Binaries normally enable this through their SoC feature
(`sidermux-bootloader/cyclone-v` forwards to `sidermux-rt/armv7a`) rather
than selecting it directly.

## Entry contract

Incoming (SoC-specific, Cyclone V Boot ROM): `r0 = rom_info` address,
`r1 = rom_info` length. Outgoing (this runtime's own convention,
independent of the incoming registers): `boot_main(r0, r1)`. Today the
startup forwards the incoming registers unchanged; the two conventions are
deliberately kept distinct.

Entry-state assumptions (Supervisor mode, IRQ/FIQ masked, cache/MMU in
Boot ROM state) are Cyclone V-scoped and must be re-examined per SoC.

Part of the [sidermux-hal](https://github.com/sidermux/sidermux-hal)
workspace, with [`sidermux-hal`](https://docs.rs/sidermux-hal) and
[`sidermux-bootloader`](https://docs.rs/sidermux-bootloader).

## License

MIT or Apache 2.0. Copyright © 2026 sidermux.
