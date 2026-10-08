// --- LLM-generated --- //
// Checks that `#[nvptx(grid_constant)]` parameters are accessed in param space without being
// copied to local memory, even when their address escapes.

//@ add-minicore
//@ assembly-output: emit-asm
//@ compile-flags: --crate-type=rlib --target=nvptx64-nvidia-cuda -Ctarget-cpu=sm_90 -Copt-level=3
//@ needs-llvm-components: nvptx
#![feature(no_core, lang_items, abi_ptx, nvptx_ext)]
#![no_core]

extern crate minicore;
use minicore::*;

#[repr(C)]
pub struct Params {
    a: u64,
    b: u64,
    c: u64,
    d: u64,
}
impl Copy for Params {}

// CHECK-LABEL: .entry escapes(
// CHECK-NEXT: .param .align 8 .b8 escapes_param_0[32]
// CHECK-NOT: .local
// CHECK: mov.b64 [[PARAM:%rd[0-9]+]], escapes_param_0;
// CHECK: cvta.param.u64 [[GENERIC:%rd[0-9]+]], [[PARAM]];
// CHECK: st.global.b64 [{{%rd[0-9]+}}], [[GENERIC]];
#[no_mangle]
pub unsafe extern "ptx-kernel" fn escapes(
    #[nvptx(grid_constant)] p: &Params,
    out: *mut *const Params,
) {
    *out = p;
}
