// --- LLM-generated --- //
//@ add-minicore
//@ needs-llvm-components: nvptx
//@ compile-flags: --target=nvptx64-nvidia-cuda --crate-type=rlib
//@ ignore-backends: gcc
#![no_core]
#![feature(no_core, lang_items, link_llvm_intrinsics)]

extern crate minicore;
use minicore::*;

unsafe extern "llvm-intrinsic" {
    #[link_name = "llvm.nvvm.ldu.global.i.i32.p1"]
    #[rustc_llvm_ptr_addrspace(args(1, 0))]
    //~^ ERROR use of an internal attribute
    fn ldu(p: *const i32, align: i32) -> i32;
}
