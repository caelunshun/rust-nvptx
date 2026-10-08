// --- LLM-generated --- //
//! Tensor map modification (`tensormap.replace`).

use super::StateSpace;
use crate::ffi::c_void;
use crate::marker::ConstParamTy;

#[allow(improper_ctypes)]
unsafe extern "llvm-intrinsic" {
    #[link_name = "llvm.nvvm.tensormap.replace.global.address.p1"]
    #[cfg_attr(target_arch = "nvptx64", rustc_llvm_ptr_addrspace(args(1, 0)))]
    fn llvm_tensormap_replace_global_address_global(tmap: *mut c_void, new_value: u64);
    #[link_name = "llvm.nvvm.tensormap.replace.global.address.p3"]
    #[cfg_attr(target_arch = "nvptx64", rustc_llvm_ptr_addrspace(args(3, 0)))]
    fn llvm_tensormap_replace_global_address_shared(tmap: *mut c_void, new_value: u64);

    #[link_name = "llvm.nvvm.tensormap.replace.rank.p1"]
    #[cfg_attr(target_arch = "nvptx64", rustc_llvm_ptr_addrspace(args(1, 0)))]
    fn llvm_tensormap_replace_rank_global(tmap: *mut c_void, new_value: u32);
    #[link_name = "llvm.nvvm.tensormap.replace.rank.p3"]
    #[cfg_attr(target_arch = "nvptx64", rustc_llvm_ptr_addrspace(args(3, 0)))]
    fn llvm_tensormap_replace_rank_shared(tmap: *mut c_void, new_value: u32);

    #[link_name = "llvm.nvvm.tensormap.replace.global.stride.p1"]
    #[cfg_attr(target_arch = "nvptx64", rustc_llvm_ptr_addrspace(args(1, 0, 0)))]
    fn llvm_tensormap_replace_global_stride_global(tmap: *mut c_void, ord: i32, new_value: u64);
    #[link_name = "llvm.nvvm.tensormap.replace.global.stride.p3"]
    #[cfg_attr(target_arch = "nvptx64", rustc_llvm_ptr_addrspace(args(3, 0, 0)))]
    fn llvm_tensormap_replace_global_stride_shared(tmap: *mut c_void, ord: i32, new_value: u64);

    #[link_name = "llvm.nvvm.tensormap.replace.element.stride.p1"]
    #[cfg_attr(target_arch = "nvptx64", rustc_llvm_ptr_addrspace(args(1, 0, 0)))]
    fn llvm_tensormap_replace_element_stride_global(tmap: *mut c_void, ord: i32, new_value: u32);
    #[link_name = "llvm.nvvm.tensormap.replace.element.stride.p3"]
    #[cfg_attr(target_arch = "nvptx64", rustc_llvm_ptr_addrspace(args(3, 0, 0)))]
    fn llvm_tensormap_replace_element_stride_shared(tmap: *mut c_void, ord: i32, new_value: u32);

    #[link_name = "llvm.nvvm.tensormap.replace.global.dim.p1"]
    #[cfg_attr(target_arch = "nvptx64", rustc_llvm_ptr_addrspace(args(1, 0, 0)))]
    fn llvm_tensormap_replace_global_dim_global(tmap: *mut c_void, ord: i32, new_value: u32);
    #[link_name = "llvm.nvvm.tensormap.replace.global.dim.p3"]
    #[cfg_attr(target_arch = "nvptx64", rustc_llvm_ptr_addrspace(args(3, 0, 0)))]
    fn llvm_tensormap_replace_global_dim_shared(tmap: *mut c_void, ord: i32, new_value: u32);

    #[link_name = "llvm.nvvm.tensormap.replace.box.dim.p1"]
    #[cfg_attr(target_arch = "nvptx64", rustc_llvm_ptr_addrspace(args(1, 0, 0)))]
    fn llvm_tensormap_replace_box_dim_global(tmap: *mut c_void, ord: i32, new_value: u32);
    #[link_name = "llvm.nvvm.tensormap.replace.box.dim.p3"]
    #[cfg_attr(target_arch = "nvptx64", rustc_llvm_ptr_addrspace(args(3, 0, 0)))]
    fn llvm_tensormap_replace_box_dim_shared(tmap: *mut c_void, ord: i32, new_value: u32);

    #[link_name = "llvm.nvvm.tensormap.replace.elemtype.p1"]
    #[cfg_attr(target_arch = "nvptx64", rustc_llvm_ptr_addrspace(args(1, 0)))]
    fn llvm_tensormap_replace_elemtype_global(tmap: *mut c_void, elemtype: i32);
    #[link_name = "llvm.nvvm.tensormap.replace.elemtype.p3"]
    #[cfg_attr(target_arch = "nvptx64", rustc_llvm_ptr_addrspace(args(3, 0)))]
    fn llvm_tensormap_replace_elemtype_shared(tmap: *mut c_void, elemtype: i32);

    #[link_name = "llvm.nvvm.tensormap.replace.interleave.layout.p1"]
    #[cfg_attr(target_arch = "nvptx64", rustc_llvm_ptr_addrspace(args(1, 0)))]
    fn llvm_tensormap_replace_interleave_layout_global(tmap: *mut c_void, layout: i32);
    #[link_name = "llvm.nvvm.tensormap.replace.interleave.layout.p3"]
    #[cfg_attr(target_arch = "nvptx64", rustc_llvm_ptr_addrspace(args(3, 0)))]
    fn llvm_tensormap_replace_interleave_layout_shared(tmap: *mut c_void, layout: i32);

    #[link_name = "llvm.nvvm.tensormap.replace.swizzle.mode.p1"]
    #[cfg_attr(target_arch = "nvptx64", rustc_llvm_ptr_addrspace(args(1, 0)))]
    fn llvm_tensormap_replace_swizzle_mode_global(tmap: *mut c_void, mode: i32);
    #[link_name = "llvm.nvvm.tensormap.replace.swizzle.mode.p3"]
    #[cfg_attr(target_arch = "nvptx64", rustc_llvm_ptr_addrspace(args(3, 0)))]
    fn llvm_tensormap_replace_swizzle_mode_shared(tmap: *mut c_void, mode: i32);

    #[link_name = "llvm.nvvm.tensormap.replace.swizzle.atomicity.p1"]
    #[cfg_attr(target_arch = "nvptx64", rustc_llvm_ptr_addrspace(args(1, 0)))]
    fn llvm_tensormap_replace_swizzle_atomicity_global(tmap: *mut c_void, atomicity: i32);
    #[link_name = "llvm.nvvm.tensormap.replace.swizzle.atomicity.p3"]
    #[cfg_attr(target_arch = "nvptx64", rustc_llvm_ptr_addrspace(args(3, 0)))]
    fn llvm_tensormap_replace_swizzle_atomicity_shared(tmap: *mut c_void, atomicity: i32);

    #[link_name = "llvm.nvvm.tensormap.replace.fill.mode.p1"]
    #[cfg_attr(target_arch = "nvptx64", rustc_llvm_ptr_addrspace(args(1, 0)))]
    fn llvm_tensormap_replace_fill_mode_global(tmap: *mut c_void, mode: i32);
    #[link_name = "llvm.nvvm.tensormap.replace.fill.mode.p3"]
    #[cfg_attr(target_arch = "nvptx64", rustc_llvm_ptr_addrspace(args(3, 0)))]
    fn llvm_tensormap_replace_fill_mode_shared(tmap: *mut c_void, mode: i32);
}

