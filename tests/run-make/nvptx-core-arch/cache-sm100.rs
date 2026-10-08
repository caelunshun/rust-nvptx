// --- LLM-generated --- //
// Checks the PTX emitted for the tensor prefetch intrinsics in `core::arch::nvptx` that require
// `sm_100a` and PTX ISA 8.6.
// nvptx-target: sm_100a +ptx86

#![feature(no_core, stdarch_nvptx, abi_ptx)]
#![no_core]

extern crate core;

use core::arch::nvptx::*;
use core::ffi::c_void;

// The tensor map is loaded from memory so that LLVM can't infer its address space.
// CHECK-LABEL: .entry cache_tensor_prefetch_gather4(
// CHECK: cp.async.bulk.prefetch.tensor.2d.L2.global.tile::gather4.L2::cache_hint [{{.*}}]
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cache_tensor_prefetch_gather4(t: *const *const c_void) {
    unsafe {
        let t = *t;
        cp_async_bulk_prefetch_tensor_2d_tile_gather4::<true>(t, [1, 2, 3, 4, 5], 0x1000);
    }
}

// CHECK-LABEL: .entry cache_tensor_prefetch_im2col_w(
// CHECK: cp.async.bulk.prefetch.tensor.3d.L2.global.im2col::w [{{.*}}]
// CHECK: cp.async.bulk.prefetch.tensor.3d.L2.global.im2col::w.L2::cache_hint [{{.*}}]
// CHECK: cp.async.bulk.prefetch.tensor.4d.L2.global.im2col::w.L2::cache_hint [{{.*}}]
// CHECK: cp.async.bulk.prefetch.tensor.5d.L2.global.im2col::w.L2::cache_hint [{{.*}}]
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cache_tensor_prefetch_im2col_w(t: *const *const c_void) {
    unsafe {
        let t = *t;
        cp_async_bulk_prefetch_tensor_3d_im2col_w::<false>(t, [1, 2, 3], 4, 5, 0);
        cp_async_bulk_prefetch_tensor_3d_im2col_w::<true>(t, [1, 2, 3], 4, 5, 0x1000);
        cp_async_bulk_prefetch_tensor_4d_im2col_w::<true>(t, [1, 2, 3, 4], 5, 6, 0x1000);
        cp_async_bulk_prefetch_tensor_5d_im2col_w::<true>(t, [1, 2, 3, 4, 5], 6, 7, 0x1000);
    }
}

// CHECK-LABEL: .entry cache_tensor_prefetch_im2col_w_128(
// CHECK: cp.async.bulk.prefetch.tensor.3d.L2.global.im2col::w::128 [{{.*}}]
// CHECK: cp.async.bulk.prefetch.tensor.3d.L2.global.im2col::w::128.L2::cache_hint [{{.*}}]
// CHECK: cp.async.bulk.prefetch.tensor.4d.L2.global.im2col::w::128.L2::cache_hint [{{.*}}]
// CHECK: cp.async.bulk.prefetch.tensor.5d.L2.global.im2col::w::128.L2::cache_hint [{{.*}}]
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cache_tensor_prefetch_im2col_w_128(t: *const *const c_void) {
    unsafe {
        let t = *t;
        cp_async_bulk_prefetch_tensor_3d_im2col_w_128::<false>(t, [1, 2, 3], 4, 5, 0);
        cp_async_bulk_prefetch_tensor_3d_im2col_w_128::<true>(t, [1, 2, 3], 4, 5, 0x1000);
        cp_async_bulk_prefetch_tensor_4d_im2col_w_128::<true>(t, [1, 2, 3, 4], 5, 6, 0x1000);
        cp_async_bulk_prefetch_tensor_5d_im2col_w_128::<true>(t, [1, 2, 3, 4, 5], 6, 7, 0x1000);
    }
}
