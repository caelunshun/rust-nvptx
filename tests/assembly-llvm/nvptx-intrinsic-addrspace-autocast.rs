// --- LLM-generated --- //
// Checks that generic pointers passed to LLVM intrinsics expecting other address spaces are
// converted with `cvta`, and that the intrinsics lower to their address-space specific forms.

//@ add-minicore
//@ assembly-output: emit-asm
//@ compile-flags: --crate-type=rlib --target=nvptx64-nvidia-cuda -Ctarget-cpu=sm_90 -Copt-level=3
//@ needs-llvm-components: nvptx
#![feature(no_core, lang_items, link_llvm_intrinsics, rustc_attrs)]
#![no_core]

extern crate minicore;
use minicore::*;

unsafe extern "llvm-intrinsic" {
    #[link_name = "llvm.nvvm.cp.async.ca.shared.global.4"]
    fn cp_async_ca_shared_global_4(dst: *mut u8, src: *const u8);
    #[link_name = "llvm.nvvm.mapa.shared.cluster"]
    fn mapa_shared_cluster(p: *mut u8, rank: u32) -> *mut u8;
    #[link_name = "llvm.nvvm.ldu.global.i.i32.p1"]
    #[rustc_llvm_ptr_addrspace(args(1, 0))]
    fn ldu_global_i32(p: *const i32, align: i32) -> i32;
}

// CHECK-LABEL: .visible .func cp_async(
// CHECK-DAG: cvta.to.shared.u64 [[DST64:%rd[0-9]+]],
// CHECK-DAG: cvt.u32.u64 [[DST:%r[0-9]+]], [[DST64]];
// CHECK-DAG: cvta.to.global.u64 [[SRC:%rd[0-9]+]],
// CHECK: cp.async.ca.shared.global [[[DST]]], [[[SRC]]], 4;
#[no_mangle]
pub unsafe fn cp_async(dst: *mut u8, src: *const u8) {
    cp_async_ca_shared_global_4(dst, src)
}

// CHECK-LABEL: .visible .func (.param .b64 func_retval0) mapa(
// CHECK: cvta.to.shared.u64
// CHECK: mapa.shared::cluster.u32
// CHECK: cvta.shared::cluster.u64
#[no_mangle]
pub unsafe fn mapa(p: *mut u8, rank: u32) -> *mut u8 {
    mapa_shared_cluster(p, rank)
}

// CHECK-LABEL: .visible .func (.param .b32 func_retval0) ldu(
// CHECK: cvta.to.global.u64 [[P:%rd[0-9]+]],
// CHECK: ldu.global.b32 {{%r[0-9]+}}, [[[P]]];
#[no_mangle]
pub unsafe fn ldu(p: *const i32) -> i32 {
    ldu_global_i32(p, 4)
}