/// Element type of a tensor map (`.elemtype`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, ConstParamTy)]
#[non_exhaustive]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub enum TensormapElemType {
    /// `u8`
    U8 = 0,
    /// `u16`
    U16 = 1,
    /// `u32`
    U32 = 2,
    /// `s32`
    S32 = 3,
    /// `u64`
    U64 = 4,
    /// `s64`
    S64 = 5,
    /// `f16`
    F16 = 6,
    /// `f32`
    F32 = 7,
    /// `f32.ftz`
    F32Ftz = 8,
    /// `f64`
    F64 = 9,
    /// `bf16`
    Bf16 = 10,
    /// `tf32`
    Tf32 = 11,
    /// `tf32.ftz`
    Tf32Ftz = 12,
    /// `b4x16`, which requires `sm_100a`, `sm_101a` or `sm_120a` with PTX ISA 8.7, or the
    /// corresponding `f` variants with PTX ISA 8.8 or 9.0.
    B4x16 = 13,
    /// `b4x16_p64`, which has the same requirements as [`TensormapElemType::B4x16`].
    B4x16P64 = 14,
    /// `b6x16_p32`, which has the same requirements as [`TensormapElemType::B4x16`].
    B6x16P32 = 15,
}

/// Interleave layout of a tensor map (`.interleave_layout`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, ConstParamTy)]
#[non_exhaustive]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub enum TensormapInterleaveLayout {
    /// No interleave.
    NoInterleave = 0,
    /// 16-byte interleave.
    Interleave16B = 1,
    /// 32-byte interleave.
    Interleave32B = 2,
}

