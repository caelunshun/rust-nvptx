// --- LLM-generated --- //
//! Asynchronous bulk tensor copies and reductions (`cp.async.bulk.tensor`, `cp.reduce.async.bulk.tensor`).

use crate::ffi::c_void;
use crate::marker::ConstParamTy;

#[allow(improper_ctypes)]
unsafe extern "llvm-intrinsic" {
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.g2s.tile.1d"]
    fn llvm_cp_async_bulk_tensor_g2s_tile_1d(
        dst: *mut c_void,
        mbar: *mut c_void,
        tmap: *const c_void,
        c0: i32,
        cta_mask: u16,
        cache_policy: u64,
        multicast: bool,
        cache_hint: bool,
        cta_group: i32,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.g2s.tile.2d"]
    fn llvm_cp_async_bulk_tensor_g2s_tile_2d(
        dst: *mut c_void,
        mbar: *mut c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        cta_mask: u16,
        cache_policy: u64,
        multicast: bool,
        cache_hint: bool,
        cta_group: i32,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.g2s.tile.3d"]
    fn llvm_cp_async_bulk_tensor_g2s_tile_3d(
        dst: *mut c_void,
        mbar: *mut c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        cta_mask: u16,
        cache_policy: u64,
        multicast: bool,
        cache_hint: bool,
        cta_group: i32,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.g2s.tile.4d"]
    fn llvm_cp_async_bulk_tensor_g2s_tile_4d(
        dst: *mut c_void,
        mbar: *mut c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        cta_mask: u16,
        cache_policy: u64,
        multicast: bool,
        cache_hint: bool,
        cta_group: i32,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.g2s.tile.5d"]
    fn llvm_cp_async_bulk_tensor_g2s_tile_5d(
        dst: *mut c_void,
        mbar: *mut c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        c4: i32,
        cta_mask: u16,
        cache_policy: u64,
        multicast: bool,
        cache_hint: bool,
        cta_group: i32,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.g2s.tile.gather4.2d"]
    fn llvm_cp_async_bulk_tensor_g2s_tile_gather4_2d(
        dst: *mut c_void,
        mbar: *mut c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        c4: i32,
        cta_mask: u16,
        cache_policy: u64,
        multicast: bool,
        cache_hint: bool,
        cta_group: i32,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.g2s.im2col.3d"]
    fn llvm_cp_async_bulk_tensor_g2s_im2col_3d(
        dst: *mut c_void,
        mbar: *mut c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        offset0: u16,
        cta_mask: u16,
        cache_policy: u64,
        multicast: bool,
        cache_hint: bool,
        cta_group: i32,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.g2s.im2col.4d"]
    fn llvm_cp_async_bulk_tensor_g2s_im2col_4d(
        dst: *mut c_void,
        mbar: *mut c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        offset0: u16,
        offset1: u16,
        cta_mask: u16,
        cache_policy: u64,
        multicast: bool,
        cache_hint: bool,
        cta_group: i32,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.g2s.im2col.5d"]
    fn llvm_cp_async_bulk_tensor_g2s_im2col_5d(
        dst: *mut c_void,
        mbar: *mut c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        c4: i32,
        offset0: u16,
        offset1: u16,
        offset2: u16,
        cta_mask: u16,
        cache_policy: u64,
        multicast: bool,
        cache_hint: bool,
        cta_group: i32,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.g2s.im2col.w.3d"]
    fn llvm_cp_async_bulk_tensor_g2s_im2col_w_3d(
        dst: *mut c_void,
        mbar: *mut c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        w_halo: u16,
        w_offset: u16,
        cta_mask: u16,
        cache_policy: u64,
        multicast: bool,
        cache_hint: bool,
        cta_group: i32,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.g2s.im2col.w.4d"]
    fn llvm_cp_async_bulk_tensor_g2s_im2col_w_4d(
        dst: *mut c_void,
        mbar: *mut c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        w_halo: u16,
        w_offset: u16,
        cta_mask: u16,
        cache_policy: u64,
        multicast: bool,
        cache_hint: bool,
        cta_group: i32,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.g2s.im2col.w.5d"]
    fn llvm_cp_async_bulk_tensor_g2s_im2col_w_5d(
        dst: *mut c_void,
        mbar: *mut c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        c4: i32,
        w_halo: u16,
        w_offset: u16,
        cta_mask: u16,
        cache_policy: u64,
        multicast: bool,
        cache_hint: bool,
        cta_group: i32,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.g2s.im2col.w.128.3d"]
    fn llvm_cp_async_bulk_tensor_g2s_im2col_w_128_3d(
        dst: *mut c_void,
        mbar: *mut c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        w_halo: u16,
        w_offset: u16,
        cta_mask: u16,
        cache_policy: u64,
        multicast: bool,
        cache_hint: bool,
        cta_group: i32,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.g2s.im2col.w.128.4d"]
    fn llvm_cp_async_bulk_tensor_g2s_im2col_w_128_4d(
        dst: *mut c_void,
        mbar: *mut c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        w_halo: u16,
        w_offset: u16,
        cta_mask: u16,
        cache_policy: u64,
        multicast: bool,
        cache_hint: bool,
        cta_group: i32,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.g2s.im2col.w.128.5d"]
    fn llvm_cp_async_bulk_tensor_g2s_im2col_w_128_5d(
        dst: *mut c_void,
        mbar: *mut c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        c4: i32,
        w_halo: u16,
        w_offset: u16,
        cta_mask: u16,
        cache_policy: u64,
        multicast: bool,
        cache_hint: bool,
        cta_group: i32,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.g2s.cta.tile.1d"]
    fn llvm_cp_async_bulk_tensor_g2s_cta_tile_1d(
        dst: *mut c_void,
        mbar: *mut c_void,
        tmap: *const c_void,
        c0: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.g2s.cta.tile.2d"]
    fn llvm_cp_async_bulk_tensor_g2s_cta_tile_2d(
        dst: *mut c_void,
        mbar: *mut c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.g2s.cta.tile.3d"]
    fn llvm_cp_async_bulk_tensor_g2s_cta_tile_3d(
        dst: *mut c_void,
        mbar: *mut c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.g2s.cta.tile.4d"]
    fn llvm_cp_async_bulk_tensor_g2s_cta_tile_4d(
        dst: *mut c_void,
        mbar: *mut c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.g2s.cta.tile.5d"]
    fn llvm_cp_async_bulk_tensor_g2s_cta_tile_5d(
        dst: *mut c_void,
        mbar: *mut c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        c4: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.g2s.cta.tile.gather4.2d"]
    fn llvm_cp_async_bulk_tensor_g2s_cta_tile_gather4_2d(
        dst: *mut c_void,
        mbar: *mut c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        c4: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.g2s.cta.im2col.3d"]
    fn llvm_cp_async_bulk_tensor_g2s_cta_im2col_3d(
        dst: *mut c_void,
        mbar: *mut c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        offset0: u16,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.g2s.cta.im2col.4d"]
    fn llvm_cp_async_bulk_tensor_g2s_cta_im2col_4d(
        dst: *mut c_void,
        mbar: *mut c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        offset0: u16,
        offset1: u16,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.g2s.cta.im2col.5d"]
    fn llvm_cp_async_bulk_tensor_g2s_cta_im2col_5d(
        dst: *mut c_void,
        mbar: *mut c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        c4: i32,
        offset0: u16,
        offset1: u16,
        offset2: u16,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.g2s.cta.im2col.w.3d"]
    fn llvm_cp_async_bulk_tensor_g2s_cta_im2col_w_3d(
        dst: *mut c_void,
        mbar: *mut c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        w_halo: u16,
        w_offset: u16,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.g2s.cta.im2col.w.4d"]
    fn llvm_cp_async_bulk_tensor_g2s_cta_im2col_w_4d(
        dst: *mut c_void,
        mbar: *mut c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        w_halo: u16,
        w_offset: u16,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.g2s.cta.im2col.w.5d"]
    fn llvm_cp_async_bulk_tensor_g2s_cta_im2col_w_5d(
        dst: *mut c_void,
        mbar: *mut c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        c4: i32,
        w_halo: u16,
        w_offset: u16,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.g2s.cta.im2col.w.128.3d"]
    fn llvm_cp_async_bulk_tensor_g2s_cta_im2col_w_128_3d(
        dst: *mut c_void,
        mbar: *mut c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        w_halo: u16,
        w_offset: u16,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.g2s.cta.im2col.w.128.4d"]
    fn llvm_cp_async_bulk_tensor_g2s_cta_im2col_w_128_4d(
        dst: *mut c_void,
        mbar: *mut c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        w_halo: u16,
        w_offset: u16,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.g2s.cta.im2col.w.128.5d"]
    fn llvm_cp_async_bulk_tensor_g2s_cta_im2col_w_128_5d(
        dst: *mut c_void,
        mbar: *mut c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        c4: i32,
        w_halo: u16,
        w_offset: u16,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.s2g.tile.1d"]
    fn llvm_cp_async_bulk_tensor_s2g_tile_1d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.s2g.tile.2d"]
    fn llvm_cp_async_bulk_tensor_s2g_tile_2d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.s2g.tile.3d"]
    fn llvm_cp_async_bulk_tensor_s2g_tile_3d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.s2g.tile.4d"]
    fn llvm_cp_async_bulk_tensor_s2g_tile_4d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.s2g.tile.5d"]
    fn llvm_cp_async_bulk_tensor_s2g_tile_5d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        c4: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.s2g.im2col.3d"]
    fn llvm_cp_async_bulk_tensor_s2g_im2col_3d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.s2g.im2col.4d"]
    fn llvm_cp_async_bulk_tensor_s2g_im2col_4d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.s2g.im2col.5d"]
    fn llvm_cp_async_bulk_tensor_s2g_im2col_5d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        c4: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.s2g.tile.scatter4.2d"]
    fn llvm_cp_async_bulk_tensor_s2g_tile_scatter4_2d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        c4: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.add.tile.1d"]
    fn llvm_cp_async_bulk_tensor_reduce_add_tile_1d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.min.tile.1d"]
    fn llvm_cp_async_bulk_tensor_reduce_min_tile_1d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.max.tile.1d"]
    fn llvm_cp_async_bulk_tensor_reduce_max_tile_1d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.inc.tile.1d"]
    fn llvm_cp_async_bulk_tensor_reduce_inc_tile_1d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.dec.tile.1d"]
    fn llvm_cp_async_bulk_tensor_reduce_dec_tile_1d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.and.tile.1d"]
    fn llvm_cp_async_bulk_tensor_reduce_and_tile_1d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.or.tile.1d"]
    fn llvm_cp_async_bulk_tensor_reduce_or_tile_1d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.xor.tile.1d"]
    fn llvm_cp_async_bulk_tensor_reduce_xor_tile_1d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.add.tile.2d"]
    fn llvm_cp_async_bulk_tensor_reduce_add_tile_2d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.min.tile.2d"]
    fn llvm_cp_async_bulk_tensor_reduce_min_tile_2d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.max.tile.2d"]
    fn llvm_cp_async_bulk_tensor_reduce_max_tile_2d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.inc.tile.2d"]
    fn llvm_cp_async_bulk_tensor_reduce_inc_tile_2d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.dec.tile.2d"]
    fn llvm_cp_async_bulk_tensor_reduce_dec_tile_2d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.and.tile.2d"]
    fn llvm_cp_async_bulk_tensor_reduce_and_tile_2d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.or.tile.2d"]
    fn llvm_cp_async_bulk_tensor_reduce_or_tile_2d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.xor.tile.2d"]
    fn llvm_cp_async_bulk_tensor_reduce_xor_tile_2d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.add.tile.3d"]
    fn llvm_cp_async_bulk_tensor_reduce_add_tile_3d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.min.tile.3d"]
    fn llvm_cp_async_bulk_tensor_reduce_min_tile_3d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.max.tile.3d"]
    fn llvm_cp_async_bulk_tensor_reduce_max_tile_3d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.inc.tile.3d"]
    fn llvm_cp_async_bulk_tensor_reduce_inc_tile_3d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.dec.tile.3d"]
    fn llvm_cp_async_bulk_tensor_reduce_dec_tile_3d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.and.tile.3d"]
    fn llvm_cp_async_bulk_tensor_reduce_and_tile_3d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.or.tile.3d"]
    fn llvm_cp_async_bulk_tensor_reduce_or_tile_3d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.xor.tile.3d"]
    fn llvm_cp_async_bulk_tensor_reduce_xor_tile_3d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.add.tile.4d"]
    fn llvm_cp_async_bulk_tensor_reduce_add_tile_4d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.min.tile.4d"]
    fn llvm_cp_async_bulk_tensor_reduce_min_tile_4d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.max.tile.4d"]
    fn llvm_cp_async_bulk_tensor_reduce_max_tile_4d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.inc.tile.4d"]
    fn llvm_cp_async_bulk_tensor_reduce_inc_tile_4d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.dec.tile.4d"]
    fn llvm_cp_async_bulk_tensor_reduce_dec_tile_4d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.and.tile.4d"]
    fn llvm_cp_async_bulk_tensor_reduce_and_tile_4d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.or.tile.4d"]
    fn llvm_cp_async_bulk_tensor_reduce_or_tile_4d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.xor.tile.4d"]
    fn llvm_cp_async_bulk_tensor_reduce_xor_tile_4d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.add.tile.5d"]
    fn llvm_cp_async_bulk_tensor_reduce_add_tile_5d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        c4: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.min.tile.5d"]
    fn llvm_cp_async_bulk_tensor_reduce_min_tile_5d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        c4: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.max.tile.5d"]
    fn llvm_cp_async_bulk_tensor_reduce_max_tile_5d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        c4: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.inc.tile.5d"]
    fn llvm_cp_async_bulk_tensor_reduce_inc_tile_5d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        c4: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.dec.tile.5d"]
    fn llvm_cp_async_bulk_tensor_reduce_dec_tile_5d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        c4: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.and.tile.5d"]
    fn llvm_cp_async_bulk_tensor_reduce_and_tile_5d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        c4: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.or.tile.5d"]
    fn llvm_cp_async_bulk_tensor_reduce_or_tile_5d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        c4: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.xor.tile.5d"]
    fn llvm_cp_async_bulk_tensor_reduce_xor_tile_5d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        c4: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.add.im2col.3d"]
    fn llvm_cp_async_bulk_tensor_reduce_add_im2col_3d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.min.im2col.3d"]
    fn llvm_cp_async_bulk_tensor_reduce_min_im2col_3d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.max.im2col.3d"]
    fn llvm_cp_async_bulk_tensor_reduce_max_im2col_3d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.inc.im2col.3d"]
    fn llvm_cp_async_bulk_tensor_reduce_inc_im2col_3d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.dec.im2col.3d"]
    fn llvm_cp_async_bulk_tensor_reduce_dec_im2col_3d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.and.im2col.3d"]
    fn llvm_cp_async_bulk_tensor_reduce_and_im2col_3d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.or.im2col.3d"]
    fn llvm_cp_async_bulk_tensor_reduce_or_im2col_3d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.xor.im2col.3d"]
    fn llvm_cp_async_bulk_tensor_reduce_xor_im2col_3d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.add.im2col.4d"]
    fn llvm_cp_async_bulk_tensor_reduce_add_im2col_4d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.min.im2col.4d"]
    fn llvm_cp_async_bulk_tensor_reduce_min_im2col_4d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.max.im2col.4d"]
    fn llvm_cp_async_bulk_tensor_reduce_max_im2col_4d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.inc.im2col.4d"]
    fn llvm_cp_async_bulk_tensor_reduce_inc_im2col_4d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.dec.im2col.4d"]
    fn llvm_cp_async_bulk_tensor_reduce_dec_im2col_4d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.and.im2col.4d"]
    fn llvm_cp_async_bulk_tensor_reduce_and_im2col_4d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.or.im2col.4d"]
    fn llvm_cp_async_bulk_tensor_reduce_or_im2col_4d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.xor.im2col.4d"]
    fn llvm_cp_async_bulk_tensor_reduce_xor_im2col_4d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.add.im2col.5d"]
    fn llvm_cp_async_bulk_tensor_reduce_add_im2col_5d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        c4: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.min.im2col.5d"]
    fn llvm_cp_async_bulk_tensor_reduce_min_im2col_5d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        c4: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.max.im2col.5d"]
    fn llvm_cp_async_bulk_tensor_reduce_max_im2col_5d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        c4: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.inc.im2col.5d"]
    fn llvm_cp_async_bulk_tensor_reduce_inc_im2col_5d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        c4: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.dec.im2col.5d"]
    fn llvm_cp_async_bulk_tensor_reduce_dec_im2col_5d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        c4: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.and.im2col.5d"]
    fn llvm_cp_async_bulk_tensor_reduce_and_im2col_5d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        c4: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.or.im2col.5d"]
    fn llvm_cp_async_bulk_tensor_reduce_or_im2col_5d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        c4: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.reduce.xor.im2col.5d"]
    fn llvm_cp_async_bulk_tensor_reduce_xor_im2col_5d(
        src: *const c_void,
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        c4: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
}

/// Reduction operation of the `cp.reduce.async.bulk.tensor` functions (`.redOp`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, ConstParamTy)]
#[non_exhaustive]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub enum TensorReduceOp {
    /// Addition (`.add`).
    Add,
    /// Minimum (`.min`).
    Min,
    /// Maximum (`.max`).
    Max,
    /// Wrapping increment (`.inc`).
    Inc,
    /// Wrapping decrement (`.dec`).
    Dec,
    /// Bitwise and (`.and`).
    And,
    /// Bitwise or (`.or`).
    Or,
    /// Bitwise xor (`.xor`).
    Xor,
}

/// Asynchronously copies the tile of the 1D tensor described by the tensor map `tmap` at the
/// coordinates `coords` from global memory to shared memory at `dst`, and performs a complete
/// transaction on the mbarrier at `mbar`.
///
/// `dst` must fall within the `.shared::cluster` state space, `mbar` within `.shared::cta`, and
/// `tmap` within `.const`, `.param` or `.global`. If `MULTICAST` is `false`, `cta_mask` is ignored.
/// Otherwise, the copy is multicast to the CTAs of the cluster selected by `cta_mask`. If
/// `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is the L2 cache
/// eviction policy used for the copy (`.L2::cache_hint`).
///
/// `CTA_GROUP` is the `.cta_group` modifier: `0` for none, `1` for `.cta_group::1` or `2` for
/// `.cta_group::2`.
///
/// Requires `sm_90` and PTX ISA 8.0 if `CTA_GROUP` is `0`, otherwise `sm_100a`/`sm_101a` with PTX
/// ISA 8.6, or `sm_100f`/`sm_101f` with PTX ISA 8.8, or `sm_100f`/`sm_110f` with PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_1d_g2s_tile<
    const CTA_GROUP: i32,
    const MULTICAST: bool,
    const CACHE_HINT: bool,
>(
    dst: *mut c_void,
    mbar: *mut c_void,
    tmap: *const c_void,
    coords: [i32; 1],
    cta_mask: u16,
    cache_policy: u64,
) {
    static_assert!(matches!(CTA_GROUP, 0..=2), "CTA_GROUP must be 0, 1 or 2");
    llvm_cp_async_bulk_tensor_g2s_tile_1d(
        dst,
        mbar,
        tmap,
        coords[0],
        cta_mask,
        cache_policy,
        MULTICAST,
        CACHE_HINT,
        CTA_GROUP,
    )
}

/// Asynchronously copies the tile of the 2D tensor described by the tensor map `tmap` at the
/// coordinates `coords` from global memory to shared memory at `dst`, and performs a complete
/// transaction on the mbarrier at `mbar`.
///
/// `dst` must fall within the `.shared::cluster` state space, `mbar` within `.shared::cta`, and
/// `tmap` within `.const`, `.param` or `.global`. If `MULTICAST` is `false`, `cta_mask` is ignored.
/// Otherwise, the copy is multicast to the CTAs of the cluster selected by `cta_mask`. If
/// `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is the L2 cache
/// eviction policy used for the copy (`.L2::cache_hint`).
///
/// `CTA_GROUP` is the `.cta_group` modifier: `0` for none, `1` for `.cta_group::1` or `2` for
/// `.cta_group::2`.
///
/// Requires `sm_90` and PTX ISA 8.0 if `CTA_GROUP` is `0`, otherwise `sm_100a`/`sm_101a` with PTX
/// ISA 8.6, or `sm_100f`/`sm_101f` with PTX ISA 8.8, or `sm_100f`/`sm_110f` with PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_2d_g2s_tile<
    const CTA_GROUP: i32,
    const MULTICAST: bool,
    const CACHE_HINT: bool,
>(
    dst: *mut c_void,
    mbar: *mut c_void,
    tmap: *const c_void,
    coords: [i32; 2],
    cta_mask: u16,
    cache_policy: u64,
) {
    static_assert!(matches!(CTA_GROUP, 0..=2), "CTA_GROUP must be 0, 1 or 2");
    llvm_cp_async_bulk_tensor_g2s_tile_2d(
        dst,
        mbar,
        tmap,
        coords[0],
        coords[1],
        cta_mask,
        cache_policy,
        MULTICAST,
        CACHE_HINT,
        CTA_GROUP,
    )
}

/// Asynchronously copies the tile of the 3D tensor described by the tensor map `tmap` at the
/// coordinates `coords` from global memory to shared memory at `dst`, and performs a complete
/// transaction on the mbarrier at `mbar`.
///
/// `dst` must fall within the `.shared::cluster` state space, `mbar` within `.shared::cta`, and
/// `tmap` within `.const`, `.param` or `.global`. If `MULTICAST` is `false`, `cta_mask` is ignored.
/// Otherwise, the copy is multicast to the CTAs of the cluster selected by `cta_mask`. If
/// `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is the L2 cache
/// eviction policy used for the copy (`.L2::cache_hint`).
///
/// `CTA_GROUP` is the `.cta_group` modifier: `0` for none, `1` for `.cta_group::1` or `2` for
/// `.cta_group::2`.
///
/// Requires `sm_90` and PTX ISA 8.0 if `CTA_GROUP` is `0`, otherwise `sm_100a`/`sm_101a` with PTX
/// ISA 8.6, or `sm_100f`/`sm_101f` with PTX ISA 8.8, or `sm_100f`/`sm_110f` with PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_3d_g2s_tile<
    const CTA_GROUP: i32,
    const MULTICAST: bool,
    const CACHE_HINT: bool,
>(
    dst: *mut c_void,
    mbar: *mut c_void,
    tmap: *const c_void,
    coords: [i32; 3],
    cta_mask: u16,
    cache_policy: u64,
) {
    static_assert!(matches!(CTA_GROUP, 0..=2), "CTA_GROUP must be 0, 1 or 2");
    llvm_cp_async_bulk_tensor_g2s_tile_3d(
        dst,
        mbar,
        tmap,
        coords[0],
        coords[1],
        coords[2],
        cta_mask,
        cache_policy,
        MULTICAST,
        CACHE_HINT,
        CTA_GROUP,
    )
}

/// Asynchronously copies the tile of the 4D tensor described by the tensor map `tmap` at the
/// coordinates `coords` from global memory to shared memory at `dst`, and performs a complete
/// transaction on the mbarrier at `mbar`.
///
/// `dst` must fall within the `.shared::cluster` state space, `mbar` within `.shared::cta`, and
/// `tmap` within `.const`, `.param` or `.global`. If `MULTICAST` is `false`, `cta_mask` is ignored.
/// Otherwise, the copy is multicast to the CTAs of the cluster selected by `cta_mask`. If
/// `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is the L2 cache
/// eviction policy used for the copy (`.L2::cache_hint`).
///
/// `CTA_GROUP` is the `.cta_group` modifier: `0` for none, `1` for `.cta_group::1` or `2` for
/// `.cta_group::2`.
///
/// Requires `sm_90` and PTX ISA 8.0 if `CTA_GROUP` is `0`, otherwise `sm_100a`/`sm_101a` with PTX
/// ISA 8.6, or `sm_100f`/`sm_101f` with PTX ISA 8.8, or `sm_100f`/`sm_110f` with PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_4d_g2s_tile<
    const CTA_GROUP: i32,
    const MULTICAST: bool,
    const CACHE_HINT: bool,
>(
    dst: *mut c_void,
    mbar: *mut c_void,
    tmap: *const c_void,
    coords: [i32; 4],
    cta_mask: u16,
    cache_policy: u64,
) {
    static_assert!(matches!(CTA_GROUP, 0..=2), "CTA_GROUP must be 0, 1 or 2");
    llvm_cp_async_bulk_tensor_g2s_tile_4d(
        dst,
        mbar,
        tmap,
        coords[0],
        coords[1],
        coords[2],
        coords[3],
        cta_mask,
        cache_policy,
        MULTICAST,
        CACHE_HINT,
        CTA_GROUP,
    )
}

/// Asynchronously copies the tile of the 5D tensor described by the tensor map `tmap` at the
/// coordinates `coords` from global memory to shared memory at `dst`, and performs a complete
/// transaction on the mbarrier at `mbar`.
///
/// `dst` must fall within the `.shared::cluster` state space, `mbar` within `.shared::cta`, and
/// `tmap` within `.const`, `.param` or `.global`. If `MULTICAST` is `false`, `cta_mask` is ignored.
/// Otherwise, the copy is multicast to the CTAs of the cluster selected by `cta_mask`. If
/// `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is the L2 cache
/// eviction policy used for the copy (`.L2::cache_hint`).
///
/// `CTA_GROUP` is the `.cta_group` modifier: `0` for none, `1` for `.cta_group::1` or `2` for
/// `.cta_group::2`.
///
/// Requires `sm_90` and PTX ISA 8.0 if `CTA_GROUP` is `0`, otherwise `sm_100a`/`sm_101a` with PTX
/// ISA 8.6, or `sm_100f`/`sm_101f` with PTX ISA 8.8, or `sm_100f`/`sm_110f` with PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_5d_g2s_tile<
    const CTA_GROUP: i32,
    const MULTICAST: bool,
    const CACHE_HINT: bool,
>(
    dst: *mut c_void,
    mbar: *mut c_void,
    tmap: *const c_void,
    coords: [i32; 5],
    cta_mask: u16,
    cache_policy: u64,
) {
    static_assert!(matches!(CTA_GROUP, 0..=2), "CTA_GROUP must be 0, 1 or 2");
    llvm_cp_async_bulk_tensor_g2s_tile_5d(
        dst,
        mbar,
        tmap,
        coords[0],
        coords[1],
        coords[2],
        coords[3],
        coords[4],
        cta_mask,
        cache_policy,
        MULTICAST,
        CACHE_HINT,
        CTA_GROUP,
    )
}

/// Asynchronously copies four rows of the 2D tensor described by the tensor map `tmap`, selected by
/// the coordinates `coords` (the first is the column, the others the rows), from global memory to
/// shared memory at `dst`, and performs a complete transaction on the mbarrier at `mbar`.
///
/// `dst` must fall within the `.shared::cluster` state space, `mbar` within `.shared::cta`, and
/// `tmap` within `.const`, `.param` or `.global`. If `MULTICAST` is `false`, `cta_mask` is ignored.
/// Otherwise, the copy is multicast to the CTAs of the cluster selected by `cta_mask`. If
/// `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is the L2 cache
/// eviction policy used for the copy (`.L2::cache_hint`).
///
/// `CTA_GROUP` is the `.cta_group` modifier: `0` for none, `1` for `.cta_group::1` or `2` for
/// `.cta_group::2`.
///
/// Requires `sm_100a`/`sm_101a` with PTX ISA 8.6, or `sm_100f`/`sm_101f` with PTX ISA 8.8, or
/// `sm_100f`/`sm_110f` with PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_2d_g2s_tile_gather4<
    const CTA_GROUP: i32,
    const MULTICAST: bool,
    const CACHE_HINT: bool,
>(
    dst: *mut c_void,
    mbar: *mut c_void,
    tmap: *const c_void,
    coords: [i32; 5],
    cta_mask: u16,
    cache_policy: u64,
) {
    static_assert!(matches!(CTA_GROUP, 0..=2), "CTA_GROUP must be 0, 1 or 2");
    llvm_cp_async_bulk_tensor_g2s_tile_gather4_2d(
        dst,
        mbar,
        tmap,
        coords[0],
        coords[1],
        coords[2],
        coords[3],
        coords[4],
        cta_mask,
        cache_policy,
        MULTICAST,
        CACHE_HINT,
        CTA_GROUP,
    )
}

/// Asynchronously copies the 3D im2col-layout tensor data described by the tensor map `tmap` at the
/// coordinates `coords` from global memory to shared memory at `dst`, and performs a complete
/// transaction on the mbarrier at `mbar`.
///
/// `dst` must fall within the `.shared::cluster` state space, `mbar` within `.shared::cta`, and
/// `tmap` within `.const`, `.param` or `.global`. If `MULTICAST` is `false`, `cta_mask` is ignored.
/// Otherwise, the copy is multicast to the CTAs of the cluster selected by `cta_mask`. If
/// `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is the L2 cache
/// eviction policy used for the copy (`.L2::cache_hint`).
///
/// `CTA_GROUP` is the `.cta_group` modifier: `0` for none, `1` for `.cta_group::1` or `2` for
/// `.cta_group::2`.
///
/// Requires `sm_90` and PTX ISA 8.0 if `CTA_GROUP` is `0`, otherwise `sm_100a`/`sm_101a` with PTX
/// ISA 8.6, or `sm_100f`/`sm_101f` with PTX ISA 8.8, or `sm_100f`/`sm_110f` with PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_3d_g2s_im2col<
    const CTA_GROUP: i32,
    const MULTICAST: bool,
    const CACHE_HINT: bool,
>(
    dst: *mut c_void,
    mbar: *mut c_void,
    tmap: *const c_void,
    coords: [i32; 3],
    im2col_offsets: [u16; 1],
    cta_mask: u16,
    cache_policy: u64,
) {
    static_assert!(matches!(CTA_GROUP, 0..=2), "CTA_GROUP must be 0, 1 or 2");
    llvm_cp_async_bulk_tensor_g2s_im2col_3d(
        dst,
        mbar,
        tmap,
        coords[0],
        coords[1],
        coords[2],
        im2col_offsets[0],
        cta_mask,
        cache_policy,
        MULTICAST,
        CACHE_HINT,
        CTA_GROUP,
    )
}

/// Asynchronously copies the 4D im2col-layout tensor data described by the tensor map `tmap` at the
/// coordinates `coords` from global memory to shared memory at `dst`, and performs a complete
/// transaction on the mbarrier at `mbar`.
///
/// `dst` must fall within the `.shared::cluster` state space, `mbar` within `.shared::cta`, and
/// `tmap` within `.const`, `.param` or `.global`. If `MULTICAST` is `false`, `cta_mask` is ignored.
/// Otherwise, the copy is multicast to the CTAs of the cluster selected by `cta_mask`. If
/// `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is the L2 cache
/// eviction policy used for the copy (`.L2::cache_hint`).
///
/// `CTA_GROUP` is the `.cta_group` modifier: `0` for none, `1` for `.cta_group::1` or `2` for
/// `.cta_group::2`.
///
/// Requires `sm_90` and PTX ISA 8.0 if `CTA_GROUP` is `0`, otherwise `sm_100a`/`sm_101a` with PTX
/// ISA 8.6, or `sm_100f`/`sm_101f` with PTX ISA 8.8, or `sm_100f`/`sm_110f` with PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_4d_g2s_im2col<
    const CTA_GROUP: i32,
    const MULTICAST: bool,
    const CACHE_HINT: bool,
>(
    dst: *mut c_void,
    mbar: *mut c_void,
    tmap: *const c_void,
    coords: [i32; 4],
    im2col_offsets: [u16; 2],
    cta_mask: u16,
    cache_policy: u64,
) {
    static_assert!(matches!(CTA_GROUP, 0..=2), "CTA_GROUP must be 0, 1 or 2");
    llvm_cp_async_bulk_tensor_g2s_im2col_4d(
        dst,
        mbar,
        tmap,
        coords[0],
        coords[1],
        coords[2],
        coords[3],
        im2col_offsets[0],
        im2col_offsets[1],
        cta_mask,
        cache_policy,
        MULTICAST,
        CACHE_HINT,
        CTA_GROUP,
    )
}

/// Asynchronously copies the 5D im2col-layout tensor data described by the tensor map `tmap` at the
/// coordinates `coords` from global memory to shared memory at `dst`, and performs a complete
/// transaction on the mbarrier at `mbar`.
///
/// `dst` must fall within the `.shared::cluster` state space, `mbar` within `.shared::cta`, and
/// `tmap` within `.const`, `.param` or `.global`. If `MULTICAST` is `false`, `cta_mask` is ignored.
/// Otherwise, the copy is multicast to the CTAs of the cluster selected by `cta_mask`. If
/// `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is the L2 cache
/// eviction policy used for the copy (`.L2::cache_hint`).
///
/// `CTA_GROUP` is the `.cta_group` modifier: `0` for none, `1` for `.cta_group::1` or `2` for
/// `.cta_group::2`.
///
/// Requires `sm_90` and PTX ISA 8.0 if `CTA_GROUP` is `0`, otherwise `sm_100a`/`sm_101a` with PTX
/// ISA 8.6, or `sm_100f`/`sm_101f` with PTX ISA 8.8, or `sm_100f`/`sm_110f` with PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_5d_g2s_im2col<
    const CTA_GROUP: i32,
    const MULTICAST: bool,
    const CACHE_HINT: bool,
>(
    dst: *mut c_void,
    mbar: *mut c_void,
    tmap: *const c_void,
    coords: [i32; 5],
    im2col_offsets: [u16; 3],
    cta_mask: u16,
    cache_policy: u64,
) {
    static_assert!(matches!(CTA_GROUP, 0..=2), "CTA_GROUP must be 0, 1 or 2");
    llvm_cp_async_bulk_tensor_g2s_im2col_5d(
        dst,
        mbar,
        tmap,
        coords[0],
        coords[1],
        coords[2],
        coords[3],
        coords[4],
        im2col_offsets[0],
        im2col_offsets[1],
        im2col_offsets[2],
        cta_mask,
        cache_policy,
        MULTICAST,
        CACHE_HINT,
        CTA_GROUP,
    )
}

/// Asynchronously copies the 3D im2col::w-layout tensor data described by the tensor map `tmap` at
/// the coordinates `coords` from global memory to shared memory at `dst`, and performs a complete
/// transaction on the mbarrier at `mbar`.
///
/// `dst` must fall within the `.shared::cluster` state space, `mbar` within `.shared::cta`, and
/// `tmap` within `.const`, `.param` or `.global`. If `MULTICAST` is `false`, `cta_mask` is ignored.
/// Otherwise, the copy is multicast to the CTAs of the cluster selected by `cta_mask`. If
/// `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is the L2 cache
/// eviction policy used for the copy (`.L2::cache_hint`).
///
/// `CTA_GROUP` is the `.cta_group` modifier: `0` for none, `1` for `.cta_group::1` or `2` for
/// `.cta_group::2`.
///
/// Requires `sm_100a`/`sm_101a` with PTX ISA 8.6, or `sm_100f`/`sm_101f` with PTX ISA 8.8, or
/// `sm_100f`/`sm_110f` with PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_3d_g2s_im2col_w<
    const CTA_GROUP: i32,
    const MULTICAST: bool,
    const CACHE_HINT: bool,
>(
    dst: *mut c_void,
    mbar: *mut c_void,
    tmap: *const c_void,
    coords: [i32; 3],
    w_halo: u16,
    w_offset: u16,
    cta_mask: u16,
    cache_policy: u64,
) {
    static_assert!(matches!(CTA_GROUP, 0..=2), "CTA_GROUP must be 0, 1 or 2");
    llvm_cp_async_bulk_tensor_g2s_im2col_w_3d(
        dst,
        mbar,
        tmap,
        coords[0],
        coords[1],
        coords[2],
        w_halo,
        w_offset,
        cta_mask,
        cache_policy,
        MULTICAST,
        CACHE_HINT,
        CTA_GROUP,
    )
}

/// Asynchronously copies the 4D im2col::w-layout tensor data described by the tensor map `tmap` at
/// the coordinates `coords` from global memory to shared memory at `dst`, and performs a complete
/// transaction on the mbarrier at `mbar`.
///
/// `dst` must fall within the `.shared::cluster` state space, `mbar` within `.shared::cta`, and
/// `tmap` within `.const`, `.param` or `.global`. If `MULTICAST` is `false`, `cta_mask` is ignored.
/// Otherwise, the copy is multicast to the CTAs of the cluster selected by `cta_mask`. If
/// `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is the L2 cache
/// eviction policy used for the copy (`.L2::cache_hint`).
///
/// `CTA_GROUP` is the `.cta_group` modifier: `0` for none, `1` for `.cta_group::1` or `2` for
/// `.cta_group::2`.
///
/// Requires `sm_100a`/`sm_101a` with PTX ISA 8.6, or `sm_100f`/`sm_101f` with PTX ISA 8.8, or
/// `sm_100f`/`sm_110f` with PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_4d_g2s_im2col_w<
    const CTA_GROUP: i32,
    const MULTICAST: bool,
    const CACHE_HINT: bool,
>(
    dst: *mut c_void,
    mbar: *mut c_void,
    tmap: *const c_void,
    coords: [i32; 4],
    w_halo: u16,
    w_offset: u16,
    cta_mask: u16,
    cache_policy: u64,
) {
    static_assert!(matches!(CTA_GROUP, 0..=2), "CTA_GROUP must be 0, 1 or 2");
    llvm_cp_async_bulk_tensor_g2s_im2col_w_4d(
        dst,
        mbar,
        tmap,
        coords[0],
        coords[1],
        coords[2],
        coords[3],
        w_halo,
        w_offset,
        cta_mask,
        cache_policy,
        MULTICAST,
        CACHE_HINT,
        CTA_GROUP,
    )
}

/// Asynchronously copies the 5D im2col::w-layout tensor data described by the tensor map `tmap` at
/// the coordinates `coords` from global memory to shared memory at `dst`, and performs a complete
/// transaction on the mbarrier at `mbar`.
///
/// `dst` must fall within the `.shared::cluster` state space, `mbar` within `.shared::cta`, and
/// `tmap` within `.const`, `.param` or `.global`. If `MULTICAST` is `false`, `cta_mask` is ignored.
/// Otherwise, the copy is multicast to the CTAs of the cluster selected by `cta_mask`. If
/// `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is the L2 cache
/// eviction policy used for the copy (`.L2::cache_hint`).
///
/// `CTA_GROUP` is the `.cta_group` modifier: `0` for none, `1` for `.cta_group::1` or `2` for
/// `.cta_group::2`.
///
/// Requires `sm_100a`/`sm_101a` with PTX ISA 8.6, or `sm_100f`/`sm_101f` with PTX ISA 8.8, or
/// `sm_100f`/`sm_110f` with PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_5d_g2s_im2col_w<
    const CTA_GROUP: i32,
    const MULTICAST: bool,
    const CACHE_HINT: bool,
>(
    dst: *mut c_void,
    mbar: *mut c_void,
    tmap: *const c_void,
    coords: [i32; 5],
    w_halo: u16,
    w_offset: u16,
    cta_mask: u16,
    cache_policy: u64,
) {
    static_assert!(matches!(CTA_GROUP, 0..=2), "CTA_GROUP must be 0, 1 or 2");
    llvm_cp_async_bulk_tensor_g2s_im2col_w_5d(
        dst,
        mbar,
        tmap,
        coords[0],
        coords[1],
        coords[2],
        coords[3],
        coords[4],
        w_halo,
        w_offset,
        cta_mask,
        cache_policy,
        MULTICAST,
        CACHE_HINT,
        CTA_GROUP,
    )
}

/// Asynchronously copies the 3D im2col::w::128-layout tensor data described by the tensor map
/// `tmap` at the coordinates `coords` from global memory to shared memory at `dst`, and performs a
/// complete transaction on the mbarrier at `mbar`.
///
/// `dst` must fall within the `.shared::cluster` state space, `mbar` within `.shared::cta`, and
/// `tmap` within `.const`, `.param` or `.global`. If `MULTICAST` is `false`, `cta_mask` is ignored.
/// Otherwise, the copy is multicast to the CTAs of the cluster selected by `cta_mask`. If
/// `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is the L2 cache
/// eviction policy used for the copy (`.L2::cache_hint`).
///
/// `CTA_GROUP` is the `.cta_group` modifier: `0` for none, `1` for `.cta_group::1` or `2` for
/// `.cta_group::2`.
///
/// Requires `sm_100a`/`sm_101a` with PTX ISA 8.6, or `sm_100f`/`sm_101f` with PTX ISA 8.8, or
/// `sm_100f`/`sm_110f` with PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_3d_g2s_im2col_w_128<
    const CTA_GROUP: i32,
    const MULTICAST: bool,
    const CACHE_HINT: bool,
>(
    dst: *mut c_void,
    mbar: *mut c_void,
    tmap: *const c_void,
    coords: [i32; 3],
    w_halo: u16,
    w_offset: u16,
    cta_mask: u16,
    cache_policy: u64,
) {
    static_assert!(matches!(CTA_GROUP, 0..=2), "CTA_GROUP must be 0, 1 or 2");
    llvm_cp_async_bulk_tensor_g2s_im2col_w_128_3d(
        dst,
        mbar,
        tmap,
        coords[0],
        coords[1],
        coords[2],
        w_halo,
        w_offset,
        cta_mask,
        cache_policy,
        MULTICAST,
        CACHE_HINT,
        CTA_GROUP,
    )
}

/// Asynchronously copies the 4D im2col::w::128-layout tensor data described by the tensor map
/// `tmap` at the coordinates `coords` from global memory to shared memory at `dst`, and performs a
/// complete transaction on the mbarrier at `mbar`.
///
/// `dst` must fall within the `.shared::cluster` state space, `mbar` within `.shared::cta`, and
/// `tmap` within `.const`, `.param` or `.global`. If `MULTICAST` is `false`, `cta_mask` is ignored.
/// Otherwise, the copy is multicast to the CTAs of the cluster selected by `cta_mask`. If
/// `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is the L2 cache
/// eviction policy used for the copy (`.L2::cache_hint`).
///
/// `CTA_GROUP` is the `.cta_group` modifier: `0` for none, `1` for `.cta_group::1` or `2` for
/// `.cta_group::2`.
///
/// Requires `sm_100a`/`sm_101a` with PTX ISA 8.6, or `sm_100f`/`sm_101f` with PTX ISA 8.8, or
/// `sm_100f`/`sm_110f` with PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_4d_g2s_im2col_w_128<
    const CTA_GROUP: i32,
    const MULTICAST: bool,
    const CACHE_HINT: bool,
>(
    dst: *mut c_void,
    mbar: *mut c_void,
    tmap: *const c_void,
    coords: [i32; 4],
    w_halo: u16,
    w_offset: u16,
    cta_mask: u16,
    cache_policy: u64,
) {
    static_assert!(matches!(CTA_GROUP, 0..=2), "CTA_GROUP must be 0, 1 or 2");
    llvm_cp_async_bulk_tensor_g2s_im2col_w_128_4d(
        dst,
        mbar,
        tmap,
        coords[0],
        coords[1],
        coords[2],
        coords[3],
        w_halo,
        w_offset,
        cta_mask,
        cache_policy,
        MULTICAST,
        CACHE_HINT,
        CTA_GROUP,
    )
}

/// Asynchronously copies the 5D im2col::w::128-layout tensor data described by the tensor map
/// `tmap` at the coordinates `coords` from global memory to shared memory at `dst`, and performs a
/// complete transaction on the mbarrier at `mbar`.
///
/// `dst` must fall within the `.shared::cluster` state space, `mbar` within `.shared::cta`, and
/// `tmap` within `.const`, `.param` or `.global`. If `MULTICAST` is `false`, `cta_mask` is ignored.
/// Otherwise, the copy is multicast to the CTAs of the cluster selected by `cta_mask`. If
/// `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is the L2 cache
/// eviction policy used for the copy (`.L2::cache_hint`).
///
/// `CTA_GROUP` is the `.cta_group` modifier: `0` for none, `1` for `.cta_group::1` or `2` for
/// `.cta_group::2`.
///
/// Requires `sm_100a`/`sm_101a` with PTX ISA 8.6, or `sm_100f`/`sm_101f` with PTX ISA 8.8, or
/// `sm_100f`/`sm_110f` with PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_5d_g2s_im2col_w_128<
    const CTA_GROUP: i32,
    const MULTICAST: bool,
    const CACHE_HINT: bool,
>(
    dst: *mut c_void,
    mbar: *mut c_void,
    tmap: *const c_void,
    coords: [i32; 5],
    w_halo: u16,
    w_offset: u16,
    cta_mask: u16,
    cache_policy: u64,
) {
    static_assert!(matches!(CTA_GROUP, 0..=2), "CTA_GROUP must be 0, 1 or 2");
    llvm_cp_async_bulk_tensor_g2s_im2col_w_128_5d(
        dst,
        mbar,
        tmap,
        coords[0],
        coords[1],
        coords[2],
        coords[3],
        coords[4],
        w_halo,
        w_offset,
        cta_mask,
        cache_policy,
        MULTICAST,
        CACHE_HINT,
        CTA_GROUP,
    )
}

/// Asynchronously copies the tile of the 1D tensor described by the tensor map `tmap` at the
/// coordinates `coords` from global memory to shared memory of the executing CTA at `dst`, and
/// performs a complete transaction on the mbarrier at `mbar`.
///
/// `dst` and `mbar` must fall within the `.shared::cta` state space, and `tmap` within `.const`,
/// `.param` or `.global`. If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise,
/// `cache_policy` is the L2 cache eviction policy used for the copy (`.L2::cache_hint`).
///
/// Requires `sm_90` and PTX ISA 8.6.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx86")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_1d_g2s_cta_tile<const CACHE_HINT: bool>(
    dst: *mut c_void,
    mbar: *mut c_void,
    tmap: *const c_void,
    coords: [i32; 1],
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_g2s_cta_tile_1d(dst, mbar, tmap, coords[0], cache_policy, CACHE_HINT)
}

/// Asynchronously copies the tile of the 2D tensor described by the tensor map `tmap` at the
/// coordinates `coords` from global memory to shared memory of the executing CTA at `dst`, and
/// performs a complete transaction on the mbarrier at `mbar`.
///
/// `dst` and `mbar` must fall within the `.shared::cta` state space, and `tmap` within `.const`,
/// `.param` or `.global`. If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise,
/// `cache_policy` is the L2 cache eviction policy used for the copy (`.L2::cache_hint`).
///
/// Requires `sm_90` and PTX ISA 8.6.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx86")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_2d_g2s_cta_tile<const CACHE_HINT: bool>(
    dst: *mut c_void,
    mbar: *mut c_void,
    tmap: *const c_void,
    coords: [i32; 2],
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_g2s_cta_tile_2d(
        dst,
        mbar,
        tmap,
        coords[0],
        coords[1],
        cache_policy,
        CACHE_HINT,
    )
}

/// Asynchronously copies the tile of the 3D tensor described by the tensor map `tmap` at the
/// coordinates `coords` from global memory to shared memory of the executing CTA at `dst`, and
/// performs a complete transaction on the mbarrier at `mbar`.
///
/// `dst` and `mbar` must fall within the `.shared::cta` state space, and `tmap` within `.const`,
/// `.param` or `.global`. If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise,
/// `cache_policy` is the L2 cache eviction policy used for the copy (`.L2::cache_hint`).
///
/// Requires `sm_90` and PTX ISA 8.6.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx86")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_3d_g2s_cta_tile<const CACHE_HINT: bool>(
    dst: *mut c_void,
    mbar: *mut c_void,
    tmap: *const c_void,
    coords: [i32; 3],
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_g2s_cta_tile_3d(
        dst,
        mbar,
        tmap,
        coords[0],
        coords[1],
        coords[2],
        cache_policy,
        CACHE_HINT,
    )
}

/// Asynchronously copies the tile of the 4D tensor described by the tensor map `tmap` at the
/// coordinates `coords` from global memory to shared memory of the executing CTA at `dst`, and
/// performs a complete transaction on the mbarrier at `mbar`.
///
/// `dst` and `mbar` must fall within the `.shared::cta` state space, and `tmap` within `.const`,
/// `.param` or `.global`. If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise,
/// `cache_policy` is the L2 cache eviction policy used for the copy (`.L2::cache_hint`).
///
/// Requires `sm_90` and PTX ISA 8.6.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx86")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_4d_g2s_cta_tile<const CACHE_HINT: bool>(
    dst: *mut c_void,
    mbar: *mut c_void,
    tmap: *const c_void,
    coords: [i32; 4],
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_g2s_cta_tile_4d(
        dst,
        mbar,
        tmap,
        coords[0],
        coords[1],
        coords[2],
        coords[3],
        cache_policy,
        CACHE_HINT,
    )
}

/// Asynchronously copies the tile of the 5D tensor described by the tensor map `tmap` at the
/// coordinates `coords` from global memory to shared memory of the executing CTA at `dst`, and
/// performs a complete transaction on the mbarrier at `mbar`.
///
/// `dst` and `mbar` must fall within the `.shared::cta` state space, and `tmap` within `.const`,
/// `.param` or `.global`. If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise,
/// `cache_policy` is the L2 cache eviction policy used for the copy (`.L2::cache_hint`).
///
/// Requires `sm_90` and PTX ISA 8.6.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx86")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_5d_g2s_cta_tile<const CACHE_HINT: bool>(
    dst: *mut c_void,
    mbar: *mut c_void,
    tmap: *const c_void,
    coords: [i32; 5],
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_g2s_cta_tile_5d(
        dst,
        mbar,
        tmap,
        coords[0],
        coords[1],
        coords[2],
        coords[3],
        coords[4],
        cache_policy,
        CACHE_HINT,
    )
}

/// Asynchronously copies four rows of the 2D tensor described by the tensor map `tmap`, selected by
/// the coordinates `coords` (the first is the column, the others the rows), from global memory to
/// shared memory of the executing CTA at `dst`, and performs a complete transaction on the mbarrier
/// at `mbar`.
///
/// `dst` and `mbar` must fall within the `.shared::cta` state space, and `tmap` within `.const`,
/// `.param` or `.global`. If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise,
/// `cache_policy` is the L2 cache eviction policy used for the copy (`.L2::cache_hint`).
///
/// Requires `sm_100` and PTX ISA 8.6.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_100,ptx86")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_2d_g2s_cta_tile_gather4<const CACHE_HINT: bool>(
    dst: *mut c_void,
    mbar: *mut c_void,
    tmap: *const c_void,
    coords: [i32; 5],
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_g2s_cta_tile_gather4_2d(
        dst,
        mbar,
        tmap,
        coords[0],
        coords[1],
        coords[2],
        coords[3],
        coords[4],
        cache_policy,
        CACHE_HINT,
    )
}

/// Asynchronously copies the 3D im2col-layout tensor data described by the tensor map `tmap` at the
/// coordinates `coords` from global memory to shared memory of the executing CTA at `dst`, and
/// performs a complete transaction on the mbarrier at `mbar`.
///
/// `dst` and `mbar` must fall within the `.shared::cta` state space, and `tmap` within `.const`,
/// `.param` or `.global`. If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise,
/// `cache_policy` is the L2 cache eviction policy used for the copy (`.L2::cache_hint`).
///
/// Requires `sm_90` and PTX ISA 8.6.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx86")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_3d_g2s_cta_im2col<const CACHE_HINT: bool>(
    dst: *mut c_void,
    mbar: *mut c_void,
    tmap: *const c_void,
    coords: [i32; 3],
    im2col_offsets: [u16; 1],
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_g2s_cta_im2col_3d(
        dst,
        mbar,
        tmap,
        coords[0],
        coords[1],
        coords[2],
        im2col_offsets[0],
        cache_policy,
        CACHE_HINT,
    )
}

/// Asynchronously copies the 4D im2col-layout tensor data described by the tensor map `tmap` at the
/// coordinates `coords` from global memory to shared memory of the executing CTA at `dst`, and
/// performs a complete transaction on the mbarrier at `mbar`.
///
/// `dst` and `mbar` must fall within the `.shared::cta` state space, and `tmap` within `.const`,
/// `.param` or `.global`. If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise,
/// `cache_policy` is the L2 cache eviction policy used for the copy (`.L2::cache_hint`).
///
/// Requires `sm_90` and PTX ISA 8.6.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx86")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_4d_g2s_cta_im2col<const CACHE_HINT: bool>(
    dst: *mut c_void,
    mbar: *mut c_void,
    tmap: *const c_void,
    coords: [i32; 4],
    im2col_offsets: [u16; 2],
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_g2s_cta_im2col_4d(
        dst,
        mbar,
        tmap,
        coords[0],
        coords[1],
        coords[2],
        coords[3],
        im2col_offsets[0],
        im2col_offsets[1],
        cache_policy,
        CACHE_HINT,
    )
}

/// Asynchronously copies the 5D im2col-layout tensor data described by the tensor map `tmap` at the
/// coordinates `coords` from global memory to shared memory of the executing CTA at `dst`, and
/// performs a complete transaction on the mbarrier at `mbar`.
///
/// `dst` and `mbar` must fall within the `.shared::cta` state space, and `tmap` within `.const`,
/// `.param` or `.global`. If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise,
/// `cache_policy` is the L2 cache eviction policy used for the copy (`.L2::cache_hint`).
///
/// Requires `sm_90` and PTX ISA 8.6.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx86")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_5d_g2s_cta_im2col<const CACHE_HINT: bool>(
    dst: *mut c_void,
    mbar: *mut c_void,
    tmap: *const c_void,
    coords: [i32; 5],
    im2col_offsets: [u16; 3],
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_g2s_cta_im2col_5d(
        dst,
        mbar,
        tmap,
        coords[0],
        coords[1],
        coords[2],
        coords[3],
        coords[4],
        im2col_offsets[0],
        im2col_offsets[1],
        im2col_offsets[2],
        cache_policy,
        CACHE_HINT,
    )
}

/// Asynchronously copies the 3D im2col::w-layout tensor data described by the tensor map `tmap` at
/// the coordinates `coords` from global memory to shared memory of the executing CTA at `dst`, and
/// performs a complete transaction on the mbarrier at `mbar`.
///
/// `dst` and `mbar` must fall within the `.shared::cta` state space, and `tmap` within `.const`,
/// `.param` or `.global`. If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise,
/// `cache_policy` is the L2 cache eviction policy used for the copy (`.L2::cache_hint`).
///
/// Requires `sm_100` and PTX ISA 8.6.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_100,ptx86")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_3d_g2s_cta_im2col_w<const CACHE_HINT: bool>(
    dst: *mut c_void,
    mbar: *mut c_void,
    tmap: *const c_void,
    coords: [i32; 3],
    w_halo: u16,
    w_offset: u16,
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_g2s_cta_im2col_w_3d(
        dst,
        mbar,
        tmap,
        coords[0],
        coords[1],
        coords[2],
        w_halo,
        w_offset,
        cache_policy,
        CACHE_HINT,
    )
}

/// Asynchronously copies the 4D im2col::w-layout tensor data described by the tensor map `tmap` at
/// the coordinates `coords` from global memory to shared memory of the executing CTA at `dst`, and
/// performs a complete transaction on the mbarrier at `mbar`.
///
/// `dst` and `mbar` must fall within the `.shared::cta` state space, and `tmap` within `.const`,
/// `.param` or `.global`. If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise,
/// `cache_policy` is the L2 cache eviction policy used for the copy (`.L2::cache_hint`).
///
/// Requires `sm_100` and PTX ISA 8.6.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_100,ptx86")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_4d_g2s_cta_im2col_w<const CACHE_HINT: bool>(
    dst: *mut c_void,
    mbar: *mut c_void,
    tmap: *const c_void,
    coords: [i32; 4],
    w_halo: u16,
    w_offset: u16,
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_g2s_cta_im2col_w_4d(
        dst,
        mbar,
        tmap,
        coords[0],
        coords[1],
        coords[2],
        coords[3],
        w_halo,
        w_offset,
        cache_policy,
        CACHE_HINT,
    )
}

/// Asynchronously copies the 5D im2col::w-layout tensor data described by the tensor map `tmap` at
/// the coordinates `coords` from global memory to shared memory of the executing CTA at `dst`, and
/// performs a complete transaction on the mbarrier at `mbar`.
///
/// `dst` and `mbar` must fall within the `.shared::cta` state space, and `tmap` within `.const`,
/// `.param` or `.global`. If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise,
/// `cache_policy` is the L2 cache eviction policy used for the copy (`.L2::cache_hint`).
///
/// Requires `sm_100` and PTX ISA 8.6.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_100,ptx86")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_5d_g2s_cta_im2col_w<const CACHE_HINT: bool>(
    dst: *mut c_void,
    mbar: *mut c_void,
    tmap: *const c_void,
    coords: [i32; 5],
    w_halo: u16,
    w_offset: u16,
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_g2s_cta_im2col_w_5d(
        dst,
        mbar,
        tmap,
        coords[0],
        coords[1],
        coords[2],
        coords[3],
        coords[4],
        w_halo,
        w_offset,
        cache_policy,
        CACHE_HINT,
    )
}

/// Asynchronously copies the 3D im2col::w::128-layout tensor data described by the tensor map
/// `tmap` at the coordinates `coords` from global memory to shared memory of the executing CTA at
/// `dst`, and performs a complete transaction on the mbarrier at `mbar`.
///
/// `dst` and `mbar` must fall within the `.shared::cta` state space, and `tmap` within `.const`,
/// `.param` or `.global`. If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise,
/// `cache_policy` is the L2 cache eviction policy used for the copy (`.L2::cache_hint`).
///
/// Requires `sm_100a`/`sm_101a` with PTX ISA 8.6, or `sm_100f`/`sm_101f` with PTX ISA 8.8, or
/// `sm_100f`/`sm_110f` with PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_3d_g2s_cta_im2col_w_128<const CACHE_HINT: bool>(
    dst: *mut c_void,
    mbar: *mut c_void,
    tmap: *const c_void,
    coords: [i32; 3],
    w_halo: u16,
    w_offset: u16,
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_g2s_cta_im2col_w_128_3d(
        dst,
        mbar,
        tmap,
        coords[0],
        coords[1],
        coords[2],
        w_halo,
        w_offset,
        cache_policy,
        CACHE_HINT,
    )
}

/// Asynchronously copies the 4D im2col::w::128-layout tensor data described by the tensor map
/// `tmap` at the coordinates `coords` from global memory to shared memory of the executing CTA at
/// `dst`, and performs a complete transaction on the mbarrier at `mbar`.
///
/// `dst` and `mbar` must fall within the `.shared::cta` state space, and `tmap` within `.const`,
/// `.param` or `.global`. If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise,
/// `cache_policy` is the L2 cache eviction policy used for the copy (`.L2::cache_hint`).
///
/// Requires `sm_100a`/`sm_101a` with PTX ISA 8.6, or `sm_100f`/`sm_101f` with PTX ISA 8.8, or
/// `sm_100f`/`sm_110f` with PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_4d_g2s_cta_im2col_w_128<const CACHE_HINT: bool>(
    dst: *mut c_void,
    mbar: *mut c_void,
    tmap: *const c_void,
    coords: [i32; 4],
    w_halo: u16,
    w_offset: u16,
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_g2s_cta_im2col_w_128_4d(
        dst,
        mbar,
        tmap,
        coords[0],
        coords[1],
        coords[2],
        coords[3],
        w_halo,
        w_offset,
        cache_policy,
        CACHE_HINT,
    )
}

/// Asynchronously copies the 5D im2col::w::128-layout tensor data described by the tensor map
/// `tmap` at the coordinates `coords` from global memory to shared memory of the executing CTA at
/// `dst`, and performs a complete transaction on the mbarrier at `mbar`.
///
/// `dst` and `mbar` must fall within the `.shared::cta` state space, and `tmap` within `.const`,
/// `.param` or `.global`. If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise,
/// `cache_policy` is the L2 cache eviction policy used for the copy (`.L2::cache_hint`).
///
/// Requires `sm_100a`/`sm_101a` with PTX ISA 8.6, or `sm_100f`/`sm_101f` with PTX ISA 8.8, or
/// `sm_100f`/`sm_110f` with PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_5d_g2s_cta_im2col_w_128<const CACHE_HINT: bool>(
    dst: *mut c_void,
    mbar: *mut c_void,
    tmap: *const c_void,
    coords: [i32; 5],
    w_halo: u16,
    w_offset: u16,
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_g2s_cta_im2col_w_128_5d(
        dst,
        mbar,
        tmap,
        coords[0],
        coords[1],
        coords[2],
        coords[3],
        coords[4],
        w_halo,
        w_offset,
        cache_policy,
        CACHE_HINT,
    )
}

/// Asynchronously copies the tile of the 1D tensor at `src` in shared memory to global memory, as
/// described by the tensor map `tmap` at the coordinates `coords`.
///
/// `src` must fall within the `.shared::cta` state space, and `tmap` within `.const`, `.param` or
/// `.global`. If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is
/// the L2 cache eviction policy used for the copy (`.L2::cache_hint`).
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_1d_s2g_tile<const CACHE_HINT: bool>(
    src: *const c_void,
    tmap: *const c_void,
    coords: [i32; 1],
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_s2g_tile_1d(src, tmap, coords[0], cache_policy, CACHE_HINT)
}

/// Asynchronously copies the tile of the 2D tensor at `src` in shared memory to global memory, as
/// described by the tensor map `tmap` at the coordinates `coords`.
///
/// `src` must fall within the `.shared::cta` state space, and `tmap` within `.const`, `.param` or
/// `.global`. If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is
/// the L2 cache eviction policy used for the copy (`.L2::cache_hint`).
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_2d_s2g_tile<const CACHE_HINT: bool>(
    src: *const c_void,
    tmap: *const c_void,
    coords: [i32; 2],
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_s2g_tile_2d(src, tmap, coords[0], coords[1], cache_policy, CACHE_HINT)
}

/// Asynchronously copies the tile of the 3D tensor at `src` in shared memory to global memory, as
/// described by the tensor map `tmap` at the coordinates `coords`.
///
/// `src` must fall within the `.shared::cta` state space, and `tmap` within `.const`, `.param` or
/// `.global`. If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is
/// the L2 cache eviction policy used for the copy (`.L2::cache_hint`).
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_3d_s2g_tile<const CACHE_HINT: bool>(
    src: *const c_void,
    tmap: *const c_void,
    coords: [i32; 3],
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_s2g_tile_3d(
        src,
        tmap,
        coords[0],
        coords[1],
        coords[2],
        cache_policy,
        CACHE_HINT,
    )
}

/// Asynchronously copies the tile of the 4D tensor at `src` in shared memory to global memory, as
/// described by the tensor map `tmap` at the coordinates `coords`.
///
/// `src` must fall within the `.shared::cta` state space, and `tmap` within `.const`, `.param` or
/// `.global`. If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is
/// the L2 cache eviction policy used for the copy (`.L2::cache_hint`).
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_4d_s2g_tile<const CACHE_HINT: bool>(
    src: *const c_void,
    tmap: *const c_void,
    coords: [i32; 4],
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_s2g_tile_4d(
        src,
        tmap,
        coords[0],
        coords[1],
        coords[2],
        coords[3],
        cache_policy,
        CACHE_HINT,
    )
}

/// Asynchronously copies the tile of the 5D tensor at `src` in shared memory to global memory, as
/// described by the tensor map `tmap` at the coordinates `coords`.
///
/// `src` must fall within the `.shared::cta` state space, and `tmap` within `.const`, `.param` or
/// `.global`. If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is
/// the L2 cache eviction policy used for the copy (`.L2::cache_hint`).
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_5d_s2g_tile<const CACHE_HINT: bool>(
    src: *const c_void,
    tmap: *const c_void,
    coords: [i32; 5],
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_s2g_tile_5d(
        src,
        tmap,
        coords[0],
        coords[1],
        coords[2],
        coords[3],
        coords[4],
        cache_policy,
        CACHE_HINT,
    )
}

/// Asynchronously copies the 3D im2col-layout tensor data at `src` in shared memory to global
/// memory, as described by the tensor map `tmap` at the coordinates `coords`.
///
/// `src` must fall within the `.shared::cta` state space, and `tmap` within `.const`, `.param` or
/// `.global`. If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is
/// the L2 cache eviction policy used for the copy (`.L2::cache_hint`).
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_3d_s2g_im2col<const CACHE_HINT: bool>(
    src: *const c_void,
    tmap: *const c_void,
    coords: [i32; 3],
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_s2g_im2col_3d(
        src,
        tmap,
        coords[0],
        coords[1],
        coords[2],
        cache_policy,
        CACHE_HINT,
    )
}

/// Asynchronously copies the 4D im2col-layout tensor data at `src` in shared memory to global
/// memory, as described by the tensor map `tmap` at the coordinates `coords`.
///
/// `src` must fall within the `.shared::cta` state space, and `tmap` within `.const`, `.param` or
/// `.global`. If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is
/// the L2 cache eviction policy used for the copy (`.L2::cache_hint`).
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_4d_s2g_im2col<const CACHE_HINT: bool>(
    src: *const c_void,
    tmap: *const c_void,
    coords: [i32; 4],
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_s2g_im2col_4d(
        src,
        tmap,
        coords[0],
        coords[1],
        coords[2],
        coords[3],
        cache_policy,
        CACHE_HINT,
    )
}

/// Asynchronously copies the 5D im2col-layout tensor data at `src` in shared memory to global
/// memory, as described by the tensor map `tmap` at the coordinates `coords`.
///
/// `src` must fall within the `.shared::cta` state space, and `tmap` within `.const`, `.param` or
/// `.global`. If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is
/// the L2 cache eviction policy used for the copy (`.L2::cache_hint`).
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_5d_s2g_im2col<const CACHE_HINT: bool>(
    src: *const c_void,
    tmap: *const c_void,
    coords: [i32; 5],
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_s2g_im2col_5d(
        src,
        tmap,
        coords[0],
        coords[1],
        coords[2],
        coords[3],
        coords[4],
        cache_policy,
        CACHE_HINT,
    )
}

/// Asynchronously copies four rows of the 2D tensor data at `src` in shared memory to global
/// memory, as described by the tensor map `tmap` at the coordinates `coords` (the first is the
/// column, the others the rows).
///
/// `src` must fall within the `.shared::cta` state space, and `tmap` within `.const`, `.param` or
/// `.global`. If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is
/// the L2 cache eviction policy used for the copy (`.L2::cache_hint`).
///
/// Requires `sm_100a`/`sm_101a` with PTX ISA 8.6, or `sm_100f`/`sm_101f` with PTX ISA 8.8, or
/// `sm_100f`/`sm_110f` with PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_2d_s2g_tile_scatter4<const CACHE_HINT: bool>(
    src: *const c_void,
    tmap: *const c_void,
    coords: [i32; 5],
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_s2g_tile_scatter4_2d(
        src,
        tmap,
        coords[0],
        coords[1],
        coords[2],
        coords[3],
        coords[4],
        cache_policy,
        CACHE_HINT,
    )
}

/// Asynchronously reduces the tile of the 1D tensor at `src` in shared memory into global memory,
/// as described by the tensor map `tmap` at the coordinates `coords`, using the reduction operation
/// `OP`.
///
/// `src` must fall within the `.shared::cta` state space, and `tmap` within `.const`, `.param` or
/// `.global`. If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is
/// the L2 cache eviction policy used for the reduction (`.L2::cache_hint`).
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-reduce-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_1d_reduce_tile<
    const OP: TensorReduceOp,
    const CACHE_HINT: bool,
>(
    src: *const c_void,
    tmap: *const c_void,
    coords: [i32; 1],
    cache_policy: u64,
) {
    match OP {
        TensorReduceOp::Add => llvm_cp_async_bulk_tensor_reduce_add_tile_1d(
            src,
            tmap,
            coords[0],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Min => llvm_cp_async_bulk_tensor_reduce_min_tile_1d(
            src,
            tmap,
            coords[0],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Max => llvm_cp_async_bulk_tensor_reduce_max_tile_1d(
            src,
            tmap,
            coords[0],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Inc => llvm_cp_async_bulk_tensor_reduce_inc_tile_1d(
            src,
            tmap,
            coords[0],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Dec => llvm_cp_async_bulk_tensor_reduce_dec_tile_1d(
            src,
            tmap,
            coords[0],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::And => llvm_cp_async_bulk_tensor_reduce_and_tile_1d(
            src,
            tmap,
            coords[0],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Or => llvm_cp_async_bulk_tensor_reduce_or_tile_1d(
            src,
            tmap,
            coords[0],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Xor => llvm_cp_async_bulk_tensor_reduce_xor_tile_1d(
            src,
            tmap,
            coords[0],
            cache_policy,
            CACHE_HINT,
        ),
    }
}

/// Asynchronously reduces the tile of the 2D tensor at `src` in shared memory into global memory,
/// as described by the tensor map `tmap` at the coordinates `coords`, using the reduction operation
/// `OP`.
///
/// `src` must fall within the `.shared::cta` state space, and `tmap` within `.const`, `.param` or
/// `.global`. If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is
/// the L2 cache eviction policy used for the reduction (`.L2::cache_hint`).
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-reduce-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_2d_reduce_tile<
    const OP: TensorReduceOp,
    const CACHE_HINT: bool,
>(
    src: *const c_void,
    tmap: *const c_void,
    coords: [i32; 2],
    cache_policy: u64,
) {
    match OP {
        TensorReduceOp::Add => llvm_cp_async_bulk_tensor_reduce_add_tile_2d(
            src,
            tmap,
            coords[0],
            coords[1],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Min => llvm_cp_async_bulk_tensor_reduce_min_tile_2d(
            src,
            tmap,
            coords[0],
            coords[1],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Max => llvm_cp_async_bulk_tensor_reduce_max_tile_2d(
            src,
            tmap,
            coords[0],
            coords[1],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Inc => llvm_cp_async_bulk_tensor_reduce_inc_tile_2d(
            src,
            tmap,
            coords[0],
            coords[1],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Dec => llvm_cp_async_bulk_tensor_reduce_dec_tile_2d(
            src,
            tmap,
            coords[0],
            coords[1],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::And => llvm_cp_async_bulk_tensor_reduce_and_tile_2d(
            src,
            tmap,
            coords[0],
            coords[1],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Or => llvm_cp_async_bulk_tensor_reduce_or_tile_2d(
            src,
            tmap,
            coords[0],
            coords[1],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Xor => llvm_cp_async_bulk_tensor_reduce_xor_tile_2d(
            src,
            tmap,
            coords[0],
            coords[1],
            cache_policy,
            CACHE_HINT,
        ),
    }
}

/// Asynchronously reduces the tile of the 3D tensor at `src` in shared memory into global memory,
/// as described by the tensor map `tmap` at the coordinates `coords`, using the reduction operation
/// `OP`.
///
/// `src` must fall within the `.shared::cta` state space, and `tmap` within `.const`, `.param` or
/// `.global`. If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is
/// the L2 cache eviction policy used for the reduction (`.L2::cache_hint`).
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-reduce-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_3d_reduce_tile<
    const OP: TensorReduceOp,
    const CACHE_HINT: bool,
>(
    src: *const c_void,
    tmap: *const c_void,
    coords: [i32; 3],
    cache_policy: u64,
) {
    match OP {
        TensorReduceOp::Add => llvm_cp_async_bulk_tensor_reduce_add_tile_3d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Min => llvm_cp_async_bulk_tensor_reduce_min_tile_3d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Max => llvm_cp_async_bulk_tensor_reduce_max_tile_3d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Inc => llvm_cp_async_bulk_tensor_reduce_inc_tile_3d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Dec => llvm_cp_async_bulk_tensor_reduce_dec_tile_3d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::And => llvm_cp_async_bulk_tensor_reduce_and_tile_3d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Or => llvm_cp_async_bulk_tensor_reduce_or_tile_3d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Xor => llvm_cp_async_bulk_tensor_reduce_xor_tile_3d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            cache_policy,
            CACHE_HINT,
        ),
    }
}

/// Asynchronously reduces the tile of the 4D tensor at `src` in shared memory into global memory,
/// as described by the tensor map `tmap` at the coordinates `coords`, using the reduction operation
/// `OP`.
///
/// `src` must fall within the `.shared::cta` state space, and `tmap` within `.const`, `.param` or
/// `.global`. If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is
/// the L2 cache eviction policy used for the reduction (`.L2::cache_hint`).
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-reduce-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_4d_reduce_tile<
    const OP: TensorReduceOp,
    const CACHE_HINT: bool,
>(
    src: *const c_void,
    tmap: *const c_void,
    coords: [i32; 4],
    cache_policy: u64,
) {
    match OP {
        TensorReduceOp::Add => llvm_cp_async_bulk_tensor_reduce_add_tile_4d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            coords[3],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Min => llvm_cp_async_bulk_tensor_reduce_min_tile_4d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            coords[3],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Max => llvm_cp_async_bulk_tensor_reduce_max_tile_4d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            coords[3],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Inc => llvm_cp_async_bulk_tensor_reduce_inc_tile_4d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            coords[3],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Dec => llvm_cp_async_bulk_tensor_reduce_dec_tile_4d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            coords[3],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::And => llvm_cp_async_bulk_tensor_reduce_and_tile_4d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            coords[3],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Or => llvm_cp_async_bulk_tensor_reduce_or_tile_4d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            coords[3],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Xor => llvm_cp_async_bulk_tensor_reduce_xor_tile_4d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            coords[3],
            cache_policy,
            CACHE_HINT,
        ),
    }
}

/// Asynchronously reduces the tile of the 5D tensor at `src` in shared memory into global memory,
/// as described by the tensor map `tmap` at the coordinates `coords`, using the reduction operation
/// `OP`.
///
/// `src` must fall within the `.shared::cta` state space, and `tmap` within `.const`, `.param` or
/// `.global`. If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is
/// the L2 cache eviction policy used for the reduction (`.L2::cache_hint`).
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-reduce-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_5d_reduce_tile<
    const OP: TensorReduceOp,
    const CACHE_HINT: bool,
>(
    src: *const c_void,
    tmap: *const c_void,
    coords: [i32; 5],
    cache_policy: u64,
) {
    match OP {
        TensorReduceOp::Add => llvm_cp_async_bulk_tensor_reduce_add_tile_5d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            coords[3],
            coords[4],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Min => llvm_cp_async_bulk_tensor_reduce_min_tile_5d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            coords[3],
            coords[4],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Max => llvm_cp_async_bulk_tensor_reduce_max_tile_5d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            coords[3],
            coords[4],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Inc => llvm_cp_async_bulk_tensor_reduce_inc_tile_5d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            coords[3],
            coords[4],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Dec => llvm_cp_async_bulk_tensor_reduce_dec_tile_5d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            coords[3],
            coords[4],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::And => llvm_cp_async_bulk_tensor_reduce_and_tile_5d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            coords[3],
            coords[4],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Or => llvm_cp_async_bulk_tensor_reduce_or_tile_5d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            coords[3],
            coords[4],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Xor => llvm_cp_async_bulk_tensor_reduce_xor_tile_5d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            coords[3],
            coords[4],
            cache_policy,
            CACHE_HINT,
        ),
    }
}

/// Asynchronously reduces the 3D im2col-layout tensor data at `src` in shared memory into global
/// memory, as described by the tensor map `tmap` at the coordinates `coords`, using the reduction
/// operation `OP`.
///
/// `src` must fall within the `.shared::cta` state space, and `tmap` within `.const`, `.param` or
/// `.global`. If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is
/// the L2 cache eviction policy used for the reduction (`.L2::cache_hint`).
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-reduce-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_3d_reduce_im2col<
    const OP: TensorReduceOp,
    const CACHE_HINT: bool,
>(
    src: *const c_void,
    tmap: *const c_void,
    coords: [i32; 3],
    cache_policy: u64,
) {
    match OP {
        TensorReduceOp::Add => llvm_cp_async_bulk_tensor_reduce_add_im2col_3d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Min => llvm_cp_async_bulk_tensor_reduce_min_im2col_3d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Max => llvm_cp_async_bulk_tensor_reduce_max_im2col_3d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Inc => llvm_cp_async_bulk_tensor_reduce_inc_im2col_3d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Dec => llvm_cp_async_bulk_tensor_reduce_dec_im2col_3d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::And => llvm_cp_async_bulk_tensor_reduce_and_im2col_3d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Or => llvm_cp_async_bulk_tensor_reduce_or_im2col_3d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Xor => llvm_cp_async_bulk_tensor_reduce_xor_im2col_3d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            cache_policy,
            CACHE_HINT,
        ),
    }
}

/// Asynchronously reduces the 4D im2col-layout tensor data at `src` in shared memory into global
/// memory, as described by the tensor map `tmap` at the coordinates `coords`, using the reduction
/// operation `OP`.
///
/// `src` must fall within the `.shared::cta` state space, and `tmap` within `.const`, `.param` or
/// `.global`. If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is
/// the L2 cache eviction policy used for the reduction (`.L2::cache_hint`).
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-reduce-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_4d_reduce_im2col<
    const OP: TensorReduceOp,
    const CACHE_HINT: bool,
>(
    src: *const c_void,
    tmap: *const c_void,
    coords: [i32; 4],
    cache_policy: u64,
) {
    match OP {
        TensorReduceOp::Add => llvm_cp_async_bulk_tensor_reduce_add_im2col_4d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            coords[3],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Min => llvm_cp_async_bulk_tensor_reduce_min_im2col_4d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            coords[3],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Max => llvm_cp_async_bulk_tensor_reduce_max_im2col_4d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            coords[3],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Inc => llvm_cp_async_bulk_tensor_reduce_inc_im2col_4d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            coords[3],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Dec => llvm_cp_async_bulk_tensor_reduce_dec_im2col_4d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            coords[3],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::And => llvm_cp_async_bulk_tensor_reduce_and_im2col_4d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            coords[3],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Or => llvm_cp_async_bulk_tensor_reduce_or_im2col_4d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            coords[3],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Xor => llvm_cp_async_bulk_tensor_reduce_xor_im2col_4d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            coords[3],
            cache_policy,
            CACHE_HINT,
        ),
    }
}

/// Asynchronously reduces the 5D im2col-layout tensor data at `src` in shared memory into global
/// memory, as described by the tensor map `tmap` at the coordinates `coords`, using the reduction
/// operation `OP`.
///
/// `src` must fall within the `.shared::cta` state space, and `tmap` within `.const`, `.param` or
/// `.global`. If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is
/// the L2 cache eviction policy used for the reduction (`.L2::cache_hint`).
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-reduce-async-bulk-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_tensor_5d_reduce_im2col<
    const OP: TensorReduceOp,
    const CACHE_HINT: bool,
>(
    src: *const c_void,
    tmap: *const c_void,
    coords: [i32; 5],
    cache_policy: u64,
) {
    match OP {
        TensorReduceOp::Add => llvm_cp_async_bulk_tensor_reduce_add_im2col_5d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            coords[3],
            coords[4],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Min => llvm_cp_async_bulk_tensor_reduce_min_im2col_5d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            coords[3],
            coords[4],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Max => llvm_cp_async_bulk_tensor_reduce_max_im2col_5d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            coords[3],
            coords[4],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Inc => llvm_cp_async_bulk_tensor_reduce_inc_im2col_5d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            coords[3],
            coords[4],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Dec => llvm_cp_async_bulk_tensor_reduce_dec_im2col_5d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            coords[3],
            coords[4],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::And => llvm_cp_async_bulk_tensor_reduce_and_im2col_5d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            coords[3],
            coords[4],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Or => llvm_cp_async_bulk_tensor_reduce_or_im2col_5d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            coords[3],
            coords[4],
            cache_policy,
            CACHE_HINT,
        ),
        TensorReduceOp::Xor => llvm_cp_async_bulk_tensor_reduce_xor_im2col_5d(
            src,
            tmap,
            coords[0],
            coords[1],
            coords[2],
            coords[3],
            coords[4],
            cache_policy,
            CACHE_HINT,
        ),
    }
}
