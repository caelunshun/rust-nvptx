// --- LLM-generated --- //
// Checks the PTX emitted for the memory and address space intrinsics in `core::arch::nvptx`.
// nvptx-target: sm_90a +ptx81

#![feature(no_core, stdarch_nvptx, abi_ptx)]
#![no_core]

extern crate core;

use core::arch::nvptx::*;
use core::ffi::c_void;

// The pointers are loaded from memory so that LLVM can't infer their address spaces.
// CHECK-LABEL: .entry mem_cvta_const(
// CHECK: cvta.to.const.u64 [[P64:%rd[0-9]+]],
// CHECK: cvt.u32.u64 [[P:%r[0-9]+]], [[P64]];
// CHECK: add.s32 [[Q:%r[0-9]+]], [[P]],
// CHECK: ld.const.b32 {{%r[0-9]+}}, [[[Q]]];
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn mem_cvta_const(out: *mut u32, p: *const *mut u32, i: u32) {
    unsafe {
        let p = cvta::<{ StateSpace::Const }, _>(*p);
        *out = *p.add(i as usize);
    }
}

// The returned pointer escapes, so the conversions to and from the shared state space cancel out.
// CHECK-LABEL: .entry mem_cvta_escape(
// CHECK-NOT: cvta.to.shared
// CHECK-NOT: cvta.shared
// CHECK: ret;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn mem_cvta_escape(out: *mut *mut u32, p: *const *mut u32) {
    unsafe { *out = cvta::<{ StateSpace::SharedCta }, _>(*p) }
}

// CHECK-LABEL: .entry mem_cvta_global(
// CHECK: ld.global.b64 [[G:%rd[0-9]+]],
// CHECK: cvta.to.global.u64 [[P:%rd[0-9]+]], [[G]];
// CHECK: add.s64 [[Q:%rd[0-9]+]], [[P]],
// CHECK: ld.global.b32 {{%r[0-9]+}}, [[[Q]]];
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn mem_cvta_global(out: *mut u32, p: *const *mut u32, i: u32) {
    unsafe {
        let p = cvta::<{ StateSpace::Global }, _>(*p);
        *out = *p.add(i as usize);
    }
}

// CHECK-LABEL: .entry mem_cvta_local(
// CHECK: cvta.to.local.u64 [[P64:%rd[0-9]+]],
// CHECK: cvt.u32.u64 [[P:%r[0-9]+]], [[P64]];
// CHECK: add.s32 [[Q:%r[0-9]+]], [[P]],
// CHECK: ld.local.b32 {{%r[0-9]+}}, [[[Q]]];
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn mem_cvta_local(out: *mut u32, p: *const *mut u32, i: u32) {
    unsafe {
        let p = cvta::<{ StateSpace::Local }, _>(*p);
        *out = *p.add(i as usize);
    }
}

// CHECK-LABEL: .entry mem_cvta_shared(
// CHECK: cvta.to.shared.u64 [[P64:%rd[0-9]+]],
// CHECK: cvt.u32.u64 [[P:%r[0-9]+]], [[P64]];
// CHECK: add.s32 [[Q:%r[0-9]+]], [[P]],
// CHECK: ld.shared.b32 {{%r[0-9]+}}, [[[Q]]];
// CHECK: ld.shared.b32 {{%r[0-9]+}}, [[[Q]]+4];
// CHECK: st.shared.b32 [[[Q]]], 0;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn mem_cvta_shared(out: *mut u32, p: *const *mut u32, i: u32) {
    unsafe {
        let p = cvta::<{ StateSpace::SharedCta }, _>(*p);
        let q = p.add(i as usize);
        *out = *q + *q.add(1);
        *q = 0;
    }
}

// CHECK-LABEL: .entry mem_cvta_shared_cluster(
// CHECK: cvta.to.shared::cluster.u64 [[P64:%rd[0-9]+]],
// CHECK: cvt.u32.u64 [[P:%r[0-9]+]], [[P64]];
// CHECK: add.s32 [[Q:%r[0-9]+]], [[P]],
// CHECK: ld.shared::cluster.b32 {{%r[0-9]+}}, [[[Q]]];
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn mem_cvta_shared_cluster(
    out: *mut u32,
    p: *const *mut u32,
    i: u32,
) {
    unsafe {
        let p = cvta::<{ StateSpace::SharedCluster }, _>(*p);
        *out = *p.add(i as usize);
    }
}

// The pointer is loaded from memory so that LLVM can't infer its address space.
// CHECK-LABEL: .entry mem_getctarank(
// CHECK: getctarank.u64 {{%r[0-9]+}}, {{%rd[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn mem_getctarank(out: *mut u32, p: *const *const u32) {
    unsafe { *out = getctarank::<{ StateSpace::Generic }, _>(*p) }
}

// CHECK-LABEL: .entry mem_getctarank_shared(
// CHECK: getctarank.shared::cluster.u32 {{%r[0-9]+}}, {{%r[0-9]+}};
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
// CHECK: mapa.shared::cluster.u32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn mem_mapa_shared(
    out: *mut *mut u32,
    p: *const *mut u32,
    rank: u32,
) {
    unsafe { *out = mapa::<{ StateSpace::SharedCta }, _>(*p, rank) }
}

// CHECK-LABEL: .entry mem_shared_offset_to_ptr(
// CHECK-NOT: cvta.to.shared
// CHECK-NOT: cvta.shared
// CHECK: shl.b32 [[I:%r[0-9]+]],
// CHECK: add.s32 [[Q:%r[0-9]+]], {{%r[0-9]+}}, [[I]];
// CHECK: ld.shared.b32 {{%r[0-9]+}}, [[[Q]]];
// CHECK: ld.shared.b32 {{%r[0-9]+}}, [[[Q]]+4];
// CHECK: st.shared.b32 [[[Q]]], 0;
// CHECK-NOT: cvta.to.shared
// CHECK-NOT: cvta.shared
// CHECK: ret;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn mem_shared_offset_to_ptr(out: *mut u32, offset: u32, i: u32) {
    unsafe {
        let q = shared_offset_to_ptr::<u32>(offset).add(i as usize);
        *out = *q + *q.add(1);
        *q = 0;
    }
}

// CHECK-LABEL: .entry mem_shared_offset_to_ptr_escape(
// CHECK: ld.param.b32 [[OFFSET:%r[0-9]+]],
// CHECK: cvt.u64.u32 [[OFFSET64:%rd[0-9]+]], [[OFFSET]];
// CHECK: cvta.shared.u64 [[P:%rd[0-9]+]], [[OFFSET64]];
// CHECK: st.global.b64 [{{%rd[0-9]+}}], [[P]];
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn mem_shared_offset_to_ptr_escape(out: *mut *mut u32, offset: u32) {
    unsafe { *out = shared_offset_to_ptr(offset) }
}

// CHECK-LABEL: .entry mem_st_async(
// CHECK: st.async.shared::cluster.mbarrier::complete_tx::bytes.b32 [{{%r[0-9]+}}], {{%r[0-9]+}}, [{{%r[0-9]+}}];
// CHECK: st.async.shared::cluster.mbarrier::complete_tx::bytes.b64 [{{%r[0-9]+}}], {{%rd[0-9]+}}, [{{%r[0-9]+}}];
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn mem_st_async(p: *const *mut c_void, value32: u32, value64: u64) {
    unsafe {
        st_async_u32(*p as *mut u32, value32, *p.add(1));
        st_async_u64(*p.add(2) as *mut u64, value64, *p.add(3));
    }
}