/// Swizzle mode of a tensor map (`.swizzle_mode`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, ConstParamTy)]
#[non_exhaustive]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub enum TensormapSwizzleMode {
    /// No swizzle.
    NoSwizzle = 0,
    /// 32-byte swizzle.
    Swizzle32B = 1,
    /// 64-byte swizzle.
    Swizzle64B = 2,
    /// 128-byte swizzle.
    Swizzle128B = 3,
    /// 96-byte swizzle, which requires `sm_103a` and PTX ISA 8.8.
    Swizzle96B = 4,
}

/// Swizzle atomicity of a tensor map (`.swizzle_atomicity`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, ConstParamTy)]
#[non_exhaustive]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub enum TensormapSwizzleAtomicity {
    /// 16-byte atomicity.
    Atomicity16B = 0,
    /// 32-byte atomicity.
    Atomicity32B = 1,
    /// 32-byte atomicity with 8-byte flip.
    Atomicity32BFlip8B = 2,
    /// 64-byte atomicity.
    Atomicity64B = 3,
}

/// Out-of-bounds fill mode of a tensor map (`.fill_mode`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, ConstParamTy)]
#[non_exhaustive]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub enum TensormapFillMode {
    /// Fill out-of-bounds elements with zeros.
    ZeroFill = 0,
    /// Fill out-of-bounds elements with NaN.
    OobNanFill = 1,
}

/// Replaces the `global_address` field of the tensor map at `tmap` with `new_value`.
///
/// `SPACE` must be [`StateSpace::Global`] or [`StateSpace::SharedCta`], and `tmap` must fall
/// within that state space.
///
/// Requires `sm_90a`, `sm_100a`, `sm_101a` or `sm_120a` with PTX ISA 8.3; `sm_90f`, `sm_100f`,
/// `sm_101f` or `sm_120f` with PTX ISA 8.8; or `sm_90f`, `sm_100f`, `sm_110f` or `sm_120f` with
/// PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-tensormap-replace>
#[inline]
#[target_feature(enable = "sm_90a,ptx83")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn tensormap_replace_global_address<const SPACE: StateSpace>(
    tmap: *mut c_void,
    new_value: u64,
) {
    static_assert!(
        matches!(SPACE, StateSpace::Global | StateSpace::SharedCta),
        "tensormap_replace_global_address only supports the global and shared::cta state spaces"
    );
    match SPACE {
        StateSpace::Global => llvm_tensormap_replace_global_address_global(tmap, new_value),
        StateSpace::SharedCta => llvm_tensormap_replace_global_address_shared(tmap, new_value),
        _ => unreachable!(),
    }
}

/// Replaces the `rank` field of the tensor map at `tmap` with `new_value`.
///
/// `new_value` must be one less than the desired tensor rank, as this field uses zero-based
/// numbering. `SPACE` must be [`StateSpace::Global`] or [`StateSpace::SharedCta`], and `tmap`
/// must fall within that state space.
///
/// Requires `sm_90a`, `sm_100a`, `sm_101a` or `sm_120a` with PTX ISA 8.3; `sm_90f`, `sm_100f`,
/// `sm_101f` or `sm_120f` with PTX ISA 8.8; or `sm_90f`, `sm_100f`, `sm_110f` or `sm_120f` with
/// PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-tensormap-replace>
#[inline]
#[target_feature(enable = "sm_90a,ptx83")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn tensormap_replace_rank<const SPACE: StateSpace>(tmap: *mut c_void, new_value: u32) {
    static_assert!(
        matches!(SPACE, StateSpace::Global | StateSpace::SharedCta),
        "tensormap_replace_rank only supports the global and shared::cta state spaces"
    );
    match SPACE {
        StateSpace::Global => llvm_tensormap_replace_rank_global(tmap, new_value),
        StateSpace::SharedCta => llvm_tensormap_replace_rank_shared(tmap, new_value),
        _ => unreachable!(),
    }
}

