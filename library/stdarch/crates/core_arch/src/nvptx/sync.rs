// --- LLM-generated --- //
//! Synchronization, barrier and memory fence instructions.

use super::{Scope, Semantics};
use crate::ffi::c_void;
use crate::marker::ConstParamTy;

#[allow(improper_ctypes)]
unsafe extern "llvm-intrinsic" {
    #[link_name = "llvm.nvvm.barrier.cta.sync.all"]
    fn llvm_barrier_sync(a: u32);
    #[link_name = "llvm.nvvm.barrier.cta.sync.aligned.all"]
    fn llvm_barrier_sync_aligned(a: u32);
    #[link_name = "llvm.nvvm.barrier.cta.sync.count"]
    fn llvm_barrier_sync_count(a: u32, b: u32);
    #[link_name = "llvm.nvvm.barrier.cta.sync.aligned.count"]
    fn llvm_barrier_sync_aligned_count(a: u32, b: u32);
    #[link_name = "llvm.nvvm.barrier.cta.arrive.count"]
    fn llvm_barrier_arrive(a: u32, b: u32);
    #[link_name = "llvm.nvvm.barrier.cta.arrive.aligned.count"]
    fn llvm_barrier_arrive_aligned(a: u32, b: u32);
    #[link_name = "llvm.nvvm.barrier.cta.red.popc.all"]
    fn llvm_barrier_red_popc(a: u32, c: bool) -> u32;
    #[link_name = "llvm.nvvm.barrier.cta.red.popc.aligned.all"]
    fn llvm_barrier_red_popc_aligned(a: u32, c: bool) -> u32;
    #[link_name = "llvm.nvvm.barrier.cta.red.popc.count"]
    fn llvm_barrier_red_popc_count(a: u32, b: u32, c: bool) -> u32;
    #[link_name = "llvm.nvvm.barrier.cta.red.popc.aligned.count"]
    fn llvm_barrier_red_popc_aligned_count(a: u32, b: u32, c: bool) -> u32;
    #[link_name = "llvm.nvvm.barrier.cta.red.and.all"]
    fn llvm_barrier_red_and(a: u32, c: bool) -> bool;
    #[link_name = "llvm.nvvm.barrier.cta.red.and.aligned.all"]
    fn llvm_barrier_red_and_aligned(a: u32, c: bool) -> bool;
    #[link_name = "llvm.nvvm.barrier.cta.red.and.count"]
    fn llvm_barrier_red_and_count(a: u32, b: u32, c: bool) -> bool;
    #[link_name = "llvm.nvvm.barrier.cta.red.and.aligned.count"]
    fn llvm_barrier_red_and_aligned_count(a: u32, b: u32, c: bool) -> bool;
    #[link_name = "llvm.nvvm.barrier.cta.red.or.all"]
    fn llvm_barrier_red_or(a: u32, c: bool) -> bool;
    #[link_name = "llvm.nvvm.barrier.cta.red.or.aligned.all"]
    fn llvm_barrier_red_or_aligned(a: u32, c: bool) -> bool;
    #[link_name = "llvm.nvvm.barrier.cta.red.or.count"]
    fn llvm_barrier_red_or_count(a: u32, b: u32, c: bool) -> bool;
    #[link_name = "llvm.nvvm.barrier.cta.red.or.aligned.count"]
    fn llvm_barrier_red_or_aligned_count(a: u32, b: u32, c: bool) -> bool;
    #[link_name = "llvm.nvvm.bar.warp.sync"]
    fn llvm_bar_warp_sync(membermask: u32);
    #[link_name = "llvm.nvvm.barrier.cluster.arrive"]
    fn llvm_barrier_cluster_arrive();
    #[link_name = "llvm.nvvm.barrier.cluster.arrive.aligned"]
    fn llvm_barrier_cluster_arrive_aligned();
    #[link_name = "llvm.nvvm.barrier.cluster.arrive.relaxed"]
    fn llvm_barrier_cluster_arrive_relaxed();
    #[link_name = "llvm.nvvm.barrier.cluster.arrive.relaxed.aligned"]
    fn llvm_barrier_cluster_arrive_relaxed_aligned();
    #[link_name = "llvm.nvvm.barrier.cluster.wait"]
    fn llvm_barrier_cluster_wait();
    #[link_name = "llvm.nvvm.barrier.cluster.wait.aligned"]
    fn llvm_barrier_cluster_wait_aligned();
    #[link_name = "llvm.nvvm.membar.cta"]
    fn llvm_membar_cta();
    #[link_name = "llvm.nvvm.membar.gl"]
    fn llvm_membar_gl();
    #[link_name = "llvm.nvvm.membar.sys"]
    fn llvm_membar_sys();
    #[link_name = "llvm.nvvm.fence.sc.cluster"]
    fn llvm_fence_sc_cluster();
    #[link_name = "llvm.nvvm.fence.acquire.sync_restrict.space.cluster.scope.cluster"]
    fn llvm_fence_acquire_sync_restrict();
    #[link_name = "llvm.nvvm.fence.release.sync_restrict.space.cta.scope.cluster"]
    fn llvm_fence_release_sync_restrict();
    #[link_name = "llvm.nvvm.fence.mbarrier_init.release.cluster"]
    fn llvm_fence_mbarrier_init();
    #[link_name = "llvm.nvvm.fence.proxy.alias"]
    fn llvm_fence_proxy_alias();
    #[link_name = "llvm.nvvm.fence.proxy.async"]
    fn llvm_fence_proxy_async();
    #[link_name = "llvm.nvvm.fence.proxy.async.global"]
    fn llvm_fence_proxy_async_global();
    #[link_name = "llvm.nvvm.fence.proxy.async.shared_cta"]
    fn llvm_fence_proxy_async_shared_cta();
    #[link_name = "llvm.nvvm.fence.proxy.async.shared_cluster"]
    fn llvm_fence_proxy_async_shared_cluster();
    #[link_name = "llvm.nvvm.fence.proxy.async_generic.acquire.sync_restrict.space.cluster.scope.cluster"]
    fn llvm_fence_proxy_async_generic_acquire_sync_restrict();
    #[link_name = "llvm.nvvm.fence.proxy.async_generic.release.sync_restrict.space.cta.scope.cluster"]
    fn llvm_fence_proxy_async_generic_release_sync_restrict();
    #[link_name = "llvm.nvvm.fence.proxy.tensormap_generic.release.cta"]
    fn llvm_fence_proxy_tensormap_release_cta();
    #[link_name = "llvm.nvvm.fence.proxy.tensormap_generic.release.cluster"]
    fn llvm_fence_proxy_tensormap_release_cluster();
    #[link_name = "llvm.nvvm.fence.proxy.tensormap_generic.release.gpu"]
    fn llvm_fence_proxy_tensormap_release_gpu();
    #[link_name = "llvm.nvvm.fence.proxy.tensormap_generic.release.sys"]
    fn llvm_fence_proxy_tensormap_release_sys();
    #[link_name = "llvm.nvvm.fence.proxy.tensormap_generic.acquire.cta"]
    fn llvm_fence_proxy_tensormap_acquire_cta(addr: *const c_void, size: u32);
    #[link_name = "llvm.nvvm.fence.proxy.tensormap_generic.acquire.cluster"]
    fn llvm_fence_proxy_tensormap_acquire_cluster(addr: *const c_void, size: u32);
    #[link_name = "llvm.nvvm.fence.proxy.tensormap_generic.acquire.gpu"]
    fn llvm_fence_proxy_tensormap_acquire_gpu(addr: *const c_void, size: u32);
    #[link_name = "llvm.nvvm.fence.proxy.tensormap_generic.acquire.sys"]
    fn llvm_fence_proxy_tensormap_acquire_sys(addr: *const c_void, size: u32);
    #[link_name = "llvm.nvvm.mbarrier.init"]
    fn llvm_mbarrier_init(addr: *mut c_void, count: u32);
    #[link_name = "llvm.nvvm.mbarrier.inval"]
    fn llvm_mbarrier_inval(addr: *mut c_void);
    #[link_name = "llvm.nvvm.mbarrier.arrive"]
    fn llvm_mbarrier_arrive(addr: *mut c_void) -> u64;
    #[link_name = "llvm.nvvm.mbarrier.arrive.noComplete"]
    fn llvm_mbarrier_arrive_nocomplete(addr: *mut c_void, count: u32) -> u64;
    #[link_name = "llvm.nvvm.mbarrier.arrive.drop"]
    fn llvm_mbarrier_arrive_drop(addr: *mut c_void) -> u64;
    #[link_name = "llvm.nvvm.mbarrier.arrive.drop.noComplete"]
    fn llvm_mbarrier_arrive_drop_nocomplete(addr: *mut c_void, count: u32) -> u64;
    #[link_name = "llvm.nvvm.mbarrier.test.wait"]
    fn llvm_mbarrier_test_wait(addr: *mut c_void, state: u64) -> bool;
    #[link_name = "llvm.nvvm.mbarrier.pending.count"]
    fn llvm_mbarrier_pending_count(state: u64) -> u32;
    #[link_name = "llvm.nvvm.griddepcontrol.launch.dependents"]
    fn llvm_griddepcontrol_launch_dependents();
    #[link_name = "llvm.nvvm.griddepcontrol.wait"]
    fn llvm_griddepcontrol_wait();
    #[link_name = "llvm.nvvm.clusterlaunchcontrol.query_cancel.is_canceled"]
    fn llvm_clusterlaunchcontrol_query_cancel_is_canceled(response: u128) -> bool;
    #[link_name = "llvm.nvvm.clusterlaunchcontrol.query_cancel.get_first_ctaid.x"]
    fn llvm_clusterlaunchcontrol_query_cancel_get_first_ctaid_x(response: u128) -> u32;
    #[link_name = "llvm.nvvm.clusterlaunchcontrol.query_cancel.get_first_ctaid.y"]
    fn llvm_clusterlaunchcontrol_query_cancel_get_first_ctaid_y(response: u128) -> u32;
    #[link_name = "llvm.nvvm.clusterlaunchcontrol.query_cancel.get_first_ctaid.z"]
    fn llvm_clusterlaunchcontrol_query_cancel_get_first_ctaid_z(response: u128) -> u32;
}

