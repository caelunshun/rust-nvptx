// --- LLM-generated --- //
//! Memory barrier objects (`mbarrier`).

use super::{Scope, Semantics, StateSpace};
use crate::ffi::c_void;

#[allow(improper_ctypes)]
unsafe extern "llvm-intrinsic" {
    #[link_name = "llvm.nvvm.mbarrier.init"]
    fn llvm_mbarrier_init(addr: *mut c_void, count: u32);
    #[link_name = "llvm.nvvm.mbarrier.init.shared"]
    fn llvm_mbarrier_init_shared(addr: *mut c_void, count: u32);
    #[link_name = "llvm.nvvm.mbarrier.inval"]
    fn llvm_mbarrier_inval(addr: *mut c_void);
    #[link_name = "llvm.nvvm.mbarrier.inval.shared"]
    fn llvm_mbarrier_inval_shared(addr: *mut c_void);
    #[link_name = "llvm.nvvm.mbarrier.arrive"]
    fn llvm_mbarrier_arrive(addr: *mut c_void) -> u64;
    #[link_name = "llvm.nvvm.mbarrier.arrive.shared"]
    fn llvm_mbarrier_arrive_shared(addr: *mut c_void) -> u64;
    #[link_name = "llvm.nvvm.mbarrier.arrive.noComplete"]
    fn llvm_mbarrier_arrive_nocomplete(addr: *mut c_void, count: u32) -> u64;
    #[link_name = "llvm.nvvm.mbarrier.arrive.noComplete.shared"]
    fn llvm_mbarrier_arrive_nocomplete_shared(addr: *mut c_void, count: u32) -> u64;
    #[link_name = "llvm.nvvm.mbarrier.arrive.drop"]
    fn llvm_mbarrier_arrive_drop(addr: *mut c_void) -> u64;
    #[link_name = "llvm.nvvm.mbarrier.arrive.drop.shared"]
    fn llvm_mbarrier_arrive_drop_shared(addr: *mut c_void) -> u64;
    #[link_name = "llvm.nvvm.mbarrier.arrive.drop.noComplete"]
    fn llvm_mbarrier_arrive_drop_nocomplete(addr: *mut c_void, count: u32) -> u64;
    #[link_name = "llvm.nvvm.mbarrier.arrive.drop.noComplete.shared"]
    fn llvm_mbarrier_arrive_drop_nocomplete_shared(addr: *mut c_void, count: u32) -> u64;
    #[link_name = "llvm.nvvm.mbarrier.test.wait"]
    fn llvm_mbarrier_test_wait(addr: *mut c_void, state: u64) -> bool;
    #[link_name = "llvm.nvvm.mbarrier.test.wait.shared"]
    fn llvm_mbarrier_test_wait_shared(addr: *mut c_void, state: u64) -> bool;
    #[link_name = "llvm.nvvm.mbarrier.pending.count"]
    fn llvm_mbarrier_pending_count(state: u64) -> u32;

    #[link_name = "llvm.nvvm.mbarrier.arrive.scope.cta.space.cta"]
    fn llvm_mbarrier_arrive_scope_cta_space_cta(addr: *mut c_void, count: u32) -> u64;
    #[link_name = "llvm.nvvm.mbarrier.arrive.scope.cta.space.cluster"]
    fn llvm_mbarrier_arrive_scope_cta_space_cluster(addr: *mut c_void, count: u32);
    #[link_name = "llvm.nvvm.mbarrier.arrive.scope.cluster.space.cta"]
    fn llvm_mbarrier_arrive_scope_cluster_space_cta(addr: *mut c_void, count: u32) -> u64;
    #[link_name = "llvm.nvvm.mbarrier.arrive.scope.cluster.space.cluster"]
    fn llvm_mbarrier_arrive_scope_cluster_space_cluster(addr: *mut c_void, count: u32);
    #[link_name = "llvm.nvvm.mbarrier.arrive.relaxed.scope.cta.space.cta"]
    fn llvm_mbarrier_arrive_relaxed_scope_cta_space_cta(addr: *mut c_void, count: u32) -> u64;
    #[link_name = "llvm.nvvm.mbarrier.arrive.relaxed.scope.cta.space.cluster"]
    fn llvm_mbarrier_arrive_relaxed_scope_cta_space_cluster(addr: *mut c_void, count: u32);
    #[link_name = "llvm.nvvm.mbarrier.arrive.relaxed.scope.cluster.space.cta"]
    fn llvm_mbarrier_arrive_relaxed_scope_cluster_space_cta(addr: *mut c_void, count: u32) -> u64;
    #[link_name = "llvm.nvvm.mbarrier.arrive.relaxed.scope.cluster.space.cluster"]
    fn llvm_mbarrier_arrive_relaxed_scope_cluster_space_cluster(addr: *mut c_void, count: u32);

    #[link_name = "llvm.nvvm.mbarrier.arrive.expect.tx.scope.cta.space.cta"]
    fn llvm_mbarrier_arrive_expect_tx_scope_cta_space_cta(addr: *mut c_void, tx: u32) -> u64;
    #[link_name = "llvm.nvvm.mbarrier.arrive.expect.tx.scope.cta.space.cluster"]
    fn llvm_mbarrier_arrive_expect_tx_scope_cta_space_cluster(addr: *mut c_void, tx: u32);
    #[link_name = "llvm.nvvm.mbarrier.arrive.expect.tx.scope.cluster.space.cta"]
    fn llvm_mbarrier_arrive_expect_tx_scope_cluster_space_cta(addr: *mut c_void, tx: u32) -> u64;
    #[link_name = "llvm.nvvm.mbarrier.arrive.expect.tx.scope.cluster.space.cluster"]
    fn llvm_mbarrier_arrive_expect_tx_scope_cluster_space_cluster(addr: *mut c_void, tx: u32);
    #[link_name = "llvm.nvvm.mbarrier.arrive.expect.tx.relaxed.scope.cta.space.cta"]
    fn llvm_mbarrier_arrive_expect_tx_relaxed_scope_cta_space_cta(
        addr: *mut c_void,
        tx: u32,
    ) -> u64;
    #[link_name = "llvm.nvvm.mbarrier.arrive.expect.tx.relaxed.scope.cta.space.cluster"]
    fn llvm_mbarrier_arrive_expect_tx_relaxed_scope_cta_space_cluster(addr: *mut c_void, tx: u32);
    #[link_name = "llvm.nvvm.mbarrier.arrive.expect.tx.relaxed.scope.cluster.space.cta"]
    fn llvm_mbarrier_arrive_expect_tx_relaxed_scope_cluster_space_cta(
        addr: *mut c_void,
        tx: u32,
    ) -> u64;
    #[link_name = "llvm.nvvm.mbarrier.arrive.expect.tx.relaxed.scope.cluster.space.cluster"]
    fn llvm_mbarrier_arrive_expect_tx_relaxed_scope_cluster_space_cluster(
        addr: *mut c_void,
        tx: u32,
    );

    #[link_name = "llvm.nvvm.mbarrier.arrive.drop.scope.cta.space.cta"]
    fn llvm_mbarrier_arrive_drop_scope_cta_space_cta(addr: *mut c_void, count: u32) -> u64;
    #[link_name = "llvm.nvvm.mbarrier.arrive.drop.scope.cta.space.cluster"]
    fn llvm_mbarrier_arrive_drop_scope_cta_space_cluster(addr: *mut c_void, count: u32);
    #[link_name = "llvm.nvvm.mbarrier.arrive.drop.scope.cluster.space.cta"]
    fn llvm_mbarrier_arrive_drop_scope_cluster_space_cta(addr: *mut c_void, count: u32) -> u64;
    #[link_name = "llvm.nvvm.mbarrier.arrive.drop.scope.cluster.space.cluster"]
    fn llvm_mbarrier_arrive_drop_scope_cluster_space_cluster(addr: *mut c_void, count: u32);
    #[link_name = "llvm.nvvm.mbarrier.arrive.drop.relaxed.scope.cta.space.cta"]
    fn llvm_mbarrier_arrive_drop_relaxed_scope_cta_space_cta(addr: *mut c_void, count: u32) -> u64;
    #[link_name = "llvm.nvvm.mbarrier.arrive.drop.relaxed.scope.cta.space.cluster"]
    fn llvm_mbarrier_arrive_drop_relaxed_scope_cta_space_cluster(addr: *mut c_void, count: u32);
    #[link_name = "llvm.nvvm.mbarrier.arrive.drop.relaxed.scope.cluster.space.cta"]
    fn llvm_mbarrier_arrive_drop_relaxed_scope_cluster_space_cta(
        addr: *mut c_void,
        count: u32,
    ) -> u64;
    #[link_name = "llvm.nvvm.mbarrier.arrive.drop.relaxed.scope.cluster.space.cluster"]
    fn llvm_mbarrier_arrive_drop_relaxed_scope_cluster_space_cluster(addr: *mut c_void, count: u32);

    #[link_name = "llvm.nvvm.mbarrier.arrive.drop.expect.tx.scope.cta.space.cta"]
    fn llvm_mbarrier_arrive_drop_expect_tx_scope_cta_space_cta(addr: *mut c_void, tx: u32) -> u64;
    #[link_name = "llvm.nvvm.mbarrier.arrive.drop.expect.tx.scope.cta.space.cluster"]
    fn llvm_mbarrier_arrive_drop_expect_tx_scope_cta_space_cluster(addr: *mut c_void, tx: u32);
    #[link_name = "llvm.nvvm.mbarrier.arrive.drop.expect.tx.scope.cluster.space.cta"]
    fn llvm_mbarrier_arrive_drop_expect_tx_scope_cluster_space_cta(
        addr: *mut c_void,
        tx: u32,
    ) -> u64;
    #[link_name = "llvm.nvvm.mbarrier.arrive.drop.expect.tx.scope.cluster.space.cluster"]
    fn llvm_mbarrier_arrive_drop_expect_tx_scope_cluster_space_cluster(addr: *mut c_void, tx: u32);
    #[link_name = "llvm.nvvm.mbarrier.arrive.drop.expect.tx.relaxed.scope.cta.space.cta"]
    fn llvm_mbarrier_arrive_drop_expect_tx_relaxed_scope_cta_space_cta(
        addr: *mut c_void,
        tx: u32,
    ) -> u64;
    #[link_name = "llvm.nvvm.mbarrier.arrive.drop.expect.tx.relaxed.scope.cta.space.cluster"]
    fn llvm_mbarrier_arrive_drop_expect_tx_relaxed_scope_cta_space_cluster(
        addr: *mut c_void,
        tx: u32,
    );
    #[link_name = "llvm.nvvm.mbarrier.arrive.drop.expect.tx.relaxed.scope.cluster.space.cta"]
    fn llvm_mbarrier_arrive_drop_expect_tx_relaxed_scope_cluster_space_cta(
        addr: *mut c_void,
        tx: u32,
    ) -> u64;
    #[link_name = "llvm.nvvm.mbarrier.arrive.drop.expect.tx.relaxed.scope.cluster.space.cluster"]
    fn llvm_mbarrier_arrive_drop_expect_tx_relaxed_scope_cluster_space_cluster(
        addr: *mut c_void,
        tx: u32,
    );

    #[link_name = "llvm.nvvm.mbarrier.expect.tx.scope.cta.space.cta"]
    fn llvm_mbarrier_expect_tx_scope_cta_space_cta(addr: *mut c_void, tx: u32);
    #[link_name = "llvm.nvvm.mbarrier.expect.tx.scope.cta.space.cluster"]
    fn llvm_mbarrier_expect_tx_scope_cta_space_cluster(addr: *mut c_void, tx: u32);
    #[link_name = "llvm.nvvm.mbarrier.expect.tx.scope.cluster.space.cta"]
    fn llvm_mbarrier_expect_tx_scope_cluster_space_cta(addr: *mut c_void, tx: u32);
    #[link_name = "llvm.nvvm.mbarrier.expect.tx.scope.cluster.space.cluster"]
    fn llvm_mbarrier_expect_tx_scope_cluster_space_cluster(addr: *mut c_void, tx: u32);
    #[link_name = "llvm.nvvm.mbarrier.complete.tx.scope.cta.space.cta"]
    fn llvm_mbarrier_complete_tx_scope_cta_space_cta(addr: *mut c_void, tx: u32);
    #[link_name = "llvm.nvvm.mbarrier.complete.tx.scope.cta.space.cluster"]
    fn llvm_mbarrier_complete_tx_scope_cta_space_cluster(addr: *mut c_void, tx: u32);
    #[link_name = "llvm.nvvm.mbarrier.complete.tx.scope.cluster.space.cta"]
    fn llvm_mbarrier_complete_tx_scope_cluster_space_cta(addr: *mut c_void, tx: u32);
    #[link_name = "llvm.nvvm.mbarrier.complete.tx.scope.cluster.space.cluster"]
    fn llvm_mbarrier_complete_tx_scope_cluster_space_cluster(addr: *mut c_void, tx: u32);

    #[link_name = "llvm.nvvm.mbarrier.test.wait.scope.cta.space.cta"]
    fn llvm_mbarrier_test_wait_scope_cta_space_cta(addr: *mut c_void, state: u64) -> bool;
    #[link_name = "llvm.nvvm.mbarrier.test.wait.scope.cluster.space.cta"]
    fn llvm_mbarrier_test_wait_scope_cluster_space_cta(addr: *mut c_void, state: u64) -> bool;
    #[link_name = "llvm.nvvm.mbarrier.test.wait.relaxed.scope.cta.space.cta"]
    fn llvm_mbarrier_test_wait_relaxed_scope_cta_space_cta(addr: *mut c_void, state: u64) -> bool;
    #[link_name = "llvm.nvvm.mbarrier.test.wait.relaxed.scope.cluster.space.cta"]
    fn llvm_mbarrier_test_wait_relaxed_scope_cluster_space_cta(
        addr: *mut c_void,
        state: u64,
    ) -> bool;
    #[link_name = "llvm.nvvm.mbarrier.test.wait.parity.scope.cta.space.cta"]
    fn llvm_mbarrier_test_wait_parity_scope_cta_space_cta(addr: *mut c_void, parity: u32) -> bool;
    #[link_name = "llvm.nvvm.mbarrier.test.wait.parity.scope.cluster.space.cta"]
    fn llvm_mbarrier_test_wait_parity_scope_cluster_space_cta(
        addr: *mut c_void,
        parity: u32,
    ) -> bool;
    #[link_name = "llvm.nvvm.mbarrier.test.wait.parity.relaxed.scope.cta.space.cta"]
    fn llvm_mbarrier_test_wait_parity_relaxed_scope_cta_space_cta(
        addr: *mut c_void,
        parity: u32,
    ) -> bool;
    #[link_name = "llvm.nvvm.mbarrier.test.wait.parity.relaxed.scope.cluster.space.cta"]
    fn llvm_mbarrier_test_wait_parity_relaxed_scope_cluster_space_cta(
        addr: *mut c_void,
        parity: u32,
    ) -> bool;

    #[link_name = "llvm.nvvm.mbarrier.try.wait.scope.cta.space.cta"]
    fn llvm_mbarrier_try_wait_scope_cta_space_cta(addr: *mut c_void, state: u64) -> bool;
    #[link_name = "llvm.nvvm.mbarrier.try.wait.scope.cluster.space.cta"]
    fn llvm_mbarrier_try_wait_scope_cluster_space_cta(addr: *mut c_void, state: u64) -> bool;
    #[link_name = "llvm.nvvm.mbarrier.try.wait.relaxed.scope.cta.space.cta"]
    fn llvm_mbarrier_try_wait_relaxed_scope_cta_space_cta(addr: *mut c_void, state: u64) -> bool;
    #[link_name = "llvm.nvvm.mbarrier.try.wait.relaxed.scope.cluster.space.cta"]
    fn llvm_mbarrier_try_wait_relaxed_scope_cluster_space_cta(
        addr: *mut c_void,
        state: u64,
    ) -> bool;
    #[link_name = "llvm.nvvm.mbarrier.try.wait.parity.scope.cta.space.cta"]
    fn llvm_mbarrier_try_wait_parity_scope_cta_space_cta(addr: *mut c_void, parity: u32) -> bool;
    #[link_name = "llvm.nvvm.mbarrier.try.wait.parity.scope.cluster.space.cta"]
    fn llvm_mbarrier_try_wait_parity_scope_cluster_space_cta(
        addr: *mut c_void,
        parity: u32,
    ) -> bool;
    #[link_name = "llvm.nvvm.mbarrier.try.wait.parity.relaxed.scope.cta.space.cta"]
    fn llvm_mbarrier_try_wait_parity_relaxed_scope_cta_space_cta(
        addr: *mut c_void,
        parity: u32,
    ) -> bool;
    #[link_name = "llvm.nvvm.mbarrier.try.wait.parity.relaxed.scope.cluster.space.cta"]
    fn llvm_mbarrier_try_wait_parity_relaxed_scope_cluster_space_cta(
        addr: *mut c_void,
        parity: u32,
    ) -> bool;
    #[link_name = "llvm.nvvm.mbarrier.try.wait.tl.scope.cta.space.cta"]
    fn llvm_mbarrier_try_wait_tl_scope_cta_space_cta(
        addr: *mut c_void,
        state: u64,
        tl: u32,
    ) -> bool;
    #[link_name = "llvm.nvvm.mbarrier.try.wait.tl.scope.cluster.space.cta"]
    fn llvm_mbarrier_try_wait_tl_scope_cluster_space_cta(
        addr: *mut c_void,
        state: u64,
        tl: u32,
    ) -> bool;
    #[link_name = "llvm.nvvm.mbarrier.try.wait.tl.relaxed.scope.cta.space.cta"]
    fn llvm_mbarrier_try_wait_tl_relaxed_scope_cta_space_cta(
        addr: *mut c_void,
        state: u64,
        tl: u32,
    ) -> bool;
    #[link_name = "llvm.nvvm.mbarrier.try.wait.tl.relaxed.scope.cluster.space.cta"]
    fn llvm_mbarrier_try_wait_tl_relaxed_scope_cluster_space_cta(
        addr: *mut c_void,
        state: u64,
        tl: u32,
    ) -> bool;
    #[link_name = "llvm.nvvm.mbarrier.try.wait.parity.tl.scope.cta.space.cta"]
    fn llvm_mbarrier_try_wait_parity_tl_scope_cta_space_cta(
        addr: *mut c_void,
        parity: u32,
        tl: u32,
    ) -> bool;
    #[link_name = "llvm.nvvm.mbarrier.try.wait.parity.tl.scope.cluster.space.cta"]
    fn llvm_mbarrier_try_wait_parity_tl_scope_cluster_space_cta(
        addr: *mut c_void,
        parity: u32,
        tl: u32,
    ) -> bool;
    #[link_name = "llvm.nvvm.mbarrier.try.wait.parity.tl.relaxed.scope.cta.space.cta"]
    fn llvm_mbarrier_try_wait_parity_tl_relaxed_scope_cta_space_cta(
        addr: *mut c_void,
        parity: u32,
        tl: u32,
    ) -> bool;
    #[link_name = "llvm.nvvm.mbarrier.try.wait.parity.tl.relaxed.scope.cluster.space.cta"]
    fn llvm_mbarrier_try_wait_parity_tl_relaxed_scope_cluster_space_cta(
        addr: *mut c_void,
        parity: u32,
        tl: u32,
    ) -> bool;
}

