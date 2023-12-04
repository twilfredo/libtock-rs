//! A simple libtock-rs SPDM example

#![no_main]
#![no_std]

extern crate alloc;

use core::fmt::Write;
use cortex_m::*;
use embedded_alloc::Heap;
use libspdm::spdm;
use libtock::console::Console;
use libtock::runtime::{set_main, stack_size};

set_main! {main}
stack_size! {0x400}

#[global_allocator]
static HEAP: Heap = Heap::empty();

// Setup the heap and the global allocator.
unsafe fn setup_heap() {
    use core::mem::MaybeUninit;
    const HEAP_SIZE: usize = 1024 * 118;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
}

fn main() {
    unsafe {
        setup_heap();
    }

    writeln!(Console::writer(), "spdm-sample app start\r",).unwrap();
    let cntx_ptr = spdm::initialise_spdm_context();
    writeln!(Console::writer(), "spdm-sample app finish\r",).unwrap();
}

// Based on https://github.com/llvm/llvm-project/blob/main/compiler-rt/lib/builtins/bswapsi2.c
#[no_mangle]
pub extern "C" fn __bswapsi2(u: u32) -> u32 {
    (((u) & 0xff000000) >> 24)
        | (((u) & 0x00ff0000) >> 8)
        | (((u) & 0x0000ff00) << 8)
        | (((u) & 0x000000ff) << 24)
}