/// Reduction operation of [`barrier_red_pred`] and [`barrier_red_pred_count`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, ConstParamTy)]
#[non_exhaustive]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub enum BarrierRedOp {
    /// True if the predicate is true for all participating threads (`.and`).
    And,
    /// True if the predicate is true for any participating thread (`.or`).
    Or,
}

/// Proxy whose memory accesses are ordered against the generic proxy by [`fence_proxy`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, ConstParamTy)]
#[non_exhaustive]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub enum ProxyKind {
    /// Accesses performed through virtually aliased addresses (`.alias`).
    Alias,
    /// The async proxy (`.async`).
    Async,
    /// The async proxy, global state space (`.async.global`).
    AsyncGlobal,
    /// The async proxy, `.shared::cta` state space (`.async.shared::cta`).
    AsyncSharedCta,
    /// The async proxy, `.shared::cluster` state space (`.async.shared::cluster`).
    AsyncSharedCluster,
}

/// Action performed by [`griddepcontrol`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, ConstParamTy)]
#[non_exhaustive]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub enum GriddepcontrolAction {
    /// Allows dependent grids to be scheduled (`.launch_dependents`).
    LaunchDependents,
    /// Waits for all prerequisite grids to complete (`.wait`).
    Wait,
}

/// Waits at the CTA barrier `a` until all non-exited threads of the CTA have arrived.
///
/// `barrier_sync::<true>(0)` is equivalent to CUDA's `__syncthreads()`. If `ALIGNED` is true,
/// all threads of the CTA must execute the same barrier instruction.
///
/// Requires PTX ISA 6.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-bar>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn barrier_sync<const ALIGNED: bool>(a: u32) {
    match ALIGNED {
        true => llvm_barrier_sync_aligned(a),
        false => llvm_barrier_sync(a),
    }
}

