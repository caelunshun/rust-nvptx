// --- LLM-generated --- //
//! Asynchronous copies: `cp.async` and the non-tensor `cp.async.bulk` instructions.
//! NVPTX intrinsics (experimental)
//!
//! These intrinsics form the foundation of the CUDA
//! programming model.
//!
//! The reference is the [CUDA C Programming Guide][cuda_c]. Relevant is also
//! the [LLVM NVPTX Backend documentation][llvm_docs].
//!
//! [cuda_c]:
//! http://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html
//! [llvm_docs]:
//! https://llvm.org/docs/NVPTXUsage.html

use super::StateSpace;
use crate::ffi::c_void;

#[allow(improper_ctypes)]
unsafe extern "llvm-intrinsic" {
    #[link_name = "llvm.nvvm.cp.async.ca.shared.global.4"]
    fn llvm_cp_async_ca_4(dst: *mut c_void, src: *const c_void);
    #[link_name = "llvm.nvvm.cp.async.ca.shared.global.4.s"]
    fn llvm_cp_async_ca_4_s(dst: *mut c_void, src: *const c_void, src_size: i32);
    #[link_name = "llvm.nvvm.cp.async.ca.shared.global.8"]
    fn llvm_cp_async_ca_8(dst: *mut c_void, src: *const c_void);
    #[link_name = "llvm.nvvm.cp.async.ca.shared.global.8.s"]
    fn llvm_cp_async_ca_8_s(dst: *mut c_void, src: *const c_void, src_size: i32);
    #[link_name = "llvm.nvvm.cp.async.ca.shared.global.16"]
    fn llvm_cp_async_ca_16(dst: *mut c_void, src: *const c_void);
    #[link_name = "llvm.nvvm.cp.async.ca.shared.global.16.s"]
    fn llvm_cp_async_ca_16_s(dst: *mut c_void, src: *const c_void, src_size: i32);
    #[link_name = "llvm.nvvm.cp.async.cg.shared.global.16"]
    fn llvm_cp_async_cg_16(dst: *mut c_void, src: *const c_void);
    #[link_name = "llvm.nvvm.cp.async.cg.shared.global.16.s"]
    fn llvm_cp_async_cg_16_s(dst: *mut c_void, src: *const c_void, src_size: i32);
    #[link_name = "llvm.nvvm.cp.async.commit.group"]
    fn llvm_cp_async_commit_group();
    #[link_name = "llvm.nvvm.cp.async.wait.group"]
    fn llvm_cp_async_wait_group(n: i32);
    #[link_name = "llvm.nvvm.cp.async.wait.all"]
    fn llvm_cp_async_wait_all();
    #[link_name = "llvm.nvvm.cp.async.mbarrier.arrive"]
    fn llvm_cp_async_mbarrier_arrive(mbar: *mut c_void);
    #[link_name = "llvm.nvvm.cp.async.mbarrier.arrive.noinc"]
    fn llvm_cp_async_mbarrier_arrive_noinc(mbar: *mut c_void);
    #[link_name = "llvm.nvvm.cp.async.mbarrier.arrive.shared"]
    fn llvm_cp_async_mbarrier_arrive_shared(mbar: *mut c_void);
    #[link_name = "llvm.nvvm.cp.async.mbarrier.arrive.noinc.shared"]
    fn llvm_cp_async_mbarrier_arrive_noinc_shared(mbar: *mut c_void);
    #[link_name = "llvm.nvvm.cp.async.bulk.commit.group"]
    fn llvm_cp_async_bulk_commit_group();
    #[link_name = "llvm.nvvm.cp.async.bulk.wait.group"]
    fn llvm_cp_async_bulk_wait_group(n: i32);
    #[link_name = "llvm.nvvm.cp.async.bulk.wait.group.read"]
    fn llvm_cp_async_bulk_wait_group_read(n: i32);
    #[link_name = "llvm.nvvm.cp.async.bulk.prefetch.L2"]
    fn llvm_cp_async_bulk_prefetch_l2(
        src: *const c_void,
        size: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.global.to.shared.cluster"]
    fn llvm_cp_async_bulk_global_to_shared_cluster(
        dst: *mut c_void,
        mbar: *mut c_void,
        src: *const c_void,
        size: i32,
        cta_mask: u16,
        cache_policy: u64,
        cache_hint: bool,
        multicast: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.global.to.shared.cta"]
    fn llvm_cp_async_bulk_global_to_shared_cta(
        dst: *mut c_void,
        mbar: *mut c_void,
        src: *const c_void,
        size: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.shared.cta.to.cluster"]
    fn llvm_cp_async_bulk_shared_cta_to_cluster(
        dst: *mut c_void,
        mbar: *mut c_void,
        src: *const c_void,
        size: i32,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.shared.cta.to.global"]
    fn llvm_cp_async_bulk_shared_cta_to_global(
        dst: *mut c_void,
        src: *const c_void,
        size: i32,
        cache_policy: u64,
        cache_hint: bool,
    );
    #[link_name = "llvm.nvvm.cp.async.bulk.shared.cta.to.global.bytemask"]
    fn llvm_cp_async_bulk_shared_cta_to_global_bytemask(
        dst: *mut c_void,
        src: *const c_void,
        size: i32,
        cache_policy: u64,
        cache_hint: bool,
        byte_mask: u16,
    );
}

/// Asynchronously copies `SIZE` bytes from global memory at `src` to shared memory of the
/// executing CTA at `dst`, caching the data at all levels.
///
/// `SIZE` must be 4, 8 or 16. `src` must point to global memory and `dst` to shared memory of
/// the executing CTA. Completion is tracked with [`cp_async_commit_group`] and
/// [`cp_async_wait_group`].
///
/// Requires `sm_80` and PTX ISA 7.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async>
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_ca<const SIZE: usize>(dst: *mut c_void, src: *const c_void) {
    static_assert!(
        matches!(SIZE, 4 | 8 | 16),
        "cp_async_ca only supports sizes of 4, 8 and 16 bytes"
    );
    match SIZE {
        4 => llvm_cp_async_ca_4(dst, src),
        8 => llvm_cp_async_ca_8(dst, src),
        16 => llvm_cp_async_ca_16(dst, src),
        _ => unreachable!(),
    }
}

/// Asynchronously copies `src_size` bytes from global memory at `src` to shared memory of the
/// executing CTA at `dst`, zero-filling the remaining bytes up to `SIZE`, and caching the data at
/// all levels.
///
/// `SIZE` must be 4, 8 or 16, and `src_size` must not exceed `SIZE`. `src` must point to global
/// memory and `dst` to shared memory of the executing CTA.
///
/// Requires `sm_80` and PTX ISA 7.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async>
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_ca_src_size<const SIZE: usize>(
    dst: *mut c_void,
    src: *const c_void,
    src_size: u32,
) {
    static_assert!(
        matches!(SIZE, 4 | 8 | 16),
        "cp_async_ca_src_size only supports sizes of 4, 8 and 16 bytes"
    );
    match SIZE {
        4 => llvm_cp_async_ca_4_s(dst, src, src_size as i32),
        8 => llvm_cp_async_ca_8_s(dst, src, src_size as i32),
        16 => llvm_cp_async_ca_16_s(dst, src, src_size as i32),
        _ => unreachable!(),
    }
}

/// Asynchronously copies 16 bytes from global memory at `src` to shared memory of the executing
/// CTA at `dst`, bypassing the L1 cache.
///
/// `src` must point to global memory and `dst` to shared memory of the executing CTA.
///
/// Requires `sm_80` and PTX ISA 7.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async>
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_cg(dst: *mut c_void, src: *const c_void) {
    llvm_cp_async_cg_16(dst, src)
}

/// Asynchronously copies `src_size` bytes from global memory at `src` to shared memory of the
/// executing CTA at `dst`, zero-filling the remaining bytes up to 16, and bypassing the L1 cache.
///
/// `src_size` must not exceed 16. `src` must point to global memory and `dst` to shared memory
/// of the executing CTA.
///
/// Requires `sm_80` and PTX ISA 7.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async>
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_cg_src_size(dst: *mut c_void, src: *const c_void, src_size: u32) {
    llvm_cp_async_cg_16_s(dst, src, src_size as i32)
}

/// Commits all prior uncommitted `cp.async` operations of the executing thread into a new
/// async group.
///
/// Requires `sm_80` and PTX ISA 7.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-commit-group>
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_commit_group() {
    llvm_cp_async_commit_group()
}

/// Waits until at most `N` of the most recently committed `cp.async` groups of the executing
/// thread are pending.
///
/// Requires `sm_80` and PTX ISA 7.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-wait-group>
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_wait_group<const N: u32>() {
    llvm_cp_async_wait_group(N as i32)
}

/// Waits until all prior `cp.async` operations of the executing thread have completed.
///
/// Requires `sm_80` and PTX ISA 7.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-wait-all>
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_wait_all() {
    llvm_cp_async_wait_all()
}

/// Makes the mbarrier object at `mbar` track the completion of all prior `cp.async` operations
/// of the executing thread.
///
/// If `NO_INC` is true, the pending count of the mbarrier is not incremented (`.noinc`). `SPACE`
/// is the state space of `mbar`: [`StateSpace::Generic`] or [`StateSpace::SharedCta`]; other
/// values are rejected at compile time.
///
/// Requires `sm_80` and PTX ISA 7.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-mbarrier-arrive>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_mbarrier_arrive<const NO_INC: bool, const SPACE: StateSpace>(
    mbar: *mut c_void,
) {
    static_assert!(
        matches!(SPACE, StateSpace::Generic | StateSpace::SharedCta),
        "cp_async_mbarrier_arrive only supports the generic and shared::cta state spaces"
    );
    match (NO_INC, SPACE) {
        (false, StateSpace::Generic) => llvm_cp_async_mbarrier_arrive(mbar),
        (true, StateSpace::Generic) => llvm_cp_async_mbarrier_arrive_noinc(mbar),
        (false, _) => llvm_cp_async_mbarrier_arrive_shared(mbar),
        (true, _) => llvm_cp_async_mbarrier_arrive_noinc_shared(mbar),
    }
}

/// Commits all prior uncommitted `cp.async.bulk` operations of the executing thread into a new
/// bulk async group.
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-commit-group>
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_commit_group() {
    llvm_cp_async_bulk_commit_group()
}

/// Waits until at most `N` of the most recently committed bulk async groups of the executing
/// thread are pending.
///
/// If `READ` is true, only waits until the source memory of the groups has been read
/// (`.read`).
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-wait-group>
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_wait_group<const N: u32, const READ: bool>() {
    match READ {
        true => llvm_cp_async_bulk_wait_group_read(N as i32),
        false => llvm_cp_async_bulk_wait_group(N as i32),
    }
}

/// Prefetches `size` bytes of global memory starting at `src` into the L2 cache.
///
/// If `CACHE_HINT` is `false`, `cache_policy` is ignored. Otherwise, `cache_policy` is the
/// L2 cache eviction policy used for the prefetch (`.L2::cache_hint`).
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk-prefetch>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_prefetch_l2<const CACHE_HINT: bool>(
    src: *const c_void,
    size: u32,
    cache_policy: u64,
) {
    llvm_cp_async_bulk_prefetch_l2(src, size as i32, cache_policy, CACHE_HINT)
}

/// Asynchronously copies `size` bytes from global memory at `src` to shared memory of the
/// cluster at `dst`, signalling completion on the mbarrier object at `mbar`.
///
/// `dst` must be a `shared::cluster` address, `mbar` a `shared::cta` address, and `src` a global
/// address. `size` must be a multiple of 16. If `MULTICAST` is true, the data is also copied to the CTAs of the cluster selected
/// by `cta_mask`; otherwise `cta_mask` is ignored. If `CACHE_HINT` is `false`, `cache_policy` is
/// ignored.
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_global_to_shared_cluster<
    const CACHE_HINT: bool,
    const MULTICAST: bool,
>(
    dst: *mut c_void,
    mbar: *mut c_void,
    src: *const c_void,
    size: u32,
    cta_mask: u16,
    cache_policy: u64,
) {
    llvm_cp_async_bulk_global_to_shared_cluster(
        dst,
        mbar,
        src,
        size as i32,
        cta_mask,
        cache_policy,
        CACHE_HINT,
        MULTICAST,
    )
}

/// Asynchronously copies `size` bytes from global memory at `src` to shared memory of the
/// executing CTA at `dst`, signalling completion on the mbarrier object at `mbar`.
///
/// `dst` and `mbar` must be `shared::cta` addresses and `src` a global address. `size` must be a
/// multiple of 16. If `CACHE_HINT` is `false`, `cache_policy` is ignored.
///
/// Requires `sm_90` and PTX ISA 8.6.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx86")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_global_to_shared_cta<const CACHE_HINT: bool>(
    dst: *mut c_void,
    mbar: *mut c_void,
    src: *const c_void,
    size: u32,
    cache_policy: u64,
) {
    llvm_cp_async_bulk_global_to_shared_cta(dst, mbar, src, size as i32, cache_policy, CACHE_HINT)
}

/// Asynchronously copies `size` bytes from shared memory of the executing CTA at `src` to shared
/// memory of the cluster at `dst`, signalling completion on the mbarrier object at `mbar`.
///
/// `dst` must be a `shared::cluster` address, and `src` and `mbar` must be `shared::cta`
/// addresses. `size` must be a multiple of 16.
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_shared_cta_to_cluster(
    dst: *mut c_void,
    mbar: *mut c_void,
    src: *const c_void,
    size: u32,
) {
    llvm_cp_async_bulk_shared_cta_to_cluster(dst, mbar, src, size as i32)
}

/// Asynchronously copies `size` bytes from shared memory of the executing CTA at `src` to global
/// memory at `dst`.
///
/// `src` must be a `shared::cta` address and `dst` a global address. `size` must be a multiple of
/// 16. If `CACHE_HINT` is `false`, `cache_policy` is ignored.
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_shared_cta_to_global<const CACHE_HINT: bool>(
    dst: *mut c_void,
    src: *const c_void,
    size: u32,
    cache_policy: u64,
) {
    llvm_cp_async_bulk_shared_cta_to_global(dst, src, size as i32, cache_policy, CACHE_HINT)
}

/// Asynchronously copies `size` bytes from shared memory of the executing CTA at `src` to global
/// memory at `dst`, copying the `i`-th byte of each 16-byte chunk only if bit `i` of `byte_mask`
/// is set.
///
/// `src` must be a `shared::cta` address and `dst` a global address. `size` must be a multiple of
/// 16. If `CACHE_HINT` is `false`, `cache_policy` is ignored.
///
/// Requires `sm_100` and PTX ISA 8.6.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_100,ptx86")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cp_async_bulk_shared_cta_to_global_bytemask<const CACHE_HINT: bool>(
    dst: *mut c_void,
    src: *const c_void,
    size: u32,
    cache_policy: u64,
    byte_mask: u16,
) {
    llvm_cp_async_bulk_shared_cta_to_global_bytemask(
        dst,
        src,
        size as i32,
        cache_policy,
        CACHE_HINT,
        byte_mask,
    )
}
