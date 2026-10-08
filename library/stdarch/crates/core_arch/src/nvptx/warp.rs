// --- LLM-generated --- //
//! Warp-level communication and reduction instructions.

use crate::marker::ConstParamTy;

#[allow(improper_ctypes)]
unsafe extern "llvm-intrinsic" {
    #[link_name = "llvm.nvvm.activemask"]
    fn llvm_activemask() -> u32;
    #[link_name = "llvm.nvvm.shfl.sync.up.i32"]
    fn llvm_shfl_sync_up_i32(mask: u32, a: u32, b: u32, c: u32) -> u32;
    #[link_name = "llvm.nvvm.shfl.sync.down.i32"]
    fn llvm_shfl_sync_down_i32(mask: u32, a: u32, b: u32, c: u32) -> u32;
    #[link_name = "llvm.nvvm.shfl.sync.bfly.i32"]
    fn llvm_shfl_sync_bfly_i32(mask: u32, a: u32, b: u32, c: u32) -> u32;
    #[link_name = "llvm.nvvm.shfl.sync.idx.i32"]
    fn llvm_shfl_sync_idx_i32(mask: u32, a: u32, b: u32, c: u32) -> u32;
    #[link_name = "llvm.nvvm.shfl.sync.up.f32"]
    fn llvm_shfl_sync_up_f32(mask: u32, a: f32, b: u32, c: u32) -> f32;
    #[link_name = "llvm.nvvm.shfl.sync.down.f32"]
    fn llvm_shfl_sync_down_f32(mask: u32, a: f32, b: u32, c: u32) -> f32;
    #[link_name = "llvm.nvvm.shfl.sync.bfly.f32"]
    fn llvm_shfl_sync_bfly_f32(mask: u32, a: f32, b: u32, c: u32) -> f32;
    #[link_name = "llvm.nvvm.shfl.sync.idx.f32"]
    fn llvm_shfl_sync_idx_f32(mask: u32, a: f32, b: u32, c: u32) -> f32;
    #[link_name = "llvm.nvvm.shfl.sync.up.i32p"]
    fn llvm_shfl_sync_up_i32p(mask: u32, a: u32, b: u32, c: u32) -> (u32, bool);
    #[link_name = "llvm.nvvm.shfl.sync.down.i32p"]
    fn llvm_shfl_sync_down_i32p(mask: u32, a: u32, b: u32, c: u32) -> (u32, bool);
    #[link_name = "llvm.nvvm.shfl.sync.bfly.i32p"]
    fn llvm_shfl_sync_bfly_i32p(mask: u32, a: u32, b: u32, c: u32) -> (u32, bool);
    #[link_name = "llvm.nvvm.shfl.sync.idx.i32p"]
    fn llvm_shfl_sync_idx_i32p(mask: u32, a: u32, b: u32, c: u32) -> (u32, bool);
    #[link_name = "llvm.nvvm.shfl.sync.up.f32p"]
    fn llvm_shfl_sync_up_f32p(mask: u32, a: f32, b: u32, c: u32) -> (f32, bool);
    #[link_name = "llvm.nvvm.shfl.sync.down.f32p"]
    fn llvm_shfl_sync_down_f32p(mask: u32, a: f32, b: u32, c: u32) -> (f32, bool);
    #[link_name = "llvm.nvvm.shfl.sync.bfly.f32p"]
    fn llvm_shfl_sync_bfly_f32p(mask: u32, a: f32, b: u32, c: u32) -> (f32, bool);
    #[link_name = "llvm.nvvm.shfl.sync.idx.f32p"]
    fn llvm_shfl_sync_idx_f32p(mask: u32, a: f32, b: u32, c: u32) -> (f32, bool);
    #[link_name = "llvm.nvvm.vote.all.sync"]
    fn llvm_vote_all_sync(mask: u32, a: bool) -> bool;
    #[link_name = "llvm.nvvm.vote.any.sync"]
    fn llvm_vote_any_sync(mask: u32, a: bool) -> bool;
    #[link_name = "llvm.nvvm.vote.uni.sync"]
    fn llvm_vote_uni_sync(mask: u32, a: bool) -> bool;
    #[link_name = "llvm.nvvm.vote.ballot.sync"]
    fn llvm_vote_ballot_sync(mask: u32, a: bool) -> u32;
    #[link_name = "llvm.nvvm.match.any.sync.i32"]
    fn llvm_match_any_sync_i32(mask: u32, value: u32) -> u32;
    #[link_name = "llvm.nvvm.match.any.sync.i64"]
    fn llvm_match_any_sync_i64(mask: u32, value: u64) -> u32;
    #[link_name = "llvm.nvvm.match.all.sync.i32p"]
    fn llvm_match_all_sync_i32p(mask: u32, value: u32) -> (u32, bool);
    #[link_name = "llvm.nvvm.match.all.sync.i64p"]
    fn llvm_match_all_sync_i64p(mask: u32, value: u64) -> (u32, bool);
    #[link_name = "llvm.nvvm.redux.sync.add"]
    fn llvm_redux_sync_add(value: u32, mask: u32) -> u32;
    #[link_name = "llvm.nvvm.redux.sync.min"]
    fn llvm_redux_sync_min(value: i32, mask: u32) -> i32;
    #[link_name = "llvm.nvvm.redux.sync.max"]
    fn llvm_redux_sync_max(value: i32, mask: u32) -> i32;
    #[link_name = "llvm.nvvm.redux.sync.umin"]
    fn llvm_redux_sync_umin(value: u32, mask: u32) -> u32;
    #[link_name = "llvm.nvvm.redux.sync.umax"]
    fn llvm_redux_sync_umax(value: u32, mask: u32) -> u32;
    #[link_name = "llvm.nvvm.redux.sync.and"]
    fn llvm_redux_sync_and(value: u32, mask: u32) -> u32;
    #[link_name = "llvm.nvvm.redux.sync.or"]
    fn llvm_redux_sync_or(value: u32, mask: u32) -> u32;
    #[link_name = "llvm.nvvm.redux.sync.xor"]
    fn llvm_redux_sync_xor(value: u32, mask: u32) -> u32;
    #[link_name = "llvm.nvvm.redux.sync.fmin"]
    fn llvm_redux_sync_fmin(value: f32, mask: u32) -> f32;
    #[link_name = "llvm.nvvm.redux.sync.fmin.NaN"]
    fn llvm_redux_sync_fmin_nan(value: f32, mask: u32) -> f32;
    #[link_name = "llvm.nvvm.redux.sync.fmin.abs"]
    fn llvm_redux_sync_fmin_abs(value: f32, mask: u32) -> f32;
    #[link_name = "llvm.nvvm.redux.sync.fmin.abs.NaN"]
    fn llvm_redux_sync_fmin_abs_nan(value: f32, mask: u32) -> f32;
    #[link_name = "llvm.nvvm.redux.sync.fmax"]
    fn llvm_redux_sync_fmax(value: f32, mask: u32) -> f32;
    #[link_name = "llvm.nvvm.redux.sync.fmax.NaN"]
    fn llvm_redux_sync_fmax_nan(value: f32, mask: u32) -> f32;
    #[link_name = "llvm.nvvm.redux.sync.fmax.abs"]
    fn llvm_redux_sync_fmax_abs(value: f32, mask: u32) -> f32;
    #[link_name = "llvm.nvvm.redux.sync.fmax.abs.NaN"]
    fn llvm_redux_sync_fmax_abs_nan(value: f32, mask: u32) -> f32;
    #[link_name = "llvm.nvvm.elect.sync"]
    fn llvm_elect_sync(mask: u32) -> (u32, bool);
}