/// Waits at the CTA barrier `a` until `b` threads have arrived.
///
/// `b` must be a multiple of the warp size. If `ALIGNED` is true, all threads of the CTA must
/// execute the same barrier instruction.
///
/// Requires PTX ISA 6.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-bar>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn barrier_sync_count<const ALIGNED: bool>(a: u32, b: u32) {
    match ALIGNED {
        true => llvm_barrier_sync_aligned_count(a, b),
        false => llvm_barrier_sync_count(a, b),
    }
}

/// Signals the arrival of the executing threads at the CTA barrier `a` expecting `b` threads,
/// without waiting for the other threads.
///
/// `b` must be a non-zero multiple of the warp size. If `ALIGNED` is true, all threads of the
/// CTA must execute the same barrier instruction.
///
/// Requires PTX ISA 6.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-bar>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn barrier_arrive<const ALIGNED: bool>(a: u32, b: u32) {
    match ALIGNED {
        true => llvm_barrier_arrive_aligned(a, b),
        false => llvm_barrier_arrive(a, b),
    }
}

/// Waits at the CTA barrier `a` until all non-exited threads of the CTA have arrived, and returns
/// the number of threads for which `c` is true.
///
/// If `ALIGNED` is true, all threads of the CTA must execute the same barrier instruction.
///
/// Requires PTX ISA 6.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-bar>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn barrier_red_popc<const ALIGNED: bool>(a: u32, c: bool) -> u32 {
    match ALIGNED {
        true => llvm_barrier_red_popc_aligned(a, c),
        false => llvm_barrier_red_popc(a, c),
    }
}

