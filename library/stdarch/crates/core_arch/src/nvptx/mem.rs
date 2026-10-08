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
    #[link_name = "llvm.nvvm.mapa.shared.cluster"]
    fn llvm_mapa_shared_cluster(p: *mut c_void, rank: u32) -> *mut c_void;
    #[link_name = "llvm.nvvm.getctarank"]
    fn llvm_getctarank(p: *const c_void) -> u32;
    #[link_name = "llvm.nvvm.getctarank.shared.cluster"]
    fn llvm_getctarank_shared_cluster(p: *const c_void) -> u32;
    #[link_name = "llvm.nvvm.st.bulk"]
    fn llvm_st_bulk(p: *mut c_void, size: u64, initval: u64);
    #[link_name = "llvm.nvvm.st.bulk.shared.cta"]
    fn llvm_st_bulk_shared_cta(p: *mut c_void, size: u64, initval: u64);
    #[link_name = "llvm.nvvm.st.async.i32"]
    #[cfg_attr(target_arch = "nvptx64", rustc_llvm_ptr_addrspace(args(7, 0, 7)))]
    fn llvm_st_async_i32(p: *mut c_void, value: u32, mbar: *mut c_void);
    #[link_name = "llvm.nvvm.st.async.i64"]
    #[cfg_attr(target_arch = "nvptx64", rustc_llvm_ptr_addrspace(args(7, 0, 7)))]
    fn llvm_st_async_i64(p: *mut c_void, value: u64, mbar: *mut c_void);
    #[link_name = "llvm.nvvm.st.async.gpu.i16"]
    #[cfg_attr(target_arch = "nvptx64", rustc_llvm_ptr_addrspace(args(1, 0, 0)))]
    fn llvm_st_async_gpu_i16(p: *mut c_void, value: u16, is_multimem: bool);
    #[link_name = "llvm.nvvm.st.async.gpu.i32"]
    #[cfg_attr(target_arch = "nvptx64", rustc_llvm_ptr_addrspace(args(1, 0, 0)))]
    fn llvm_st_async_gpu_i32(p: *mut c_void, value: u32, is_multimem: bool);
    #[link_name = "llvm.nvvm.st.async.gpu.i64"]
    #[cfg_attr(target_arch = "nvptx64", rustc_llvm_ptr_addrspace(args(1, 0, 0)))]
    fn llvm_st_async_gpu_i64(p: *mut c_void, value: u64, is_multimem: bool);
    #[link_name = "llvm.nvvm.st.async.sys.i16"]
    #[cfg_attr(target_arch = "nvptx64", rustc_llvm_ptr_addrspace(args(1, 0, 0)))]
    fn llvm_st_async_sys_i16(p: *mut c_void, value: u16, is_multimem: bool);
    #[link_name = "llvm.nvvm.st.async.sys.i32"]
    #[cfg_attr(target_arch = "nvptx64", rustc_llvm_ptr_addrspace(args(1, 0, 0)))]
    fn llvm_st_async_sys_i32(p: *mut c_void, value: u32, is_multimem: bool);
    #[link_name = "llvm.nvvm.st.async.sys.i64"]
    #[cfg_attr(target_arch = "nvptx64", rustc_llvm_ptr_addrspace(args(1, 0, 0)))]
    fn llvm_st_async_sys_i64(p: *mut c_void, value: u64, is_multimem: bool);
    #[link_name = "llvm.nvvm.st.async.mmio.sys.i16"]
    #[cfg_attr(target_arch = "nvptx64", rustc_llvm_ptr_addrspace(args(1, 0)))]
    fn llvm_st_async_mmio_sys_i16(p: *mut c_void, value: u16);
    #[link_name = "llvm.nvvm.st.async.mmio.sys.i32"]
    #[cfg_attr(target_arch = "nvptx64", rustc_llvm_ptr_addrspace(args(1, 0)))]
    fn llvm_st_async_mmio_sys_i32(p: *mut c_void, value: u32);
    #[link_name = "llvm.nvvm.st.async.mmio.sys.i64"]
    #[cfg_attr(target_arch = "nvptx64", rustc_llvm_ptr_addrspace(args(1, 0)))]
    fn llvm_st_async_mmio_sys_i64(p: *mut c_void, value: u64);
}

/// A PTX state space.
///
/// Functions taking a state space as a const generic parameter document which state spaces they
/// support; unsupported ones are rejected at compile time. Pointers are always passed as generic
/// addresses, which must fall within the window of the selected state space.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ConstParamTy)]
#[non_exhaustive]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub enum StateSpace {
    /// The generic address space, which contains the windows of the other state spaces (no
    /// state space qualifier).
    Generic,
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
/// [`StateSpace::Generic`] is not supported. [`StateSpace::SharedCluster`] requires `sm_90` and
/// PTX ISA 7.8.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-isspacep>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn isspacep<const SPACE: StateSpace>(ptr: *const c_void) -> bool {
    static_assert!(
        !matches!(SPACE, StateSpace::Generic),
        "isspacep doesn't support the generic state space"
    );
    match SPACE {
        StateSpace::Generic => unreachable!(),
        StateSpace::Const => llvm_isspacep_const(ptr),
        StateSpace::Global => llvm_isspacep_global(ptr),
        StateSpace::Local => llvm_isspacep_local(ptr),
        StateSpace::SharedCta => llvm_isspacep_shared(ptr),
        StateSpace::SharedCluster => llvm_isspacep_shared_cluster(ptr),
    }
}

