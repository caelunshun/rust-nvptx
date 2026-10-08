// --- LLM-generated --- //
//@ add-minicore
//@ needs-llvm-components: amdgpu
//@ compile-flags: --target=amdgcn-amd-amdhsa -Ctarget-cpu=gfx900 --crate-type=rlib
//@ ignore-backends: gcc
#![no_core]
#![feature(no_core, lang_items, abi_gpu_kernel, nvptx_ext)]

extern crate minicore;
use minicore::*;

extern "gpu-kernel" fn kernel(#[nvptx(grid_constant)] p: &u32) {}
//~^ ERROR `#[nvptx]` is only supported on nvptx targets