/// Initializes the mbarrier object at the generic address `addr` with the expected arrival
/// count `count`.
///
/// `SPACE` must be [`StateSpace::Generic`] or [`StateSpace::SharedCta`], and `addr` must fall
/// within the window of `SPACE`. Other state spaces are rejected at compile time.
///
/// Requires `sm_80` and PTX ISA 7.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-mbarrier-init>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mbarrier_init<const SPACE: StateSpace>(addr: *mut u64, count: u32) {
    static_assert!(
        matches!(SPACE, StateSpace::Generic | StateSpace::SharedCta),
        "mbarrier_init only supports the generic and shared::cta state spaces"
    );
    match SPACE {
        StateSpace::Generic => llvm_mbarrier_init(addr.cast(), count),
        _ => llvm_mbarrier_init_shared(addr.cast(), count),
    }
}

/// Invalidates the mbarrier object at the generic address `addr`.
///
/// `SPACE` must be [`StateSpace::Generic`] or [`StateSpace::SharedCta`], and `addr` must fall
/// within the window of `SPACE`. Other state spaces are rejected at compile time.
///
/// Requires `sm_80` and PTX ISA 7.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-mbarrier-inval>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mbarrier_inval<const SPACE: StateSpace>(addr: *mut u64) {
    static_assert!(
        matches!(SPACE, StateSpace::Generic | StateSpace::SharedCta),
        "mbarrier_inval only supports the generic and shared::cta state spaces"
    );
    match SPACE {
        StateSpace::Generic => llvm_mbarrier_inval(addr.cast()),
        _ => llvm_mbarrier_inval_shared(addr.cast()),
    }
}

