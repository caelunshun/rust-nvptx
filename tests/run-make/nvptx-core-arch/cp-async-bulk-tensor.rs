// --- LLM-generated --- //
// Checks the PTX emitted for the tensor copy and reduction intrinsics in `core::arch::nvptx`
// that require `sm_90` and PTX ISA 8.0, or PTX ISA 8.6 for the CTA-destination copies.
// nvptx-target: sm_90a +ptx86

#![feature(no_core, stdarch_nvptx, abi_ptx)]
#![no_core]

extern crate core;

use core::arch::nvptx::*;
use core::ffi::c_void;

// CHECK-LABEL: .entry g2s_cta_im2col_3d(
// CHECK: cp.async.bulk.tensor.3d.shared::cta.global.im2col.mbarrier::complete_tx::bytes [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cta.global.im2col.mbarrier::complete_tx::bytes.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_cta_im2col_3d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_3d_g2s_cta_im2col::<false>(smem, smem, tmap, [1, 2, 3], [1], 0x1000);
        cp_async_bulk_tensor_3d_g2s_cta_im2col::<true>(smem, smem, tmap, [1, 2, 3], [1], 0x1000);
    }
}

// CHECK-LABEL: .entry g2s_cta_im2col_4d(
// CHECK: cp.async.bulk.tensor.4d.shared::cta.global.im2col.mbarrier::complete_tx::bytes [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cta.global.im2col.mbarrier::complete_tx::bytes.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_cta_im2col_4d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_4d_g2s_cta_im2col::<false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            [1, 2],
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_cta_im2col::<true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            [1, 2],
            0x1000,
        );
    }
}

// CHECK-LABEL: .entry g2s_cta_im2col_5d(
// CHECK: cp.async.bulk.tensor.5d.shared::cta.global.im2col.mbarrier::complete_tx::bytes [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cta.global.im2col.mbarrier::complete_tx::bytes.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_cta_im2col_5d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_5d_g2s_cta_im2col::<false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            [1, 2, 3],
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_cta_im2col::<true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            [1, 2, 3],
            0x1000,
        );
    }
}

// CHECK-LABEL: .entry g2s_cta_tile_1d(
// CHECK: cp.async.bulk.tensor.1d.shared::cta.global.tile.mbarrier::complete_tx::bytes [{{.*}}
// CHECK: cp.async.bulk.tensor.1d.shared::cta.global.tile.mbarrier::complete_tx::bytes.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_cta_tile_1d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_1d_g2s_cta_tile::<false>(smem, smem, tmap, [1], 0x1000);
        cp_async_bulk_tensor_1d_g2s_cta_tile::<true>(smem, smem, tmap, [1], 0x1000);
    }
}

// CHECK-LABEL: .entry g2s_cta_tile_2d(
// CHECK: cp.async.bulk.tensor.2d.shared::cta.global.tile.mbarrier::complete_tx::bytes [{{.*}}
// CHECK: cp.async.bulk.tensor.2d.shared::cta.global.tile.mbarrier::complete_tx::bytes.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_cta_tile_2d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_2d_g2s_cta_tile::<false>(smem, smem, tmap, [1, 2], 0x1000);
        cp_async_bulk_tensor_2d_g2s_cta_tile::<true>(smem, smem, tmap, [1, 2], 0x1000);
    }
}

// CHECK-LABEL: .entry g2s_cta_tile_3d(
// CHECK: cp.async.bulk.tensor.3d.shared::cta.global.tile.mbarrier::complete_tx::bytes [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cta.global.tile.mbarrier::complete_tx::bytes.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_cta_tile_3d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_3d_g2s_cta_tile::<false>(smem, smem, tmap, [1, 2, 3], 0x1000);
        cp_async_bulk_tensor_3d_g2s_cta_tile::<true>(smem, smem, tmap, [1, 2, 3], 0x1000);
    }
}

// CHECK-LABEL: .entry g2s_cta_tile_4d(
// CHECK: cp.async.bulk.tensor.4d.shared::cta.global.tile.mbarrier::complete_tx::bytes [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cta.global.tile.mbarrier::complete_tx::bytes.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_cta_tile_4d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_4d_g2s_cta_tile::<false>(smem, smem, tmap, [1, 2, 3, 4], 0x1000);
        cp_async_bulk_tensor_4d_g2s_cta_tile::<true>(smem, smem, tmap, [1, 2, 3, 4], 0x1000);
    }
}

// CHECK-LABEL: .entry g2s_cta_tile_5d(
// CHECK: cp.async.bulk.tensor.5d.shared::cta.global.tile.mbarrier::complete_tx::bytes [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cta.global.tile.mbarrier::complete_tx::bytes.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_cta_tile_5d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_5d_g2s_cta_tile::<false>(smem, smem, tmap, [1, 2, 3, 4, 5], 0x1000);
        cp_async_bulk_tensor_5d_g2s_cta_tile::<true>(smem, smem, tmap, [1, 2, 3, 4, 5], 0x1000);
    }
}

