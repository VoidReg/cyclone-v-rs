//! ARMv7-A CPU operations (Cortex-A9 class).
//!
//! Reusable CPU building blocks only. The complete reset-entry sequence is
//! not generic: entry mode, security state, cache state, and incoming
//! argument registers are SoC-specific. The supported startup contract is
//! documented with `_start` in `sidermux-rt`: ARMv7-A startup for the
//! supported Cyclone V Boot ROM handoff.
