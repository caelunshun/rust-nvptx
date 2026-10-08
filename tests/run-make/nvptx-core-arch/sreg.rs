// --- LLM-generated --- //
// Checks the PTX emitted for the special register intrinsics in `core::arch::nvptx`.
// nvptx-target: sm_90a +ptx81

#![feature(no_core, stdarch_nvptx, abi_ptx)]
#![no_core]

extern crate core;

use core::arch::nvptx::*;

// CHECK-LABEL: .entry sreg_aggr_smem_size(
// CHECK: mov.u32 {{%r[0-9]+}}, %aggr_smem_size;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_aggr_smem_size(out: *mut u32) {
    unsafe { *out = aggr_smem_size() }
}

// CHECK-LABEL: .entry sreg_clock(
// CHECK: mov.u32 {{%r[0-9]+}}, %clock;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_clock(out: *mut u32) {
    unsafe { *out = clock() }
}

// CHECK-LABEL: .entry sreg_clock64(
// CHECK: mov.u64 {{%rd[0-9]+}}, %clock64;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_clock64(out: *mut u64) {
    unsafe { *out = clock64() }
}

// CHECK-LABEL: .entry sreg_cluster_ctaid_x(
// CHECK: mov.u32 {{%r[0-9]+}}, %cluster_ctaid.x;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_cluster_ctaid_x(out: *mut u32) {
    unsafe { *out = cluster_ctaid_x() }
}

// CHECK-LABEL: .entry sreg_cluster_ctaid_y(
// CHECK: mov.u32 {{%r[0-9]+}}, %cluster_ctaid.y;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_cluster_ctaid_y(out: *mut u32) {
    unsafe { *out = cluster_ctaid_y() }
}

// CHECK-LABEL: .entry sreg_cluster_ctaid_z(
// CHECK: mov.u32 {{%r[0-9]+}}, %cluster_ctaid.z;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_cluster_ctaid_z(out: *mut u32) {
    unsafe { *out = cluster_ctaid_z() }
}

// CHECK-LABEL: .entry sreg_cluster_ctarank(
// CHECK: mov.u32 {{%r[0-9]+}}, %cluster_ctarank;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_cluster_ctarank(out: *mut u32) {
    unsafe { *out = cluster_ctarank() }
}

// CHECK-LABEL: .entry sreg_cluster_nctaid_x(
// CHECK: mov.u32 {{%r[0-9]+}}, %cluster_nctaid.x;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_cluster_nctaid_x(out: *mut u32) {
    unsafe { *out = cluster_nctaid_x() }
}

// CHECK-LABEL: .entry sreg_cluster_nctaid_y(
// CHECK: mov.u32 {{%r[0-9]+}}, %cluster_nctaid.y;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_cluster_nctaid_y(out: *mut u32) {
    unsafe { *out = cluster_nctaid_y() }
}

// CHECK-LABEL: .entry sreg_cluster_nctaid_z(
// CHECK: mov.u32 {{%r[0-9]+}}, %cluster_nctaid.z;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_cluster_nctaid_z(out: *mut u32) {
    unsafe { *out = cluster_nctaid_z() }
}

// CHECK-LABEL: .entry sreg_cluster_nctarank(
// CHECK: mov.u32 {{%r[0-9]+}}, %cluster_nctarank;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_cluster_nctarank(out: *mut u32) {
    unsafe { *out = cluster_nctarank() }
}

// CHECK-LABEL: .entry sreg_clusterid_x(
// CHECK: mov.u32 {{%r[0-9]+}}, %clusterid.x;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_clusterid_x(out: *mut u32) {
    unsafe { *out = clusterid_x() }
}

// CHECK-LABEL: .entry sreg_clusterid_y(
// CHECK: mov.u32 {{%r[0-9]+}}, %clusterid.y;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_clusterid_y(out: *mut u32) {
    unsafe { *out = clusterid_y() }
}

// CHECK-LABEL: .entry sreg_clusterid_z(
// CHECK: mov.u32 {{%r[0-9]+}}, %clusterid.z;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_clusterid_z(out: *mut u32) {
    unsafe { *out = clusterid_z() }
}

// CHECK-LABEL: .entry sreg_ctaid_x(
// CHECK: mov.u32 {{%r[0-9]+}}, %ctaid.x;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_ctaid_x(out: *mut u32) {
    unsafe { *out = ctaid_x() }
}

