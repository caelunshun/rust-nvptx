// --- LLM-generated --- //
//@ add-minicore
//@ needs-llvm-components: nvptx
//@ compile-flags: --target=nvptx64-nvidia-cuda --crate-type=rlib
//@ ignore-backends: gcc
#![no_core]
#![feature(no_core, lang_items, abi_ptx)]

extern crate minicore;
use minicore::*;

#[nvptx(max_registers(32))] //~ ERROR the `nvptx` attribute is an experimental feature
extern "ptx-kernel" fn kernel() {}