/// Waits at the CTA barrier `a` until `b` threads have arrived, and returns the number of
/// threads for which `c` is true.
///
/// `b` must be a multiple of the warp size. If `ALIGNED` is true, all threads of the CTA must
/// execute the same barrier instruction.
///
/// Requires PTX ISA 6.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-bar>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn barrier_red_popc_count<const ALIGNED: bool>(a: u32, b: u32, c: bool) -> u32 {
    match ALIGNED {
        true => llvm_barrier_red_popc_aligned_count(a, b, c),
        false => llvm_barrier_red_popc_count(a, b, c),
    }
}

/// Waits at the CTA barrier `a` until all non-exited threads of the CTA have arrived, and returns
/// the reduction of `c` over the threads with the operation `OP`.
///
/// If `ALIGNED` is true, all threads of the CTA must execute the same barrier instruction.
///
/// Requires PTX ISA 6.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-bar>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn barrier_red_pred<const OP: BarrierRedOp, const ALIGNED: bool>(
    a: u32,
    c: bool,
) -> bool {
    match (OP, ALIGNED) {
        (BarrierRedOp::And, false) => llvm_barrier_red_and(a, c),
        (BarrierRedOp::And, true) => llvm_barrier_red_and_aligned(a, c),
        (BarrierRedOp::Or, false) => llvm_barrier_red_or(a, c),
        (BarrierRedOp::Or, true) => llvm_barrier_red_or_aligned(a, c),
    }
}

/// Waits at the CTA barrier `a` until `b` threads have arrived, and returns the reduction of `c`
/// over the threads with the operation `OP`.
///
/// `b` must be a multiple of the warp size. If `ALIGNED` is true, all threads of the CTA must
/// execute the same barrier instruction.
///
/// Requires PTX ISA 6.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-bar>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn barrier_red_pred_count<const OP: BarrierRedOp, const ALIGNED: bool>(
    a: u32,
    b: u32,
    c: bool,
) -> bool {
    match (OP, ALIGNED) {
        (BarrierRedOp::And, false) => llvm_barrier_red_and_count(a, b, c),
        (BarrierRedOp::And, true) => llvm_barrier_red_and_aligned_count(a, b, c),
        (BarrierRedOp::Or, false) => llvm_barrier_red_or_count(a, b, c),
        (BarrierRedOp::Or, true) => llvm_barrier_red_or_aligned_count(a, b, c),
    }
}

/// Waits until all threads in `membermask` have executed `bar.warp.sync` with the same mask.
///
/// The executing thread must be a member of `membermask`. Also guarantees memory ordering
/// among the participating threads of the warp.
///
/// Requires PTX ISA 6.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-bar-warp-sync>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn bar_warp_sync(membermask: u32) {
    llvm_bar_warp_sync(membermask)
}

/// Signals the arrival of the executing thread at the cluster barrier, without waiting for the
/// other threads of the cluster.
///
/// `SEM` is [`Semantics::Release`] (the default, `barrier.cluster.arrive`) or
/// [`Semantics::Relaxed`] (`barrier.cluster.arrive.relaxed`); other values are rejected at
/// compile time. If `ALIGNED` is true, all threads of the warp must execute the same barrier
/// instruction.
///
/// Requires `sm_90`. `Release` requires PTX ISA 7.8, and `Relaxed` requires PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-barrier-cluster>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn barrier_cluster_arrive<const SEM: Semantics, const ALIGNED: bool>() {
    static_assert!(
        matches!(SEM, Semantics::Release | Semantics::Relaxed),
        "barrier_cluster_arrive only supports Release and Relaxed semantics"
    );
    match (SEM, ALIGNED) {
        (Semantics::Release, false) => llvm_barrier_cluster_arrive(),
        (Semantics::Release, true) => llvm_barrier_cluster_arrive_aligned(),
        (_, false) => llvm_barrier_cluster_arrive_relaxed(),
        (_, true) => llvm_barrier_cluster_arrive_relaxed_aligned(),
    }
}

