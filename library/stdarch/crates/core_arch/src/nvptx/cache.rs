// --- LLM-generated --- //
//! Cache control: prefetches, eviction priorities and read-only uniform loads.

use crate::ffi::c_void;

#[allow(improper_ctypes)]
unsafe extern "llvm-intrinsic" {
    #[link_name = "llvm.nvvm.prefetch.L1"]
    fn llvm_prefetch_l1(p: *const c_void);
    #[link_name = "llvm.nvvm.prefetch.L2"]
    fn llvm_prefetch_l2(p: *const c_void);
    #[link_name = "llvm.nvvm.prefetchu.L1"]
    fn llvm_prefetchu_l1(p: *const c_void);
    #[link_name = "llvm.nvvm.prefetch.tensormap.p0"]
    fn llvm_prefetch_tensormap(p: *const c_void);
    #[link_name = "llvm.nvvm.applypriority.L2.evict.normal"]
    fn llvm_applypriority_l2_evict_normal(p: *const c_void, size: u64);
    #[link_name = "llvm.nvvm.discard.L2"]
    fn llvm_discard_l2(p: *const c_void, size: u64);
    #[link_name = "llvm.nvvm.ldu.global.i.i8.p0"]
    fn llvm_ldu_global_i8(p: *const c_void, align: i32) -> u8;
    #[link_name = "llvm.nvvm.ldu.global.i.i16.p0"]
    fn llvm_ldu_global_i16(p: *const c_void, align: i32) -> u16;
    #[link_name = "llvm.nvvm.ldu.global.i.i32.p0"]
    fn llvm_ldu_global_i32(p: *const c_void, align: i32) -> u32;
    #[link_name = "llvm.nvvm.ldu.global.i.i64.p0"]
    fn llvm_ldu_global_i64(p: *const c_void, align: i32) -> u64;
    #[link_name = "llvm.nvvm.ldu.global.f.f32.p0"]
    fn llvm_ldu_global_f32(p: *const c_void, align: i32) -> f32;
    #[link_name = "llvm.nvvm.ldu.global.f.f64.p0"]
    fn llvm_ldu_global_f64(p: *const c_void, align: i32) -> f64;
    #[link_name = "llvm.nvvm.ldu.global.p.p0.p0"]
    fn llvm_ldu_global_p(p: *const c_void, align: i32) -> *const c_void;
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.prefetch.tile.1d"]
    fn llvm_cp_async_bulk_tensor_prefetch_tile_1d(
        tmap: *const c_void,
        c0: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.prefetch.tile.2d"]
    fn llvm_cp_async_bulk_tensor_prefetch_tile_2d(
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.prefetch.tile.3d"]
    fn llvm_cp_async_bulk_tensor_prefetch_tile_3d(
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.prefetch.tile.4d"]
    fn llvm_cp_async_bulk_tensor_prefetch_tile_4d(
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.prefetch.tile.5d"]
    fn llvm_cp_async_bulk_tensor_prefetch_tile_5d(
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        c4: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.prefetch.im2col.3d"]
    fn llvm_cp_async_bulk_tensor_prefetch_im2col_3d(
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        offset0: u16,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.prefetch.im2col.4d"]
    fn llvm_cp_async_bulk_tensor_prefetch_im2col_4d(
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
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.prefetch.im2col.5d"]
    fn llvm_cp_async_bulk_tensor_prefetch_im2col_5d(
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
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.prefetch.im2col.w.3d"]
    fn llvm_cp_async_bulk_tensor_prefetch_im2col_w_3d(
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        w_halo: u16,
        w_offset: u16,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.prefetch.im2col.w.4d"]
    fn llvm_cp_async_bulk_tensor_prefetch_im2col_w_4d(
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
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.prefetch.im2col.w.5d"]
    fn llvm_cp_async_bulk_tensor_prefetch_im2col_w_5d(
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
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.prefetch.im2col.w.128.3d"]
    fn llvm_cp_async_bulk_tensor_prefetch_im2col_w_128_3d(
        tmap: *const c_void,
        c0: i32,
        c1: i32,
        c2: i32,
        w_halo: u16,
        w_offset: u16,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.prefetch.im2col.w.128.4d"]
    fn llvm_cp_async_bulk_tensor_prefetch_im2col_w_128_4d(
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
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.prefetch.im2col.w.128.5d"]
    fn llvm_cp_async_bulk_tensor_prefetch_im2col_w_128_5d(
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
    #[link_name = "llvm.nvvm.cp.async.bulk.tensor.prefetch.tile.gather4.2d"]
    fn llvm_cp_async_bulk_tensor_prefetch_tile_gather4_2d(
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

/// Prefetches the cache line containing `ptr` into the L1 cache.
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-prefetch>
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn prefetch_l1<T>(ptr: *const T) {
    llvm_prefetch_l1(ptr.cast())
}

/// Prefetches the cache line containing `ptr` into the L2 cache.
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-prefetch>
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn prefetch_l2<T>(ptr: *const T) {
    llvm_prefetch_l2(ptr.cast())
}

/// Prefetches the cache line containing `ptr` into the L1 cache for uniform (read-only) use.
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-prefetchu>
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn prefetchu_l1<T>(ptr: *const T) {
    llvm_prefetchu_l1(ptr.cast())
}

/// Prefetches the tensor map at the generic address `tmap`.
///
/// `tmap` must fall within the `.const`, `.param` or `.global` state space.
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-prefetch-tensormap>
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn prefetch_tensormap(tmap: *const c_void) {
    llvm_prefetch_tensormap(tmap)
}

/// Sets the eviction priority of the 128 bytes of global memory starting at `ptr` to
/// `evict_normal` in the L2 cache.
///
/// `ptr` must point to global memory. Exactly 128 bytes are affected, as PTX only supports
/// that size.
///
/// Requires `sm_80` and PTX ISA 7.4.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-applypriority>
#[inline]
#[target_feature(enable = "sm_80,ptx74")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn applypriority_l2_evict_normal<T>(ptr: *const T) {
    llvm_applypriority_l2_evict_normal(ptr.cast(), 128)
}

/// Invalidates the 128 bytes of global memory starting at `ptr` in the L2 cache, without
/// writing them back to memory.
///
/// `ptr` must point to global memory. Exactly 128 bytes are affected, as PTX only supports
/// that size. The contents of the affected memory are undefined afterwards.
///
/// Requires `sm_80` and PTX ISA 7.4.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-discard>
#[inline]
#[target_feature(enable = "sm_80,ptx74")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn discard_l2<T>(ptr: *const T) {
    llvm_discard_l2(ptr.cast(), 128)
}

/// Loads a `u8` from `ptr` through the read-only data path.
///
/// `ptr` must point to global memory.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-ldu>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn ldu_u8(ptr: *const u8) -> u8 {
    llvm_ldu_global_i8(ptr.cast(), 1)
}

/// Loads a `u16` from `ptr` through the read-only data path.
///
/// `ptr` must point to global memory.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-ldu>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn ldu_u16(ptr: *const u16) -> u16 {
    llvm_ldu_global_i16(ptr.cast(), 2)
}

/// Loads a `u32` from `ptr` through the read-only data path.
///
/// `ptr` must point to global memory.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-ldu>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn ldu_u32(ptr: *const u32) -> u32 {
    llvm_ldu_global_i32(ptr.cast(), 4)
}

/// Loads a `u64` from `ptr` through the read-only data path.
///
/// `ptr` must point to global memory.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-ldu>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn ldu_u64(ptr: *const u64) -> u64 {
    llvm_ldu_global_i64(ptr.cast(), 8)
}

/// Loads an `f32` from `ptr` through the read-only data path.
///
/// `ptr` must point to global memory.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-ldu>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn ldu_f32(ptr: *const f32) -> f32 {
    llvm_ldu_global_f32(ptr.cast(), 4)
}

/// Loads an `f64` from `ptr` through the read-only data path.
///
/// `ptr` must point to global memory.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-ldu>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn ldu_f64(ptr: *const f64) -> f64 {
    llvm_ldu_global_f64(ptr.cast(), 8)
}

/// Loads a pointer from `ptr` through the read-only data path.
///
/// `ptr` must point to global memory.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-ldu>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn ldu_ptr<T>(ptr: *const *const T) -> *const T {
    llvm_ldu_global_p(ptr.cast(), 8).cast()
}

/// Prefetches the tile of the tensor described by the tensor map `tmap` at the coordinates
/// `coords` into the L2 cache.
///
/// If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is the
/// L2 cache eviction policy used for the prefetch (`.L2::cache_hint`).
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-prefetch-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_prefetch_tensor_1d_tile<const CACHE_HINT: bool>(
    tmap: *const c_void,
    coords: [i32; 1],
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_prefetch_tile_1d(tmap, coords[0], cache_policy, CACHE_HINT)
}

/// Prefetches the 2D tile of the tensor described by the tensor map `tmap` at the coordinates
/// `coords` into the L2 cache.
///
/// If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is the
/// L2 cache eviction policy used for the prefetch (`.L2::cache_hint`).
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-prefetch-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_prefetch_tensor_2d_tile<const CACHE_HINT: bool>(
    tmap: *const c_void,
    coords: [i32; 2],
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_prefetch_tile_2d(tmap, coords[0], coords[1], cache_policy, CACHE_HINT)
}

/// Prefetches the 3D tile of the tensor described by the tensor map `tmap` at the coordinates
/// `coords` into the L2 cache.
///
/// If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is the
/// L2 cache eviction policy used for the prefetch (`.L2::cache_hint`).
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-prefetch-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_prefetch_tensor_3d_tile<const CACHE_HINT: bool>(
    tmap: *const c_void,
    coords: [i32; 3],
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_prefetch_tile_3d(
        tmap,
        coords[0],
        coords[1],
        coords[2],
        cache_policy,
        CACHE_HINT,
    )
}

/// Prefetches the 4D tile of the tensor described by the tensor map `tmap` at the coordinates
/// `coords` into the L2 cache.
///
/// If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is the
/// L2 cache eviction policy used for the prefetch (`.L2::cache_hint`).
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-prefetch-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_prefetch_tensor_4d_tile<const CACHE_HINT: bool>(
    tmap: *const c_void,
    coords: [i32; 4],
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_prefetch_tile_4d(
        tmap,
        coords[0],
        coords[1],
        coords[2],
        coords[3],
        cache_policy,
        CACHE_HINT,
    )
}

/// Prefetches the 5D tile of the tensor described by the tensor map `tmap` at the coordinates
/// `coords` into the L2 cache.
///
/// If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is the
/// L2 cache eviction policy used for the prefetch (`.L2::cache_hint`).
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-prefetch-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_prefetch_tensor_5d_tile<const CACHE_HINT: bool>(
    tmap: *const c_void,
    coords: [i32; 5],
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_prefetch_tile_5d(
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

/// Prefetches the 3D im2col-layout tensor data described by the tensor map `tmap` at the
/// coordinates `coords` into the L2 cache, using the im2col offsets `im2col_offsets`.
///
/// If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is the
/// L2 cache eviction policy used for the prefetch (`.L2::cache_hint`).
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-prefetch-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_prefetch_tensor_3d_im2col<const CACHE_HINT: bool>(
    tmap: *const c_void,
    coords: [i32; 3],
    im2col_offsets: [u16; 1],
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_prefetch_im2col_3d(
        tmap,
        coords[0],
        coords[1],
        coords[2],
        im2col_offsets[0],
        cache_policy,
        CACHE_HINT,
    )
}

/// Prefetches the 4D im2col-layout tensor data described by the tensor map `tmap` at the
/// coordinates `coords` into the L2 cache, using the im2col offsets `im2col_offsets`.
///
/// If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is the
/// L2 cache eviction policy used for the prefetch (`.L2::cache_hint`).
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-prefetch-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_prefetch_tensor_4d_im2col<const CACHE_HINT: bool>(
    tmap: *const c_void,
    coords: [i32; 4],
    im2col_offsets: [u16; 2],
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_prefetch_im2col_4d(
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

/// Prefetches the 5D im2col-layout tensor data described by the tensor map `tmap` at the
/// coordinates `coords` into the L2 cache, using the im2col offsets `im2col_offsets`.
///
/// If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is the
/// L2 cache eviction policy used for the prefetch (`.L2::cache_hint`).
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-prefetch-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_prefetch_tensor_5d_im2col<const CACHE_HINT: bool>(
    tmap: *const c_void,
    coords: [i32; 5],
    im2col_offsets: [u16; 3],
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_prefetch_im2col_5d(
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

/// Prefetches the 3D im2col::w-layout tensor data described by the tensor map `tmap` at the
/// coordinates `coords` into the L2 cache, using the `w_halo` and `w_offset` offsets.
///
/// If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is the
/// L2 cache eviction policy used for the prefetch (`.L2::cache_hint`).
///
/// Requires `sm_100a`/`sm_101a` with PTX ISA 8.6, or `sm_100f`/`sm_101f` with PTX ISA 8.8, or
/// `sm_100f`/`sm_110f` with PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-prefetch-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_prefetch_tensor_3d_im2col_w<const CACHE_HINT: bool>(
    tmap: *const c_void,
    coords: [i32; 3],
    w_halo: u16,
    w_offset: u16,
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_prefetch_im2col_w_3d(
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

/// Prefetches the 4D im2col::w-layout tensor data described by the tensor map `tmap` at the
/// coordinates `coords` into the L2 cache, using the `w_halo` and `w_offset` offsets.
///
/// If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is the
/// L2 cache eviction policy used for the prefetch (`.L2::cache_hint`).
///
/// Requires `sm_100a`/`sm_101a` with PTX ISA 8.6, or `sm_100f`/`sm_101f` with PTX ISA 8.8, or
/// `sm_100f`/`sm_110f` with PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-prefetch-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_prefetch_tensor_4d_im2col_w<const CACHE_HINT: bool>(
    tmap: *const c_void,
    coords: [i32; 4],
    w_halo: u16,
    w_offset: u16,
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_prefetch_im2col_w_4d(
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

/// Prefetches the 5D im2col::w-layout tensor data described by the tensor map `tmap` at the
/// coordinates `coords` into the L2 cache, using the `w_halo` and `w_offset` offsets.
///
/// If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is the
/// L2 cache eviction policy used for the prefetch (`.L2::cache_hint`).
///
/// Requires `sm_100a`/`sm_101a` with PTX ISA 8.6, or `sm_100f`/`sm_101f` with PTX ISA 8.8, or
/// `sm_100f`/`sm_110f` with PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-prefetch-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_prefetch_tensor_5d_im2col_w<const CACHE_HINT: bool>(
    tmap: *const c_void,
    coords: [i32; 5],
    w_halo: u16,
    w_offset: u16,
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_prefetch_im2col_w_5d(
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

/// Prefetches the 3D im2col::w::128-layout tensor data described by the tensor map `tmap` at
/// the coordinates `coords` into the L2 cache, using the `w_halo` and `w_offset` offsets.
///
/// If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is the
/// L2 cache eviction policy used for the prefetch (`.L2::cache_hint`).
///
/// Requires `sm_100a`/`sm_101a` with PTX ISA 8.6, or `sm_100f`/`sm_101f` with PTX ISA 8.8, or
/// `sm_100f`/`sm_110f` with PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-prefetch-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_prefetch_tensor_3d_im2col_w_128<const CACHE_HINT: bool>(
    tmap: *const c_void,
    coords: [i32; 3],
    w_halo: u16,
    w_offset: u16,
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_prefetch_im2col_w_128_3d(
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

/// Prefetches the 4D im2col::w::128-layout tensor data described by the tensor map `tmap` at
/// the coordinates `coords` into the L2 cache, using the `w_halo` and `w_offset` offsets.
///
/// If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is the
/// L2 cache eviction policy used for the prefetch (`.L2::cache_hint`).
///
/// Requires `sm_100a`/`sm_101a` with PTX ISA 8.6, or `sm_100f`/`sm_101f` with PTX ISA 8.8, or
/// `sm_100f`/`sm_110f` with PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-prefetch-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_prefetch_tensor_4d_im2col_w_128<const CACHE_HINT: bool>(
    tmap: *const c_void,
    coords: [i32; 4],
    w_halo: u16,
    w_offset: u16,
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_prefetch_im2col_w_128_4d(
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

/// Prefetches the 5D im2col::w::128-layout tensor data described by the tensor map `tmap` at
/// the coordinates `coords` into the L2 cache, using the `w_halo` and `w_offset` offsets.
///
/// If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is the
/// L2 cache eviction policy used for the prefetch (`.L2::cache_hint`).
///
/// Requires `sm_100a`/`sm_101a` with PTX ISA 8.6, or `sm_100f`/`sm_101f` with PTX ISA 8.8, or
/// `sm_100f`/`sm_110f` with PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-prefetch-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_prefetch_tensor_5d_im2col_w_128<const CACHE_HINT: bool>(
    tmap: *const c_void,
    coords: [i32; 5],
    w_halo: u16,
    w_offset: u16,
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_prefetch_im2col_w_128_5d(
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

/// Prefetches the 2D gather4 tile of the tensor described by the tensor map `tmap` at the
/// coordinates `coords` into the L2 cache.
///
/// If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is the
/// L2 cache eviction policy used for the prefetch (`.L2::cache_hint`).
///
/// Requires `sm_100a`/`sm_101a` with PTX ISA 8.6, or `sm_100f`/`sm_101f` with PTX ISA 8.8, or
/// `sm_100f`/`sm_110f` with PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-prefetch-tensor>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_prefetch_tensor_2d_tile_gather4<const CACHE_HINT: bool>(
    tmap: *const c_void,
    coords: [i32; 5],
    cache_policy: u64,
) {
    llvm_cp_async_bulk_tensor_prefetch_tile_gather4_2d(
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