/// Performs an arrive-on operation on the mbarrier object at the generic address `addr`, and
/// returns the opaque state of the mbarrier object prior to the operation.
///
/// `SPACE` must be [`StateSpace::Generic`] or [`StateSpace::SharedCta`], and `addr` must fall
/// within the window of `SPACE`. Other state spaces are rejected at compile time.
///
/// Requires `sm_80` and PTX ISA 7.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-mbarrier-arrive>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mbarrier_arrive<const SPACE: StateSpace>(addr: *mut u64) -> u64 {
    static_assert!(
        matches!(SPACE, StateSpace::Generic | StateSpace::SharedCta),
        "mbarrier_arrive only supports the generic and shared::cta state spaces"
    );
    match SPACE {
        StateSpace::Generic => llvm_mbarrier_arrive(addr.cast()),
        _ => llvm_mbarrier_arrive_shared(addr.cast()),
    }
}

/// Performs an arrive-on operation with the count `count` on the mbarrier object at the generic
/// address `addr`, and returns the opaque state of the mbarrier object prior to the operation.
///
/// The operation must not cause the mbarrier object to complete its current phase.
///
/// `SPACE` must be [`StateSpace::Generic`] or [`StateSpace::SharedCta`], and `addr` must fall
/// within the window of `SPACE`. Other state spaces are rejected at compile time.
///
/// Requires `sm_80` and PTX ISA 7.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-mbarrier-arrive>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mbarrier_arrive_nocomplete<const SPACE: StateSpace>(
    addr: *mut u64,
    count: u32,
) -> u64 {
    static_assert!(
        matches!(SPACE, StateSpace::Generic | StateSpace::SharedCta),
        "mbarrier_arrive_nocomplete only supports the generic and shared::cta state spaces"
    );
    match SPACE {
        StateSpace::Generic => llvm_mbarrier_arrive_nocomplete(addr.cast(), count),
        _ => llvm_mbarrier_arrive_nocomplete_shared(addr.cast(), count),
    }
}