// CHECK-LABEL: .entry sreg_ctaid_y(
// CHECK: mov.u32 {{%r[0-9]+}}, %ctaid.y;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_ctaid_y(out: *mut u32) {
    unsafe { *out = ctaid_y() }
}

// CHECK-LABEL: .entry sreg_ctaid_z(
// CHECK: mov.u32 {{%r[0-9]+}}, %ctaid.z;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_ctaid_z(out: *mut u32) {
    unsafe { *out = ctaid_z() }
}

// CHECK-LABEL: .entry sreg_dynamic_smem_size(
// CHECK: mov.u32 {{%r[0-9]+}}, %dynamic_smem_size;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_dynamic_smem_size(out: *mut u32) {
    unsafe { *out = dynamic_smem_size() }
}

// CHECK-LABEL: .entry sreg_envreg_00(
// CHECK: mov.b32 {{%r[0-9]+}}, %envreg0;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_envreg_00(out: *mut u32) {
    unsafe { *out = envreg::<0>() }
}

// CHECK-LABEL: .entry sreg_envreg_01(
// CHECK: mov.b32 {{%r[0-9]+}}, %envreg1;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_envreg_01(out: *mut u32) {
    unsafe { *out = envreg::<1>() }
}

// CHECK-LABEL: .entry sreg_envreg_02(
// CHECK: mov.b32 {{%r[0-9]+}}, %envreg2;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_envreg_02(out: *mut u32) {
    unsafe { *out = envreg::<2>() }
}

// CHECK-LABEL: .entry sreg_envreg_03(
// CHECK: mov.b32 {{%r[0-9]+}}, %envreg3;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_envreg_03(out: *mut u32) {
    unsafe { *out = envreg::<3>() }
}

// CHECK-LABEL: .entry sreg_envreg_04(
// CHECK: mov.b32 {{%r[0-9]+}}, %envreg4;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_envreg_04(out: *mut u32) {
    unsafe { *out = envreg::<4>() }
}

// CHECK-LABEL: .entry sreg_envreg_05(
// CHECK: mov.b32 {{%r[0-9]+}}, %envreg5;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_envreg_05(out: *mut u32) {
    unsafe { *out = envreg::<5>() }
}

// CHECK-LABEL: .entry sreg_envreg_06(
// CHECK: mov.b32 {{%r[0-9]+}}, %envreg6;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_envreg_06(out: *mut u32) {
    unsafe { *out = envreg::<6>() }
}

// CHECK-LABEL: .entry sreg_envreg_07(
// CHECK: mov.b32 {{%r[0-9]+}}, %envreg7;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_envreg_07(out: *mut u32) {
    unsafe { *out = envreg::<7>() }
}

// CHECK-LABEL: .entry sreg_envreg_08(
// CHECK: mov.b32 {{%r[0-9]+}}, %envreg8;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_envreg_08(out: *mut u32) {
    unsafe { *out = envreg::<8>() }
}

// CHECK-LABEL: .entry sreg_envreg_09(
// CHECK: mov.b32 {{%r[0-9]+}}, %envreg9;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_envreg_09(out: *mut u32) {
    unsafe { *out = envreg::<9>() }
}

// CHECK-LABEL: .entry sreg_envreg_10(
// CHECK: mov.b32 {{%r[0-9]+}}, %envreg10;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_envreg_10(out: *mut u32) {
    unsafe { *out = envreg::<10>() }
}

// CHECK-LABEL: .entry sreg_envreg_11(
// CHECK: mov.b32 {{%r[0-9]+}}, %envreg11;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_envreg_11(out: *mut u32) {
    unsafe { *out = envreg::<11>() }
}

// CHECK-LABEL: .entry sreg_envreg_12(
// CHECK: mov.b32 {{%r[0-9]+}}, %envreg12;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_envreg_12(out: *mut u32) {
    unsafe { *out = envreg::<12>() }
}

// CHECK-LABEL: .entry sreg_envreg_13(
// CHECK: mov.b32 {{%r[0-9]+}}, %envreg13;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_envreg_13(out: *mut u32) {
    unsafe { *out = envreg::<13>() }
}

// CHECK-LABEL: .entry sreg_envreg_14(
// CHECK: mov.b32 {{%r[0-9]+}}, %envreg14;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_envreg_14(out: *mut u32) {
    unsafe { *out = envreg::<14>() }
}

