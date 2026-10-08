// --- LLM-generated --- //
// Generic pointers can't be autocast to tensor memory pointers.

//@ add-minicore
//@ build-fail
//@ needs-llvm-components: nvptx
//@ compile-flags: --target=nvptx64-nvidia-cuda --crate-type=rlib -Ctarget-cpu=sm_100a
//@ ignore-backends: gcc
#![no_core]
#![feature(no_core, lang_items, link_llvm_intrinsics)]

extern crate minicore;
use minicore::*;

unsafe extern "llvm-intrinsic" {
    #[link_name = "llvm.nvvm.tcgen05.dealloc.cg1"]
    fn tcgen05_dealloc(tmem_addr: *mut u8, ncols: u32);
    //~^ ERROR intrinsic signature mismatch
}

#[no_mangle]
pub unsafe fn dealloc(p: *mut u8) {
    tcgen05_dealloc(p, 32)
}