/// Replaces the `ORD`-th element of the `global_stride` field of the tensor map at `tmap` with
/// `new_value`.
///
/// `ORD` must be less than 5. `SPACE` must be [`StateSpace::Global`] or
/// [`StateSpace::SharedCta`], and `tmap` must fall within that state space.
///
/// Requires `sm_90a`, `sm_100a`, `sm_101a` or `sm_120a` with PTX ISA 8.3; `sm_90f`, `sm_100f`,
/// `sm_101f` or `sm_120f` with PTX ISA 8.8; or `sm_90f`, `sm_100f`, `sm_110f` or `sm_120f` with
/// PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-tensormap-replace>
#[inline]
#[target_feature(enable = "sm_90a,ptx83")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn tensormap_replace_global_stride<const SPACE: StateSpace, const ORD: u32>(
    tmap: *mut c_void,
    new_value: u64,
) {
    static_assert!(
        matches!(SPACE, StateSpace::Global | StateSpace::SharedCta),
        "tensormap_replace_global_stride only supports the global and shared::cta state spaces"
    );
    static_assert!(ORD < 5, "tensormap_replace_global_stride requires ORD < 5");
    match SPACE {
        StateSpace::Global => {
            llvm_tensormap_replace_global_stride_global(tmap, ORD as i32, new_value)
        }
        StateSpace::SharedCta => {
            llvm_tensormap_replace_global_stride_shared(tmap, ORD as i32, new_value)
        }
        _ => unreachable!(),
    }
}

/// Replaces the `ORD`-th element of the `element_stride` field of the tensor map at `tmap` with
/// `new_value`.
///
/// `ORD` must be less than 5. `SPACE` must be [`StateSpace::Global`] or
/// [`StateSpace::SharedCta`], and `tmap` must fall within that state space.
///
/// Requires `sm_90a`, `sm_100a`, `sm_101a` or `sm_120a` with PTX ISA 8.3; `sm_90f`, `sm_100f`,
/// `sm_101f` or `sm_120f` with PTX ISA 8.8; or `sm_90f`, `sm_100f`, `sm_110f` or `sm_120f` with
/// PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-tensormap-replace>
#[inline]
#[target_feature(enable = "sm_90a,ptx83")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn tensormap_replace_element_stride<const SPACE: StateSpace, const ORD: u32>(
    tmap: *mut c_void,
    new_value: u32,
) {
    static_assert!(
        matches!(SPACE, StateSpace::Global | StateSpace::SharedCta),
        "tensormap_replace_element_stride only supports the global and shared::cta state spaces"
    );
    static_assert!(ORD < 5, "tensormap_replace_element_stride requires ORD < 5");
    match SPACE {
        StateSpace::Global => {
            llvm_tensormap_replace_element_stride_global(tmap, ORD as i32, new_value)
        }
        StateSpace::SharedCta => {
            llvm_tensormap_replace_element_stride_shared(tmap, ORD as i32, new_value)
        }
        _ => unreachable!(),
    }
}

/// Replaces the `ORD`-th element of the `global_dim` field of the tensor map at `tmap` with
/// `new_value`.
///
/// `ORD` must be less than 5. `SPACE` must be [`StateSpace::Global`] or
/// [`StateSpace::SharedCta`], and `tmap` must fall within that state space.
///
/// Requires `sm_90a`, `sm_100a`, `sm_101a` or `sm_120a` with PTX ISA 8.3; `sm_90f`, `sm_100f`,
/// `sm_101f` or `sm_120f` with PTX ISA 8.8; or `sm_90f`, `sm_100f`, `sm_110f` or `sm_120f` with
/// PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-tensormap-replace>
#[inline]
#[target_feature(enable = "sm_90a,ptx83")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn tensormap_replace_global_dim<const SPACE: StateSpace, const ORD: u32>(
    tmap: *mut c_void,
    new_value: u32,
) {
    static_assert!(
        matches!(SPACE, StateSpace::Global | StateSpace::SharedCta),
        "tensormap_replace_global_dim only supports the global and shared::cta state spaces"
    );
    static_assert!(ORD < 5, "tensormap_replace_global_dim requires ORD < 5");
    match SPACE {
        StateSpace::Global => llvm_tensormap_replace_global_dim_global(tmap, ORD as i32, new_value),
        StateSpace::SharedCta => {
            llvm_tensormap_replace_global_dim_shared(tmap, ORD as i32, new_value)
        }
        _ => unreachable!(),
    }
}

