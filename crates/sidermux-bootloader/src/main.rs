#![no_std]
#![no_main]

use core::{arch::global_asm, panic::PanicInfo};

global_asm!(
    include_str!("boot-entry.S"),
    runtime_start = sym sidermux_rt::_start,
);

#[unsafe(no_mangle)]
pub extern "C" fn boot_main(
    _rom_info: usize,
    _rom_info_len: usize,
) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}