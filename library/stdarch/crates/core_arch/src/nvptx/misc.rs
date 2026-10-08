// --- LLM-generated --- //
//! Miscellaneous instructions.

use crate::marker::ConstParamTy;

#[allow(improper_ctypes)]
unsafe extern "llvm-intrinsic" {
    #[link_name = "llvm.nvvm.prmt"]
    fn llvm_prmt(a: u32, b: u32, c: u32) -> u32;
    #[link_name = "llvm.nvvm.prmt.f4e"]
    fn llvm_prmt_f4e(a: u32, b: u32, c: u32) -> u32;
    #[link_name = "llvm.nvvm.prmt.b4e"]
    fn llvm_prmt_b4e(a: u32, b: u32, c: u32) -> u32;
    #[link_name = "llvm.nvvm.prmt.rc8"]
    fn llvm_prmt_rc8(a: u32, c: u32) -> u32;
    #[link_name = "llvm.nvvm.prmt.ecl"]
    fn llvm_prmt_ecl(a: u32, c: u32) -> u32;
    #[link_name = "llvm.nvvm.prmt.ecr"]
    fn llvm_prmt_ecr(a: u32, c: u32) -> u32;
    #[link_name = "llvm.nvvm.prmt.rc16"]
    fn llvm_prmt_rc16(a: u32, c: u32) -> u32;
    #[link_name = "llvm.nvvm.nanosleep"]
    fn llvm_nanosleep(t: u32);
    #[link_name = "llvm.nvvm.pm.event.mask"]
    fn llvm_pmevent_mask(mask: u16);
    #[link_name = "llvm.nvvm.exit"]
    fn llvm_exit() -> !;
    #[link_name = "llvm.nvvm.setmaxnreg.inc.sync.aligned.u32"]
    fn llvm_setmaxnreg_inc(count: u32);
    #[link_name = "llvm.nvvm.setmaxnreg.dec.sync.aligned.u32"]
    fn llvm_setmaxnreg_dec(count: u32);
}

/// Byte selection mode of [`prmt`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, ConstParamTy)]
#[non_exhaustive]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub enum PrmtMode {
    /// Generic form: each 4-bit nibble of `c[15:0]` selects one source byte, optionally
    /// sign-extending it.
    Generic,
    /// Forward 4 extract.
    F4e,
    /// Backward 4 extract.
    B4e,
    /// Replicate 8.
    Rc8,
    /// Edge clamp left.
    Ecl,
    /// Edge clamp right.
    Ecr,
    /// Replicate 16.
    Rc16,
}

/// Picks four bytes from the 64-bit value `{b, a}` and assembles them into a 32-bit result.
///
/// In [`PrmtMode::Generic`] mode, the selection is controlled by four 4-bit selectors in
/// `c[15:0]`. In all other modes, it is controlled by `c[1:0]`. The modes
/// [`PrmtMode::Rc8`], [`PrmtMode::Ecl`], [`PrmtMode::Ecr`] and [`PrmtMode::Rc16`] only read
/// bytes from `a`, and `b` is ignored.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-prmt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn prmt<const MODE: PrmtMode>(a: u32, b: u32, c: u32) -> u32 {
    match MODE {
        PrmtMode::Generic => llvm_prmt(a, b, c),
        PrmtMode::F4e => llvm_prmt_f4e(a, b, c),
        PrmtMode::B4e => llvm_prmt_b4e(a, b, c),
        PrmtMode::Rc8 => llvm_prmt_rc8(a, c),
        PrmtMode::Ecl => llvm_prmt_ecl(a, c),
        PrmtMode::Ecr => llvm_prmt_ecr(a, c),
        PrmtMode::Rc16 => llvm_prmt_rc16(a, c),
    }
}

/// Suspends the thread for approximately `t` nanoseconds.
///
/// The sleep duration is guaranteed to be in the interval `[0, 2 * t]`, and is at most one
/// millisecond.
///
/// Requires PTX ISA 6.3.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#miscellaneous-instructions-nanosleep>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn nanosleep(t: u32) {
    llvm_nanosleep(t)
}

/// Triggers the performance monitor events whose bits are set in `MASK`.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#miscellaneous-instructions-pmevent>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn pmevent_mask<const MASK: u16>() {
    llvm_pmevent_mask(MASK)
}

/// Terminates the executing thread.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#control-flow-instructions-exit>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn exit() -> ! {
    llvm_exit()
}

/// Register adjustment performed by [`setmaxnreg`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, ConstParamTy)]
#[non_exhaustive]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub enum SetmaxnregAction {
    /// Request additional registers from the CTA's register pool.
    Inc,
    /// Release registers to the CTA's register pool.
    Dec,
}

/// Hints that the maximum number of per-thread registers owned by the executing warp should
/// be changed to `COUNT`.
///
/// `COUNT` must be a multiple of 8 in the range `24..=256`. All warps of a warpgroup must
/// execute the same `setmaxnreg` instruction.
///
/// Requires `sm_90a` with PTX ISA 8.0, or `sm_100a`/`sm_101a`/`sm_120a` with PTX ISA 8.6, or
/// the `sm_100f`/`sm_101f`/`sm_120f` families with PTX ISA 8.8.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#miscellaneous-instructions-setmaxnreg>
///
#[doc = include_str!("../amdgpu/intrinsic_is_convergent.md")]
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn setmaxnreg<const ACTION: SetmaxnregAction, const COUNT: u32>() {
    static_assert!(
        COUNT >= 24 && COUNT <= 256 && COUNT % 8 == 0,
        "COUNT must be a multiple of 8 in the range 24..=256"
    );
    match ACTION {
        SetmaxnregAction::Inc => llvm_setmaxnreg_inc(COUNT),
        SetmaxnregAction::Dec => llvm_setmaxnreg_dec(COUNT),
    }
}