// CHECK-LABEL: .entry g2s_im2col_3d(
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.im2col.mbarrier::complete_tx::bytes [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.im2col.mbarrier::complete_tx::bytes.L2::cache_hint [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.im2col.mbarrier::complete_tx::bytes.multicast::cluster [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.im2col.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_im2col_3d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_3d_g2s_im2col::<0, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            [1],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_im2col::<0, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            [1],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_im2col::<0, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            [1],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_im2col::<0, true, true>(
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
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.im2col.mbarrier::complete_tx::bytes [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.im2col.mbarrier::complete_tx::bytes.L2::cache_hint [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.im2col.mbarrier::complete_tx::bytes.multicast::cluster [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.im2col.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_im2col_4d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_4d_g2s_im2col::<0, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            [1, 2],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_im2col::<0, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            [1, 2],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_im2col::<0, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            [1, 2],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_im2col::<0, true, true>(
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
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.im2col.mbarrier::complete_tx::bytes [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.im2col.mbarrier::complete_tx::bytes.L2::cache_hint [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.im2col.mbarrier::complete_tx::bytes.multicast::cluster [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.im2col.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_im2col_5d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_5d_g2s_im2col::<0, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            [1, 2, 3],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_im2col::<0, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            [1, 2, 3],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_im2col::<0, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            [1, 2, 3],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_im2col::<0, true, true>(
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

// CHECK-LABEL: .entry g2s_tile_1d(
// CHECK: cp.async.bulk.tensor.1d.shared::cluster.global.tile.mbarrier::complete_tx::bytes [{{.*}}
// CHECK: cp.async.bulk.tensor.1d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.L2::cache_hint [{{.*}}
// CHECK: cp.async.bulk.tensor.1d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.multicast::cluster [{{.*}}
// CHECK: cp.async.bulk.tensor.1d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_tile_1d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_1d_g2s_tile::<0, false, false>(smem, smem, tmap, [1], 0x3, 0x1000);
        cp_async_bulk_tensor_1d_g2s_tile::<0, false, true>(smem, smem, tmap, [1], 0x3, 0x1000);
        cp_async_bulk_tensor_1d_g2s_tile::<0, true, false>(smem, smem, tmap, [1], 0x3, 0x1000);
        cp_async_bulk_tensor_1d_g2s_tile::<0, true, true>(smem, smem, tmap, [1], 0x3, 0x1000);
    }
}

// CHECK-LABEL: .entry g2s_tile_2d(
// CHECK: cp.async.bulk.tensor.2d.shared::cluster.global.tile.mbarrier::complete_tx::bytes [{{.*}}
// CHECK: cp.async.bulk.tensor.2d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.L2::cache_hint [{{.*}}
// CHECK: cp.async.bulk.tensor.2d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.multicast::cluster [{{.*}}
// CHECK: cp.async.bulk.tensor.2d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_tile_2d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_2d_g2s_tile::<0, false, false>(smem, smem, tmap, [1, 2], 0x3, 0x1000);
        cp_async_bulk_tensor_2d_g2s_tile::<0, false, true>(smem, smem, tmap, [1, 2], 0x3, 0x1000);
        cp_async_bulk_tensor_2d_g2s_tile::<0, true, false>(smem, smem, tmap, [1, 2], 0x3, 0x1000);
        cp_async_bulk_tensor_2d_g2s_tile::<0, true, true>(smem, smem, tmap, [1, 2], 0x3, 0x1000);
    }
}

// CHECK-LABEL: .entry g2s_tile_3d(
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.tile.mbarrier::complete_tx::bytes [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.L2::cache_hint [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.multicast::cluster [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_tile_3d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_3d_g2s_tile::<0, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_tile::<0, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_tile::<0, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_3d_g2s_tile::<0, true, true>(smem, smem, tmap, [1, 2, 3], 0x3, 0x1000);
    }
}

// CHECK-LABEL: .entry g2s_tile_4d(
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.tile.mbarrier::complete_tx::bytes [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.L2::cache_hint [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.multicast::cluster [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_tile_4d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_4d_g2s_tile::<0, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_tile::<0, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_tile::<0, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_4d_g2s_tile::<0, true, true>(
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
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.tile.mbarrier::complete_tx::bytes [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.L2::cache_hint [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.multicast::cluster [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.shared::cluster.global.tile.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn g2s_tile_5d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_5d_g2s_tile::<0, false, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_tile::<0, false, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_tile::<0, true, false>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x3,
            0x1000,
        );
        cp_async_bulk_tensor_5d_g2s_tile::<0, true, true>(
            smem,
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x3,
            0x1000,
        );
    }
}

// CHECK-LABEL: .entry reduce_im2col_3d(
// CHECK: cp.reduce.async.bulk.tensor.3d.global.shared::cta.add.im2col_no_offs.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.3d.global.shared::cta.add.im2col_no_offs.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.3d.global.shared::cta.min.im2col_no_offs.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.3d.global.shared::cta.min.im2col_no_offs.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.3d.global.shared::cta.max.im2col_no_offs.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.3d.global.shared::cta.max.im2col_no_offs.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.3d.global.shared::cta.inc.im2col_no_offs.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.3d.global.shared::cta.inc.im2col_no_offs.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.3d.global.shared::cta.dec.im2col_no_offs.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.3d.global.shared::cta.dec.im2col_no_offs.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.3d.global.shared::cta.and.im2col_no_offs.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.3d.global.shared::cta.and.im2col_no_offs.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.3d.global.shared::cta.or.im2col_no_offs.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.3d.global.shared::cta.or.im2col_no_offs.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.3d.global.shared::cta.xor.im2col_no_offs.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.3d.global.shared::cta.xor.im2col_no_offs.bulk_group.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn reduce_im2col_3d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_3d_reduce_im2col::<{ TensorReduceOp::Add }, false>(
            smem,
            tmap,
            [1, 2, 3],
            0x1000,
        );
        cp_async_bulk_tensor_3d_reduce_im2col::<{ TensorReduceOp::Add }, true>(
            smem,
            tmap,
            [1, 2, 3],
            0x1000,
        );
        cp_async_bulk_tensor_3d_reduce_im2col::<{ TensorReduceOp::Min }, false>(
            smem,
            tmap,
            [1, 2, 3],
            0x1000,
        );
        cp_async_bulk_tensor_3d_reduce_im2col::<{ TensorReduceOp::Min }, true>(
            smem,
            tmap,
            [1, 2, 3],
            0x1000,
        );
        cp_async_bulk_tensor_3d_reduce_im2col::<{ TensorReduceOp::Max }, false>(
            smem,
            tmap,
            [1, 2, 3],
            0x1000,
        );
        cp_async_bulk_tensor_3d_reduce_im2col::<{ TensorReduceOp::Max }, true>(
            smem,
            tmap,
            [1, 2, 3],
            0x1000,
        );
        cp_async_bulk_tensor_3d_reduce_im2col::<{ TensorReduceOp::Inc }, false>(
            smem,
            tmap,
            [1, 2, 3],
            0x1000,
        );
        cp_async_bulk_tensor_3d_reduce_im2col::<{ TensorReduceOp::Inc }, true>(
            smem,
            tmap,
            [1, 2, 3],
            0x1000,
        );
        cp_async_bulk_tensor_3d_reduce_im2col::<{ TensorReduceOp::Dec }, false>(
            smem,
            tmap,
            [1, 2, 3],
            0x1000,
        );
        cp_async_bulk_tensor_3d_reduce_im2col::<{ TensorReduceOp::Dec }, true>(
            smem,
            tmap,
            [1, 2, 3],
            0x1000,
        );
        cp_async_bulk_tensor_3d_reduce_im2col::<{ TensorReduceOp::And }, false>(
            smem,
            tmap,
            [1, 2, 3],
            0x1000,
        );
        cp_async_bulk_tensor_3d_reduce_im2col::<{ TensorReduceOp::And }, true>(
            smem,
            tmap,
            [1, 2, 3],
            0x1000,
        );
        cp_async_bulk_tensor_3d_reduce_im2col::<{ TensorReduceOp::Or }, false>(
            smem,
            tmap,
            [1, 2, 3],
            0x1000,
        );
        cp_async_bulk_tensor_3d_reduce_im2col::<{ TensorReduceOp::Or }, true>(
            smem,
            tmap,
            [1, 2, 3],
            0x1000,
        );
        cp_async_bulk_tensor_3d_reduce_im2col::<{ TensorReduceOp::Xor }, false>(
            smem,
            tmap,
            [1, 2, 3],
            0x1000,
        );
        cp_async_bulk_tensor_3d_reduce_im2col::<{ TensorReduceOp::Xor }, true>(
            smem,
            tmap,
            [1, 2, 3],
            0x1000,
        );
    }
}

// CHECK-LABEL: .entry reduce_im2col_4d(
// CHECK: cp.reduce.async.bulk.tensor.4d.global.shared::cta.add.im2col_no_offs.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.4d.global.shared::cta.add.im2col_no_offs.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.4d.global.shared::cta.min.im2col_no_offs.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.4d.global.shared::cta.min.im2col_no_offs.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.4d.global.shared::cta.max.im2col_no_offs.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.4d.global.shared::cta.max.im2col_no_offs.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.4d.global.shared::cta.inc.im2col_no_offs.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.4d.global.shared::cta.inc.im2col_no_offs.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.4d.global.shared::cta.dec.im2col_no_offs.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.4d.global.shared::cta.dec.im2col_no_offs.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.4d.global.shared::cta.and.im2col_no_offs.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.4d.global.shared::cta.and.im2col_no_offs.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.4d.global.shared::cta.or.im2col_no_offs.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.4d.global.shared::cta.or.im2col_no_offs.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.4d.global.shared::cta.xor.im2col_no_offs.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.4d.global.shared::cta.xor.im2col_no_offs.bulk_group.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn reduce_im2col_4d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_4d_reduce_im2col::<{ TensorReduceOp::Add }, false>(
            smem,
            tmap,
            [1, 2, 3, 4],
            0x1000,
        );
        cp_async_bulk_tensor_4d_reduce_im2col::<{ TensorReduceOp::Add }, true>(
            smem,
            tmap,
            [1, 2, 3, 4],
            0x1000,
        );
        cp_async_bulk_tensor_4d_reduce_im2col::<{ TensorReduceOp::Min }, false>(
            smem,
            tmap,
            [1, 2, 3, 4],
            0x1000,
        );
        cp_async_bulk_tensor_4d_reduce_im2col::<{ TensorReduceOp::Min }, true>(
            smem,
            tmap,
            [1, 2, 3, 4],
            0x1000,
        );
        cp_async_bulk_tensor_4d_reduce_im2col::<{ TensorReduceOp::Max }, false>(
            smem,
            tmap,
            [1, 2, 3, 4],
            0x1000,
        );
        cp_async_bulk_tensor_4d_reduce_im2col::<{ TensorReduceOp::Max }, true>(
            smem,
            tmap,
            [1, 2, 3, 4],
            0x1000,
        );
        cp_async_bulk_tensor_4d_reduce_im2col::<{ TensorReduceOp::Inc }, false>(
            smem,
            tmap,
            [1, 2, 3, 4],
            0x1000,
        );
        cp_async_bulk_tensor_4d_reduce_im2col::<{ TensorReduceOp::Inc }, true>(
            smem,
            tmap,
            [1, 2, 3, 4],
            0x1000,
        );
        cp_async_bulk_tensor_4d_reduce_im2col::<{ TensorReduceOp::Dec }, false>(
            smem,
            tmap,
            [1, 2, 3, 4],
            0x1000,
        );
        cp_async_bulk_tensor_4d_reduce_im2col::<{ TensorReduceOp::Dec }, true>(
            smem,
            tmap,
            [1, 2, 3, 4],
            0x1000,
        );
        cp_async_bulk_tensor_4d_reduce_im2col::<{ TensorReduceOp::And }, false>(
            smem,
            tmap,
            [1, 2, 3, 4],
            0x1000,
        );
        cp_async_bulk_tensor_4d_reduce_im2col::<{ TensorReduceOp::And }, true>(
            smem,
            tmap,
            [1, 2, 3, 4],
            0x1000,
        );
        cp_async_bulk_tensor_4d_reduce_im2col::<{ TensorReduceOp::Or }, false>(
            smem,
            tmap,
            [1, 2, 3, 4],
            0x1000,
        );
        cp_async_bulk_tensor_4d_reduce_im2col::<{ TensorReduceOp::Or }, true>(
            smem,
            tmap,
            [1, 2, 3, 4],
            0x1000,
        );
        cp_async_bulk_tensor_4d_reduce_im2col::<{ TensorReduceOp::Xor }, false>(
            smem,
            tmap,
            [1, 2, 3, 4],
            0x1000,
        );
        cp_async_bulk_tensor_4d_reduce_im2col::<{ TensorReduceOp::Xor }, true>(
            smem,
            tmap,
            [1, 2, 3, 4],
            0x1000,
        );
    }
}

// CHECK-LABEL: .entry reduce_im2col_5d(
// CHECK: cp.reduce.async.bulk.tensor.5d.global.shared::cta.add.im2col_no_offs.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.5d.global.shared::cta.add.im2col_no_offs.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.5d.global.shared::cta.min.im2col_no_offs.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.5d.global.shared::cta.min.im2col_no_offs.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.5d.global.shared::cta.max.im2col_no_offs.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.5d.global.shared::cta.max.im2col_no_offs.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.5d.global.shared::cta.inc.im2col_no_offs.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.5d.global.shared::cta.inc.im2col_no_offs.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.5d.global.shared::cta.dec.im2col_no_offs.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.5d.global.shared::cta.dec.im2col_no_offs.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.5d.global.shared::cta.and.im2col_no_offs.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.5d.global.shared::cta.and.im2col_no_offs.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.5d.global.shared::cta.or.im2col_no_offs.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.5d.global.shared::cta.or.im2col_no_offs.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.5d.global.shared::cta.xor.im2col_no_offs.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.5d.global.shared::cta.xor.im2col_no_offs.bulk_group.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn reduce_im2col_5d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_5d_reduce_im2col::<{ TensorReduceOp::Add }, false>(
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x1000,
        );
        cp_async_bulk_tensor_5d_reduce_im2col::<{ TensorReduceOp::Add }, true>(
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x1000,
        );
        cp_async_bulk_tensor_5d_reduce_im2col::<{ TensorReduceOp::Min }, false>(
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x1000,
        );
        cp_async_bulk_tensor_5d_reduce_im2col::<{ TensorReduceOp::Min }, true>(
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x1000,
        );
        cp_async_bulk_tensor_5d_reduce_im2col::<{ TensorReduceOp::Max }, false>(
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x1000,
        );
        cp_async_bulk_tensor_5d_reduce_im2col::<{ TensorReduceOp::Max }, true>(
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x1000,
        );
        cp_async_bulk_tensor_5d_reduce_im2col::<{ TensorReduceOp::Inc }, false>(
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x1000,
        );
        cp_async_bulk_tensor_5d_reduce_im2col::<{ TensorReduceOp::Inc }, true>(
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x1000,
        );
        cp_async_bulk_tensor_5d_reduce_im2col::<{ TensorReduceOp::Dec }, false>(
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x1000,
        );
        cp_async_bulk_tensor_5d_reduce_im2col::<{ TensorReduceOp::Dec }, true>(
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x1000,
        );
        cp_async_bulk_tensor_5d_reduce_im2col::<{ TensorReduceOp::And }, false>(
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x1000,
        );
        cp_async_bulk_tensor_5d_reduce_im2col::<{ TensorReduceOp::And }, true>(
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x1000,
        );
        cp_async_bulk_tensor_5d_reduce_im2col::<{ TensorReduceOp::Or }, false>(
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x1000,
        );
        cp_async_bulk_tensor_5d_reduce_im2col::<{ TensorReduceOp::Or }, true>(
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x1000,
        );
        cp_async_bulk_tensor_5d_reduce_im2col::<{ TensorReduceOp::Xor }, false>(
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x1000,
        );
        cp_async_bulk_tensor_5d_reduce_im2col::<{ TensorReduceOp::Xor }, true>(
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x1000,
        );
    }
}

// CHECK-LABEL: .entry reduce_tile_1d(
// CHECK: cp.reduce.async.bulk.tensor.1d.global.shared::cta.add.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.1d.global.shared::cta.add.tile.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.1d.global.shared::cta.min.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.1d.global.shared::cta.min.tile.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.1d.global.shared::cta.max.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.1d.global.shared::cta.max.tile.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.1d.global.shared::cta.inc.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.1d.global.shared::cta.inc.tile.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.1d.global.shared::cta.dec.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.1d.global.shared::cta.dec.tile.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.1d.global.shared::cta.and.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.1d.global.shared::cta.and.tile.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.1d.global.shared::cta.or.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.1d.global.shared::cta.or.tile.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.1d.global.shared::cta.xor.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.1d.global.shared::cta.xor.tile.bulk_group.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn reduce_tile_1d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_1d_reduce_tile::<{ TensorReduceOp::Add }, false>(
            smem,
            tmap,
            [1],
            0x1000,
        );
        cp_async_bulk_tensor_1d_reduce_tile::<{ TensorReduceOp::Add }, true>(
            smem,
            tmap,
            [1],
            0x1000,
        );
        cp_async_bulk_tensor_1d_reduce_tile::<{ TensorReduceOp::Min }, false>(
            smem,
            tmap,
            [1],
            0x1000,
        );
        cp_async_bulk_tensor_1d_reduce_tile::<{ TensorReduceOp::Min }, true>(
            smem,
            tmap,
            [1],
            0x1000,
        );
        cp_async_bulk_tensor_1d_reduce_tile::<{ TensorReduceOp::Max }, false>(
            smem,
            tmap,
            [1],
            0x1000,
        );
        cp_async_bulk_tensor_1d_reduce_tile::<{ TensorReduceOp::Max }, true>(
            smem,
            tmap,
            [1],
            0x1000,
        );
        cp_async_bulk_tensor_1d_reduce_tile::<{ TensorReduceOp::Inc }, false>(
            smem,
            tmap,
            [1],
            0x1000,
        );
        cp_async_bulk_tensor_1d_reduce_tile::<{ TensorReduceOp::Inc }, true>(
            smem,
            tmap,
            [1],
            0x1000,
        );
        cp_async_bulk_tensor_1d_reduce_tile::<{ TensorReduceOp::Dec }, false>(
            smem,
            tmap,
            [1],
            0x1000,
        );
        cp_async_bulk_tensor_1d_reduce_tile::<{ TensorReduceOp::Dec }, true>(
            smem,
            tmap,
            [1],
            0x1000,
        );
        cp_async_bulk_tensor_1d_reduce_tile::<{ TensorReduceOp::And }, false>(
            smem,
            tmap,
            [1],
            0x1000,
        );
        cp_async_bulk_tensor_1d_reduce_tile::<{ TensorReduceOp::And }, true>(
            smem,
            tmap,
            [1],
            0x1000,
        );
        cp_async_bulk_tensor_1d_reduce_tile::<{ TensorReduceOp::Or }, false>(
            smem,
            tmap,
            [1],
            0x1000,
        );
        cp_async_bulk_tensor_1d_reduce_tile::<{ TensorReduceOp::Or }, true>(
            smem,
            tmap,
            [1],
            0x1000,
        );
        cp_async_bulk_tensor_1d_reduce_tile::<{ TensorReduceOp::Xor }, false>(
            smem,
            tmap,
            [1],
            0x1000,
        );
        cp_async_bulk_tensor_1d_reduce_tile::<{ TensorReduceOp::Xor }, true>(
            smem,
            tmap,
            [1],
            0x1000,
        );
    }
}

// CHECK-LABEL: .entry reduce_tile_2d(
// CHECK: cp.reduce.async.bulk.tensor.2d.global.shared::cta.add.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.2d.global.shared::cta.add.tile.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.2d.global.shared::cta.min.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.2d.global.shared::cta.min.tile.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.2d.global.shared::cta.max.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.2d.global.shared::cta.max.tile.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.2d.global.shared::cta.inc.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.2d.global.shared::cta.inc.tile.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.2d.global.shared::cta.dec.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.2d.global.shared::cta.dec.tile.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.2d.global.shared::cta.and.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.2d.global.shared::cta.and.tile.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.2d.global.shared::cta.or.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.2d.global.shared::cta.or.tile.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.2d.global.shared::cta.xor.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.2d.global.shared::cta.xor.tile.bulk_group.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn reduce_tile_2d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_2d_reduce_tile::<{ TensorReduceOp::Add }, false>(
            smem,
            tmap,
            [1, 2],
            0x1000,
        );
        cp_async_bulk_tensor_2d_reduce_tile::<{ TensorReduceOp::Add }, true>(
            smem,
            tmap,
            [1, 2],
            0x1000,
        );
        cp_async_bulk_tensor_2d_reduce_tile::<{ TensorReduceOp::Min }, false>(
            smem,
            tmap,
            [1, 2],
            0x1000,
        );
        cp_async_bulk_tensor_2d_reduce_tile::<{ TensorReduceOp::Min }, true>(
            smem,
            tmap,
            [1, 2],
            0x1000,
        );
        cp_async_bulk_tensor_2d_reduce_tile::<{ TensorReduceOp::Max }, false>(
            smem,
            tmap,
            [1, 2],
            0x1000,
        );
        cp_async_bulk_tensor_2d_reduce_tile::<{ TensorReduceOp::Max }, true>(
            smem,
            tmap,
            [1, 2],
            0x1000,
        );
        cp_async_bulk_tensor_2d_reduce_tile::<{ TensorReduceOp::Inc }, false>(
            smem,
            tmap,
            [1, 2],
            0x1000,
        );
        cp_async_bulk_tensor_2d_reduce_tile::<{ TensorReduceOp::Inc }, true>(
            smem,
            tmap,
            [1, 2],
            0x1000,
        );
        cp_async_bulk_tensor_2d_reduce_tile::<{ TensorReduceOp::Dec }, false>(
            smem,
            tmap,
            [1, 2],
            0x1000,
        );
        cp_async_bulk_tensor_2d_reduce_tile::<{ TensorReduceOp::Dec }, true>(
            smem,
            tmap,
            [1, 2],
            0x1000,
        );
        cp_async_bulk_tensor_2d_reduce_tile::<{ TensorReduceOp::And }, false>(
            smem,
            tmap,
            [1, 2],
            0x1000,
        );
        cp_async_bulk_tensor_2d_reduce_tile::<{ TensorReduceOp::And }, true>(
            smem,
            tmap,
            [1, 2],
            0x1000,
        );
        cp_async_bulk_tensor_2d_reduce_tile::<{ TensorReduceOp::Or }, false>(
            smem,
            tmap,
            [1, 2],
            0x1000,
        );
        cp_async_bulk_tensor_2d_reduce_tile::<{ TensorReduceOp::Or }, true>(
            smem,
            tmap,
            [1, 2],
            0x1000,
        );
        cp_async_bulk_tensor_2d_reduce_tile::<{ TensorReduceOp::Xor }, false>(
            smem,
            tmap,
            [1, 2],
            0x1000,
        );
        cp_async_bulk_tensor_2d_reduce_tile::<{ TensorReduceOp::Xor }, true>(
            smem,
            tmap,
            [1, 2],
            0x1000,
        );
    }
}