/// Maps the address `ptr` of a shared memory location in the executing CTA to the generic address
/// of the corresponding location in the CTA with rank `rank` in the cluster.
///
/// `SPACE` is the state space of `ptr`: [`StateSpace::Generic`] or [`StateSpace::SharedCta`].
/// Other values are rejected at compile time. With [`StateSpace::SharedCta`], `ptr` must point to
/// shared memory of the executing CTA.
///
/// Requires `sm_90` and PTX ISA 7.8.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-mapa>
#[inline]
#[target_feature(enable = "sm_90,ptx78")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mapa<const SPACE: StateSpace, T>(ptr: *mut T, rank: u32) -> *mut T {
    static_assert!(
        matches!(SPACE, StateSpace::Generic | StateSpace::SharedCta),
        "mapa only supports the generic and shared::cta state spaces"
    );
    match SPACE {
        StateSpace::Generic => llvm_mapa(ptr.cast(), rank).cast(),
        StateSpace::SharedCta => llvm_mapa_shared_cluster(ptr.cast(), rank).cast(),
        _ => unreachable!(),
    }
}

/// Returns the rank of the CTA in the cluster whose shared memory contains the address `ptr`.
///
/// `SPACE` is the state space of `ptr`: [`StateSpace::Generic`] or [`StateSpace::SharedCta`].
/// Other values are rejected at compile time. With [`StateSpace::SharedCta`], `ptr` must point to
/// shared memory of the executing CTA.
///
/// Requires `sm_90` and PTX ISA 7.8.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-getctarank>
#[inline]
#[target_feature(enable = "sm_90,ptx78")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn getctarank<const SPACE: StateSpace, T>(ptr: *const T) -> u32 {
    static_assert!(
        matches!(SPACE, StateSpace::Generic | StateSpace::SharedCta),
        "getctarank only supports the generic and shared::cta state spaces"
    );
    match SPACE {
        StateSpace::Generic => llvm_getctarank(ptr.cast()),
        StateSpace::SharedCta => llvm_getctarank_shared_cluster(ptr.cast()),
        _ => unreachable!(),
    }
}

/// Initializes the `size` bytes of memory starting at `ptr` to zero, using the bulk store
/// instruction.
///
/// `size` must be a multiple of 8. `SPACE` is the state space of `ptr`:
/// [`StateSpace::Generic`] or [`StateSpace::SharedCta`]. Other values are rejected at compile
/// time.
///
/// Requires `sm_100` and PTX ISA 8.6.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-st-bulk>
#[inline]
#[target_feature(enable = "sm_100,ptx86")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn st_bulk<const SPACE: StateSpace>(ptr: *mut c_void, size: u64) {
    static_assert!(
        matches!(SPACE, StateSpace::Generic | StateSpace::SharedCta),
        "st_bulk only supports the generic and shared::cta state spaces"
    );
    match SPACE {
        StateSpace::Generic => llvm_st_bulk(ptr, size, 0),
        StateSpace::SharedCta => llvm_st_bulk_shared_cta(ptr, size, 0),
        _ => unreachable!(),
    }
}

/// Asynchronously stores `value` to the `shared::cluster` address `dst`, signalling completion on
/// the mbarrier object at `mbar`, which must also be a `shared::cluster` address.
///
/// The store is a weak memory operation: its effects become visible to other threads only when
/// synchronization is established by other means.
///
/// Requires `sm_90` and PTX ISA 8.1.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-st-async>
#[inline]
#[target_feature(enable = "sm_90,ptx81")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn st_async_u32(dst: *mut u32, value: u32, mbar: *mut c_void) {
    llvm_st_async_i32(dst.cast(), value, mbar)
}

/// Asynchronously stores `value` to the `shared::cluster` address `dst`, signalling completion on
/// the mbarrier object at `mbar`, which must also be a `shared::cluster` address.
///
/// The store is a weak memory operation: its effects become visible to other threads only when
/// synchronization is established by other means.
///
/// Requires `sm_90` and PTX ISA 8.1.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-st-async>
#[inline]
#[target_feature(enable = "sm_90,ptx81")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn st_async_u64(dst: *mut u64, value: u64, mbar: *mut c_void) {
    llvm_st_async_i64(dst.cast(), value, mbar)
}