/// Returns the mask of active threads in the executing warp.
///
/// Requires PTX ISA 6.2.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-activemask>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn activemask() -> u32 {
    llvm_activemask()
}

/// Source lane computation mode of [`shfl_sync_b32`] and its variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ConstParamTy)]
#[non_exhaustive]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub enum ShflMode {
    /// Reads from lane `laneid - b`.
    Up,
    /// Reads from lane `laneid + b`.
    Down,
    /// Reads from lane `laneid ^ b`.
    Bfly,
    /// Reads from lane `b`.
    Idx,
}

/// Exchanges the 32-bit value `a` between the threads of `membermask`.
///
/// `b` is the source lane or the lane offset, depending on `MODE`. `c` packs two values: the
/// clamp value in bits `[4:0]` and the segmentation mask in bits `[12:8]`. If the computed
/// source lane is out of range, the thread receives its own `a`.
///
/// Requires PTX ISA 6.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-shfl-sync>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn shfl_sync_b32<const MODE: ShflMode>(a: u32, b: u32, c: u32, membermask: u32) -> u32 {
    match MODE {
        ShflMode::Up => llvm_shfl_sync_up_i32(membermask, a, b, c),
        ShflMode::Down => llvm_shfl_sync_down_i32(membermask, a, b, c),
        ShflMode::Bfly => llvm_shfl_sync_bfly_i32(membermask, a, b, c),
        ShflMode::Idx => llvm_shfl_sync_idx_i32(membermask, a, b, c),
    }
}