// CHECK-LABEL: .entry reduce_tile_3d(
// CHECK: cp.reduce.async.bulk.tensor.3d.global.shared::cta.add.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.3d.global.shared::cta.add.tile.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.3d.global.shared::cta.min.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.3d.global.shared::cta.min.tile.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.3d.global.shared::cta.max.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.3d.global.shared::cta.max.tile.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.3d.global.shared::cta.inc.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.3d.global.shared::cta.inc.tile.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.3d.global.shared::cta.dec.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.3d.global.shared::cta.dec.tile.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.3d.global.shared::cta.and.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.3d.global.shared::cta.and.tile.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.3d.global.shared::cta.or.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.3d.global.shared::cta.or.tile.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.3d.global.shared::cta.xor.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.3d.global.shared::cta.xor.tile.bulk_group.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn reduce_tile_3d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_3d_reduce_tile::<{ TensorReduceOp::Add }, false>(
            smem,
            tmap,
            [1, 2, 3],
            0x1000,
        );
        cp_async_bulk_tensor_3d_reduce_tile::<{ TensorReduceOp::Add }, true>(
            smem,
            tmap,
            [1, 2, 3],
            0x1000,
        );
        cp_async_bulk_tensor_3d_reduce_tile::<{ TensorReduceOp::Min }, false>(
            smem,
            tmap,
            [1, 2, 3],
            0x1000,
        );
        cp_async_bulk_tensor_3d_reduce_tile::<{ TensorReduceOp::Min }, true>(
            smem,
            tmap,
            [1, 2, 3],
            0x1000,
        );
        cp_async_bulk_tensor_3d_reduce_tile::<{ TensorReduceOp::Max }, false>(
            smem,
            tmap,
            [1, 2, 3],
            0x1000,
        );
        cp_async_bulk_tensor_3d_reduce_tile::<{ TensorReduceOp::Max }, true>(
            smem,
            tmap,
            [1, 2, 3],
            0x1000,
        );
        cp_async_bulk_tensor_3d_reduce_tile::<{ TensorReduceOp::Inc }, false>(
            smem,
            tmap,
            [1, 2, 3],
            0x1000,
        );
        cp_async_bulk_tensor_3d_reduce_tile::<{ TensorReduceOp::Inc }, true>(
            smem,
            tmap,
            [1, 2, 3],
            0x1000,
        );
        cp_async_bulk_tensor_3d_reduce_tile::<{ TensorReduceOp::Dec }, false>(
            smem,
            tmap,
            [1, 2, 3],
            0x1000,
        );
        cp_async_bulk_tensor_3d_reduce_tile::<{ TensorReduceOp::Dec }, true>(
            smem,
            tmap,
            [1, 2, 3],
            0x1000,
        );
        cp_async_bulk_tensor_3d_reduce_tile::<{ TensorReduceOp::And }, false>(
            smem,
            tmap,
            [1, 2, 3],
            0x1000,
        );
        cp_async_bulk_tensor_3d_reduce_tile::<{ TensorReduceOp::And }, true>(
            smem,
            tmap,
            [1, 2, 3],
            0x1000,
        );
        cp_async_bulk_tensor_3d_reduce_tile::<{ TensorReduceOp::Or }, false>(
            smem,
            tmap,
            [1, 2, 3],
            0x1000,
        );
        cp_async_bulk_tensor_3d_reduce_tile::<{ TensorReduceOp::Or }, true>(
            smem,
            tmap,
            [1, 2, 3],
            0x1000,
        );
        cp_async_bulk_tensor_3d_reduce_tile::<{ TensorReduceOp::Xor }, false>(
            smem,
            tmap,
            [1, 2, 3],
            0x1000,
        );
        cp_async_bulk_tensor_3d_reduce_tile::<{ TensorReduceOp::Xor }, true>(
            smem,
            tmap,
            [1, 2, 3],
            0x1000,
        );
    }
}