/// Asynchronously stores `value` to the global address `dst` with `.release` semantics at `gpu`
/// scope.
///
/// If `MULTIMEM` is true, the store is a `multimem.st.async` and `dst` must be a multimem
/// address; otherwise `dst` must be a regular global address.
///
/// Requires `sm_100` and PTX ISA 8.7. `MULTIMEM` additionally requires PTX ISA 9.3.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-st-async>
#[inline]
#[target_feature(enable = "sm_100,ptx87")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn st_async_gpu_u16<const MULTIMEM: bool>(dst: *mut u16, value: u16) {
    llvm_st_async_gpu_i16(dst.cast(), value, MULTIMEM)
}

/// Asynchronously stores `value` to the global address `dst` with `.release` semantics at `gpu`
/// scope.
///
/// If `MULTIMEM` is true, the store is a `multimem.st.async` and `dst` must be a multimem
/// address; otherwise `dst` must be a regular global address.
///
/// Requires `sm_100` and PTX ISA 8.7. `MULTIMEM` additionally requires PTX ISA 9.3.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-st-async>
#[inline]
#[target_feature(enable = "sm_100,ptx87")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn st_async_gpu_u32<const MULTIMEM: bool>(dst: *mut u32, value: u32) {
    llvm_st_async_gpu_i32(dst.cast(), value, MULTIMEM)
}

/// Asynchronously stores `value` to the global address `dst` with `.release` semantics at `gpu`
/// scope.
///
/// If `MULTIMEM` is true, the store is a `multimem.st.async` and `dst` must be a multimem
/// address; otherwise `dst` must be a regular global address.
///
/// Requires `sm_100` and PTX ISA 8.7. `MULTIMEM` additionally requires PTX ISA 9.3.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-st-async>
#[inline]
#[target_feature(enable = "sm_100,ptx87")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn st_async_gpu_u64<const MULTIMEM: bool>(dst: *mut u64, value: u64) {
    llvm_st_async_gpu_i64(dst.cast(), value, MULTIMEM)
}

/// Asynchronously stores `value` to the global address `dst` with `.release` semantics at `sys`
/// scope.
///
/// If `MULTIMEM` is true, the store is a `multimem.st.async` and `dst` must be a multimem
/// address; otherwise `dst` must be a regular global address.
///
/// Requires `sm_100` and PTX ISA 8.7. `MULTIMEM` additionally requires PTX ISA 9.3.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-st-async>
#[inline]
#[target_feature(enable = "sm_100,ptx87")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn st_async_sys_u16<const MULTIMEM: bool>(dst: *mut u16, value: u16) {
    llvm_st_async_sys_i16(dst.cast(), value, MULTIMEM)
}

/// Asynchronously stores `value` to the global address `dst` with `.release` semantics at `sys`
/// scope.
///
/// If `MULTIMEM` is true, the store is a `multimem.st.async` and `dst` must be a multimem
/// address; otherwise `dst` must be a regular global address.
///
/// Requires `sm_100` and PTX ISA 8.7. `MULTIMEM` additionally requires PTX ISA 9.3.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-st-async>
#[inline]
#[target_feature(enable = "sm_100,ptx87")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn st_async_sys_u32<const MULTIMEM: bool>(dst: *mut u32, value: u32) {
    llvm_st_async_sys_i32(dst.cast(), value, MULTIMEM)
}

/// Asynchronously stores `value` to the global address `dst` with `.release` semantics at `sys`
/// scope.
///
/// If `MULTIMEM` is true, the store is a `multimem.st.async` and `dst` must be a multimem
/// address; otherwise `dst` must be a regular global address.
///
/// Requires `sm_100` and PTX ISA 8.7. `MULTIMEM` additionally requires PTX ISA 9.3.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-st-async>
#[inline]
#[target_feature(enable = "sm_100,ptx87")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn st_async_sys_u64<const MULTIMEM: bool>(dst: *mut u64, value: u64) {
    llvm_st_async_sys_i64(dst.cast(), value, MULTIMEM)
}

/// Performs an MMIO store of `value` to the global address `dst` with `.release` semantics at
/// `sys` scope.
///
/// Requires `sm_100` and PTX ISA 8.7.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-st-async>
#[inline]
#[target_feature(enable = "sm_100,ptx87")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn st_async_mmio_sys_u16(dst: *mut u16, value: u16) {
    llvm_st_async_mmio_sys_i16(dst.cast(), value)
}

/// Performs an MMIO store of `value` to the global address `dst` with `.release` semantics at
/// `sys` scope.
///
/// Requires `sm_100` and PTX ISA 8.7.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-st-async>
#[inline]
#[target_feature(enable = "sm_100,ptx87")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn st_async_mmio_sys_u32(dst: *mut u32, value: u32) {
    llvm_st_async_mmio_sys_i32(dst.cast(), value)
}

/// Performs an MMIO store of `value` to the global address `dst` with `.release` semantics at
/// `sys` scope.
///
/// Requires `sm_100` and PTX ISA 8.7.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-st-async>
#[inline]
#[target_feature(enable = "sm_100,ptx87")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn st_async_mmio_sys_u64(dst: *mut u64, value: u64) {
    llvm_st_async_mmio_sys_i64(dst.cast(), value)
}