/// Waits until all non-exited threads of the cluster have executed [`barrier_cluster_arrive`]
/// for the current barrier phase.
///
/// Also provides acquire semantics. If `ALIGNED` is true, all threads of the warp must execute
/// the same barrier instruction.
///
/// Requires `sm_90` and PTX ISA 7.8.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-barrier-cluster>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx78")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn barrier_cluster_wait<const ALIGNED: bool>() {
    match ALIGNED {
        true => llvm_barrier_cluster_wait_aligned(),
        false => llvm_barrier_cluster_wait(),
    }
}

/// Establishes a sequentially consistent memory fence at scope `SCOPE`.
///
/// `Scope::Cta`, `Scope::Gpu` and `Scope::Sys` use `membar.cta`, `membar.gl` and `membar.sys`.
/// On `sm_70` and higher these are synonyms of `fence.sc.cta`, `fence.sc.gpu` and `fence.sc.sys`.
/// `Scope::Cluster` uses `fence.sc.cluster`, which requires `sm_90` and PTX ISA 7.8.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-membar>
///
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn fence_sc<const SCOPE: Scope>() {
    match SCOPE {
        Scope::Cta => llvm_membar_cta(),
        Scope::Gpu => llvm_membar_gl(),
        Scope::Sys => llvm_membar_sys(),
        Scope::Cluster => llvm_fence_sc_cluster(),
    }
}

/// Establishes a memory fence restricted to the shared memory synchronization of the cluster.
///
/// `SEM` must be [`Semantics::Acquire`] (`fence.acquire.sync_restrict::shared::cluster.cluster`)
/// or [`Semantics::Release`] (`fence.release.sync_restrict::shared::cta.cluster`); other values
/// are rejected at compile time.
///
/// Requires `sm_90` and PTX ISA 8.6.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-membar>
///
#[inline]
#[target_feature(enable = "sm_90,ptx86")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn fence_sync_restrict<const SEM: Semantics>() {
    static_assert!(
        matches!(SEM, Semantics::Acquire | Semantics::Release),
        "fence_sync_restrict only supports Acquire and Release semantics"
    );
    match SEM {
        Semantics::Acquire => llvm_fence_acquire_sync_restrict(),
        _ => llvm_fence_release_sync_restrict(),
    }
}

/// Orders prior `mbarrier.init` operations on `.shared::cta` mbarrier objects by this thread
/// against subsequent operations at cluster scope.
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-membar>
///
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn fence_mbarrier_init() {
    llvm_fence_mbarrier_init()
}

/// Establishes a bi-directional memory ordering between the generic proxy and the proxy `KIND`.
///
/// [`ProxyKind::Alias`] requires PTX ISA 7.5. The async kinds require `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-membar>
///
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn fence_proxy<const KIND: ProxyKind>() {
    match KIND {
        ProxyKind::Alias => llvm_fence_proxy_alias(),
        ProxyKind::Async => llvm_fence_proxy_async(),
        ProxyKind::AsyncGlobal => llvm_fence_proxy_async_global(),
        ProxyKind::AsyncSharedCta => llvm_fence_proxy_async_shared_cta(),
        ProxyKind::AsyncSharedCluster => llvm_fence_proxy_async_shared_cluster(),
    }
}

/// Establishes a memory fence between the async proxy and the generic proxy, restricted to the
/// shared memory synchronization of the cluster.
///
/// `SEM` must be [`Semantics::Acquire`] (`fence.proxy.async::generic.acquire.sync_restrict::shared::cluster.cluster`)
/// or [`Semantics::Release`] (`fence.proxy.async::generic.release.sync_restrict::shared::cta.cluster`);
/// other values are rejected at compile time.
///
/// Requires `sm_90` and PTX ISA 8.6.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-membar>
///
#[inline]
#[target_feature(enable = "sm_90,ptx86")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn fence_proxy_async_generic_sync_restrict<const SEM: Semantics>() {
    static_assert!(
        matches!(SEM, Semantics::Acquire | Semantics::Release),
        "fence_proxy_async_generic_sync_restrict only supports Acquire and Release semantics"
    );
    match SEM {
        Semantics::Acquire => llvm_fence_proxy_async_generic_acquire_sync_restrict(),
        _ => llvm_fence_proxy_async_generic_release_sync_restrict(),
    }
}