/// Decrements the expected arrival count of the mbarrier object at the generic address `addr`,
/// performs an arrive-on operation, and returns the opaque state of the mbarrier object prior to
/// the arrive-on operation.
///
/// The decrement applies to all subsequent phases of the mbarrier object.
///
/// `SPACE` must be [`StateSpace::Generic`] or [`StateSpace::SharedCta`], and `addr` must fall
/// within the window of `SPACE`. Other state spaces are rejected at compile time.
///
/// Requires `sm_80` and PTX ISA 7.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-mbarrier-arrive-drop>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mbarrier_arrive_drop<const SPACE: StateSpace>(addr: *mut u64) -> u64 {
    static_assert!(
        matches!(SPACE, StateSpace::Generic | StateSpace::SharedCta),
        "mbarrier_arrive_drop only supports the generic and shared::cta state spaces"
    );
    match SPACE {
        StateSpace::Generic => llvm_mbarrier_arrive_drop(addr.cast()),
        _ => llvm_mbarrier_arrive_drop_shared(addr.cast()),
    }
}

/// Decrements the expected arrival count of the mbarrier object at the generic address `addr` by
/// `count`, performs an arrive-on operation with the count `count`, and returns the opaque state
/// of the mbarrier object prior to the arrive-on operation.
///
/// The decrement applies to all subsequent phases of the mbarrier object. The operation must not
/// cause the mbarrier object to complete its current phase.
///
/// `SPACE` must be [`StateSpace::Generic`] or [`StateSpace::SharedCta`], and `addr` must fall
/// within the window of `SPACE`. Other state spaces are rejected at compile time.
///
/// Requires `sm_80` and PTX ISA 7.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-mbarrier-arrive-drop>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mbarrier_arrive_drop_nocomplete<const SPACE: StateSpace>(
    addr: *mut u64,
    count: u32,
) -> u64 {
    static_assert!(
        matches!(SPACE, StateSpace::Generic | StateSpace::SharedCta),
        "mbarrier_arrive_drop_nocomplete only supports the generic and shared::cta state spaces"
    );
    match SPACE {
        StateSpace::Generic => llvm_mbarrier_arrive_drop_nocomplete(addr.cast(), count),
        _ => llvm_mbarrier_arrive_drop_nocomplete_shared(addr.cast(), count),
    }
}

/// Tests whether the phase of the mbarrier object at the generic address `addr` identified by
/// `state` has completed.
///
/// `state` must be returned by an arrive operation on the same mbarrier object during the current
/// or the immediately preceding phase.
///
/// `SPACE` must be [`StateSpace::Generic`] or [`StateSpace::SharedCta`], and `addr` must fall
/// within the window of `SPACE`. Other state spaces are rejected at compile time.
///
/// Requires `sm_80` and PTX ISA 7.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-mbarrier-test-wait-try-wait>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mbarrier_test_wait<const SPACE: StateSpace>(addr: *mut u64, state: u64) -> bool {
    static_assert!(
        matches!(SPACE, StateSpace::Generic | StateSpace::SharedCta),
        "mbarrier_test_wait only supports the generic and shared::cta state spaces"
    );
    match SPACE {
        StateSpace::Generic => llvm_mbarrier_test_wait(addr.cast(), state),
        _ => llvm_mbarrier_test_wait_shared(addr.cast(), state),
    }
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

/// Performs an arrive-on operation with the count `count` on the mbarrier object at the generic
/// address `addr` at scope `SCOPE`, with memory ordering `SEM`, and returns the opaque state of
/// the mbarrier object prior to the operation.
///
/// `SCOPE` must be [`Scope::Cta`] or [`Scope::Cluster`], and `SEM` must be [`Semantics::Release`]
/// (the default) or [`Semantics::Relaxed`]. Other values are rejected at compile time. `addr` must
/// fall within the `.shared::cta` state space. See [`mbarrier_arrive_scoped_cluster`] for the
/// `.shared::cluster` state space.
///
/// Requires `sm_90`. `Semantics::Release` requires PTX ISA 8.0, and `Semantics::Relaxed` requires
/// PTX ISA 8.6.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-mbarrier-arrive>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mbarrier_arrive_scoped<const SCOPE: Scope, const SEM: Semantics>(
    addr: *mut u64,
    count: u32,
) -> u64 {
    static_assert!(
        matches!(SCOPE, Scope::Cta | Scope::Cluster),
        "mbarrier_arrive_scoped only supports the cta and cluster scopes"
    );
    static_assert!(
        matches!(SEM, Semantics::Release | Semantics::Relaxed),
        "mbarrier_arrive_scoped only supports Release and Relaxed semantics"
    );
    match (SCOPE, SEM) {
        (Scope::Cta, Semantics::Release) => {
            llvm_mbarrier_arrive_scope_cta_space_cta(addr.cast(), count)
        }
        (Scope::Cluster, Semantics::Release) => {
            llvm_mbarrier_arrive_scope_cluster_space_cta(addr.cast(), count)
        }
        (Scope::Cta, _) => llvm_mbarrier_arrive_relaxed_scope_cta_space_cta(addr.cast(), count),
        _ => llvm_mbarrier_arrive_relaxed_scope_cluster_space_cta(addr.cast(), count),
    }
}

/// Performs an arrive-on operation with the count `count` on the mbarrier object at the generic
/// address `addr` at scope `SCOPE`, with memory ordering `SEM`.
///
/// Unlike [`mbarrier_arrive_scoped`], this returns nothing, as the state of an mbarrier object in
/// `.shared::cluster` memory is not returned. `SCOPE` must be [`Scope::Cta`] or [`Scope::Cluster`],
/// and `SEM` must be [`Semantics::Release`] (the default) or [`Semantics::Relaxed`]. Other values
/// are rejected at compile time. `addr` must fall within the `.shared::cluster` state space.
///
/// Requires `sm_90`. `Semantics::Release` requires PTX ISA 8.0, and `Semantics::Relaxed` requires
/// PTX ISA 8.6.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-mbarrier-arrive>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mbarrier_arrive_scoped_cluster<const SCOPE: Scope, const SEM: Semantics>(
    addr: *mut u64,
    count: u32,
) {
    static_assert!(
        matches!(SCOPE, Scope::Cta | Scope::Cluster),
        "mbarrier_arrive_scoped_cluster only supports the cta and cluster scopes"
    );
    static_assert!(
        matches!(SEM, Semantics::Release | Semantics::Relaxed),
        "mbarrier_arrive_scoped_cluster only supports Release and Relaxed semantics"
    );
    match (SCOPE, SEM) {
        (Scope::Cta, Semantics::Release) => {
            llvm_mbarrier_arrive_scope_cta_space_cluster(addr.cast(), count)
        }
        (Scope::Cluster, Semantics::Release) => {
            llvm_mbarrier_arrive_scope_cluster_space_cluster(addr.cast(), count)
        }
        (Scope::Cta, _) => llvm_mbarrier_arrive_relaxed_scope_cta_space_cluster(addr.cast(), count),
        _ => llvm_mbarrier_arrive_relaxed_scope_cluster_space_cluster(addr.cast(), count),
    }
}

