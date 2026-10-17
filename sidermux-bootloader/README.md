<div align="center">
  <picture>
    <source
      media="(prefers-color-scheme: dark)"
      srcset="https://raw.githubusercontent.com/sidermux/sidermux-assets/main/logos/logo-horizontal.svg"
    >
    <img
      alt="sidermux-bootloader"
      src="https://raw.githubusercontent.com/sidermux/sidermux-assets/main/logos/logo-horizontal-light.svg"
      width="420"
    >
  </picture>

  <p>
    <strong>Bootable image flow for sidermux targets; SoC boot profiles live here.</strong>
  </p>

  <p>
    <a href="https://docs.rs/sidermux-bootloader">Documentation</a>
  </p>
</div>

> [!WARNING]
> **Beta software.** This project is pre-release and breaking changes may
> happen frequently between versions. Pin the exact version you depend on.

---

Layering: architecture startup (`sidermux-rt`), SoC handoff adapter
(`boot_main`), common boot logic. `boot_main`'s signature mirrors the
Cyclone V Boot ROM handoff; it is the adapter, not a portable ABI.

## Quick start

```sh
cargo build -p sidermux-bootloader --target armv7a-none-eabi --features cyclone-v
```

## SoC selection

Exactly one SoC feature is required; a build without one fails with an
explicit error rather than linking a broken image.

| Feature | Effect |
|---|---|
| `cyclone-v` | Builds the Cyclone V boot image (forwards `sidermux-hal/cyclone-v` and `sidermux-rt/armv7a`) |

## Boot profiles

```text
ld/linkall.x              # application layout, generic over staged names
ld/profiles/cyclone-v/    # boot contract: vectors, header slot, entry
                          # slot, size limits, trailer reservation
src/profiles/cyclone-v/   # matching entry assembly
```

Profiles stage under uniform names (`sidermux-boot.x`,
`sidermux-boot-reserve.x`); the name is a build interface, the contents are
per-SoC. The Cyclone V profile loads at `0xFFFF0000` with vectors at the
image base, a Boot ROM header slot at `+0x40`, the handoff entry at
`+0x4C`, 16-byte packaging, and a 60 KiB arena.

Part of the [sidermux-hal](https://github.com/sidermux/sidermux-hal)
workspace, with [`sidermux-hal`](https://docs.rs/sidermux-hal) and
[`sidermux-rt`](https://docs.rs/sidermux-rt).

## License

MIT or Apache 2.0. Copyright © 2026 sidermux.