/// Releases prior generic proxy accesses to tensor maps, so that they are visible to the
/// tensor proxy at scope `SCOPE`.
///
/// Requires `sm_90` and PTX ISA 8.3.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-membar>
///
#[inline]
#[target_feature(enable = "sm_90,ptx83")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn fence_proxy_tensormap_generic_release<const SCOPE: Scope>() {
    match SCOPE {
        Scope::Cta => llvm_fence_proxy_tensormap_release_cta(),
        Scope::Cluster => llvm_fence_proxy_tensormap_release_cluster(),
        Scope::Gpu => llvm_fence_proxy_tensormap_release_gpu(),
        Scope::Sys => llvm_fence_proxy_tensormap_release_sys(),
    }
}

/// Acquires the tensor map at the generic address `addr` for the tensor proxy at scope `SCOPE`.
///
/// `addr` must fall within the `.global` state space. The 128-byte size of the tensor map is
/// passed implicitly, as it is the only supported value.
///
/// Requires `sm_90` and PTX ISA 8.3.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-membar>
///
#[inline]
#[target_feature(enable = "sm_90,ptx83")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn fence_proxy_tensormap_generic_acquire<const SCOPE: Scope>(addr: *const c_void) {
    match SCOPE {
        Scope::Cta => llvm_fence_proxy_tensormap_acquire_cta(addr, 128),
        Scope::Cluster => llvm_fence_proxy_tensormap_acquire_cluster(addr, 128),
        Scope::Gpu => llvm_fence_proxy_tensormap_acquire_gpu(addr, 128),
        Scope::Sys => llvm_fence_proxy_tensormap_acquire_sys(addr, 128),
    }
}

/// Initializes the mbarrier object at the generic address `addr` with the expected arrival
/// count `count`.
///
/// Requires `sm_80` and PTX ISA 7.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-mbarrier-init>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mbarrier_init(addr: *mut u64, count: u32) {
    llvm_mbarrier_init(addr.cast(), count)
}

/// Invalidates the mbarrier object at the generic address `addr`.
///
/// Requires `sm_80` and PTX ISA 7.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-mbarrier-inval>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mbarrier_inval(addr: *mut u64) {
    llvm_mbarrier_inval(addr.cast())
}

/// Performs an arrive-on operation on the mbarrier object at the generic address `addr`, and
/// returns the opaque state of the mbarrier object prior to the operation.
/// Requires `sm_80` and PTX ISA 7.0.
///
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-mbarrier-arrive>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mbarrier_arrive(addr: *mut u64) -> u64 {
    llvm_mbarrier_arrive(addr.cast())
}

/// Performs an arrive-on operation with the count `count` on the mbarrier object at the generic
/// address `addr`, and returns the opaque state of the mbarrier object prior to the operation.
///
/// The operation must not cause the mbarrier object to complete its current phase.
/// Requires `sm_80` and PTX ISA 7.0.
///
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-mbarrier-arrive>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mbarrier_arrive_nocomplete(addr: *mut u64, count: u32) -> u64 {
    llvm_mbarrier_arrive_nocomplete(addr.cast(), count)
}

/// Decrements the expected arrival count of the mbarrier object at the generic address `addr`,
/// performs an arrive-on operation, and returns the opaque state of the mbarrier object prior to
/// the arrive-on operation.
///
/// The decrement applies to all subsequent phases of the mbarrier object.
/// Requires `sm_80` and PTX ISA 7.0.
///
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-mbarrier-arrive-drop>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mbarrier_arrive_drop(addr: *mut u64) -> u64 {
    llvm_mbarrier_arrive_drop(addr.cast())
}

