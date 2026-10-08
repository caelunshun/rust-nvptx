// --- LLM-generated --- //
// Checks the PTX emitted for the cache control intrinsics in `core::arch::nvptx`.
// nvptx-target: sm_90a +ptx81

#![feature(no_core, stdarch_nvptx, abi_ptx)]
#![no_core]

extern crate core;

use core::arch::nvptx::*;
use core::ffi::c_void;

// The pointers are loaded from memory so that LLVM can't infer their address space.
// CHECK-LABEL: .entry cache_applypriority_l2(
// CHECK: applypriority.L2::evict_normal [{{%rd[0-9]+}}], 128;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cache_applypriority_l2(p: *const *const u32) {
    unsafe { applypriority_l2_evict_normal(*p) }
}

// CHECK-LABEL: .entry cache_discard_l2(
// CHECK: discard.L2 [{{%rd[0-9]+}}], 128;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cache_discard_l2(p: *const *const u32) {
    unsafe { discard_l2(*p) }
}

// CHECK-LABEL: .entry cache_ldu(
// CHECK: ldu.global.b8 {{%rs[0-9]+}}, [{{%rd[0-9]+}}];
// CHECK: ldu.global.b16 {{%rs[0-9]+}}, [{{%rd[0-9]+}}];
// CHECK: ldu.global.b32 {{%r[0-9]+}}, [{{%rd[0-9]+}}];
// CHECK: ldu.global.b64 {{%rd[0-9]+}}, [{{%rd[0-9]+}}];
// CHECK: ldu.global.b32 {{%r[0-9]+}}, [{{%rd[0-9]+}}];
// CHECK: ldu.global.b64 {{%rd[0-9]+}}, [{{%rd[0-9]+}}];
// CHECK: ldu.global.b64 {{%rd[0-9]+}}, [{{%rd[0-9]+}}];
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cache_ldu(out: *mut u64, p: *const *const c_void) {
    unsafe {
        let p = *p;
        *out = ldu_u8(p.cast()) as u64;
        *out.add(1) = ldu_u16(p.cast()) as u64;
        *out.add(2) = ldu_u32(p.cast()) as u64;
        *out.add(3) = ldu_u64(p.cast());
        *out.add(4) = ldu_f32(p.cast()).to_bits() as u64;
        *out.add(5) = ldu_f64(p.cast()).to_bits();
        *out.add(6) = ldu_ptr::<u32>(p.cast()) as u64;
    }
}

// CHECK-LABEL: .entry cache_prefetch(
// CHECK: prefetch.L1 [{{%rd[0-9]+}}];
// CHECK: prefetch.L2 [{{%rd[0-9]+}}];
// CHECK: prefetchu.L1 [{{%rd[0-9]+}}];
// CHECK: prefetch.tensormap [{{%rd[0-9]+}}];
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cache_prefetch(p: *const *const c_void) {
    unsafe {
        let p = *p;
        prefetch_l1(p);
        prefetch_l2(p);
        prefetchu_l1(p);
        prefetch_tensormap(p);
    }
}

// CHECK-LABEL: .entry cache_tensor_prefetch_im2col(
// CHECK: cp.async.bulk.prefetch.tensor.3d.L2.global.im2col [{{.*}}]
// CHECK: cp.async.bulk.prefetch.tensor.3d.L2.global.im2col.L2::cache_hint [{{.*}}]
// CHECK: cp.async.bulk.prefetch.tensor.4d.L2.global.im2col.L2::cache_hint [{{.*}}]
// CHECK: cp.async.bulk.prefetch.tensor.5d.L2.global.im2col.L2::cache_hint [{{.*}}]
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cache_tensor_prefetch_im2col(t: *const *const c_void) {
    unsafe {
        let t = *t;
        cp_async_bulk_prefetch_tensor_3d_im2col::<false>(t, [1, 2, 3], [4], 0);
        cp_async_bulk_prefetch_tensor_3d_im2col::<true>(t, [1, 2, 3], [4], 0x1000);
        cp_async_bulk_prefetch_tensor_4d_im2col::<true>(t, [1, 2, 3, 4], [5, 6], 0x1000);
        cp_async_bulk_prefetch_tensor_5d_im2col::<true>(t, [1, 2, 3, 4, 5], [6, 7, 8], 0x1000);
    }
}

// CHECK-LABEL: .entry cache_tensor_prefetch_tile(
// CHECK: cp.async.bulk.prefetch.tensor.1d.L2.global.tile [{{.*}}]
// CHECK: cp.async.bulk.prefetch.tensor.2d.L2.global.tile.L2::cache_hint [{{.*}}]
// CHECK: cp.async.bulk.prefetch.tensor.3d.L2.global.tile.L2::cache_hint [{{.*}}]
// CHECK: cp.async.bulk.prefetch.tensor.4d.L2.global.tile.L2::cache_hint [{{.*}}]
// CHECK: cp.async.bulk.prefetch.tensor.5d.L2.global.tile.L2::cache_hint [{{.*}}]
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cache_tensor_prefetch_tile(t: *const *const c_void) {
    unsafe {
        let t = *t;
        cp_async_bulk_prefetch_tensor_1d_tile::<false>(t, [1], 0);
        cp_async_bulk_prefetch_tensor_2d_tile::<true>(t, [1, 2], 0x1000);
        cp_async_bulk_prefetch_tensor_3d_tile::<true>(t, [1, 2, 3], 0x1000);
        cp_async_bulk_prefetch_tensor_4d_tile::<true>(t, [1, 2, 3, 4], 0x1000);
        cp_async_bulk_prefetch_tensor_5d_tile::<true>(t, [1, 2, 3, 4, 5], 0x1000);
    }
}