/// Performs an arrive-on operation with the expect-tx count `tx` on the mbarrier object at the
/// generic address `addr` at scope `SCOPE`, with memory ordering `SEM`, and returns the opaque
/// state of the mbarrier object prior to the operation.
///
/// `SCOPE` must be [`Scope::Cta`] or [`Scope::Cluster`], and `SEM` must be [`Semantics::Release`]
/// (the default) or [`Semantics::Relaxed`]. Other values are rejected at compile time. `addr` must
/// fall within the `.shared::cta` state space. See [`mbarrier_arrive_expect_tx_cluster`] for the
/// `.shared::cluster` state space.
///
/// Requires `sm_90`. `Semantics::Release` requires PTX ISA 8.0, and `Semantics::Relaxed` requires
/// PTX ISA 8.6.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-mbarrier-arrive>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mbarrier_arrive_expect_tx<const SCOPE: Scope, const SEM: Semantics>(
    addr: *mut u64,
    tx: u32,
) -> u64 {
    static_assert!(
        matches!(SCOPE, Scope::Cta | Scope::Cluster),
        "mbarrier_arrive_expect_tx only supports the cta and cluster scopes"
    );
    static_assert!(
        matches!(SEM, Semantics::Release | Semantics::Relaxed),
        "mbarrier_arrive_expect_tx only supports Release and Relaxed semantics"
    );
    match (SCOPE, SEM) {
        (Scope::Cta, Semantics::Release) => {
            llvm_mbarrier_arrive_expect_tx_scope_cta_space_cta(addr.cast(), tx)
        }
        (Scope::Cluster, Semantics::Release) => {
            llvm_mbarrier_arrive_expect_tx_scope_cluster_space_cta(addr.cast(), tx)
        }
        (Scope::Cta, _) => {
            llvm_mbarrier_arrive_expect_tx_relaxed_scope_cta_space_cta(addr.cast(), tx)
        }
        _ => llvm_mbarrier_arrive_expect_tx_relaxed_scope_cluster_space_cta(addr.cast(), tx),
    }
}

/// Performs an arrive-on operation with the expect-tx count `tx` on the mbarrier object at the
/// generic address `addr` at scope `SCOPE`, with memory ordering `SEM`.
///
/// Unlike [`mbarrier_arrive_expect_tx`], this returns nothing, as the state of an mbarrier object
/// in `.shared::cluster` memory is not returned. `SCOPE` must be [`Scope::Cta`] or
/// [`Scope::Cluster`], and `SEM` must be [`Semantics::Release`] (the default) or
/// [`Semantics::Relaxed`]. Other values are rejected at compile time. `addr` must fall within the
/// `.shared::cluster` state space.
///
/// Requires `sm_90`. `Semantics::Release` requires PTX ISA 8.0, and `Semantics::Relaxed` requires
/// PTX ISA 8.6.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-mbarrier-arrive>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mbarrier_arrive_expect_tx_cluster<const SCOPE: Scope, const SEM: Semantics>(
    addr: *mut u64,
    tx: u32,
) {
    static_assert!(
        matches!(SCOPE, Scope::Cta | Scope::Cluster),
        "mbarrier_arrive_expect_tx_cluster only supports the cta and cluster scopes"
    );
    static_assert!(
        matches!(SEM, Semantics::Release | Semantics::Relaxed),
        "mbarrier_arrive_expect_tx_cluster only supports Release and Relaxed semantics"
    );
    match (SCOPE, SEM) {
        (Scope::Cta, Semantics::Release) => {
            llvm_mbarrier_arrive_expect_tx_scope_cta_space_cluster(addr.cast(), tx)
        }
        (Scope::Cluster, Semantics::Release) => {
            llvm_mbarrier_arrive_expect_tx_scope_cluster_space_cluster(addr.cast(), tx)
        }
        (Scope::Cta, _) => {
            llvm_mbarrier_arrive_expect_tx_relaxed_scope_cta_space_cluster(addr.cast(), tx)
        }
        _ => llvm_mbarrier_arrive_expect_tx_relaxed_scope_cluster_space_cluster(addr.cast(), tx),
    }
}

/// Decrements the expected arrival count of the mbarrier object at the generic address `addr` by
/// `count`, performs an arrive-on operation with the count `count` at scope `SCOPE` with memory
/// ordering `SEM`, and returns the opaque state of the mbarrier object prior to the arrive-on
/// operation.
///
/// The decrement applies to all subsequent phases of the mbarrier object.
///
/// `SCOPE` must be [`Scope::Cta`] or [`Scope::Cluster`], and `SEM` must be [`Semantics::Release`]
/// (the default) or [`Semantics::Relaxed`]. Other values are rejected at compile time. `addr` must
/// fall within the `.shared::cta` state space. See [`mbarrier_arrive_drop_scoped_cluster`] for the
/// `.shared::cluster` state space.
///
/// Requires `sm_90`. `Semantics::Release` requires PTX ISA 8.0, and `Semantics::Relaxed` requires
/// PTX ISA 8.6.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-mbarrier-arrive-drop>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mbarrier_arrive_drop_scoped<const SCOPE: Scope, const SEM: Semantics>(
    addr: *mut u64,
    count: u32,
) -> u64 {
    static_assert!(
        matches!(SCOPE, Scope::Cta | Scope::Cluster),
        "mbarrier_arrive_drop_scoped only supports the cta and cluster scopes"
    );
    static_assert!(
        matches!(SEM, Semantics::Release | Semantics::Relaxed),
        "mbarrier_arrive_drop_scoped only supports Release and Relaxed semantics"
    );
    match (SCOPE, SEM) {
        (Scope::Cta, Semantics::Release) => {
            llvm_mbarrier_arrive_drop_scope_cta_space_cta(addr.cast(), count)
        }
        (Scope::Cluster, Semantics::Release) => {
            llvm_mbarrier_arrive_drop_scope_cluster_space_cta(addr.cast(), count)
        }
        (Scope::Cta, _) => {
            llvm_mbarrier_arrive_drop_relaxed_scope_cta_space_cta(addr.cast(), count)
        }
        _ => llvm_mbarrier_arrive_drop_relaxed_scope_cluster_space_cta(addr.cast(), count),
    }
}