// CHECK-LABEL: .entry reduce_tile_4d(
// CHECK: cp.reduce.async.bulk.tensor.4d.global.shared::cta.add.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.4d.global.shared::cta.add.tile.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.4d.global.shared::cta.min.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.4d.global.shared::cta.min.tile.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.4d.global.shared::cta.max.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.4d.global.shared::cta.max.tile.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.4d.global.shared::cta.inc.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.4d.global.shared::cta.inc.tile.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.4d.global.shared::cta.dec.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.4d.global.shared::cta.dec.tile.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.4d.global.shared::cta.and.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.4d.global.shared::cta.and.tile.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.4d.global.shared::cta.or.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.4d.global.shared::cta.or.tile.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.4d.global.shared::cta.xor.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.4d.global.shared::cta.xor.tile.bulk_group.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn reduce_tile_4d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_4d_reduce_tile::<{ TensorReduceOp::Add }, false>(
            smem,
            tmap,
            [1, 2, 3, 4],
            0x1000,
        );
        cp_async_bulk_tensor_4d_reduce_tile::<{ TensorReduceOp::Add }, true>(
            smem,
            tmap,
            [1, 2, 3, 4],
            0x1000,
        );
        cp_async_bulk_tensor_4d_reduce_tile::<{ TensorReduceOp::Min }, false>(
            smem,
            tmap,
            [1, 2, 3, 4],
            0x1000,
        );
        cp_async_bulk_tensor_4d_reduce_tile::<{ TensorReduceOp::Min }, true>(
            smem,
            tmap,
            [1, 2, 3, 4],
            0x1000,
        );
        cp_async_bulk_tensor_4d_reduce_tile::<{ TensorReduceOp::Max }, false>(
            smem,
            tmap,
            [1, 2, 3, 4],
            0x1000,
        );
        cp_async_bulk_tensor_4d_reduce_tile::<{ TensorReduceOp::Max }, true>(
            smem,
            tmap,
            [1, 2, 3, 4],
            0x1000,
        );
        cp_async_bulk_tensor_4d_reduce_tile::<{ TensorReduceOp::Inc }, false>(
            smem,
            tmap,
            [1, 2, 3, 4],
            0x1000,
        );
        cp_async_bulk_tensor_4d_reduce_tile::<{ TensorReduceOp::Inc }, true>(
            smem,
            tmap,
            [1, 2, 3, 4],
            0x1000,
        );
        cp_async_bulk_tensor_4d_reduce_tile::<{ TensorReduceOp::Dec }, false>(
            smem,
            tmap,
            [1, 2, 3, 4],
            0x1000,
        );
        cp_async_bulk_tensor_4d_reduce_tile::<{ TensorReduceOp::Dec }, true>(
            smem,
            tmap,
            [1, 2, 3, 4],
            0x1000,
        );
        cp_async_bulk_tensor_4d_reduce_tile::<{ TensorReduceOp::And }, false>(
            smem,
            tmap,
            [1, 2, 3, 4],
            0x1000,
        );
        cp_async_bulk_tensor_4d_reduce_tile::<{ TensorReduceOp::And }, true>(
            smem,
            tmap,
            [1, 2, 3, 4],
            0x1000,
        );
        cp_async_bulk_tensor_4d_reduce_tile::<{ TensorReduceOp::Or }, false>(
            smem,
            tmap,
            [1, 2, 3, 4],
            0x1000,
        );
        cp_async_bulk_tensor_4d_reduce_tile::<{ TensorReduceOp::Or }, true>(
            smem,
            tmap,
            [1, 2, 3, 4],
            0x1000,
        );
        cp_async_bulk_tensor_4d_reduce_tile::<{ TensorReduceOp::Xor }, false>(
            smem,
            tmap,
            [1, 2, 3, 4],
            0x1000,
        );
        cp_async_bulk_tensor_4d_reduce_tile::<{ TensorReduceOp::Xor }, true>(
            smem,
            tmap,
            [1, 2, 3, 4],
            0x1000,
        );
    }
}

