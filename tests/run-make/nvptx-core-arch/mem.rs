// --- LLM-generated --- //
// Checks the PTX emitted for the memory and address space intrinsics in `core::arch::nvptx`.
// nvptx-target: sm_90a +ptx81

#![feature(no_core, stdarch_nvptx, abi_ptx)]
#![no_core]

extern crate core;

use core::arch::nvptx::*;
use core::ffi::c_void;

// CHECK-LABEL: .entry mem_getctarank(
// CHECK: getctarank.u64 {{%r[0-9]+}}, {{%rd[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn mem_getctarank(out: *mut u32, p: *const *const u32) {
    unsafe { *out = getctarank(*p) }
}

// The pointer is loaded from memory so that LLVM can't infer its address space.
// CHECK-LABEL: .entry mem_isspacep(
// CHECK: isspacep.const {{%p[0-9]+}}, [[P:%rd[0-9]+]];
// CHECK: isspacep.global {{%p[0-9]+}}, [[P]];
// CHECK: isspacep.local {{%p[0-9]+}}, [[P]];
// CHECK: isspacep.shared {{%p[0-9]+}}, [[P]];
// CHECK: isspacep.shared::cluster {{%p[0-9]+}}, [[P]];
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn mem_isspacep(out: *mut bool, p: *const *const c_void) {
    unsafe {
        let p = *p;
        *out = isspacep::<{ StateSpace::Const }>(p);
        *out.add(1) = isspacep::<{ StateSpace::Global }>(p);
        *out.add(2) = isspacep::<{ StateSpace::Local }>(p);
        *out.add(3) = isspacep::<{ StateSpace::SharedCta }>(p);
        *out.add(4) = isspacep::<{ StateSpace::SharedCluster }>(p);
    }
}

// CHECK-LABEL: .entry mem_mapa(
// CHECK: mapa.u64 {{%rd[0-9]+}}, {{%rd[0-9]+}}, {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn mem_mapa(out: *mut *mut u32, p: *const *mut u32, rank: u32) {
    unsafe { *out = mapa(*p, rank) }
}
