// --- LLM-generated --- //
//@ add-minicore
//@ needs-llvm-components: nvptx
//@ compile-flags: --target=nvptx64-nvidia-cuda --crate-type=rlib
//@ ignore-backends: gcc
#![no_core]
#![feature(no_core, lang_items, link_llvm_intrinsics, rustc_attrs)]

extern crate minicore;
use minicore::*;

unsafe extern "llvm-intrinsic" {
    #[link_name = "llvm.nvvm.ldu.global.i.i32.p1"]
    #[rustc_llvm_ptr_addrspace(args(1, 0))]
    fn ok_args(p: *const i32, align: i32) -> i32;
    #[link_name = "llvm.nvvm.ldu.global.p.p3.p1"]
    #[rustc_llvm_ptr_addrspace(ret(3), args(1, 0))]
    fn ok_ret(p: *const *mut u8, align: i32) -> *mut u8;

    #[link_name = "llvm.nvvm.mapa.shared.cluster"]
    #[rustc_llvm_ptr_addrspace(args(3, 0), ret(7))]
    fn ok_int(p: u32, rank: u32) -> u32;
    #[link_name = "llvm.nvvm.ldu.global.i.i32.p1"]
    #[rustc_llvm_ptr_addrspace(args(1, 0))]
    fn ok_usize(p: usize, align: i32) -> i32;
    #[link_name = "llvm.nvvm.tcgen05.dealloc.cg1"]
    #[rustc_llvm_ptr_addrspace(args(6, 0))]
    fn ok_tmem_int(tmem_addr: u32, ncols: u32);

    #[link_name = "llvm.nvvm.mapa.shared.cluster"]
    #[rustc_llvm_ptr_addrspace(args(3, 0), ret(7))]
    //~^ ERROR address space 3 specified for 64-bit integer `u64`, but pointers in it are 32-bit
    //~| ERROR address space 7 specified for 16-bit integer `i16`, but pointers in it are 32-bit
    fn int_wrong_size(p: u64, rank: u32) -> i16;

    #[link_name = "llvm.nvvm.ldu.global.i.i32.p1"]
    #[rustc_llvm_ptr_addrspace(args(1))]
    //~^ ERROR expected 2 argument address spaces, found 1
    fn wrong_count(p: *const i32, align: i32) -> i32;

    #[link_name = "llvm.nvvm.ldu.global.i.i32.p1"]
    #[rustc_llvm_ptr_addrspace(args(1, 0))]
    //~^ ERROR address space specified for `f32`, which is not a thin raw pointer or an integer
    fn not_pointer(p: f32, align: i32) -> i32;

    #[link_name = "llvm.nvvm.ldu.global.i.i32.p1"]
    #[rustc_llvm_ptr_addrspace(args(1, 0))]
    //~^ ERROR address space specified for `*const [i32]`, which is not a thin raw pointer or an integer
    fn fat_pointer(p: *const [i32], align: i32) -> i32;

    #[link_name = "llvm.nvvm.ldu.global.i.i32.p6"]
    #[rustc_llvm_ptr_addrspace(args(6, 0))]
    //~^ ERROR pointers in address space 6 cannot be converted from generic pointers on this target
    fn tmem(p: *const i32, align: i32) -> i32;

    #[link_name = "llvm.nvvm.ldu.global.p.p3.p1"]
    #[rustc_llvm_ptr_addrspace(ret(3, 1))]
    //~^ ERROR malformed `rustc_llvm_ptr_addrspace` attribute input
    fn ret_two(p: *const *mut u8, align: i32) -> *mut u8;

    #[link_name = "llvm.nvvm.ldu.global.i.i32.p1"]
    #[rustc_llvm_ptr_addrspace(args(1, 0), args(1, 0))]
    //~^ ERROR malformed `rustc_llvm_ptr_addrspace` attribute input
    fn duplicate(p: *const i32, align: i32) -> i32;

    #[link_name = "llvm.nvvm.ldu.global.i.i32.p1"]
    #[rustc_llvm_ptr_addrspace(params(1, 0))]
    //~^ ERROR malformed `rustc_llvm_ptr_addrspace` attribute input
    fn unknown_key(p: *const i32, align: i32) -> i32;

    #[link_name = "llvm.nvvm.ldu.global.i.i32.p1"]
    #[rustc_llvm_ptr_addrspace(args(global, 0))]
    //~^ ERROR malformed `rustc_llvm_ptr_addrspace` attribute input
    fn not_integer(p: *const i32, align: i32) -> i32;

    #[link_name = "llvm.nvvm.ldu.global.i.i32.p1"]
    #[rustc_llvm_ptr_addrspace]
    //~^ ERROR malformed `rustc_llvm_ptr_addrspace` attribute input
    fn no_args(p: *const i32, align: i32) -> i32;
}

unsafe extern "C" {
    #[rustc_llvm_ptr_addrspace(args(1))]
    //~^ ERROR `#[rustc_llvm_ptr_addrspace]` can only be applied to LLVM intrinsics
    fn not_intrinsic(p: *const i32);
}

#[rustc_llvm_ptr_addrspace(args(1))]
//~^ ERROR the `rustc_llvm_ptr_addrspace` attribute cannot be used on functions
unsafe fn not_foreign(p: *const i32) {}