// CHECK-LABEL: .entry sreg_envreg_15(
// CHECK: mov.b32 {{%r[0-9]+}}, %envreg15;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_envreg_15(out: *mut u32) {
    unsafe { *out = envreg::<15>() }
}

// CHECK-LABEL: .entry sreg_envreg_16(
// CHECK: mov.b32 {{%r[0-9]+}}, %envreg16;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_envreg_16(out: *mut u32) {
    unsafe { *out = envreg::<16>() }
}

// CHECK-LABEL: .entry sreg_envreg_17(
// CHECK: mov.b32 {{%r[0-9]+}}, %envreg17;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_envreg_17(out: *mut u32) {
    unsafe { *out = envreg::<17>() }
}

// CHECK-LABEL: .entry sreg_envreg_18(
// CHECK: mov.b32 {{%r[0-9]+}}, %envreg18;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_envreg_18(out: *mut u32) {
    unsafe { *out = envreg::<18>() }
}

// CHECK-LABEL: .entry sreg_envreg_19(
// CHECK: mov.b32 {{%r[0-9]+}}, %envreg19;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_envreg_19(out: *mut u32) {
    unsafe { *out = envreg::<19>() }
}

// CHECK-LABEL: .entry sreg_envreg_20(
// CHECK: mov.b32 {{%r[0-9]+}}, %envreg20;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_envreg_20(out: *mut u32) {
    unsafe { *out = envreg::<20>() }
}

// CHECK-LABEL: .entry sreg_envreg_21(
// CHECK: mov.b32 {{%r[0-9]+}}, %envreg21;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_envreg_21(out: *mut u32) {
    unsafe { *out = envreg::<21>() }
}

// CHECK-LABEL: .entry sreg_envreg_22(
// CHECK: mov.b32 {{%r[0-9]+}}, %envreg22;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_envreg_22(out: *mut u32) {
    unsafe { *out = envreg::<22>() }
}

// CHECK-LABEL: .entry sreg_envreg_23(
// CHECK: mov.b32 {{%r[0-9]+}}, %envreg23;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_envreg_23(out: *mut u32) {
    unsafe { *out = envreg::<23>() }
}

// CHECK-LABEL: .entry sreg_envreg_24(
// CHECK: mov.b32 {{%r[0-9]+}}, %envreg24;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_envreg_24(out: *mut u32) {
    unsafe { *out = envreg::<24>() }
}

// CHECK-LABEL: .entry sreg_envreg_25(
// CHECK: mov.b32 {{%r[0-9]+}}, %envreg25;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_envreg_25(out: *mut u32) {
    unsafe { *out = envreg::<25>() }
}

// CHECK-LABEL: .entry sreg_envreg_26(
// CHECK: mov.b32 {{%r[0-9]+}}, %envreg26;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_envreg_26(out: *mut u32) {
    unsafe { *out = envreg::<26>() }
}

// CHECK-LABEL: .entry sreg_envreg_27(
// CHECK: mov.b32 {{%r[0-9]+}}, %envreg27;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_envreg_27(out: *mut u32) {
    unsafe { *out = envreg::<27>() }
}

// CHECK-LABEL: .entry sreg_envreg_28(
// CHECK: mov.b32 {{%r[0-9]+}}, %envreg28;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_envreg_28(out: *mut u32) {
    unsafe { *out = envreg::<28>() }
}

// CHECK-LABEL: .entry sreg_envreg_29(
// CHECK: mov.b32 {{%r[0-9]+}}, %envreg29;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_envreg_29(out: *mut u32) {
    unsafe { *out = envreg::<29>() }
}

// CHECK-LABEL: .entry sreg_envreg_30(
// CHECK: mov.b32 {{%r[0-9]+}}, %envreg30;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_envreg_30(out: *mut u32) {
    unsafe { *out = envreg::<30>() }
}

// CHECK-LABEL: .entry sreg_envreg_31(
// CHECK: mov.b32 {{%r[0-9]+}}, %envreg31;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_envreg_31(out: *mut u32) {
    unsafe { *out = envreg::<31>() }
}

// CHECK-LABEL: .entry sreg_globaltimer(
// CHECK: mov.u64 {{%rd[0-9]+}}, %globaltimer;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_globaltimer(out: *mut u64) {
    unsafe { *out = globaltimer() }
}

// CHECK-LABEL: .entry sreg_globaltimer_lo(
// CHECK: mov.u32 {{%r[0-9]+}}, %globaltimer_lo;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_globaltimer_lo(out: *mut u32) {
    unsafe { *out = globaltimer_lo() }
}

