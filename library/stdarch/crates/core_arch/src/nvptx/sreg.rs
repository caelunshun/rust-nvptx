// --- LLM-generated --- //
//! Special registers.
//!
//! <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers>

#[allow(improper_ctypes)]
unsafe extern "llvm-intrinsic" {
    #[link_name = "llvm.nvvm.read.ptx.sreg.tid.x"]
    fn llvm_tid_x() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.tid.y"]
    fn llvm_tid_y() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.tid.z"]
    fn llvm_tid_z() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.ntid.x"]
    fn llvm_ntid_x() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.ntid.y"]
    fn llvm_ntid_y() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.ntid.z"]
    fn llvm_ntid_z() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.ctaid.x"]
    fn llvm_ctaid_x() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.ctaid.y"]
    fn llvm_ctaid_y() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.ctaid.z"]
    fn llvm_ctaid_z() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.nctaid.x"]
    fn llvm_nctaid_x() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.nctaid.y"]
    fn llvm_nctaid_y() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.nctaid.z"]
    fn llvm_nctaid_z() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.clusterid.x"]
    fn llvm_clusterid_x() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.clusterid.y"]
    fn llvm_clusterid_y() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.clusterid.z"]
    fn llvm_clusterid_z() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.nclusterid.x"]
    fn llvm_nclusterid_x() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.nclusterid.y"]
    fn llvm_nclusterid_y() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.nclusterid.z"]
    fn llvm_nclusterid_z() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.cluster.ctaid.x"]
    fn llvm_cluster_ctaid_x() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.cluster.ctaid.y"]
    fn llvm_cluster_ctaid_y() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.cluster.ctaid.z"]
    fn llvm_cluster_ctaid_z() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.cluster.nctaid.x"]
    fn llvm_cluster_nctaid_x() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.cluster.nctaid.y"]
    fn llvm_cluster_nctaid_y() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.cluster.nctaid.z"]
    fn llvm_cluster_nctaid_z() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.cluster.ctarank"]
    fn llvm_cluster_ctarank() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.cluster.nctarank"]
    fn llvm_cluster_nctarank() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.laneid"]
    fn llvm_laneid() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.warpid"]
    fn llvm_warpid() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.nwarpid"]
    fn llvm_nwarpid() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.smid"]
    fn llvm_smid() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.nsmid"]
    fn llvm_nsmid() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.gridid"]
    fn llvm_gridid() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.lanemask.eq"]
    fn llvm_lanemask_eq() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.lanemask.le"]
    fn llvm_lanemask_le() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.lanemask.lt"]
    fn llvm_lanemask_lt() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.lanemask.ge"]
    fn llvm_lanemask_ge() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.lanemask.gt"]
    fn llvm_lanemask_gt() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.clock"]
    fn llvm_clock() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.clock64"]
    fn llvm_clock64() -> u64;
    #[link_name = "llvm.nvvm.read.ptx.sreg.globaltimer"]
    fn llvm_globaltimer() -> u64;
    #[link_name = "llvm.nvvm.read.ptx.sreg.globaltimer.lo"]
    fn llvm_globaltimer_lo() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.total_smem_size"]
    fn llvm_total_smem_size() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.aggr_smem_size"]
    fn llvm_aggr_smem_size() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.dynamic_smem_size"]
    fn llvm_dynamic_smem_size() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.reserved_smem_offset_begin"]
    fn llvm_reserved_smem_offset_begin() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.reserved_smem_offset_end"]
    fn llvm_reserved_smem_offset_end() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.reserved_smem_offset_cap"]
    fn llvm_reserved_smem_offset_cap() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.warpsize"]
    fn llvm_warpsize() -> u32;
    #[link_name = "llvm.nvvm.is_explicit_cluster"]
    fn llvm_is_explicit_cluster() -> bool;
    #[link_name = "llvm.nvvm.read.ptx.sreg.envreg0"]
    fn llvm_envreg_0() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.envreg1"]
    fn llvm_envreg_1() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.envreg2"]
    fn llvm_envreg_2() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.envreg3"]
    fn llvm_envreg_3() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.envreg4"]
    fn llvm_envreg_4() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.envreg5"]
    fn llvm_envreg_5() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.envreg6"]
    fn llvm_envreg_6() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.envreg7"]
    fn llvm_envreg_7() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.envreg8"]
    fn llvm_envreg_8() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.envreg9"]
    fn llvm_envreg_9() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.envreg10"]
    fn llvm_envreg_10() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.envreg11"]
    fn llvm_envreg_11() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.envreg12"]
    fn llvm_envreg_12() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.envreg13"]
    fn llvm_envreg_13() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.envreg14"]
    fn llvm_envreg_14() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.envreg15"]
    fn llvm_envreg_15() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.envreg16"]
    fn llvm_envreg_16() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.envreg17"]
    fn llvm_envreg_17() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.envreg18"]
    fn llvm_envreg_18() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.envreg19"]
    fn llvm_envreg_19() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.envreg20"]
    fn llvm_envreg_20() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.envreg21"]
    fn llvm_envreg_21() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.envreg22"]
    fn llvm_envreg_22() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.envreg23"]
    fn llvm_envreg_23() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.envreg24"]
    fn llvm_envreg_24() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.envreg25"]
    fn llvm_envreg_25() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.envreg26"]
    fn llvm_envreg_26() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.envreg27"]
    fn llvm_envreg_27() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.envreg28"]
    fn llvm_envreg_28() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.envreg29"]
    fn llvm_envreg_29() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.envreg30"]
    fn llvm_envreg_30() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.envreg31"]
    fn llvm_envreg_31() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.pm0"]
    fn llvm_pm_0() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.pm1"]
    fn llvm_pm_1() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.pm2"]
    fn llvm_pm_2() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.pm3"]
    fn llvm_pm_3() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.reserved_smem_offset_0"]
    fn llvm_reserved_smem_offset_0() -> u32;
    #[link_name = "llvm.nvvm.read.ptx.sreg.reserved_smem_offset_1"]
    fn llvm_reserved_smem_offset_1() -> u32;
}

