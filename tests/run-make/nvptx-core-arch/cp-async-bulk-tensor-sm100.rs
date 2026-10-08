// --- LLM-generated --- //
// Checks the PTX emitted for the tensor copy and reduction intrinsics in `core::arch::nvptx`
// that require `sm_100a` and PTX ISA 8.6.
// nvptx-target: sm_100a +ptx86

#![feature(no_core, stdarch_nvptx, abi_ptx)]
#![no_core]

extern crate core;

use core::arch::nvptx::*;
use core::ffi::c_void;

// CHECK-LABEL: .entry g2s_cta_im2col_w_128_3d(
// CHECK: cp.async.bulk.tensor.3d.shared::cta.global.im2col::w::128.mbarrier::complete_tx::bytes [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cta.global.im2col::w::128.mbarrier::complete_tx::bytes.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_cta_im2col_w_128_3d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_3d_g2s_cta_im2col_w_128::<false>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            2,
            3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_cta_im2col_w_128::<true>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            2,
            3,
            0x1000,
        );
    }
}

// CHECK-LABEL: .entry g2s_cta_im2col_w_128_4d(
// CHECK: cp.async.bulk.tensor.4d.shared::cta.global.im2col::w::128.mbarrier::complete_tx::bytes [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cta.global.im2col::w::128.mbarrier::complete_tx::bytes.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_cta_im2col_w_128_4d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_4d_g2s_cta_im2col_w_128::<false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            2,
            3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_cta_im2col_w_128::<true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            2,
            3,
            0x1000,
        );
    }
}

// CHECK-LABEL: .entry g2s_cta_im2col_w_128_5d(
// CHECK: cp.async.bulk.tensor.5d.shared::cta.global.im2col::w::128.mbarrier::complete_tx::bytes [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cta.global.im2col::w::128.mbarrier::complete_tx::bytes.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_cta_im2col_w_128_5d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_5d_g2s_cta_im2col_w_128::<false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            2,
            3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_cta_im2col_w_128::<true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            2,
            3,
            0x1000,
        );
    }
}

// CHECK-LABEL: .entry g2s_cta_im2col_w_3d(
// CHECK: cp.async.bulk.tensor.3d.shared::cta.global.im2col::w.mbarrier::complete_tx::bytes [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cta.global.im2col::w.mbarrier::complete_tx::bytes.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_cta_im2col_w_3d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_3d_g2s_cta_im2col_w::<false>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            2,
            3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_cta_im2col_w::<true>(smem, smem, tmap, [1, 2, 3], 2, 3, 0x1000);
    }
}

// CHECK-LABEL: .entry g2s_cta_im2col_w_4d(
// CHECK: cp.async.bulk.tensor.4d.shared::cta.global.im2col::w.mbarrier::complete_tx::bytes [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cta.global.im2col::w.mbarrier::complete_tx::bytes.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_cta_im2col_w_4d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_4d_g2s_cta_im2col_w::<false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            2,
            3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_cta_im2col_w::<true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            2,
            3,
            0x1000,
        );
    }
}

// CHECK-LABEL: .entry g2s_cta_im2col_w_5d(
// CHECK: cp.async.bulk.tensor.5d.shared::cta.global.im2col::w.mbarrier::complete_tx::bytes [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cta.global.im2col::w.mbarrier::complete_tx::bytes.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_cta_im2col_w_5d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_5d_g2s_cta_im2col_w::<false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            2,
            3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_cta_im2col_w::<true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            2,
            3,
            0x1000,
        );
    }
}

// CHECK-LABEL: .entry g2s_cta_tile_gather4_2d(
// CHECK: cp.async.bulk.tensor.2d.shared::cta.global.tile::gather4.mbarrier::complete_tx::bytes [{{.*}}
// CHECK: cp.async.bulk.tensor.2d.shared::cta.global.tile::gather4.mbarrier::complete_tx::bytes.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_cta_tile_gather4_2d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_2d_g2s_cta_tile_gather4::<false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x1000,
        );
        cp_async_bulk_tensor_2d_g2s_cta_tile_gather4::<true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x1000,
        );
    }
}

