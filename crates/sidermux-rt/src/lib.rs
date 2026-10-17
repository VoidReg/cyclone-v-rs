#![no_std]

use core::arch::global_asm;

global_asm!(include_str!("startup.S"), options(raw));

unsafe extern "C" {
    pub fn _start() -> !;
}