// --- LLM-generated --- //
//@ add-minicore
//@ needs-llvm-components: nvptx
//@ compile-flags: --target=nvptx64-nvidia-cuda --crate-type=rlib
//@ ignore-backends: gcc
#![no_core]
#![feature(no_core, lang_items, abi_ptx, abi_gpu_kernel, nvptx_ext)]

extern crate minicore;
use minicore::*;

struct Params {
    a: u32,
    b: [f32; 4],
}
impl Copy for Params {}

struct NotCopy {
    a: u32,
}

struct Interior {
    a: UnsafeCell<u32>,
}

extern "ptx-kernel" fn ok_ptx(#[nvptx(grid_constant)] p: &Params, #[nvptx(grid_constant)] q: &u64) {}

extern "gpu-kernel" fn ok_gpu<'a>(_x: u32, #[nvptx(grid_constant)] p: &'a Params) {}

extern "ptx-kernel" fn ok_generic<T: Copy + Freeze>(#[nvptx(grid_constant)] p: &T) {}

extern "ptx-kernel" fn not_freeze_generic<T: Copy>(#[nvptx(grid_constant)] p: &T) {}
//~^ ERROR `#[nvptx(grid_constant)]` parameters must point to a type implementing `Freeze`

extern "ptx-kernel" fn mut_ref(#[nvptx(grid_constant)] p: &mut Params) {}
//~^ ERROR `#[nvptx(grid_constant)]` parameters must be shared references

extern "ptx-kernel" fn by_value(#[nvptx(grid_constant)] p: Params) {}
//~^ ERROR `#[nvptx(grid_constant)]` parameters must be shared references

extern "ptx-kernel" fn raw_ptr(#[nvptx(grid_constant)] p: *const Params) {}
//~^ ERROR `#[nvptx(grid_constant)]` parameters must be shared references

extern "ptx-kernel" fn not_copy(#[nvptx(grid_constant)] p: &NotCopy) {}
//~^ ERROR `#[nvptx(grid_constant)]` parameters must point to a type implementing `Copy`

extern "ptx-kernel" fn interior(#[nvptx(grid_constant)] p: &Interior) {}
//~^ ERROR `#[nvptx(grid_constant)]` parameters must point to a type implementing `Copy`
//~| ERROR `#[nvptx(grid_constant)]` parameters must point to a type implementing `Freeze`

fn rust_abi(#[nvptx(grid_constant)] p: &Params) {}
//~^ ERROR `#[nvptx(grid_constant)]` can only be applied to parameters of `extern "gpu-kernel"` or `extern "ptx-kernel"` functions

extern "C" fn c_abi(#[nvptx(grid_constant)] p: &Params) {}
//~^ ERROR `#[nvptx(grid_constant)]` can only be applied to parameters of `extern "gpu-kernel"` or `extern "ptx-kernel"` functions

fn closure() {
    let _ = |#[nvptx(grid_constant)] p: &Params| {};
    //~^ ERROR `#[nvptx(grid_constant)]` can only be applied to parameters of `extern "gpu-kernel"` or `extern "ptx-kernel"` functions
}

#[nvptx(grid_constant)]
//~^ ERROR malformed `nvptx` attribute input
extern "ptx-kernel" fn on_fn(p: &Params) {}

extern "ptx-kernel" fn fn_key_on_param(#[nvptx(max_registers(32))] p: &Params) {}
//~^ ERROR malformed `nvptx` attribute input

extern "ptx-kernel" fn grid_constant_args(#[nvptx(grid_constant(1))] p: &Params) {}
//~^ ERROR malformed `nvptx` attribute input

extern "ptx-kernel" fn bare(#[nvptx] p: &Params) {}
//~^ ERROR malformed `nvptx` attribute input
