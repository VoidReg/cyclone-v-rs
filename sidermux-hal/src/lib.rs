//! Unified bare-metal HAL for SoC FPGAs: one interface across supported
//! architectures, SoCs, and boards. New hardware is added as a module behind
//! a feature flag, not as a new project.
//!
//! ## Responsibility split
//!
//! | Concern | Owner |
//! |---|---|
//! | Physical addresses and capacities | [`soc`] support |
//! | CPU-level operations | [`arch`] support |
//! | Installed DDR geometry, board hardware | [`boards`] support |
//! | Boot arena and reserved ranges | boot profile (`sidermux-bootloader`) |
//! | Application section destinations | application linker script |
//!
//! ## Target selection
//!
//! SoC features imply their architecture (for example `cyclone-v` enables
//! `arch-armv7a`). This crate builds target-independent with no SoC
//! selected; all SoC and architecture code is feature-gated. Binaries that
//! link boot images must select exactly one SoC and fail loudly otherwise.
//!
//! ## Boot flow layering
//!
//! ```text
//! architecture startup (sidermux-rt)
//!   -> SoC handoff adapter (SoC module, e.g. soc::cyclone_v)
//!   -> common boot logic
//! ```
//!
//! The current Cyclone V handoff signature is the adapter's; it is not a
//! portable ABI. Entry-state assumptions (mode, security state, cache state,
//! argument registers) are SoC-scoped.

#![no_std]

pub mod arch;
pub mod boards;
pub mod soc;