/// Replaces the `ORD`-th element of the `box_dim` field of the tensor map at `tmap` with
/// `new_value`.
///
/// `ORD` must be less than 5. `SPACE` must be [`StateSpace::Global`] or
/// [`StateSpace::SharedCta`], and `tmap` must fall within that state space.
///
/// Requires `sm_90a`, `sm_100a`, `sm_101a` or `sm_120a` with PTX ISA 8.3; `sm_90f`, `sm_100f`,
/// `sm_101f` or `sm_120f` with PTX ISA 8.8; or `sm_90f`, `sm_100f`, `sm_110f` or `sm_120f` with
/// PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-tensormap-replace>
#[inline]
#[target_feature(enable = "sm_90a,ptx83")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn tensormap_replace_box_dim<const SPACE: StateSpace, const ORD: u32>(
    tmap: *mut c_void,
    new_value: u32,
) {
    static_assert!(
        matches!(SPACE, StateSpace::Global | StateSpace::SharedCta),
        "tensormap_replace_box_dim only supports the global and shared::cta state spaces"
    );
    static_assert!(ORD < 5, "tensormap_replace_box_dim requires ORD < 5");
    match SPACE {
        StateSpace::Global => llvm_tensormap_replace_box_dim_global(tmap, ORD as i32, new_value),
        StateSpace::SharedCta => llvm_tensormap_replace_box_dim_shared(tmap, ORD as i32, new_value),
        _ => unreachable!(),
    }
}

/// Replaces the `elemtype` field of the tensor map at `tmap` with `ELEMTYPE`.
///
/// `SPACE` must be [`StateSpace::Global`] or [`StateSpace::SharedCta`], and `tmap` must fall
/// within that state space.
///
/// Requires `sm_90a`, `sm_100a`, `sm_101a` or `sm_120a` with PTX ISA 8.3; `sm_90f`, `sm_100f`,
/// `sm_101f` or `sm_120f` with PTX ISA 8.8; or `sm_90f`, `sm_100f`, `sm_110f` or `sm_120f` with
/// PTX ISA 9.0. The `ELEMTYPE` values [`TensormapElemType::B4x16`],
/// [`TensormapElemType::B4x16P64`] and [`TensormapElemType::B6x16P32`] have stricter requirements,
/// as documented on [`TensormapElemType`].
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-tensormap-replace>
#[inline]
#[target_feature(enable = "sm_90a,ptx83")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn tensormap_replace_elemtype<
    const SPACE: StateSpace,
    const ELEMTYPE: TensormapElemType,
>(
    tmap: *mut c_void,
) {
    static_assert!(
        matches!(SPACE, StateSpace::Global | StateSpace::SharedCta),
        "tensormap_replace_elemtype only supports the global and shared::cta state spaces"
    );
    match SPACE {
        StateSpace::Global => llvm_tensormap_replace_elemtype_global(tmap, ELEMTYPE as i32),
        StateSpace::SharedCta => llvm_tensormap_replace_elemtype_shared(tmap, ELEMTYPE as i32),
        _ => unreachable!(),
    }
}

/// Replaces the `interleave_layout` field of the tensor map at `tmap` with `LAYOUT`.
///
/// `SPACE` must be [`StateSpace::Global`] or [`StateSpace::SharedCta`], and `tmap` must fall
/// within that state space.
///
/// Requires `sm_90a`, `sm_100a`, `sm_101a` or `sm_120a` with PTX ISA 8.3; `sm_90f`, `sm_100f`,
/// `sm_101f` or `sm_120f` with PTX ISA 8.8; or `sm_90f`, `sm_100f`, `sm_110f` or `sm_120f` with
/// PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-tensormap-replace>
#[inline]
#[target_feature(enable = "sm_90a,ptx83")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn tensormap_replace_interleave_layout<
    const SPACE: StateSpace,
    const LAYOUT: TensormapInterleaveLayout,
>(
    tmap: *mut c_void,
) {
    static_assert!(
        matches!(SPACE, StateSpace::Global | StateSpace::SharedCta),
        "tensormap_replace_interleave_layout only supports the global and shared::cta state spaces"
    );
    match SPACE {
        StateSpace::Global => llvm_tensormap_replace_interleave_layout_global(tmap, LAYOUT as i32),
        StateSpace::SharedCta => {
            llvm_tensormap_replace_interleave_layout_shared(tmap, LAYOUT as i32)
        }
        _ => unreachable!(),
    }
}

