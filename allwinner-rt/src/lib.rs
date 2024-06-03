//! Allwinner D1 ROM runtime.
//!
//! # Usage
//!
//! Here's an sample usage of this crate:
//!
//! ```no_run
//! use allwinner_rt::{entry, Parameters, Handover};
//!
//! #[entry]
//! fn main(params: Parameters) -> Handover {
//!     /* code */
//! }
//! ```
#![feature(naked_functions, asm_const)]
#![no_std]

#[macro_use]
mod macros;

#[cfg(any(feature = "nezha", feature = "lichee"))]
mod mctl;
#[cfg(any(feature = "nezha", feature = "lichee"))]
/// Dram initializing function.
pub use mctl::init as dram_init;

pub use allwinner_rt_macros::entry;

use allwinner_hal::time::U32Ext;
use core::arch::asm;

pub mod soc {
    pub mod d1;
}

/// eGON.BT0 identifying structure.
#[repr(C)]
pub struct EgonHead {
    magic: [u8; 8],
    pub checksum: u32,
    pub length: u32,
    _head_size: u32,
    fel_script_address: u32,
    fel_uenv_length: u32,
    dt_name_offset: u32,
    dram_size: u32,
    boot_media: u32,
    string_pool: [u32; 13],
}

/// Jump over head data to executable code.
///
/// # Safety
///
/// Naked function.
///
/// NOTE: `mxstatus` is a custom T-Head register. Do not confuse with `mstatus`.
/// It allows for configuring special eXtensions. See further below for details.
#[naked]
#[link_section = ".text.entry"]
unsafe extern "C" fn start() -> ! {
    const STACK_SIZE: usize = 1024;
    #[link_section = ".bss.uninit"]
    static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
    asm!(
        // Enable T-Head ISA extension
        "li t1, 1 << 22",
        "csrs 0x7C0, t1",
        // Invalidate instruction and data cache, branch history table
        // and branch target buffer table
        "li t1, 0x30013",
        "csrs 0x7C2, t1",
        // 关中断
        "csrw mie, zero",
        "la  sp, {stack}
        li   t0, {stack_size}
        add  sp, sp, t0",
        // 清空bss
        "la  t1, sbss
        la   t2, ebss
    1:  bgeu t1, t2, 1f
        sd   zero, 0(t1)
        addi t1, t1, 8
        j    1b
    1:  ",
        // 启动！
        "call {main}
    1:  wfi
        j 1b",
        stack      =   sym STACK,
        stack_size = const STACK_SIZE,
        main       =   sym wrap_main,
        options(noreturn)
    )
}

#[rustfmt::skip]
extern "Rust" {
    // This symbol is generated by `#[rom_rt::entry]` macro
    fn main(p: Peripherals, c: Clocks) -> !;
}

fn wrap_main() {
    let p = unsafe { core::mem::transmute(()) };
    let c = Clocks {
        psi: 600_000_000.hz(),
        apb1: 24_000_000.hz(),
    };
    let _ = unsafe { main(p, c) };

    // the actual Handover is dropped to ensure ownership is returned
}

#[no_mangle]
#[link_section = ".head.egon"]
static EGON_HEAD: EgonHead = EgonHead {
    magic: *b"eGON.BT0",
    checksum: 0x5F0A6C39, // real checksum filled by blob generator
    length: 0x8000,
    _head_size: 0,
    fel_script_address: 0,
    fel_uenv_length: 0,
    dt_name_offset: 0,
    dram_size: 0,
    boot_media: 0,
    string_pool: [0; 13],
};

core::arch::global_asm! {
    ".section .text.head",
    "head_jump:",
    "j  {}",
    sym start,
}

#[cfg(any(feature = "nezha", feature = "lichee"))]
pub use {self::soc::d1::Peripherals, allwinner_hal::ccu::Clocks};

#[cfg(not(any(feature = "nezha", feature = "lichee")))]
pub struct Peripherals {}
#[cfg(not(any(feature = "nezha", feature = "lichee")))]
pub struct Clocks {}