/// Thread identifier within the CTA, x component (`%tid.x`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-tid>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn tid_x() -> u32 {
    llvm_tid_x()
}

/// Thread identifier within the CTA, y component (`%tid.y`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-tid>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn tid_y() -> u32 {
    llvm_tid_y()
}

/// Thread identifier within the CTA, z component (`%tid.z`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-tid>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn tid_z() -> u32 {
    llvm_tid_z()
}

/// Number of threads in the CTA, x component (`%ntid.x`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-ntid>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn ntid_x() -> u32 {
    llvm_ntid_x()
}

/// Number of threads in the CTA, y component (`%ntid.y`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-ntid>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn ntid_y() -> u32 {
    llvm_ntid_y()
}

/// Number of threads in the CTA, z component (`%ntid.z`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-ntid>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn ntid_z() -> u32 {
    llvm_ntid_z()
}

/// CTA identifier within the grid, x component (`%ctaid.x`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-ctaid>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn ctaid_x() -> u32 {
    llvm_ctaid_x()
}

/// CTA identifier within the grid, y component (`%ctaid.y`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-ctaid>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn ctaid_y() -> u32 {
    llvm_ctaid_y()
}

/// CTA identifier within the grid, z component (`%ctaid.z`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-ctaid>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn ctaid_z() -> u32 {
    llvm_ctaid_z()
}

/// Number of CTAs in the grid, x component (`%nctaid.x`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-nctaid>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn nctaid_x() -> u32 {
    llvm_nctaid_x()
}

/// Number of CTAs in the grid, y component (`%nctaid.y`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-nctaid>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn nctaid_y() -> u32 {
    llvm_nctaid_y()
}

/// Number of CTAs in the grid, z component (`%nctaid.z`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-nctaid>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn nctaid_z() -> u32 {
    llvm_nctaid_z()
}

/// Cluster identifier within the grid, x component (`%clusterid.x`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-clusterid>
#[inline]
#[target_feature(enable = "sm_90,ptx78")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn clusterid_x() -> u32 {
    llvm_clusterid_x()
}

/// Cluster identifier within the grid, y component (`%clusterid.y`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-clusterid>
#[inline]
#[target_feature(enable = "sm_90,ptx78")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn clusterid_y() -> u32 {
    llvm_clusterid_y()
}

/// Cluster identifier within the grid, z component (`%clusterid.z`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-clusterid>
#[inline]
#[target_feature(enable = "sm_90,ptx78")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn clusterid_z() -> u32 {
    llvm_clusterid_z()
}

/// Number of clusters in the grid, x component (`%nclusterid.x`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-nclusterid>
#[inline]
#[target_feature(enable = "sm_90,ptx78")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn nclusterid_x() -> u32 {
    llvm_nclusterid_x()
}

/// Number of clusters in the grid, y component (`%nclusterid.y`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-nclusterid>
#[inline]
#[target_feature(enable = "sm_90,ptx78")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn nclusterid_y() -> u32 {
    llvm_nclusterid_y()
}

/// Number of clusters in the grid, z component (`%nclusterid.z`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-nclusterid>
#[inline]
#[target_feature(enable = "sm_90,ptx78")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn nclusterid_z() -> u32 {
    llvm_nclusterid_z()
}