// CHECK-LABEL: .entry g2s_im2col_3d(
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.im2col.mbarrier::complete_tx::bytes.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.im2col.mbarrier::complete_tx::bytes.L2::cache_hint.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.im2col.mbarrier::complete_tx::bytes.multicast::cluster.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.im2col.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.im2col.mbarrier::complete_tx::bytes.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.im2col.mbarrier::complete_tx::bytes.L2::cache_hint.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.im2col.mbarrier::complete_tx::bytes.multicast::cluster.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.im2col.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint.cta_group::2 [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_im2col_3d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_3d_g2s_im2col::<1, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            [1],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_im2col::<1, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            [1],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_im2col::<1, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            [1],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_im2col::<1, true, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            [1],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_im2col::<2, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            [1],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_im2col::<2, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            [1],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_im2col::<2, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            [1],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_im2col::<2, true, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            [1],
            0x3,
            0x1000,
        );
    }
}

// CHECK-LABEL: .entry g2s_im2col_4d(
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.im2col.mbarrier::complete_tx::bytes.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.im2col.mbarrier::complete_tx::bytes.L2::cache_hint.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.im2col.mbarrier::complete_tx::bytes.multicast::cluster.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.im2col.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.im2col.mbarrier::complete_tx::bytes.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.im2col.mbarrier::complete_tx::bytes.L2::cache_hint.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.im2col.mbarrier::complete_tx::bytes.multicast::cluster.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.im2col.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint.cta_group::2 [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_im2col_4d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_4d_g2s_im2col::<1, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            [1, 2],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_im2col::<1, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            [1, 2],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_im2col::<1, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            [1, 2],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_im2col::<1, true, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            [1, 2],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_im2col::<2, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            [1, 2],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_im2col::<2, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            [1, 2],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_im2col::<2, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            [1, 2],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_im2col::<2, true, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            [1, 2],
            0x3,
            0x1000,
        );
    }
}

// CHECK-LABEL: .entry g2s_im2col_5d(
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.im2col.mbarrier::complete_tx::bytes.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.im2col.mbarrier::complete_tx::bytes.L2::cache_hint.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.im2col.mbarrier::complete_tx::bytes.multicast::cluster.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.im2col.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.im2col.mbarrier::complete_tx::bytes.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.im2col.mbarrier::complete_tx::bytes.L2::cache_hint.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.im2col.mbarrier::complete_tx::bytes.multicast::cluster.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.im2col.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint.cta_group::2 [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_im2col_5d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_5d_g2s_im2col::<1, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            [1, 2, 3],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_im2col::<1, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            [1, 2, 3],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_im2col::<1, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            [1, 2, 3],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_im2col::<1, true, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            [1, 2, 3],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_im2col::<2, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            [1, 2, 3],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_im2col::<2, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            [1, 2, 3],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_im2col::<2, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            [1, 2, 3],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_im2col::<2, true, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            [1, 2, 3],
            0x3,
            0x1000,
        );
    }
}