/// Decrements the expected arrival count of the mbarrier object at the generic address `addr` by
/// `count`, performs an arrive-on operation with the count `count` at scope `SCOPE` with memory
/// ordering `SEM`.
///
/// The decrement applies to all subsequent phases of the mbarrier object. Unlike
/// [`mbarrier_arrive_drop_scoped`], this returns nothing, as the state of an mbarrier object in
/// `.shared::cluster` memory is not returned. `SCOPE` must be [`Scope::Cta`] or [`Scope::Cluster`],
/// and `SEM` must be [`Semantics::Release`] (the default) or [`Semantics::Relaxed`]. Other values
/// are rejected at compile time. `addr` must fall within the `.shared::cluster` state space.
///
/// Requires `sm_90`. `Semantics::Release` requires PTX ISA 8.0, and `Semantics::Relaxed` requires
/// PTX ISA 8.6.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-mbarrier-arrive-drop>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mbarrier_arrive_drop_scoped_cluster<const SCOPE: Scope, const SEM: Semantics>(
    addr: *mut u64,
    count: u32,
) {
    static_assert!(
        matches!(SCOPE, Scope::Cta | Scope::Cluster),
        "mbarrier_arrive_drop_scoped_cluster only supports the cta and cluster scopes"
    );
    static_assert!(
        matches!(SEM, Semantics::Release | Semantics::Relaxed),
        "mbarrier_arrive_drop_scoped_cluster only supports Release and Relaxed semantics"
    );
    match (SCOPE, SEM) {
        (Scope::Cta, Semantics::Release) => {
            llvm_mbarrier_arrive_drop_scope_cta_space_cluster(addr.cast(), count)
        }
        (Scope::Cluster, Semantics::Release) => {
            llvm_mbarrier_arrive_drop_scope_cluster_space_cluster(addr.cast(), count)
        }
        (Scope::Cta, _) => {
            llvm_mbarrier_arrive_drop_relaxed_scope_cta_space_cluster(addr.cast(), count)
        }
        _ => llvm_mbarrier_arrive_drop_relaxed_scope_cluster_space_cluster(addr.cast(), count),
    }
}

/// Performs an arrive-on operation with the expect-tx count `tx` on the mbarrier object at the
/// generic address `addr`, decrementing its expected arrival count as in
/// [`mbarrier_arrive_drop_scoped`], and returns the opaque state of the mbarrier object prior to
/// the operation.
///
/// The decrement applies to all subsequent phases of the mbarrier object.
///
/// `SCOPE` must be [`Scope::Cta`] or [`Scope::Cluster`], and `SEM` must be [`Semantics::Release`]
/// (the default) or [`Semantics::Relaxed`]. Other values are rejected at compile time. `addr` must
/// fall within the `.shared::cta` state space. See [`mbarrier_arrive_drop_expect_tx_cluster`] for
/// the `.shared::cluster` state space.
///
/// Requires `sm_90`. `Semantics::Release` requires PTX ISA 8.0, and `Semantics::Relaxed` requires
/// PTX ISA 8.6.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-mbarrier-arrive-drop>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mbarrier_arrive_drop_expect_tx<const SCOPE: Scope, const SEM: Semantics>(
    addr: *mut u64,
    tx: u32,
) -> u64 {
    static_assert!(
        matches!(SCOPE, Scope::Cta | Scope::Cluster),
        "mbarrier_arrive_drop_expect_tx only supports the cta and cluster scopes"
    );
    static_assert!(
        matches!(SEM, Semantics::Release | Semantics::Relaxed),
        "mbarrier_arrive_drop_expect_tx only supports Release and Relaxed semantics"
    );
    match (SCOPE, SEM) {
        (Scope::Cta, Semantics::Release) => {
            llvm_mbarrier_arrive_drop_expect_tx_scope_cta_space_cta(addr.cast(), tx)
        }
        (Scope::Cluster, Semantics::Release) => {
            llvm_mbarrier_arrive_drop_expect_tx_scope_cluster_space_cta(addr.cast(), tx)
        }
        (Scope::Cta, _) => {
            llvm_mbarrier_arrive_drop_expect_tx_relaxed_scope_cta_space_cta(addr.cast(), tx)
        }
        _ => llvm_mbarrier_arrive_drop_expect_tx_relaxed_scope_cluster_space_cta(addr.cast(), tx),
    }
}

/// Performs an arrive-on operation with the expect-tx count `tx` on the mbarrier object at the
/// generic address `addr`, decrementing its expected arrival count as in
/// [`mbarrier_arrive_drop_scoped_cluster`].
///
/// The decrement applies to all subsequent phases of the mbarrier object. Unlike
/// [`mbarrier_arrive_drop_expect_tx`], this returns nothing, as the state of an mbarrier object in
/// `.shared::cluster` memory is not returned. `SCOPE` must be [`Scope::Cta`] or [`Scope::Cluster`],
/// and `SEM` must be [`Semantics::Release`] (the default) or [`Semantics::Relaxed`]. Other values
/// are rejected at compile time. `addr` must fall within the `.shared::cluster` state space.
///
/// Requires `sm_90`. `Semantics::Release` requires PTX ISA 8.0, and `Semantics::Relaxed` requires
/// PTX ISA 8.6.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-mbarrier-arrive-drop>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mbarrier_arrive_drop_expect_tx_cluster<const SCOPE: Scope, const SEM: Semantics>(
    addr: *mut u64,
    tx: u32,
) {
    static_assert!(
        matches!(SCOPE, Scope::Cta | Scope::Cluster),
        "mbarrier_arrive_drop_expect_tx_cluster only supports the cta and cluster scopes"
    );
    static_assert!(
        matches!(SEM, Semantics::Release | Semantics::Relaxed),
        "mbarrier_arrive_drop_expect_tx_cluster only supports Release and Relaxed semantics"
    );
    match (SCOPE, SEM) {
        (Scope::Cta, Semantics::Release) => {
            llvm_mbarrier_arrive_drop_expect_tx_scope_cta_space_cluster(addr.cast(), tx)
        }
        (Scope::Cluster, Semantics::Release) => {
            llvm_mbarrier_arrive_drop_expect_tx_scope_cluster_space_cluster(addr.cast(), tx)
        }
        (Scope::Cta, _) => {
            llvm_mbarrier_arrive_drop_expect_tx_relaxed_scope_cta_space_cluster(addr.cast(), tx)
        }
        _ => {
            llvm_mbarrier_arrive_drop_expect_tx_relaxed_scope_cluster_space_cluster(addr.cast(), tx)
        }
    }
}

/// Performs an expect-tx operation with the transaction count `tx_count` on the mbarrier object
/// at the generic address `addr` at scope `SCOPE`.
///
/// `SCOPE` must be [`Scope::Cta`] or [`Scope::Cluster`], and `SPACE` must be
/// [`StateSpace::SharedCta`] or [`StateSpace::SharedCluster`]. Other values are rejected at
/// compile time, and `addr` must fall within the window of `SPACE`.
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-mbarrier-expect-tx>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mbarrier_expect_tx<const SCOPE: Scope, const SPACE: StateSpace>(
    addr: *mut u64,
    tx_count: u32,
) {
    static_assert!(
        matches!(SCOPE, Scope::Cta | Scope::Cluster),
        "mbarrier_expect_tx only supports the cta and cluster scopes"
    );
    static_assert!(
        matches!(SPACE, StateSpace::SharedCta | StateSpace::SharedCluster),
        "mbarrier_expect_tx only supports the shared::cta and shared::cluster state spaces"
    );
    match (SCOPE, SPACE) {
        (Scope::Cta, StateSpace::SharedCta) => {
            llvm_mbarrier_expect_tx_scope_cta_space_cta(addr.cast(), tx_count)
        }
        (Scope::Cta, _) => llvm_mbarrier_expect_tx_scope_cta_space_cluster(addr.cast(), tx_count),
        (Scope::Cluster, StateSpace::SharedCta) => {
            llvm_mbarrier_expect_tx_scope_cluster_space_cta(addr.cast(), tx_count)
        }
        _ => llvm_mbarrier_expect_tx_scope_cluster_space_cluster(addr.cast(), tx_count),
    }
}

