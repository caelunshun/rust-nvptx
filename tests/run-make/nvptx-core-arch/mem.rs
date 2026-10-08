// --- LLM-generated --- //
// Checks the PTX emitted for the memory and address space intrinsics in `core::arch::nvptx`.
// nvptx-target: sm_90a +ptx81

#![feature(no_core, stdarch_nvptx, abi_ptx)]
#![no_core]

extern crate core;

use core::arch::nvptx::*;
use core::ffi::c_void;

// The pointer is loaded from memory so that LLVM can't infer its address space.
// CHECK-LABEL: .entry mem_getctarank(
// CHECK: getctarank.u64 {{%r[0-9]+}}, {{%rd[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn mem_getctarank(out: *mut u32, p: *const *const u32) {
    unsafe { *out = getctarank::<{ StateSpace::Generic }, _>(*p) }
}

// CHECK-LABEL: .entry mem_getctarank_shared(
// CHECK: getctarank.shared::cluster.u64 {{%r[0-9]+}}, {{%rd[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn mem_getctarank_shared(out: *mut u32, p: *const *const u32) {
    unsafe { *out = getctarank::<{ StateSpace::SharedCta }, _>(*p) }
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
    unsafe { *out = mapa::<{ StateSpace::Generic }, _>(*p, rank) }
}

// CHECK-LABEL: .entry mem_mapa_shared(
// CHECK: mapa.shared::cluster.u64 {{%rd[0-9]+}}, {{%rd[0-9]+}}, {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn mem_mapa_shared(
    out: *mut *mut u32,
    p: *const *mut u32,
    rank: u32,
) {
    unsafe { *out = mapa::<{ StateSpace::SharedCta }, _>(*p, rank) }
}

// CHECK-LABEL: .entry mem_st_async(
// CHECK: st.async.shared::cluster.mbarrier::complete_tx::bytes.b32 [{{%rd[0-9]+}}], {{%r[0-9]+}}, [{{%rd[0-9]+}}];
// CHECK: st.async.shared::cluster.mbarrier::complete_tx::bytes.b64 [{{%rd[0-9]+}}], {{%rd[0-9]+}}, [{{%rd[0-9]+}}];
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn mem_st_async(p: *const *mut c_void, value32: u32, value64: u64) {
    unsafe {
        st_async_u32(*p as *mut u32, value32, *p.add(1));
        st_async_u64(*p.add(2) as *mut u64, value64, *p.add(3));
    }
}