// CHECK-LABEL: .entry reduce_tile_5d(
// CHECK: cp.reduce.async.bulk.tensor.5d.global.shared::cta.add.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.5d.global.shared::cta.add.tile.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.5d.global.shared::cta.min.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.5d.global.shared::cta.min.tile.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.5d.global.shared::cta.max.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.5d.global.shared::cta.max.tile.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.5d.global.shared::cta.inc.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.5d.global.shared::cta.inc.tile.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.5d.global.shared::cta.dec.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.5d.global.shared::cta.dec.tile.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.5d.global.shared::cta.and.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.5d.global.shared::cta.and.tile.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.5d.global.shared::cta.or.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.5d.global.shared::cta.or.tile.bulk_group.L2::cache_hint [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.5d.global.shared::cta.xor.tile.bulk_group [{{.*}}
// CHECK: cp.reduce.async.bulk.tensor.5d.global.shared::cta.xor.tile.bulk_group.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn reduce_tile_5d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_5d_reduce_tile::<{ TensorReduceOp::Add }, false>(
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x1000,
        );
        cp_async_bulk_tensor_5d_reduce_tile::<{ TensorReduceOp::Add }, true>(
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x1000,
        );
        cp_async_bulk_tensor_5d_reduce_tile::<{ TensorReduceOp::Min }, false>(
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x1000,
        );
        cp_async_bulk_tensor_5d_reduce_tile::<{ TensorReduceOp::Min }, true>(
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x1000,
        );
        cp_async_bulk_tensor_5d_reduce_tile::<{ TensorReduceOp::Max }, false>(
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x1000,
        );
        cp_async_bulk_tensor_5d_reduce_tile::<{ TensorReduceOp::Max }, true>(
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x1000,
        );
        cp_async_bulk_tensor_5d_reduce_tile::<{ TensorReduceOp::Inc }, false>(
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x1000,
        );
        cp_async_bulk_tensor_5d_reduce_tile::<{ TensorReduceOp::Inc }, true>(
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x1000,
        );
        cp_async_bulk_tensor_5d_reduce_tile::<{ TensorReduceOp::Dec }, false>(
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x1000,
        );
        cp_async_bulk_tensor_5d_reduce_tile::<{ TensorReduceOp::Dec }, true>(
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x1000,
        );
        cp_async_bulk_tensor_5d_reduce_tile::<{ TensorReduceOp::And }, false>(
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x1000,
        );
        cp_async_bulk_tensor_5d_reduce_tile::<{ TensorReduceOp::And }, true>(
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x1000,
        );
        cp_async_bulk_tensor_5d_reduce_tile::<{ TensorReduceOp::Or }, false>(
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x1000,
        );
        cp_async_bulk_tensor_5d_reduce_tile::<{ TensorReduceOp::Or }, true>(
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x1000,
        );
        cp_async_bulk_tensor_5d_reduce_tile::<{ TensorReduceOp::Xor }, false>(
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x1000,
        );
        cp_async_bulk_tensor_5d_reduce_tile::<{ TensorReduceOp::Xor }, true>(
            smem,
            tmap,
            [1, 2, 3, 4, 5],
            0x1000,
        );
    }
}