/// Performs a complete-tx operation with the transaction count `tx_count` on the mbarrier object
/// at the generic address `addr` at scope `SCOPE`.
///
/// `SCOPE` must be [`Scope::Cta`] or [`Scope::Cluster`], and `SPACE` must be
/// [`StateSpace::SharedCta`] or [`StateSpace::SharedCluster`]. Other values are rejected at
/// compile time, and `addr` must fall within the window of `SPACE`.
///
/// Requires `sm_90` and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-mbarrier-complete-tx>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mbarrier_complete_tx<const SCOPE: Scope, const SPACE: StateSpace>(
    addr: *mut u64,
    tx_count: u32,
) {
    static_assert!(
        matches!(SCOPE, Scope::Cta | Scope::Cluster),
        "mbarrier_complete_tx only supports the cta and cluster scopes"
    );
    static_assert!(
        matches!(SPACE, StateSpace::SharedCta | StateSpace::SharedCluster),
        "mbarrier_complete_tx only supports the shared::cta and shared::cluster state spaces"
    );
    match (SCOPE, SPACE) {
        (Scope::Cta, StateSpace::SharedCta) => {
            llvm_mbarrier_complete_tx_scope_cta_space_cta(addr.cast(), tx_count)
        }
        (Scope::Cta, _) => llvm_mbarrier_complete_tx_scope_cta_space_cluster(addr.cast(), tx_count),
        (Scope::Cluster, StateSpace::SharedCta) => {
            llvm_mbarrier_complete_tx_scope_cluster_space_cta(addr.cast(), tx_count)
        }
        _ => llvm_mbarrier_complete_tx_scope_cluster_space_cluster(addr.cast(), tx_count),
    }
}

/// Tests whether the phase of the mbarrier object in `.shared::cta` memory at the generic address
/// `addr` identified by `state` has completed, with memory ordering `SEM` at scope `SCOPE`.
///
/// `state` must be returned by an arrive operation on the same mbarrier object during the current
/// or the immediately preceding phase. `SCOPE` must be [`Scope::Cta`] or [`Scope::Cluster`], and
/// `SEM` must be [`Semantics::Acquire`] (the default) or [`Semantics::Relaxed`]. Other values are
/// rejected at compile time.
///
/// Requires `sm_80` and PTX ISA 7.0 for `Scope::Cta` with `Semantics::Acquire`, `sm_90` and PTX
/// ISA 8.0 for `Scope::Cluster` with `Semantics::Acquire`, and `sm_90` and PTX ISA 8.6 for
/// `Semantics::Relaxed`.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-mbarrier-test-wait-try-wait>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mbarrier_test_wait_scoped<const SCOPE: Scope, const SEM: Semantics>(
    addr: *mut u64,
    state: u64,
) -> bool {
    static_assert!(
        matches!(SCOPE, Scope::Cta | Scope::Cluster),
        "mbarrier_test_wait_scoped only supports the cta and cluster scopes"
    );
    static_assert!(
        matches!(SEM, Semantics::Acquire | Semantics::Relaxed),
        "mbarrier_test_wait_scoped only supports Acquire and Relaxed semantics"
    );
    match (SCOPE, SEM) {
        (Scope::Cta, Semantics::Acquire) => {
            llvm_mbarrier_test_wait_scope_cta_space_cta(addr.cast(), state)
        }
        (Scope::Cluster, Semantics::Acquire) => {
            llvm_mbarrier_test_wait_scope_cluster_space_cta(addr.cast(), state)
        }
        (Scope::Cta, _) => llvm_mbarrier_test_wait_relaxed_scope_cta_space_cta(addr.cast(), state),
        _ => llvm_mbarrier_test_wait_relaxed_scope_cluster_space_cta(addr.cast(), state),
    }
}

/// Tests whether the phase of the mbarrier object in `.shared::cta` memory at the generic address
/// `addr` with parity `phase_parity` has completed, with memory ordering `SEM` at scope `SCOPE`.
///
/// `SCOPE` must be [`Scope::Cta`] or [`Scope::Cluster`], and `SEM` must be [`Semantics::Acquire`]
/// (the default) or [`Semantics::Relaxed`]. Other values are rejected at compile time.
///
/// Requires `sm_80` and PTX ISA 7.1 for `Scope::Cta` with `Semantics::Acquire`, `sm_90` and PTX
/// ISA 8.0 for `Scope::Cluster` with `Semantics::Acquire`, and `sm_90` and PTX ISA 8.6 for
/// `Semantics::Relaxed`.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-mbarrier-test-wait-try-wait>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mbarrier_test_wait_parity<const SCOPE: Scope, const SEM: Semantics>(
    addr: *mut u64,
    phase_parity: u32,
) -> bool {
    static_assert!(
        matches!(SCOPE, Scope::Cta | Scope::Cluster),
        "mbarrier_test_wait_parity only supports the cta and cluster scopes"
    );
    static_assert!(
        matches!(SEM, Semantics::Acquire | Semantics::Relaxed),
        "mbarrier_test_wait_parity only supports Acquire and Relaxed semantics"
    );
    match (SCOPE, SEM) {
        (Scope::Cta, Semantics::Acquire) => {
            llvm_mbarrier_test_wait_parity_scope_cta_space_cta(addr.cast(), phase_parity)
        }
        (Scope::Cluster, Semantics::Acquire) => {
            llvm_mbarrier_test_wait_parity_scope_cluster_space_cta(addr.cast(), phase_parity)
        }
        (Scope::Cta, _) => {
            llvm_mbarrier_test_wait_parity_relaxed_scope_cta_space_cta(addr.cast(), phase_parity)
        }
        _ => llvm_mbarrier_test_wait_parity_relaxed_scope_cluster_space_cta(
            addr.cast(),
            phase_parity,
        ),
    }
}

/// Waits until the phase of the mbarrier object in `.shared::cta` memory at the generic address
/// `addr` identified by `state` has completed, with memory ordering `SEM` at scope `SCOPE`, and
/// returns whether it has completed.
///
/// `state` must be returned by an arrive operation on the same mbarrier object during the current
/// or the immediately preceding phase. `SCOPE` must be [`Scope::Cta`] or [`Scope::Cluster`], and
/// `SEM` must be [`Semantics::Acquire`] (the default) or [`Semantics::Relaxed`]. Other values are
/// rejected at compile time.
///
/// Requires `sm_90` and PTX ISA 7.8 for `Scope::Cta` with `Semantics::Acquire`, `sm_90` and PTX
/// ISA 8.0 for `Scope::Cluster` with `Semantics::Acquire`, and `sm_90` and PTX ISA 8.6 for
/// `Semantics::Relaxed`.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-mbarrier-test-wait-try-wait>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mbarrier_try_wait_scoped<const SCOPE: Scope, const SEM: Semantics>(
    addr: *mut u64,
    state: u64,
) -> bool {
    static_assert!(
        matches!(SCOPE, Scope::Cta | Scope::Cluster),
        "mbarrier_try_wait_scoped only supports the cta and cluster scopes"
    );
    static_assert!(
        matches!(SEM, Semantics::Acquire | Semantics::Relaxed),
        "mbarrier_try_wait_scoped only supports Acquire and Relaxed semantics"
    );
    match (SCOPE, SEM) {
        (Scope::Cta, Semantics::Acquire) => {
            llvm_mbarrier_try_wait_scope_cta_space_cta(addr.cast(), state)
        }
        (Scope::Cluster, Semantics::Acquire) => {
            llvm_mbarrier_try_wait_scope_cluster_space_cta(addr.cast(), state)
        }
        (Scope::Cta, _) => llvm_mbarrier_try_wait_relaxed_scope_cta_space_cta(addr.cast(), state),
        _ => llvm_mbarrier_try_wait_relaxed_scope_cluster_space_cta(addr.cast(), state),
    }
}