// CHECK-LABEL: .entry sreg_gridid(
// CHECK: mov.u32 {{%r[0-9]+}}, %gridid;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_gridid(out: *mut u32) {
    unsafe { *out = gridid() }
}

// CHECK-LABEL: .entry sreg_is_explicit_cluster(
// CHECK: mov.pred {{%p[0-9]+}}, %is_explicit_cluster;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_is_explicit_cluster(out: *mut bool) {
    unsafe { *out = is_explicit_cluster() }
}

// CHECK-LABEL: .entry sreg_laneid(
// CHECK: mov.u32 {{%r[0-9]+}}, %laneid;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_laneid(out: *mut u32) {
    unsafe { *out = laneid() }
}

// CHECK-LABEL: .entry sreg_lanemask_eq(
// CHECK: mov.u32 {{%r[0-9]+}}, %lanemask_eq;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_lanemask_eq(out: *mut u32) {
    unsafe { *out = lanemask_eq() }
}

// CHECK-LABEL: .entry sreg_lanemask_ge(
// CHECK: mov.u32 {{%r[0-9]+}}, %lanemask_ge;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_lanemask_ge(out: *mut u32) {
    unsafe { *out = lanemask_ge() }
}

// CHECK-LABEL: .entry sreg_lanemask_gt(
// CHECK: mov.u32 {{%r[0-9]+}}, %lanemask_gt;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_lanemask_gt(out: *mut u32) {
    unsafe { *out = lanemask_gt() }
}

// CHECK-LABEL: .entry sreg_lanemask_le(
// CHECK: mov.u32 {{%r[0-9]+}}, %lanemask_le;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_lanemask_le(out: *mut u32) {
    unsafe { *out = lanemask_le() }
}

// CHECK-LABEL: .entry sreg_lanemask_lt(
// CHECK: mov.u32 {{%r[0-9]+}}, %lanemask_lt;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_lanemask_lt(out: *mut u32) {
    unsafe { *out = lanemask_lt() }
}

// CHECK-LABEL: .entry sreg_nclusterid_x(
// CHECK: mov.u32 {{%r[0-9]+}}, %nclusterid.x;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_nclusterid_x(out: *mut u32) {
    unsafe { *out = nclusterid_x() }
}

// CHECK-LABEL: .entry sreg_nclusterid_y(
// CHECK: mov.u32 {{%r[0-9]+}}, %nclusterid.y;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_nclusterid_y(out: *mut u32) {
    unsafe { *out = nclusterid_y() }
}

// CHECK-LABEL: .entry sreg_nclusterid_z(
// CHECK: mov.u32 {{%r[0-9]+}}, %nclusterid.z;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_nclusterid_z(out: *mut u32) {
    unsafe { *out = nclusterid_z() }
}

// CHECK-LABEL: .entry sreg_nctaid_x(
// CHECK: mov.u32 {{%r[0-9]+}}, %nctaid.x;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_nctaid_x(out: *mut u32) {
    unsafe { *out = nctaid_x() }
}

// CHECK-LABEL: .entry sreg_nctaid_y(
// CHECK: mov.u32 {{%r[0-9]+}}, %nctaid.y;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_nctaid_y(out: *mut u32) {
    unsafe { *out = nctaid_y() }
}

// CHECK-LABEL: .entry sreg_nctaid_z(
// CHECK: mov.u32 {{%r[0-9]+}}, %nctaid.z;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_nctaid_z(out: *mut u32) {
    unsafe { *out = nctaid_z() }
}

// CHECK-LABEL: .entry sreg_nsmid(
// CHECK: mov.u32 {{%r[0-9]+}}, %nsmid;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_nsmid(out: *mut u32) {
    unsafe { *out = nsmid() }
}

// CHECK-LABEL: .entry sreg_ntid_x(
// CHECK: mov.u32 {{%r[0-9]+}}, %ntid.x;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_ntid_x(out: *mut u32) {
    unsafe { *out = ntid_x() }
}

// CHECK-LABEL: .entry sreg_ntid_y(
// CHECK: mov.u32 {{%r[0-9]+}}, %ntid.y;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_ntid_y(out: *mut u32) {
    unsafe { *out = ntid_y() }
}

// CHECK-LABEL: .entry sreg_ntid_z(
// CHECK: mov.u32 {{%r[0-9]+}}, %ntid.z;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_ntid_z(out: *mut u32) {
    unsafe { *out = ntid_z() }
}