/// CTA identifier within the cluster, x component (`%cluster_ctaid.x`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-cluster-ctaid>
#[inline]
#[target_feature(enable = "sm_90,ptx78")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cluster_ctaid_x() -> u32 {
    llvm_cluster_ctaid_x()
}

/// CTA identifier within the cluster, y component (`%cluster_ctaid.y`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-cluster-ctaid>
#[inline]
#[target_feature(enable = "sm_90,ptx78")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cluster_ctaid_y() -> u32 {
    llvm_cluster_ctaid_y()
}

/// CTA identifier within the cluster, z component (`%cluster_ctaid.z`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-cluster-ctaid>
#[inline]
#[target_feature(enable = "sm_90,ptx78")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cluster_ctaid_z() -> u32 {
    llvm_cluster_ctaid_z()
}

/// Number of CTAs in the cluster, x component (`%cluster_nctaid.x`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-cluster-nctaid>
#[inline]
#[target_feature(enable = "sm_90,ptx78")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cluster_nctaid_x() -> u32 {
    llvm_cluster_nctaid_x()
}

/// Number of CTAs in the cluster, y component (`%cluster_nctaid.y`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-cluster-nctaid>
#[inline]
#[target_feature(enable = "sm_90,ptx78")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cluster_nctaid_y() -> u32 {
    llvm_cluster_nctaid_y()
}

/// Number of CTAs in the cluster, z component (`%cluster_nctaid.z`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-cluster-nctaid>
#[inline]
#[target_feature(enable = "sm_90,ptx78")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cluster_nctaid_z() -> u32 {
    llvm_cluster_nctaid_z()
}

/// CTA rank within the cluster (`%cluster_ctarank`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-cluster-ctarank>
#[inline]
#[target_feature(enable = "sm_90,ptx78")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cluster_ctarank() -> u32 {
    llvm_cluster_ctarank()
}

/// Number of CTAs in the cluster (`%cluster_nctarank`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-cluster-nctarank>
#[inline]
#[target_feature(enable = "sm_90,ptx78")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cluster_nctarank() -> u32 {
    llvm_cluster_nctarank()
}

/// Lane identifier within the warp (`%laneid`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-laneid>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn laneid() -> u32 {
    llvm_laneid()
}

/// Warp identifier within the CTA (`%warpid`).
///
/// The value may change during execution, e.g. due to preemption; it is intended for
/// profiling and diagnostics.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-warpid>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn warpid() -> u32 {
    llvm_warpid()
}

/// Maximum number of warp identifiers (`%nwarpid`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-nwarpid>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn nwarpid() -> u32 {
    llvm_nwarpid()
}

/// Identifier of the SM the thread is executing on (`%smid`).
///
/// The value may change during execution, e.g. due to preemption; it is intended for
/// profiling and diagnostics.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-smid>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn smid() -> u32 {
    llvm_smid()
}

/// Maximum number of SM identifiers (`%nsmid`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-nsmid>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn nsmid() -> u32 {
    llvm_nsmid()
}

/// Temporal grid launch number (`%gridid`), truncated to 32 bits.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-gridid>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn gridid() -> u32 {
    llvm_gridid()
}

/// Mask with bits set for lanes equal to the thread's lane in the warp (`%lanemask_eq`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-lanemask-eq>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn lanemask_eq() -> u32 {
    llvm_lanemask_eq()
}

/// Mask with bits set for lanes less than or equal to the thread's lane in the warp (`%lanemask_le`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-lanemask-le>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn lanemask_le() -> u32 {
    llvm_lanemask_le()
}

/// Mask with bits set for lanes less than the thread's lane in the warp (`%lanemask_lt`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-lanemask-lt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn lanemask_lt() -> u32 {
    llvm_lanemask_lt()
}

/// Mask with bits set for lanes greater than or equal to the thread's lane in the warp (`%lanemask_ge`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-lanemask-ge>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn lanemask_ge() -> u32 {
    llvm_lanemask_ge()
}

/// Mask with bits set for lanes greater than the thread's lane in the warp (`%lanemask_gt`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-lanemask-gt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn lanemask_gt() -> u32 {
    llvm_lanemask_gt()
}

/// 32-bit cycle counter (`%clock`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-clock>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn clock() -> u32 {
    llvm_clock()
}

/// 64-bit cycle counter (`%clock64`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-clock64>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn clock64() -> u64 {
    llvm_clock64()
}

/// 64-bit global nanosecond timer (`%globaltimer`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-globaltimer>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn globaltimer() -> u64 {
    llvm_globaltimer()
}