/// Exchanges the `f32` value `a` between the threads of `membermask`.
///
/// See [`shfl_sync_b32`] for the meaning of `b` and `c`.
///
/// Requires PTX ISA 6.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-shfl-sync>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn shfl_sync_f32<const MODE: ShflMode>(a: f32, b: u32, c: u32, membermask: u32) -> f32 {
    match MODE {
        ShflMode::Up => llvm_shfl_sync_up_f32(membermask, a, b, c),
        ShflMode::Down => llvm_shfl_sync_down_f32(membermask, a, b, c),
        ShflMode::Bfly => llvm_shfl_sync_bfly_f32(membermask, a, b, c),
        ShflMode::Idx => llvm_shfl_sync_idx_f32(membermask, a, b, c),
    }
}

/// Exchanges the 32-bit value `a` between the threads of `membermask`, and returns whether the
/// source lane was in range alongside the result.
///
/// See [`shfl_sync_b32`] for the meaning of `b` and `c`. The returned predicate is `true` if the
/// computed source lane is in range.
///
/// Requires PTX ISA 6.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-shfl-sync>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn shfl_sync_pred_b32<const MODE: ShflMode>(
    a: u32,
    b: u32,
    c: u32,
    membermask: u32,
) -> (u32, bool) {
    match MODE {
        ShflMode::Up => llvm_shfl_sync_up_i32p(membermask, a, b, c),
        ShflMode::Down => llvm_shfl_sync_down_i32p(membermask, a, b, c),
        ShflMode::Bfly => llvm_shfl_sync_bfly_i32p(membermask, a, b, c),
        ShflMode::Idx => llvm_shfl_sync_idx_i32p(membermask, a, b, c),
    }
}

/// Exchanges the `f32` value `a` between the threads of `membermask`, and returns whether the
/// source lane was in range alongside the result.
///
/// See [`shfl_sync_pred_b32`] for the meaning of the predicate.
///
/// Requires PTX ISA 6.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-shfl-sync>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn shfl_sync_pred_f32<const MODE: ShflMode>(
    a: f32,
    b: u32,
    c: u32,
    membermask: u32,
) -> (f32, bool) {
    match MODE {
        ShflMode::Up => llvm_shfl_sync_up_f32p(membermask, a, b, c),
        ShflMode::Down => llvm_shfl_sync_down_f32p(membermask, a, b, c),
        ShflMode::Bfly => llvm_shfl_sync_bfly_f32p(membermask, a, b, c),
        ShflMode::Idx => llvm_shfl_sync_idx_f32p(membermask, a, b, c),
    }
}