/// Replaces the `swizzle_mode` field of the tensor map at `tmap` with `MODE`.
///
/// `SPACE` must be [`StateSpace::Global`] or [`StateSpace::SharedCta`], and `tmap` must fall
/// within that state space.
///
/// Requires `sm_90a`, `sm_100a`, `sm_101a` or `sm_120a` with PTX ISA 8.3; `sm_90f`, `sm_100f`,
/// `sm_101f` or `sm_120f` with PTX ISA 8.8; or `sm_90f`, `sm_100f`, `sm_110f` or `sm_120f` with
/// PTX ISA 9.0. The `MODE` value [`TensormapSwizzleMode::Swizzle96B`] requires `sm_103a` and PTX
/// ISA 8.8.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-tensormap-replace>
#[inline]
#[target_feature(enable = "sm_90a,ptx83")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn tensormap_replace_swizzle_mode<
    const SPACE: StateSpace,
    const MODE: TensormapSwizzleMode,
>(
    tmap: *mut c_void,
) {
    static_assert!(
        matches!(SPACE, StateSpace::Global | StateSpace::SharedCta),
        "tensormap_replace_swizzle_mode only supports the global and shared::cta state spaces"
    );
    match SPACE {
        StateSpace::Global => llvm_tensormap_replace_swizzle_mode_global(tmap, MODE as i32),
        StateSpace::SharedCta => llvm_tensormap_replace_swizzle_mode_shared(tmap, MODE as i32),
        _ => unreachable!(),
    }
}

/// Replaces the `swizzle_atomicity` field of the tensor map at `tmap` with `ATOMICITY`.
///
/// `SPACE` must be [`StateSpace::Global`] or [`StateSpace::SharedCta`], and `tmap` must fall
/// within that state space.
///
/// Requires `sm_100a`, `sm_101a` or `sm_120a` with PTX ISA 8.7; `sm_100f`, `sm_101f` or `sm_120f`
/// with PTX ISA 8.8; or `sm_100f`, `sm_110f` or `sm_120f` with PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-tensormap-replace>
#[inline]
#[target_feature(enable = "sm_100a,ptx87")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn tensormap_replace_swizzle_atomicity<
    const SPACE: StateSpace,
    const ATOMICITY: TensormapSwizzleAtomicity,
>(
    tmap: *mut c_void,
) {
    static_assert!(
        matches!(SPACE, StateSpace::Global | StateSpace::SharedCta),
        "tensormap_replace_swizzle_atomicity only supports the global and shared::cta state spaces"
    );
    match SPACE {
        StateSpace::Global => {
            llvm_tensormap_replace_swizzle_atomicity_global(tmap, ATOMICITY as i32)
        }
        StateSpace::SharedCta => {
            llvm_tensormap_replace_swizzle_atomicity_shared(tmap, ATOMICITY as i32)
        }
        _ => unreachable!(),
    }
}

/// Replaces the `fill_mode` field of the tensor map at `tmap` with `MODE`.
///
/// `SPACE` must be [`StateSpace::Global`] or [`StateSpace::SharedCta`], and `tmap` must fall
/// within that state space.
///
/// Requires `sm_90a`, `sm_100a`, `sm_101a` or `sm_120a` with PTX ISA 8.3; `sm_90f`, `sm_100f`,
/// `sm_101f` or `sm_120f` with PTX ISA 8.8; or `sm_90f`, `sm_100f`, `sm_110f` or `sm_120f` with
/// PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-tensormap-replace>
#[inline]
#[target_feature(enable = "sm_90a,ptx83")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn tensormap_replace_fill_mode<
    const SPACE: StateSpace,
    const MODE: TensormapFillMode,
>(
    tmap: *mut c_void,
) {
    static_assert!(
        matches!(SPACE, StateSpace::Global | StateSpace::SharedCta),
        "tensormap_replace_fill_mode only supports the global and shared::cta state spaces"
    );
    match SPACE {
        StateSpace::Global => llvm_tensormap_replace_fill_mode_global(tmap, MODE as i32),
        StateSpace::SharedCta => llvm_tensormap_replace_fill_mode_shared(tmap, MODE as i32),
        _ => unreachable!(),
    }
}
