// --- LLM-generated --- //
//! Memory and address space instructions.

use crate::ffi::c_void;
use crate::marker::ConstParamTy;

#[allow(improper_ctypes)]
unsafe extern "llvm-intrinsic" {
    #[link_name = "llvm.nvvm.isspacep.const"]
    fn llvm_isspacep_const(p: *const c_void) -> bool;
    #[link_name = "llvm.nvvm.isspacep.global"]
    fn llvm_isspacep_global(p: *const c_void) -> bool;
    #[link_name = "llvm.nvvm.isspacep.local"]
    fn llvm_isspacep_local(p: *const c_void) -> bool;
    #[link_name = "llvm.nvvm.isspacep.shared"]
    fn llvm_isspacep_shared(p: *const c_void) -> bool;
    #[link_name = "llvm.nvvm.isspacep.shared.cluster"]
    fn llvm_isspacep_shared_cluster(p: *const c_void) -> bool;
    #[link_name = "llvm.nvvm.mapa"]
    fn llvm_mapa(p: *mut c_void, rank: u32) -> *mut c_void;
    #[link_name = "llvm.nvvm.getctarank"]
    fn llvm_getctarank(p: *const c_void) -> u32;
    #[link_name = "llvm.nvvm.st.bulk"]
    fn llvm_st_bulk(p: *mut c_void, size: u64, initval: u64);
}

/// A PTX state space.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ConstParamTy)]
#[non_exhaustive]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub enum StateSpace {
    /// Constant memory (`.const`).
    Const,
    /// Global memory (`.global`).
    Global,
    /// Thread-local memory (`.local`).
    Local,
    /// Shared memory of the executing CTA (`.shared::cta`).
    SharedCta,
    /// Shared memory of any CTA in the executing cluster (`.shared::cluster`).
    SharedCluster,
}

/// Returns whether the generic address `ptr` falls within the window of state space `SPACE`.
///
/// [`StateSpace::SharedCluster`] requires `sm_90` and PTX ISA 7.8.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-isspacep>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn isspacep<const SPACE: StateSpace>(ptr: *const c_void) -> bool {
    match SPACE {
        StateSpace::Const => llvm_isspacep_const(ptr),
        StateSpace::Global => llvm_isspacep_global(ptr),
        StateSpace::Local => llvm_isspacep_local(ptr),
        StateSpace::SharedCta => llvm_isspacep_shared(ptr),
        StateSpace::SharedCluster => llvm_isspacep_shared_cluster(ptr),
    }
}

/// Maps the generic address `ptr` of a shared memory location in the executing CTA to the
/// generic address of the corresponding location in the CTA with rank `rank` in the cluster.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-mapa>
#[inline]
#[target_feature(enable = "sm_90,ptx78")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mapa<T>(ptr: *mut T, rank: u32) -> *mut T {
    llvm_mapa(ptr.cast(), rank).cast()
}

/// Returns the rank of the CTA in the cluster whose shared memory contains the generic
/// address `ptr`.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-getctarank>
#[inline]
#[target_feature(enable = "sm_90,ptx78")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn getctarank<T>(ptr: *const T) -> u32 {
    llvm_getctarank(ptr.cast())
}

/// Initializes the `size` bytes of memory starting at `ptr` to zero, using the bulk store
/// instruction.
///
/// `size` must be a multiple of 8.
///
/// Requires `sm_100` and PTX ISA 8.6.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-st-bulk>
#[inline]
#[target_feature(enable = "sm_100,ptx86")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn st_bulk(ptr: *mut c_void, size: u64) {
    llvm_st_bulk(ptr, size, 0)
}
