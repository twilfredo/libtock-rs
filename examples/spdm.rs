//! A simple libtock-rs SPDM example

#![no_main]
#![no_std]

use emballoc;
use libspdm::spdm;
use libtock::runtime::{set_main, stack_size};

set_main! {main}
stack_size! {0x200}

#[global_allocator]
static ALLOCATOR: emballoc::Allocator<4096> = emballoc::Allocator::new();

fn main() {
    let cntx_ptr = spdm::initialise_spdm_context();
}

// Based on https://github.com/llvm/llvm-project/blob/main/compiler-rt/lib/builtins/bswapsi2.c
#[no_mangle]
pub extern "C" fn __bswapsi2(u: u32) -> u32 {
    (((u) & 0xff000000) >> 24)
        | (((u) & 0x00ff0000) >> 8)
        | (((u) & 0x0000ff00) << 8)
        | (((u) & 0x000000ff) << 24)
}