// CHECK-LABEL: .entry s2g_im2col_3d(
// CHECK: cp.async.bulk.tensor.3d.global.shared::cta.im2col_no_offs.bulk_group [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.global.shared::cta.im2col_no_offs.bulk_group.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn s2g_im2col_3d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_3d_s2g_im2col::<false>(smem, tmap, [1, 2, 3], 0x1000);
        cp_async_bulk_tensor_3d_s2g_im2col::<true>(smem, tmap, [1, 2, 3], 0x1000);
    }
}

// CHECK-LABEL: .entry s2g_im2col_4d(
// CHECK: cp.async.bulk.tensor.4d.global.shared::cta.im2col_no_offs.bulk_group [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.global.shared::cta.im2col_no_offs.bulk_group.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn s2g_im2col_4d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_4d_s2g_im2col::<false>(smem, tmap, [1, 2, 3, 4], 0x1000);
        cp_async_bulk_tensor_4d_s2g_im2col::<true>(smem, tmap, [1, 2, 3, 4], 0x1000);
    }
}

// CHECK-LABEL: .entry s2g_im2col_5d(
// CHECK: cp.async.bulk.tensor.5d.global.shared::cta.im2col_no_offs.bulk_group [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.global.shared::cta.im2col_no_offs.bulk_group.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn s2g_im2col_5d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_5d_s2g_im2col::<false>(smem, tmap, [1, 2, 3, 4, 5], 0x1000);
        cp_async_bulk_tensor_5d_s2g_im2col::<true>(smem, tmap, [1, 2, 3, 4, 5], 0x1000);
    }
}