// CHECK-LABEL: .entry g2s_im2col_w_128_3d(
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.im2col::w::128.mbarrier::complete_tx::bytes [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.im2col::w::128.mbarrier::complete_tx::bytes.L2::cache_hint [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.im2col::w::128.mbarrier::complete_tx::bytes.multicast::cluster [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.im2col::w::128.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.im2col::w::128.mbarrier::complete_tx::bytes.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.im2col::w::128.mbarrier::complete_tx::bytes.L2::cache_hint.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.im2col::w::128.mbarrier::complete_tx::bytes.multicast::cluster.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.im2col::w::128.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.im2col::w::128.mbarrier::complete_tx::bytes.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.im2col::w::128.mbarrier::complete_tx::bytes.L2::cache_hint.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.im2col::w::128.mbarrier::complete_tx::bytes.multicast::cluster.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.im2col::w::128.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint.cta_group::2 [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_im2col_w_128_3d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_3d_g2s_im2col_w_128::<0, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_im2col_w_128::<0, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_im2col_w_128::<0, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_im2col_w_128::<0, true, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_im2col_w_128::<1, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_im2col_w_128::<1, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_im2col_w_128::<1, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_im2col_w_128::<1, true, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_im2col_w_128::<2, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_im2col_w_128::<2, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_im2col_w_128::<2, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_im2col_w_128::<2, true, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            2,
            3,
            0x3,
            0x1000,
        );
    }
}

// CHECK-LABEL: .entry g2s_im2col_w_128_4d(
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.im2col::w::128.mbarrier::complete_tx::bytes [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.im2col::w::128.mbarrier::complete_tx::bytes.L2::cache_hint [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.im2col::w::128.mbarrier::complete_tx::bytes.multicast::cluster [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.im2col::w::128.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.im2col::w::128.mbarrier::complete_tx::bytes.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.im2col::w::128.mbarrier::complete_tx::bytes.L2::cache_hint.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.im2col::w::128.mbarrier::complete_tx::bytes.multicast::cluster.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.im2col::w::128.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.im2col::w::128.mbarrier::complete_tx::bytes.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.im2col::w::128.mbarrier::complete_tx::bytes.L2::cache_hint.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.im2col::w::128.mbarrier::complete_tx::bytes.multicast::cluster.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.im2col::w::128.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint.cta_group::2 [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_im2col_w_128_4d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_4d_g2s_im2col_w_128::<0, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_im2col_w_128::<0, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_im2col_w_128::<0, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_im2col_w_128::<0, true, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_im2col_w_128::<1, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_im2col_w_128::<1, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_im2col_w_128::<1, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_im2col_w_128::<1, true, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_im2col_w_128::<2, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_im2col_w_128::<2, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_im2col_w_128::<2, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_im2col_w_128::<2, true, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            2,
            3,
            0x3,
            0x1000,
        );
    }
}

// CHECK-LABEL: .entry g2s_im2col_w_128_5d(
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.im2col::w::128.mbarrier::complete_tx::bytes [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.im2col::w::128.mbarrier::complete_tx::bytes.L2::cache_hint [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.im2col::w::128.mbarrier::complete_tx::bytes.multicast::cluster [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.im2col::w::128.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.im2col::w::128.mbarrier::complete_tx::bytes.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.im2col::w::128.mbarrier::complete_tx::bytes.L2::cache_hint.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.im2col::w::128.mbarrier::complete_tx::bytes.multicast::cluster.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.im2col::w::128.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.im2col::w::128.mbarrier::complete_tx::bytes.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.im2col::w::128.mbarrier::complete_tx::bytes.L2::cache_hint.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.im2col::w::128.mbarrier::complete_tx::bytes.multicast::cluster.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.im2col::w::128.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint.cta_group::2 [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_im2col_w_128_5d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_5d_g2s_im2col_w_128::<0, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_im2col_w_128::<0, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_im2col_w_128::<0, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_im2col_w_128::<0, true, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_im2col_w_128::<1, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_im2col_w_128::<1, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_im2col_w_128::<1, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_im2col_w_128::<1, true, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_im2col_w_128::<2, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_im2col_w_128::<2, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_im2col_w_128::<2, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_im2col_w_128::<2, true, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            2,
            3,
            0x3,
            0x1000,
        );
    }
}

// CHECK-LABEL: .entry g2s_im2col_w_3d(
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.im2col::w.mbarrier::complete_tx::bytes [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.im2col::w.mbarrier::complete_tx::bytes.L2::cache_hint [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.im2col::w.mbarrier::complete_tx::bytes.multicast::cluster [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.im2col::w.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.im2col::w.mbarrier::complete_tx::bytes.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.im2col::w.mbarrier::complete_tx::bytes.L2::cache_hint.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.im2col::w.mbarrier::complete_tx::bytes.multicast::cluster.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.im2col::w.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.im2col::w.mbarrier::complete_tx::bytes.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.im2col::w.mbarrier::complete_tx::bytes.L2::cache_hint.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.im2col::w.mbarrier::complete_tx::bytes.multicast::cluster.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.im2col::w.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint.cta_group::2 [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_im2col_w_3d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_3d_g2s_im2col_w::<0, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_im2col_w::<0, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_im2col_w::<0, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_im2col_w::<0, true, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_im2col_w::<1, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_im2col_w::<1, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_im2col_w::<1, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_im2col_w::<1, true, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_im2col_w::<2, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_im2col_w::<2, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_im2col_w::<2, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_im2col_w::<2, true, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            2,
            3,
            0x3,
            0x1000,
        );
    }
}

// CHECK-LABEL: .entry g2s_im2col_w_4d(
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.im2col::w.mbarrier::complete_tx::bytes [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.im2col::w.mbarrier::complete_tx::bytes.L2::cache_hint [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.im2col::w.mbarrier::complete_tx::bytes.multicast::cluster [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.im2col::w.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.im2col::w.mbarrier::complete_tx::bytes.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.im2col::w.mbarrier::complete_tx::bytes.L2::cache_hint.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.im2col::w.mbarrier::complete_tx::bytes.multicast::cluster.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.im2col::w.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.im2col::w.mbarrier::complete_tx::bytes.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.im2col::w.mbarrier::complete_tx::bytes.L2::cache_hint.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.im2col::w.mbarrier::complete_tx::bytes.multicast::cluster.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.im2col::w.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint.cta_group::2 [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_im2col_w_4d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_4d_g2s_im2col_w::<0, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_im2col_w::<0, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_im2col_w::<0, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_im2col_w::<0, true, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_im2col_w::<1, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_im2col_w::<1, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_im2col_w::<1, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_im2col_w::<1, true, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_im2col_w::<2, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_im2col_w::<2, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_im2col_w::<2, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_im2col_w::<2, true, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            2,
            3,
            0x3,
            0x1000,
        );
    }
}

// CHECK-LABEL: .entry g2s_im2col_w_5d(
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.im2col::w.mbarrier::complete_tx::bytes [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.im2col::w.mbarrier::complete_tx::bytes.L2::cache_hint [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.im2col::w.mbarrier::complete_tx::bytes.multicast::cluster [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.im2col::w.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.im2col::w.mbarrier::complete_tx::bytes.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.im2col::w.mbarrier::complete_tx::bytes.L2::cache_hint.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.im2col::w.mbarrier::complete_tx::bytes.multicast::cluster.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.im2col::w.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.im2col::w.mbarrier::complete_tx::bytes.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.im2col::w.mbarrier::complete_tx::bytes.L2::cache_hint.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.im2col::w.mbarrier::complete_tx::bytes.multicast::cluster.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.im2col::w.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint.cta_group::2 [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_im2col_w_5d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_5d_g2s_im2col_w::<0, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_im2col_w::<0, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_im2col_w::<0, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_im2col_w::<0, true, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_im2col_w::<1, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_im2col_w::<1, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_im2col_w::<1, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_im2col_w::<1, true, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_im2col_w::<2, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_im2col_w::<2, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_im2col_w::<2, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            2,
            3,
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_im2col_w::<2, true, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            2,
            3,
            0x3,
            0x1000,
        );
    }
}

// CHECK-LABEL: .entry g2s_tile_1d(
// CHECK: cp.async.bulk.tensor.1d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.1d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.L2::cache_hint.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.1d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.multicast::cluster.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.1d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.1d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.1d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.L2::cache_hint.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.1d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.multicast::cluster.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.1d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint.cta_group::2 [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_tile_1d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_1d_g2s_tile::<1, false, false>(smem, smem, tmap, [1], 0x3, 0x1000);
        cp_async_bulk_tensor_1d_g2s_tile::<1, false, true>(smem, smem, tmap, [1], 0x3, 0x1000);
        cp_async_bulk_tensor_1d_g2s_tile::<1, true, false>(smem, smem, tmap, [1], 0x3, 0x1000);
        cp_async_bulk_tensor_1d_g2s_tile::<1, true, true>(smem, smem, tmap, [1], 0x3, 0x1000);
        cp_async_bulk_tensor_1d_g2s_tile::<2, false, false>(smem, smem, tmap, [1], 0x3, 0x1000);
        cp_async_bulk_tensor_1d_g2s_tile::<2, false, true>(smem, smem, tmap, [1], 0x3, 0x1000);
        cp_async_bulk_tensor_1d_g2s_tile::<2, true, false>(smem, smem, tmap, [1], 0x3, 0x1000);
        cp_async_bulk_tensor_1d_g2s_tile::<2, true, true>(smem, smem, tmap, [1], 0x3, 0x1000);
    }
}

// CHECK-LABEL: .entry g2s_tile_2d(
// CHECK: cp.async.bulk.tensor.2d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.2d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.L2::cache_hint.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.2d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.multicast::cluster.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.2d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.2d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.2d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.L2::cache_hint.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.2d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.multicast::cluster.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.2d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint.cta_group::2 [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_tile_2d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_2d_g2s_tile::<1, false, false>(smem, smem, tmap, [1, 2], 0x3, 0x1000);
        cp_async_bulk_tensor_2d_g2s_tile::<1, false, true>(smem, smem, tmap, [1, 2], 0x3, 0x1000);
        cp_async_bulk_tensor_2d_g2s_tile::<1, true, false>(smem, smem, tmap, [1, 2], 0x3, 0x1000);
        cp_async_bulk_tensor_2d_g2s_tile::<1, true, true>(smem, smem, tmap, [1, 2], 0x3, 0x1000);
        cp_async_bulk_tensor_2d_g2s_tile::<2, false, false>(smem, smem, tmap, [1, 2], 0x3, 0x1000);
        cp_async_bulk_tensor_2d_g2s_tile::<2, false, true>(smem, smem, tmap, [1, 2], 0x3, 0x1000);
        cp_async_bulk_tensor_2d_g2s_tile::<2, true, false>(smem, smem, tmap, [1, 2], 0x3, 0x1000);
        cp_async_bulk_tensor_2d_g2s_tile::<2, true, true>(smem, smem, tmap, [1, 2], 0x3, 0x1000);
    }
}

// CHECK-LABEL: .entry g2s_tile_3d(
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.L2::cache_hint.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.multicast::cluster.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.L2::cache_hint.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.multicast::cluster.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint.cta_group::2 [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_tile_3d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_3d_g2s_tile::<1, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_tile::<1, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_tile::<1, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_tile::<1, true, true>(smem, smem, tmap, [1, 2, 3], 0x3, 0x1000);
        cp_async_bulk_tensor_3d_g2s_tile::<2, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_tile::<2, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_tile::<2, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_tile::<2, true, true>(smem, smem, tmap, [1, 2, 3], 0x3, 0x1000);
    }
}

// CHECK-LABEL: .entry g2s_tile_4d(
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.L2::cache_hint.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.multicast::cluster.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.L2::cache_hint.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.multicast::cluster.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint.cta_group::2 [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_tile_4d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_4d_g2s_tile::<1, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_tile::<1, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_tile::<1, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_tile::<1, true, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_tile::<2, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_tile::<2, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_tile::<2, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_tile::<2, true, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            0x3,
            0x1000,
        );
    }
}

