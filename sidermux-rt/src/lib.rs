//! Runtime startup for sidermux targets.
//!
//! `_start` is the entry symbol handed control by the SoC's boot ROM. Its
//! contract is SoC-scoped, not generic: the armv7a implementation is the
//! ARMv7-A startup for the supported Cyclone V Boot ROM handoff. Entry
//! mode, security state, cache state, and incoming argument registers are
//! assumptions of that handoff and must be re-examined per SoC.

#![no_std]

// ARMv7-A startup for the supported Cyclone V Boot ROM handoff.
//
// Incoming (SoC-specific, Cyclone V Boot ROM): `r0 = rom_info` address,
// `r1 = rom_info` length. Outgoing (this runtime's own convention,
// independent of the incoming registers): `boot_main(r0, r1)`. Today the
// startup forwards the incoming registers unchanged; the two conventions
// are deliberately kept distinct.
#[cfg(feature = "armv7a")]
core::arch::global_asm!(include_str!("armv7a/startup.S"), options(raw));

#[cfg(feature = "armv7a")]
unsafe extern "C" {
    /// Entry symbol; contract documented with the startup assembly above.
    pub fn _start() -> !;
}