/// Reduction mode of the vote instructions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ConstParamTy)]
#[non_exhaustive]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub enum VoteMode {
    /// True if `a` is true for all threads in `membermask`.
    All,
    /// True if `a` is true for any thread in `membermask`.
    Any,
    /// True if `a` is either true for all threads in `membermask` or false for all of them.
    Uni,
}

/// Evaluates the predicate `a` across the threads of `membermask` according to `MODE`.
///
/// The result is the same for all threads in `membermask`.
///
/// Requires PTX ISA 6.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-vote-sync>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn vote_sync_pred<const MODE: VoteMode>(a: bool, membermask: u32) -> bool {
    match MODE {
        VoteMode::All => llvm_vote_all_sync(membermask, a),
        VoteMode::Any => llvm_vote_any_sync(membermask, a),
        VoteMode::Uni => llvm_vote_uni_sync(membermask, a),
    }
}

/// Returns a bitmask with bit `laneid` set for each thread in `membermask` where `a` is true.
///
/// The result is the same for all threads in `membermask`.
///
/// Requires PTX ISA 6.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-vote-sync>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn vote_sync_ballot_b32(a: bool, membermask: u32) -> u32 {
    llvm_vote_ballot_sync(membermask, a)
}

/// Returns the mask of threads in `membermask` whose `a` is equal to this thread's `a`.
///
/// Requires sm_70 and PTX ISA 6.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-match-sync>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn match_any_sync_b32(a: u32, membermask: u32) -> u32 {
    llvm_match_any_sync_i32(membermask, a)
}

/// Returns the mask of threads in `membermask` whose `a` is equal to this thread's `a`.
///
/// Requires sm_70 and PTX ISA 6.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-match-sync>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn match_any_sync_b64(a: u64, membermask: u32) -> u32 {
    llvm_match_any_sync_i64(membermask, a)
}

/// Returns the mask of threads in `membermask` whose `a` is equal to this thread's `a`, and
/// whether all threads in `membermask` have the same `a`.
///
/// Requires sm_70 and PTX ISA 6.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-match-sync>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn match_all_sync_b32(a: u32, membermask: u32) -> (u32, bool) {
    llvm_match_all_sync_i32p(membermask, a)
}

/// Returns the mask of threads in `membermask` whose `a` is equal to this thread's `a`, and
/// whether all threads in `membermask` have the same `a`.
///
/// Requires sm_70 and PTX ISA 6.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-match-sync>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn match_all_sync_b64(a: u64, membermask: u32) -> (u32, bool) {
    llvm_match_all_sync_i64p(membermask, a)
}

/// Reduction operation of the redux instructions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ConstParamTy)]
#[non_exhaustive]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub enum ReduxOp {
    /// Sum of all values.
    Add,
    /// Minimum of all values.
    Min,
    /// Maximum of all values.
    Max,
    /// Bitwise AND of all values.
    And,
    /// Bitwise OR of all values.
    Or,
    /// Bitwise XOR of all values.
    Xor,
}

/// Reduces `src` across the threads of `membermask` with the unsigned operation `OP`, and
/// returns the result to all threads.
///
/// `OP` must be [`ReduxOp::Add`], [`ReduxOp::Min`] or [`ReduxOp::Max`].
///
/// Requires sm_80 and PTX ISA 7.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-redux-sync>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn redux_sync_u32<const OP: ReduxOp>(src: u32, membermask: u32) -> u32 {
    static_assert!(
        matches!(OP, ReduxOp::Add | ReduxOp::Min | ReduxOp::Max),
        "redux_sync_u32 only supports the Add, Min and Max operations"
    );
    match OP {
        ReduxOp::Add => llvm_redux_sync_add(src, membermask),
        ReduxOp::Min => llvm_redux_sync_umin(src, membermask),
        ReduxOp::Max => llvm_redux_sync_umax(src, membermask),
        _ => unreachable!(),
    }
}