/// Waits until the phase of the mbarrier object in `.shared::cta` memory at the generic address
/// `addr` identified by `state` has completed, or the thread is suspended for at most
/// `suspend_time_hint` nanoseconds, with memory ordering `SEM` at scope `SCOPE`, and returns
/// whether it has completed.
///
/// `state` must be returned by an arrive operation on the same mbarrier object during the current
/// or the immediately preceding phase. `SCOPE` must be [`Scope::Cta`] or [`Scope::Cluster`], and
/// `SEM` must be [`Semantics::Acquire`] (the default) or [`Semantics::Relaxed`]. Other values are
/// rejected at compile time.
///
/// Requires `sm_90` and PTX ISA 7.8 for `Scope::Cta` with `Semantics::Acquire`, `sm_90` and PTX
/// ISA 8.0 for `Scope::Cluster` with `Semantics::Acquire`, and `sm_90` and PTX ISA 8.6 for
/// `Semantics::Relaxed`.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-mbarrier-test-wait-try-wait>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mbarrier_try_wait_scoped_tl<const SCOPE: Scope, const SEM: Semantics>(
    addr: *mut u64,
    state: u64,
    suspend_time_hint: u32,
) -> bool {
    static_assert!(
        matches!(SCOPE, Scope::Cta | Scope::Cluster),
        "mbarrier_try_wait_scoped_tl only supports the cta and cluster scopes"
    );
    static_assert!(
        matches!(SEM, Semantics::Acquire | Semantics::Relaxed),
        "mbarrier_try_wait_scoped_tl only supports Acquire and Relaxed semantics"
    );
    match (SCOPE, SEM) {
        (Scope::Cta, Semantics::Acquire) => {
            llvm_mbarrier_try_wait_tl_scope_cta_space_cta(addr.cast(), state, suspend_time_hint)
        }
        (Scope::Cluster, Semantics::Acquire) => {
            llvm_mbarrier_try_wait_tl_scope_cluster_space_cta(addr.cast(), state, suspend_time_hint)
        }
        (Scope::Cta, _) => llvm_mbarrier_try_wait_tl_relaxed_scope_cta_space_cta(
            addr.cast(),
            state,
            suspend_time_hint,
        ),
        _ => llvm_mbarrier_try_wait_tl_relaxed_scope_cluster_space_cta(
            addr.cast(),
            state,
            suspend_time_hint,
        ),
    }
}

/// Waits until the phase of the mbarrier object in `.shared::cta` memory at the generic address
/// `addr` with parity `phase_parity` has completed, with memory ordering `SEM` at scope `SCOPE`,
/// and returns whether it has completed.
///
/// `SCOPE` must be [`Scope::Cta`] or [`Scope::Cluster`], and `SEM` must be [`Semantics::Acquire`]
/// (the default) or [`Semantics::Relaxed`]. Other values are rejected at compile time.
///
/// Requires `sm_90` and PTX ISA 7.8 for `Scope::Cta` with `Semantics::Acquire`, `sm_90` and PTX
/// ISA 8.0 for `Scope::Cluster` with `Semantics::Acquire`, and `sm_90` and PTX ISA 8.6 for
/// `Semantics::Relaxed`.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-mbarrier-test-wait-try-wait>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mbarrier_try_wait_parity<const SCOPE: Scope, const SEM: Semantics>(
    addr: *mut u64,
    phase_parity: u32,
) -> bool {
    static_assert!(
        matches!(SCOPE, Scope::Cta | Scope::Cluster),
        "mbarrier_try_wait_parity only supports the cta and cluster scopes"
    );
    static_assert!(
        matches!(SEM, Semantics::Acquire | Semantics::Relaxed),
        "mbarrier_try_wait_parity only supports Acquire and Relaxed semantics"
    );
    match (SCOPE, SEM) {
        (Scope::Cta, Semantics::Acquire) => {
            llvm_mbarrier_try_wait_parity_scope_cta_space_cta(addr.cast(), phase_parity)
        }
        (Scope::Cluster, Semantics::Acquire) => {
            llvm_mbarrier_try_wait_parity_scope_cluster_space_cta(addr.cast(), phase_parity)
        }
        (Scope::Cta, _) => {
            llvm_mbarrier_try_wait_parity_relaxed_scope_cta_space_cta(addr.cast(), phase_parity)
        }
        _ => {
            llvm_mbarrier_try_wait_parity_relaxed_scope_cluster_space_cta(addr.cast(), phase_parity)
        }
    }
}

/// Waits until the phase of the mbarrier object in `.shared::cta` memory at the generic address
/// `addr` with parity `phase_parity` has completed, or the thread is suspended for at most
/// `suspend_time_hint` nanoseconds, with memory ordering `SEM` at scope `SCOPE`, and returns
/// whether it has completed.
///
/// `SCOPE` must be [`Scope::Cta`] or [`Scope::Cluster`], and `SEM` must be [`Semantics::Acquire`]
/// (the default) or [`Semantics::Relaxed`]. Other values are rejected at compile time.
///
/// Requires `sm_90` and PTX ISA 7.8 for `Scope::Cta` with `Semantics::Acquire`, `sm_90` and PTX
/// ISA 8.0 for `Scope::Cluster` with `Semantics::Acquire`, and `sm_90` and PTX ISA 8.6 for
/// `Semantics::Relaxed`.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-mbarrier-test-wait-try-wait>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mbarrier_try_wait_parity_tl<const SCOPE: Scope, const SEM: Semantics>(
    addr: *mut u64,
    phase_parity: u32,
    suspend_time_hint: u32,
) -> bool {
    static_assert!(
        matches!(SCOPE, Scope::Cta | Scope::Cluster),
        "mbarrier_try_wait_parity_tl only supports the cta and cluster scopes"
    );
    static_assert!(
        matches!(SEM, Semantics::Acquire | Semantics::Relaxed),
        "mbarrier_try_wait_parity_tl only supports Acquire and Relaxed semantics"
    );
    match (SCOPE, SEM) {
        (Scope::Cta, Semantics::Acquire) => llvm_mbarrier_try_wait_parity_tl_scope_cta_space_cta(
            addr.cast(),
            phase_parity,
            suspend_time_hint,
        ),
        (Scope::Cluster, Semantics::Acquire) => {
            llvm_mbarrier_try_wait_parity_tl_scope_cluster_space_cta(
                addr.cast(),
                phase_parity,
                suspend_time_hint,
            )
        }
        (Scope::Cta, _) => llvm_mbarrier_try_wait_parity_tl_relaxed_scope_cta_space_cta(
            addr.cast(),
            phase_parity,
            suspend_time_hint,
        ),
        _ => llvm_mbarrier_try_wait_parity_tl_relaxed_scope_cluster_space_cta(
            addr.cast(),
            phase_parity,
            suspend_time_hint,
        ),
    }
}