/// Decrements the expected arrival count of the mbarrier object at the generic address `addr` by
/// `count`, performs an arrive-on operation with the count `count`, and returns the opaque state
/// of the mbarrier object prior to the arrive-on operation.
///
/// The decrement applies to all subsequent phases of the mbarrier object. The operation must not
/// cause the mbarrier object to complete its current phase.
/// Requires `sm_80` and PTX ISA 7.0.
///
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-mbarrier-arrive-drop>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mbarrier_arrive_drop_nocomplete(addr: *mut u64, count: u32) -> u64 {
    llvm_mbarrier_arrive_drop_nocomplete(addr.cast(), count)
}

/// Tests whether the phase of the mbarrier object at the generic address `addr` identified by
/// `state` has completed.
///
/// Requires `sm_80` and PTX ISA 7.0.
///
/// `state` must be returned by an arrive operation on the same mbarrier object during the current
/// or the immediately preceding phase.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-mbarrier-test-wait-try-wait>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mbarrier_test_wait(addr: *mut u64, state: u64) -> bool {
    llvm_mbarrier_test_wait(addr.cast(), state)
}

/// Returns the pending arrival count of the mbarrier object from the opaque `state`.
///
/// Requires `sm_80` and PTX ISA 7.0.
///
/// `state` must be returned by [`mbarrier_arrive_nocomplete`] or [`mbarrier_arrive_drop_nocomplete`].
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-mbarrier-pending-count>
///
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mbarrier_pending_count(state: u64) -> u32 {
    llvm_mbarrier_pending_count(state)
}

/// Controls the execution of dependent grids: either allows dependents to be launched, or waits
/// for all prerequisite grids to complete.
///
/// Requires `sm_90` and PTX ISA 7.8.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-griddepcontrol>
///
#[inline]
#[target_feature(enable = "sm_90,ptx78")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn griddepcontrol<const ACTION: GriddepcontrolAction>() {
    match ACTION {
        GriddepcontrolAction::LaunchDependents => llvm_griddepcontrol_launch_dependents(),
        GriddepcontrolAction::Wait => llvm_griddepcontrol_wait(),
    }
}

/// Returns whether the cluster launch request recorded in `response` succeeded.
///
/// `response` is the 16-byte opaque response of a `clusterlaunchcontrol.try_cancel` operation.
///
/// Requires `sm_100` or higher and PTX ISA 8.6.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-clusterlaunchcontrol-query-cancel>
///
#[inline]
#[target_feature(enable = "sm_100,ptx86")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn clusterlaunchcontrol_query_cancel_is_canceled(response: u128) -> bool {
    llvm_clusterlaunchcontrol_query_cancel_is_canceled(response)
}

/// Returns the x coordinate of the first CTA of the canceled cluster recorded in `response`.
///
/// Only valid if [`clusterlaunchcontrol_query_cancel_is_canceled`] returns `true` for `response`.
///
/// Requires `sm_100` or higher and PTX ISA 8.6.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-clusterlaunchcontrol-query-cancel>
///
#[inline]
#[target_feature(enable = "sm_100,ptx86")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn clusterlaunchcontrol_query_cancel_get_first_ctaid_x(response: u128) -> u32 {
    llvm_clusterlaunchcontrol_query_cancel_get_first_ctaid_x(response)
}

/// Returns the y coordinate of the first CTA of the canceled cluster recorded in `response`.
///
/// Only valid if [`clusterlaunchcontrol_query_cancel_is_canceled`] returns `true` for `response`.
///
/// Requires `sm_100` or higher and PTX ISA 8.6.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-clusterlaunchcontrol-query-cancel>
///
#[inline]
#[target_feature(enable = "sm_100,ptx86")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn clusterlaunchcontrol_query_cancel_get_first_ctaid_y(response: u128) -> u32 {
    llvm_clusterlaunchcontrol_query_cancel_get_first_ctaid_y(response)
}

/// Returns the z coordinate of the first CTA of the canceled cluster recorded in `response`.
///
/// Only valid if [`clusterlaunchcontrol_query_cancel_is_canceled`] returns `true` for `response`.
///
/// Requires `sm_100` or higher and PTX ISA 8.6.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-clusterlaunchcontrol-query-cancel>
///
#[inline]
#[target_feature(enable = "sm_100,ptx86")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn clusterlaunchcontrol_query_cancel_get_first_ctaid_z(response: u128) -> u32 {
    llvm_clusterlaunchcontrol_query_cancel_get_first_ctaid_z(response)
}