/// Lower 32 bits of the global nanosecond timer (`%globaltimer_lo`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-globaltimer>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn globaltimer_lo() -> u32 {
    llvm_globaltimer_lo()
}

/// Total shared memory allocated for the CTA at launch, excluding memory reserved for
/// system software (`%total_smem_size`).
///
/// The size is in units of the target's shared memory allocation unit.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-total-smem-size>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn total_smem_size() -> u32 {
    llvm_total_smem_size()
}

/// Total shared memory allocated for the CTA at launch, including memory reserved for
/// system software (`%aggr_smem_size`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-aggr-smem-size>
#[inline]
#[target_feature(enable = "sm_90,ptx81")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn aggr_smem_size() -> u32 {
    llvm_aggr_smem_size()
}

/// Shared memory allocated dynamically for the CTA at launch (`%dynamic_smem_size`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-dynamic-smem-size>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn dynamic_smem_size() -> u32 {
    llvm_dynamic_smem_size()
}

/// Start of the shared memory region reserved for system software (`%reserved_smem_offset_begin`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-reserved-smem>
#[inline]
#[target_feature(enable = "sm_80,ptx76")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn reserved_smem_offset_begin() -> u32 {
    llvm_reserved_smem_offset_begin()
}

/// End of the shared memory region reserved for system software (`%reserved_smem_offset_end`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-reserved-smem>
#[inline]
#[target_feature(enable = "sm_80,ptx76")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn reserved_smem_offset_end() -> u32 {
    llvm_reserved_smem_offset_end()
}

/// Total size of the shared memory region reserved for system software (`%reserved_smem_offset_cap`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-reserved-smem>
#[inline]
#[target_feature(enable = "sm_80,ptx76")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn reserved_smem_offset_cap() -> u32 {
    llvm_reserved_smem_offset_cap()
}

/// Number of threads in a warp (`WARP_SZ`).
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn warpsize() -> u32 {
    llvm_warpsize()
}

/// Whether the cluster launch was explicitly specified (`%is_explicit_cluster`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-is-explicit-cluster>
#[inline]
#[target_feature(enable = "sm_90,ptx78")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn is_explicit_cluster() -> bool {
    llvm_is_explicit_cluster()
}

/// Driver-defined read-only register `%envreg<N>`.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-envreg-32>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn envreg<const N: u32>() -> u32 {
    static_assert!(N < 32);
    match N {
        0 => llvm_envreg_0(),
        1 => llvm_envreg_1(),
        2 => llvm_envreg_2(),
        3 => llvm_envreg_3(),
        4 => llvm_envreg_4(),
        5 => llvm_envreg_5(),
        6 => llvm_envreg_6(),
        7 => llvm_envreg_7(),
        8 => llvm_envreg_8(),
        9 => llvm_envreg_9(),
        10 => llvm_envreg_10(),
        11 => llvm_envreg_11(),
        12 => llvm_envreg_12(),
        13 => llvm_envreg_13(),
        14 => llvm_envreg_14(),
        15 => llvm_envreg_15(),
        16 => llvm_envreg_16(),
        17 => llvm_envreg_17(),
        18 => llvm_envreg_18(),
        19 => llvm_envreg_19(),
        20 => llvm_envreg_20(),
        21 => llvm_envreg_21(),
        22 => llvm_envreg_22(),
        23 => llvm_envreg_23(),
        24 => llvm_envreg_24(),
        25 => llvm_envreg_25(),
        26 => llvm_envreg_26(),
        27 => llvm_envreg_27(),
        28 => llvm_envreg_28(),
        29 => llvm_envreg_29(),
        30 => llvm_envreg_30(),
        31 => llvm_envreg_31(),
        _ => unreachable!(),
    }
}

/// Performance monitoring counter `%pm<N>`.
///
/// The behavior of the counters is currently undefined by the PTX ISA.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-pm0-pm7>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn pm<const N: u32>() -> u32 {
    static_assert!(N < 4);
    match N {
        0 => llvm_pm_0(),
        1 => llvm_pm_1(),
        2 => llvm_pm_2(),
        3 => llvm_pm_3(),
        _ => unreachable!(),
    }
}

/// Offset `%reserved_smem_offset_<N>` in the shared memory region reserved for
/// system software.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#special-registers-reserved-smem>
#[inline]
#[target_feature(enable = "sm_80,ptx76")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn reserved_smem_offset<const N: u32>() -> u32 {
    static_assert!(N < 2);
    match N {
        0 => llvm_reserved_smem_offset_0(),
        1 => llvm_reserved_smem_offset_1(),
        _ => unreachable!(),
    }
}