// CHECK-LABEL: .entry s2g_tile_1d(
// CHECK: cp.async.bulk.tensor.1d.global.shared::cta.tile.bulk_group [{{.*}}
// CHECK: cp.async.bulk.tensor.1d.global.shared::cta.tile.bulk_group.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn s2g_tile_1d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_1d_s2g_tile::<false>(smem, tmap, [1], 0x1000);
        cp_async_bulk_tensor_1d_s2g_tile::<true>(smem, tmap, [1], 0x1000);
    }
}

// CHECK-LABEL: .entry s2g_tile_2d(
// CHECK: cp.async.bulk.tensor.2d.global.shared::cta.tile.bulk_group [{{.*}}
// CHECK: cp.async.bulk.tensor.2d.global.shared::cta.tile.bulk_group.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn s2g_tile_2d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_2d_s2g_tile::<false>(smem, tmap, [1, 2], 0x1000);
        cp_async_bulk_tensor_2d_s2g_tile::<true>(smem, tmap, [1, 2], 0x1000);
    }
}

// CHECK-LABEL: .entry s2g_tile_3d(
// CHECK: cp.async.bulk.tensor.3d.global.shared::cta.tile.bulk_group [{{.*}}
// CHECK: cp.async.bulk.tensor.3d.global.shared::cta.tile.bulk_group.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn s2g_tile_3d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_3d_s2g_tile::<false>(smem, tmap, [1, 2, 3], 0x1000);
        cp_async_bulk_tensor_3d_s2g_tile::<true>(smem, tmap, [1, 2, 3], 0x1000);
    }
}