// CHECK-LABEL: .entry sreg_nwarpid(
// CHECK: mov.u32 {{%r[0-9]+}}, %nwarpid;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_nwarpid(out: *mut u32) {
    unsafe { *out = nwarpid() }
}

// CHECK-LABEL: .entry sreg_pm_00(
// CHECK: mov.u32 {{%r[0-9]+}}, %pm0;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_pm_00(out: *mut u32) {
    unsafe { *out = pm::<0>() }
}

// CHECK-LABEL: .entry sreg_pm_01(
// CHECK: mov.u32 {{%r[0-9]+}}, %pm1;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_pm_01(out: *mut u32) {
    unsafe { *out = pm::<1>() }
}

// CHECK-LABEL: .entry sreg_pm_02(
// CHECK: mov.u32 {{%r[0-9]+}}, %pm2;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_pm_02(out: *mut u32) {
    unsafe { *out = pm::<2>() }
}

// CHECK-LABEL: .entry sreg_pm_03(
// CHECK: mov.u32 {{%r[0-9]+}}, %pm3;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_pm_03(out: *mut u32) {
    unsafe { *out = pm::<3>() }
}

// CHECK-LABEL: .entry sreg_reserved_smem_offset_00(
// CHECK: mov.u32 {{%r[0-9]+}}, %reserved_smem_offset_0;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_reserved_smem_offset_00(out: *mut u32) {
    unsafe { *out = reserved_smem_offset::<0>() }
}

// CHECK-LABEL: .entry sreg_reserved_smem_offset_01(
// CHECK: mov.u32 {{%r[0-9]+}}, %reserved_smem_offset_1;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_reserved_smem_offset_01(out: *mut u32) {
    unsafe { *out = reserved_smem_offset::<1>() }
}

// CHECK-LABEL: .entry sreg_reserved_smem_offset_begin(
// CHECK: mov.u32 {{%r[0-9]+}}, %reserved_smem_offset_begin;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_reserved_smem_offset_begin(out: *mut u32) {
    unsafe { *out = reserved_smem_offset_begin() }
}

// CHECK-LABEL: .entry sreg_reserved_smem_offset_cap(
// CHECK: mov.u32 {{%r[0-9]+}}, %reserved_smem_offset_cap;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_reserved_smem_offset_cap(out: *mut u32) {
    unsafe { *out = reserved_smem_offset_cap() }
}

// CHECK-LABEL: .entry sreg_reserved_smem_offset_end(
// CHECK: mov.u32 {{%r[0-9]+}}, %reserved_smem_offset_end;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_reserved_smem_offset_end(out: *mut u32) {
    unsafe { *out = reserved_smem_offset_end() }
}

// CHECK-LABEL: .entry sreg_smid(
// CHECK: mov.u32 {{%r[0-9]+}}, %smid;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_smid(out: *mut u32) {
    unsafe { *out = smid() }
}

// CHECK-LABEL: .entry sreg_tid_x(
// CHECK: mov.u32 {{%r[0-9]+}}, %tid.x;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_tid_x(out: *mut u32) {
    unsafe { *out = tid_x() }
}

// CHECK-LABEL: .entry sreg_tid_y(
// CHECK: mov.u32 {{%r[0-9]+}}, %tid.y;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_tid_y(out: *mut u32) {
    unsafe { *out = tid_y() }
}

// CHECK-LABEL: .entry sreg_tid_z(
// CHECK: mov.u32 {{%r[0-9]+}}, %tid.z;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_tid_z(out: *mut u32) {
    unsafe { *out = tid_z() }
}

// CHECK-LABEL: .entry sreg_total_smem_size(
// CHECK: mov.u32 {{%r[0-9]+}}, %total_smem_size;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_total_smem_size(out: *mut u32) {
    unsafe { *out = total_smem_size() }
}

// CHECK-LABEL: .entry sreg_warpid(
// CHECK: mov.u32 {{%r[0-9]+}}, %warpid;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_warpid(out: *mut u32) {
    unsafe { *out = warpid() }
}

// `WARP_SZ` is a compile-time constant.
// CHECK-LABEL: .entry sreg_warpsize(
// CHECK: st.global.b32 [{{%rd[0-9]+}}], 32;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sreg_warpsize(out: *mut u32) {
    unsafe { *out = warpsize() }
}