/// Reduces `src` across the threads of `membermask` with the signed operation `OP`, and
/// returns the result to all threads.
///
/// `OP` must be [`ReduxOp::Add`], [`ReduxOp::Min`] or [`ReduxOp::Max`].
///
/// Requires sm_80 and PTX ISA 7.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-redux-sync>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn redux_sync_s32<const OP: ReduxOp>(src: i32, membermask: u32) -> i32 {
    static_assert!(
        matches!(OP, ReduxOp::Add | ReduxOp::Min | ReduxOp::Max),
        "redux_sync_s32 only supports the Add, Min and Max operations"
    );
    match OP {
        ReduxOp::Add => llvm_redux_sync_add(src as u32, membermask) as i32,
        ReduxOp::Min => llvm_redux_sync_min(src, membermask),
        ReduxOp::Max => llvm_redux_sync_max(src, membermask),
        _ => unreachable!(),
    }
}

/// Reduces `src` across the threads of `membermask` with the bitwise operation `OP`, and
/// returns the result to all threads.
///
/// `OP` must be [`ReduxOp::And`], [`ReduxOp::Or`] or [`ReduxOp::Xor`].
///
/// Requires sm_80 and PTX ISA 7.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-redux-sync>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn redux_sync_b32<const OP: ReduxOp>(src: u32, membermask: u32) -> u32 {
    static_assert!(
        matches!(OP, ReduxOp::And | ReduxOp::Or | ReduxOp::Xor),
        "redux_sync_b32 only supports the And, Or and Xor operations"
    );
    match OP {
        ReduxOp::And => llvm_redux_sync_and(src, membermask),
        ReduxOp::Or => llvm_redux_sync_or(src, membermask),
        ReduxOp::Xor => llvm_redux_sync_xor(src, membermask),
        _ => unreachable!(),
    }
}

/// Reduces `src` across the threads of `membermask` with the floating-point operation `OP`,
/// and returns the result to all threads.
///
/// `OP` must be [`ReduxOp::Min`] or [`ReduxOp::Max`]. If `ABS` is set, the absolute values of
/// the inputs are reduced. If `NAN` is set, NaN inputs are propagated to the result.
///
/// Requires `sm_100a` with PTX ISA 8.6, or the `sm_100f` family with PTX ISA 8.8.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-redux-sync>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn redux_sync_f32<const OP: ReduxOp, const ABS: bool, const NAN: bool>(
    src: f32,
    membermask: u32,
) -> f32 {
    static_assert!(
        matches!(OP, ReduxOp::Min | ReduxOp::Max),
        "redux_sync_f32 only supports the Min and Max operations"
    );
    match (OP, ABS, NAN) {
        (ReduxOp::Min, false, false) => llvm_redux_sync_fmin(src, membermask),
        (ReduxOp::Min, false, true) => llvm_redux_sync_fmin_nan(src, membermask),
        (ReduxOp::Min, true, false) => llvm_redux_sync_fmin_abs(src, membermask),
        (ReduxOp::Min, true, true) => llvm_redux_sync_fmin_abs_nan(src, membermask),
        (ReduxOp::Max, false, false) => llvm_redux_sync_fmax(src, membermask),
        (ReduxOp::Max, false, true) => llvm_redux_sync_fmax_nan(src, membermask),
        (ReduxOp::Max, true, false) => llvm_redux_sync_fmax_abs(src, membermask),
        (ReduxOp::Max, true, true) => llvm_redux_sync_fmax_abs_nan(src, membermask),
        _ => unreachable!(),
    }
}

/// Elects a leader thread among the threads of `membermask`.
///
/// Returns the lane ID of the elected thread, and whether the calling thread is the leader.
///
/// Requires sm_90 and PTX ISA 8.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-elect-sync>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[target_feature(enable = "sm_90,ptx80")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn elect_sync(membermask: u32) -> (u32, bool) {
    llvm_elect_sync(membermask)
}