// CHECK-LABEL: .entry s2g_tile_4d(
// CHECK: cp.async.bulk.tensor.4d.global.shared::cta.tile.bulk_group [{{.*}}
// CHECK: cp.async.bulk.tensor.4d.global.shared::cta.tile.bulk_group.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn s2g_tile_4d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_4d_s2g_tile::<false>(smem, tmap, [1, 2, 3, 4], 0x1000);
        cp_async_bulk_tensor_4d_s2g_tile::<true>(smem, tmap, [1, 2, 3, 4], 0x1000);
    }
}

// CHECK-LABEL: .entry s2g_tile_5d(
// CHECK: cp.async.bulk.tensor.5d.global.shared::cta.tile.bulk_group [{{.*}}
// CHECK: cp.async.bulk.tensor.5d.global.shared::cta.tile.bulk_group.L2::cache_hint [{{.*}}
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn s2g_tile_5d(
    tmap: *const *const c_void,
    smem: *const *mut c_void,
) {
    unsafe {
        let tmap = *tmap;
        let smem = *smem;
        cp_async_bulk_tensor_5d_s2g_tile::<false>(smem, tmap, [1, 2, 3, 4, 5], 0x1000);
        cp_async_bulk_tensor_5d_s2g_tile::<true>(smem, tmap, [1, 2, 3, 4, 5], 0x1000);
    }
}