// CHECK-LABEL: .entry g2s_tile_5d(
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.L2::cache_hint.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.multicast::cluster.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.L2::cache_hint.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.multicast::cluster.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint.cta_group::2 [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_tile_5d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_5d_g2s_tile::<1, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_tile::<1, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_tile::<1, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_tile::<1, true, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_tile::<2, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_tile::<2, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_tile::<2, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_tile::<2, true, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x3,
            0x1000,
        );
    }
}

// CHECK-LABEL: .entry g2s_tile_gather4_2d(
// CHECK: cp.async.bulk.tensor.2d.shared::cluster.global.tile::gather4.mbarrier::complete_tx::bytes [{{.*}}
// CHECK: cp.async.bulk.tensor.2d.shared::cluster.global.tile::gather4.mbarrier::complete_tx::bytes.L2::cache_hint [{{.*}}
// CHECK: cp.async.bulk.tensor.2d.shared::cluster.global.tile::gather4.mbarrier::complete_tx::bytes.multicast::cluster [{{.*}}
// CHECK: cp.async.bulk.tensor.2d.shared::cluster.global.tile::gather4.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint [{{.*}}
// CHECK: cp.async.bulk.tensor.2d.shared::cluster.global.tile::gather4.mbarrier::complete_tx::bytes.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.2d.shared::cluster.global.tile::gather4.mbarrier::complete_tx::bytes.L2::cache_hint.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.2d.shared::cluster.global.tile::gather4.mbarrier::complete_tx::bytes.multicast::cluster.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.2d.shared::cluster.global.tile::gather4.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint.cta_group::1 [{{.*}}
// CHECK: cp.async.bulk.tensor.2d.shared::cluster.global.tile::gather4.mbarrier::complete_tx::bytes.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.2d.shared::cluster.global.tile::gather4.mbarrier::complete_tx::bytes.L2::cache_hint.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.2d.shared::cluster.global.tile::gather4.mbarrier::complete_tx::bytes.multicast::cluster.cta_group::2 [{{.*}}
// CHECK: cp.async.bulk.tensor.2d.shared::cluster.global.tile::gather4.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint.cta_group::2 [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_tile_gather4_2d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_2d_g2s_tile_gather4::<0, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_2d_g2s_tile_gather4::<0, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_2d_g2s_tile_gather4::<0, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_2d_g2s_tile_gather4::<0, true, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_2d_g2s_tile_gather4::<1, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_2d_g2s_tile_gather4::<1, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_2d_g2s_tile_gather4::<1, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_2d_g2s_tile_gather4::<1, true, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_2d_g2s_tile_gather4::<2, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_2d_g2s_tile_gather4::<2, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_2d_g2s_tile_gather4::<2, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_2d_g2s_tile_gather4::<2, true, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x3,
            0x1000,
        );
    }
}

// CHECK-LABEL: .entry s2g_tile_scatter4_2d(
// CHECK: cp.async.bulk.tensor.2d.global.shared::cta.tile::scatter4.bulk_group [{{.*}}
// CHECK: cp.async.bulk.tensor.2d.global.shared::cta.tile::scatter4.bulk_group.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn s2g_tile_scatter4_2d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_2d_s2g_tile_scatter4::<false>(smem, tmap, [1, 2, 3, 4, 5], 0x1000);
        cp_async_bulk_tensor_2d_s2g_tile_scatter4::<true>(smem, tmap, [1, 2, 3, 4, 5], 0x1000);
    }
}
