// --- LLM-generated --- //
// Checks the PTX emitted for the memory intrinsics in `core::arch::nvptx` that require `sm_100`
// and PTX ISA 8.6.
// nvptx-target: sm_100a +ptx86

#![feature(no_core, stdarch_nvptx, abi_ptx)]
#![no_core]

extern crate core;

use core::arch::nvptx::*;
use core::ffi::c_void;

// The pointer is loaded from memory so that LLVM can't infer its address space.
// CHECK-LABEL: .entry mem_st_bulk(
// CHECK: st.bulk [{{%rd[0-9]+}}], {{%rd[0-9]+}}, 0;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn mem_st_bulk(p: *const *mut c_void) {
    unsafe { st_bulk(*p, 64) }
}
